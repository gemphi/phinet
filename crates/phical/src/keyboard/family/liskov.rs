//! The Liskov family — the fifth block of the keyboard, indices 40 to 47.
//!
//! Eight Operators over `phical::liskov`, Barbara Liskov's (then Huberman's)
//! forcing-tree endgame read for words: a move is kept when the position it
//! reaches is *better*, refuted when a reply makes it *worse*, and `loss` is
//! what training minimises so `better` on held-out text agrees with the
//! corpus. Six are lifted to the declared ABI — the four top-level comparisons
//! `better`, `worse`, `loss` and `forcing_choice`, and the two lexicographic
//! ones `staged::better` and `staged::worse` — and two are declared new,
//! `stage` and `measure`, each reading a field off the same `Staged` the
//! lifted pair compares. No arithmetic is repeated here: each row unpacks its
//! operands, calls the existing function, and repacks.
//!
//! **How the nine types carry a Function and a Staged.** A **Function** is a
//! single-phasor slice `&[x]`, the one-channel reading the Circle family lifts
//! its functions at (see `circle::twins`, which lifts `liskov::better` the
//! same way): the operand's phasor *is* the one-channel function. A **Staged**
//! — Huberman's coarse stage and fine measure, compared lexicographically and
//! never summed — rides one [`Operand::Phasor`]: its stage on the imaginary
//! axis (`im = stage`, a non-negative integer) and its measure on the real
//! axis (`re = measure`), so one phasor carries the pair the staged order
//! reads. A scalar result — a `better` margin, a `loss`, a chosen index —
//! comes back on the real axis as `Phasor(v + 0i)`, and a yes-or-no answer
//! rides that same axis as `1.0` or `0.0`, there being no boolean among the
//! nine types.
//!
//! The rows, in index order: 40 better, 41 worse, 42 loss, 43 forcing_choice,
//! 44 staged_better, 45 staged_worse, 46 stage, 47 measure.

use crate::c64;
use crate::{liskov, Function};

use super::super::arena::{Arena, Fault};
use super::super::operator::{Family, Inverse, Operator};
use super::super::types::{Operand, Type};

/// The hinge margin `loss` is read at. Requirement 6's ABI carries three
/// operands at most, so `loss(pred, target, decoy, margin)`'s fourth argument
/// is this declared constant rather than an operand — the standard unit hinge
/// margin. The arithmetic `liskov::loss` performs is untouched; only the
/// margin is fixed.
pub const MARGIN: f64 = 1.0;

/// The eight Liskov Operators in index order, 40 to 47.
pub fn table() -> Vec<Operator> {
    vec![
        op(40, "better", &[Type::Phasor; 3], Type::Phasor, better),
        op(41, "worse", &[Type::Phasor; 3], Type::Phasor, worse),
        op(42, "loss", &[Type::Phasor; 3], Type::Phasor, loss),
        op(43, "forcing_choice", &[Type::Phasor; 3], Type::Phasor, forcing_choice),
        op(44, "staged_better", &[Type::Phasor; 2], Type::Phasor, staged_better),
        op(45, "staged_worse", &[Type::Phasor; 2], Type::Phasor, staged_worse),
        op(46, "stage", &[Type::Phasor], Type::Class, stage),
        op(47, "measure", &[Type::Phasor], Type::Phasor, measure),
    ]
}

/// One Liskov row. The arity is the operand list's length, so the two cannot
/// drift apart; no row is idempotent — a comparison's result (a scalar on the
/// real axis) is never the staged pair it compared, and `stage`/`measure`
/// read a field rather than return the whole — none has an inverse (every
/// comparison is many-to-one) and none corresponds to a spectral gate.
fn op(
    index: u8,
    name: &'static str,
    operands: &'static [Type],
    result: Type,
    compute: fn(&[Operand], &mut Arena) -> Result<Operand, Fault>,
) -> Operator {
    Operator {
        index,
        name,
        family: Family::Liskov,
        arity: operands.len() as u8,
        operands,
        result,
        idempotent: false,
        inverse: Inverse::None,
        gate: None,
        compute,
    }
}

