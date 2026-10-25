//! Closure: make the operator obey the laws.
//!
//! For every told `w k₁ v` under a kept law `k₁ ⇒ k₂`, the operator's reading
//! of `w` under `k₂` is pulled toward its reading of `v` under `k₂` and
//! pushed from its reading of a random item — whether or not `w`'s own `k₂`
//! was told. That is how a told sibling's answer reaches a word the quest
//! never asked about (phiano docs/91 §2.3; the physics-informed operator's
//! residual loss, a law without an answer key).

use super::{Law, Triple};
use crate::joint::Joint;
use crate::{mix64, Adam};

/// `(w, v, k₂)` for every told `w k₁ v` under `laws`.
pub fn closure_pairs(told: &[Triple], laws: &[&Law]) -> Vec<(usize, usize, usize)> {
    let mut out = Vec::new();
    for law in laws {
        for t in told.iter().filter(|t| t.kind == law.k1 && t.about != t.other) {
            out.push((t.about, t.other, law.k2));
        }
    }
    out
}

/// One pass of closure, hinge scaled by `alpha`; returns the mean loss.
/// `items` are the rows a random comparison is drawn from.
pub fn closure_epoch(
    joint: &mut Joint,
    pairs: &[(usize, usize, usize)],
    items: &[usize],
    alpha: f64,
    adam_op: &mut Adam,
    adam_table: &mut Adam,
    seed: u64,
) -> f64 {
    let params = joint.fno.params();
    let table = joint.table;
    let margin = joint.fno.config.margin;
    let mut order: Vec<usize> = (0..pairs.len()).collect();
    order.sort_by_key(|i| mix64(*i as u64 ^ seed));
    let (mut total, mut n) = (0.0, 0usize);
    for chunk in order.chunks(32) {
        joint.fno.reset();
        let pick = |f: fn(&(usize, usize, usize)) -> usize| chunk.iter().map(|&i| f(&pairs[i])).collect::<Vec<usize>>();
        let (w, v, k) = (pick(|p| p.0), pick(|p| p.1), pick(|p| p.2));
        let u: Vec<usize> = chunk.iter().map(|&i| items[(mix64(seed ^ ((i as u64) << 16)) % items.len() as u64) as usize]).collect();
        let mut read = |ids: &[usize]| {
            let t = &mut joint.fno.tape;
            let (a, b) = (t.gather(table, ids), t.gather(table, &k));
            let s = t.add(a, b);
            let x = t.unit(s);
            joint.fno.forward(x)
        };
        let (yw, yv, yu) = (read(&w), read(&v), read(&u));
        let t = &mut joint.fno.tape;
        let (sp, sn) = (t.cosine(yw, yv), t.cosine(yw, yu));
        let h = t.hinge(sp, sn, margin);
        let loss = t.scale(h, alpha);
        total += t.value(loss).get(0, 0, 0).re;
        n += 1;
        t.backward(loss);
        adam_op.step(&mut joint.fno.tape, &params);
        adam_table.step(&mut joint.fno.tape, &[table]);
        joint.fno.mask_modes();
    }
    joint.fno.reset();
    total / n.max(1) as f64
}
