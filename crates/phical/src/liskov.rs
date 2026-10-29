//! The three operators: **better**, **worse**, **loss** — Liskov's endgame,
//! read for words.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c). Barbara Liskov's 1968 thesis (then Huberman, Stanford,
//! under McCarthy) played chess endgames with no evaluation number at all:
//! a *forcing tree* coordinated with two functions, `better` and `worse`. A
//! move is kept if the position it reaches is better, and refuted if some
//! reply makes it worse; the killer heuristic came out of the same program.
//! That is the right shape for a word: the operator's prediction says which
//! candidate is *better* than another under this condition, the next step
//! says whether the choice is *refuted* (a word that leads nowhere is worse
//! however well it fits), and the *loss* is what training minimises so that
//! `better` on held-out text agrees with the corpus. Hinge and softmax are
//! both `loss` written over `better`: the hinge is `max(0, margin − better)`
//! against one decoy, softmax is `−log` of `better` exponentiated over the
//! table.
//!
//! Nothing here is a number a position *has*; everything is a comparison a
//! pair of functions makes under a prediction. That is the paradigm.

pub mod staged;

use crate::{c64, cosine, Function};

/// How much `a` is better than `b` under the prediction `pred`: the
/// difference of their cosines. Positive means `a` wins. Antisymmetric.
pub fn better(pred: &[c64], a: &[c64], b: &[c64]) -> f64 {
    cosine(pred, a) - cosine(pred, b)
}

/// How much choosing `a` is refuted by what follows it: the gap between how
/// well `a` fits now and how well the best continuation fits *after* `a`.
/// `after` is the operator's prediction once `a` is in the context;
/// `continuations` are the functions a next word could be. Positive means
/// the best reply is weaker than the move: the move leads downhill.
pub fn worse(pred: &[c64], a: &[c64], after: &[c64], continuations: &[Function]) -> f64 {
    let now = cosine(pred, a);
    let best_reply = continuations.iter().map(|t| cosine(after, t)).fold(f64::NEG_INFINITY, f64::max);
    match best_reply.is_finite() {
        true => now - best_reply,
        false => 0.0,
    }
}

/// The hinge over `better`: `max(0, margin − better(pred, target, decoy))`.
/// Zero when the target already beats the decoy by the margin.
pub fn loss(pred: &[c64], target: &[c64], decoy: &[c64], margin: f64) -> f64 {
    (margin - better(pred, target, decoy)).max(0.0)
}

/// A forcing choice among candidates: the one whose fit now, less `lambda`
/// times its refutation, is largest. `after_of(i)` must return the operator's
/// prediction after candidate `i` is appended to the context. With
/// `lambda = 0` this is the greedy nearest candidate.
pub fn forcing_choice(
    pred: &[c64],
    candidates: &[(usize, Function)],
    continuations: &[Function],
    lambda: f64,
    mut after_of: impl FnMut(usize) -> Option<Function>,
) -> Option<(usize, f64)> {
    let mut best: Option<(usize, f64)> = None;
    for (i, f) in candidates {
        let refuted = match lambda != 0.0 {
            true => after_of(*i).map(|after| worse(pred, f, &after, continuations)).unwrap_or(0.0),
            false => 0.0,
        };
        let score = cosine(pred, f) - lambda * refuted;
        if best.is_none_or(|(_, s)| score > s) {
            best = Some((*i, score));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_random;

    #[test]
    fn better_is_antisymmetric_and_loss_is_zero_past_the_margin() {
        let p = unit_random(16, 1);
        let a = unit_random(16, 2);
        let b = unit_random(16, 3);
        assert!((better(&p, &a, &b) + better(&p, &b, &a)).abs() < 1e-12);
        assert_eq!(loss(&p, &p, &b, 0.5), 0.0f64.max(0.5 - better(&p, &p, &b)));
        assert_eq!(loss(&p, &a, &a, 0.0), 0.0);
    }

    #[test]
    fn a_move_that_leads_nowhere_is_refuted() {
        let p = unit_random(16, 4);
        let a = p.clone();
        let table = vec![unit_random(16, 5), unit_random(16, 6)];
        // After `a` the operator predicts something nothing in the table matches.
        let dead = unit_random(16, 7);
        let w = worse(&p, &a, &dead, &table);
        assert!(w > 0.0, "refutation {w}");
        // After `a` the operator predicts exactly a table entry: not refuted.
        let alive = table[0].clone();
        assert!(worse(&p, &a, &alive, &table) <= 1e-12);
    }

    #[test]
    fn forcing_prefers_the_move_with_a_future() {
        let p = unit_random(16, 8);
        let mut near = p.clone();
        near[0] += c64::new(0.05, 0.0);
        crate::normalise(&mut near);
        let far = unit_random(16, 9);
        let table = vec![unit_random(16, 10)];
        let cands = vec![(0usize, near.clone()), (1usize, far.clone())];
        // Greedy: the nearer wins.
        let (g, _) = forcing_choice(&p, &cands, &table, 0.0, |_| None).unwrap();
        assert_eq!(g, 0);
        // With refutation: the nearer leads nowhere, the farther leads to the table.
        let (f, _) = forcing_choice(&p, &cands, &table, 2.0, |i| Some(if i == 0 { unit_random(16, 11) } else { table[0].clone() })).unwrap();
        assert_eq!(f, 1);
    }
}