/// The one-channel Function a phasor operand names: the phasor itself, as a
/// length-1 slice — the reading `circle::twins` lifts `liskov::better` at.
fn func(a: &Operand) -> Function {
    match a {
        Operand::Phasor(c) => vec![*c],
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The [`Staged`](liskov::staged::Staged) a phasor operand carries: its stage
/// on the imaginary axis (read as a non-negative integer) and its measure on
/// the real axis.
fn staged(a: &Operand) -> liskov::staged::Staged {
    match a {
        Operand::Phasor(c) => {
            liskov::staged::Staged { stage: c.im.max(0.0) as u64, measure: c.re }
        }
        _ => unreachable!("apply checked the operand type"),
    }
}

/// Repack a scalar (a margin, a loss, a chosen index) on the real axis.
fn scalar(v: f64) -> Operand {
    Operand::Phasor(c64::new(v, 0.0))
}

/// A yes-or-no answer on that same real axis: 1.0 for yes, 0.0 for no.
fn truth(held: bool) -> Operand {
    scalar(if held { 1.0 } else { 0.0 })
}

// --- the six lifts: each calls the existing `phical::liskov` function over
// the one-channel Functions (or the Staged pair) its operands name. ---

/// `liskov::better`: how much the move `a` beats the decoy `b` under the
/// prediction `pred`, the difference of their cosines. Positive means `a`
/// wins; antisymmetric in `a` and `b`.
fn better(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(liskov::better(&func(&a[0]), &func(&a[1]), &func(&a[2]))))
}

/// `liskov::worse`: how much choosing `a` is refuted by what follows it, the
/// move `a` read against the best of the continuations named *after* it. The
/// one continuation an ABI call names is the move itself — the faithful
/// minimal pool, the reply `a` is its own refutation against — so the third
/// operand is `after`, the prediction once `a` is in context, and the
/// continuation pool is `[a]`.
fn worse(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (pred, mv, after) = (func(&a[0]), func(&a[1]), func(&a[2]));
    Ok(scalar(liskov::worse(&pred, &mv, &after, std::slice::from_ref(&mv))))
}

/// `liskov::loss`: the hinge over `better`, `max(0, MARGIN − better(pred,
/// target, decoy))`, at the declared [`MARGIN`]. Zero once the target already
/// beats the decoy by the margin.
fn loss(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(liskov::loss(&func(&a[0]), &func(&a[1]), &func(&a[2]), MARGIN)))
}

/// `liskov::forcing_choice` read greedily over two candidates: of `a` and
/// `b`, the one that fits `pred` better, with `lambda = 0` so no refutation is
/// consulted and the choice is the nearer candidate. The result is the chosen
/// candidate's index on the real axis — `0.0` for the first, `1.0` for the
/// second — ties going to the first, exactly as `forcing_choice` resolves
/// them.
fn forcing_choice(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let pred = func(&a[0]);
    let candidates = [(0usize, func(&a[1])), (1usize, func(&a[2]))];
    let chosen = liskov::forcing_choice(&pred, &candidates, &[], 0.0, |_| None)
        .map_or(0.0, |(i, _)| i as f64);
    Ok(scalar(chosen))
}

/// `liskov::staged::better`: whether the second Staged is better than the
/// first — a higher stage, or the same stage with a smaller measure. The
/// comparison is lexicographic, the stage outranking any measure.
fn staged_better(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(truth(liskov::staged::better(staged(&a[0]), staged(&a[1]))))
}

/// `liskov::staged::worse`: whether the second Staged is worse than the first
/// — a lower stage, or the same stage with a larger measure. Read backwards,
/// this is `staged_better` with the operands swapped.
fn staged_worse(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(truth(liskov::staged::worse(staged(&a[0]), staged(&a[1]))))
}

// --- the two new rows: each reads one field off the same Staged the lifted
// staged pair compares, so the keyboard can name a position's stage and its
// measure apart. No arithmetic is added — a field read, not a computation. ---

/// New: a Staged's coarse stage, the discrete subgoal index
/// `staged_better`/`staged_worse` compare first, read back as a class id. The
/// phasor's imaginary axis carries it; this reads it out.
fn stage(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(Operand::Class(staged(&a[0]).stage as u16))
}

/// New: a Staged's fine measure, the within-stage progress (less is nearer)
/// `staged_better`/`staged_worse` compare once the stages tie, read back on a
/// phasor's real axis. The reading and the carry are the same axis the staged
/// pair reads the measure off, so `measure` and the comparison agree.
fn measure(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(staged(&a[0]).measure))
}

#[cfg(test)]
mod tests;
