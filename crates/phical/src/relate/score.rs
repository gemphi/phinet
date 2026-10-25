//! Reading the operator back, and the bar it has to clear.
//!
//! A held-out triple can rank well without inference: a kind whose answers
//! are five tools scores H(5)/5 by ranking the five first. The [`prior`] —
//! answers ranked by how often the kind named them in training — is that
//! bar, and `within` (rank among the kind's own answers) removes the type.

use super::Triple;
use crate::joint::Joint;
use crate::{c64, cosine, Function};
use std::collections::HashMap;

/// Overall and within-kind MRR, hit@5, and per kind `(kind, n, mrr, within)`.
#[derive(Debug, Clone, Default)]
pub struct Score {
    pub mrr: f64,
    pub within: f64,
    pub hit5: f64,
    pub per_kind: Vec<(usize, usize, f64, f64)>,
}

/// MRR of a uniformly random ranking over `n`.
pub fn chance(n: usize) -> f64 {
    (1..=n).map(|k| 1.0 / k as f64).sum::<f64>() / n.max(1) as f64
}

fn rank(pool: &[usize], truth: usize, s: &dyn Fn(usize) -> f64) -> f64 {
    let t = s(truth);
    let (mut above, mut tied) = (0usize, 0usize);
    for &w in pool.iter().filter(|&&w| w != truth) {
        let x = s(w);
        above += (x > t) as usize;
        tied += (x == t) as usize;
    }
    1.0 + above as f64 + tied as f64 / 2.0
}

/// Tallies `rows` with `score(i, item)` for row `i`.
pub fn tally(rows: &[Triple], pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>, score: impl Fn(usize, usize) -> f64) -> Score {
    let mut per: Vec<(usize, usize, f64, f64)> = Vec::new();
    let (mut mrr, mut within, mut hit5) = (0.0, 0.0, 0usize);
    for (i, r) in rows.iter().enumerate() {
        let s = |w: usize| score(i, w);
        let all = rank(pool, r.other, &s);
        let own = rank(by_kind.get(&r.kind).map_or(pool, |v| v.as_slice()), r.other, &s);
        mrr += 1.0 / all;
        within += 1.0 / own;
        hit5 += (all <= 5.0) as usize;
        match per.iter_mut().find(|p| p.0 == r.kind) {
            Some(p) => {
                p.1 += 1;
                p.2 += 1.0 / all;
                p.3 += 1.0 / own;
            }
            None => per.push((r.kind, 1, 1.0 / all, 1.0 / own)),
        }
    }
    for p in per.iter_mut() {
        p.2 /= p.1 as f64;
        p.3 /= p.1 as f64;
    }
    let n = rows.len().max(1) as f64;
    Score { mrr: mrr / n, within: within / n, hit5: hit5 as f64 / n, per_kind: per }
}

fn context(table: &[Function], about: usize, kind: usize) -> Function {
    let mut f: Function = table[about].iter().zip(&table[kind]).map(|(x, y)| x + y).collect();
    crate::normalise(&mut f);
    f
}

/// The operator's reading of each row: cosine of its prediction to each item.
pub fn score(joint: &mut Joint, rows: &[Triple], pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>) -> Score {
    let table = joint.functions();
    let inputs: Vec<Function> = rows.iter().map(|r| context(&table, r.about, r.kind)).collect();
    let preds = joint.fno.apply(&inputs);
    tally(rows, pool, by_kind, |i, w| cosine(&preds[i], &table[w]))
}

/// The bar: each answer scored by how often the row's kind named it in `told`.
pub fn prior(told: &[Triple], rows: &[Triple], pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>) -> Score {
    let mut n: HashMap<(usize, usize), f64> = HashMap::new();
    for t in told {
        *n.entry((t.kind, t.other)).or_default() += 1.0;
    }
    tally(rows, pool, by_kind, |i, w| n.get(&(rows[i].kind, w)).copied().unwrap_or(0.0))
}

/// Two hops through the items `about` was told about — through every told
/// neighbour, or only those joined by a law's `k₁` when `via` is given
/// (`via(k2)` lists the `k₁` kinds). Predictions summed, each unit length.
pub fn two_hop(
    joint: &mut Joint,
    told: &[Triple],
    rows: &[Triple],
    pool: &[usize],
    by_kind: &HashMap<usize, Vec<usize>>,
    via: Option<&dyn Fn(usize) -> Vec<usize>>,
) -> Score {
    let table = joint.functions();
    let s = table.first().map_or(0, Vec::len);
    let mut preds: Vec<Function> = Vec::with_capacity(rows.len());
    for r in rows {
        let allowed = via.map(|f| f(r.kind));
        let hops: Vec<Function> = told
            .iter()
            .filter(|t| t.about == r.about && t.other != r.about)
            .filter(|t| allowed.as_ref().is_none_or(|a| a.contains(&t.kind)))
            .map(|t| context(&table, t.other, r.kind))
            .collect();
        let hops = match hops.is_empty() {
            true => vec![context(&table, r.about, r.kind)],
            false => hops,
        };
        let mut sum = vec![c64::new(0.0, 0.0); s];
        for y in joint.fno.apply(&hops) {
            let n = y.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt().max(1e-12);
            for (a, z) in sum.iter_mut().zip(&y) {
                *a += z / n;
            }
        }
        preds.push(sum);
    }
    tally(rows, pool, by_kind, |i, w| cosine(&preds[i], &table[w]))
}
