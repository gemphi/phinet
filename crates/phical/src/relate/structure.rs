//! Declared laws as structure: inheritance and transitivity ([`Inherit`])
//! and converses ([`Converse`]), in the forward pass of training and of
//! reading alike (phiano docs/97 §1, docs/101).
//!
//! A converse pair `has ⇄ part-of` says `w has v ⇔ v part-of w`. As
//! structure, the score of `w · k · v` is read both ways:
//! `cos(F(w+k), v) + μ · cos(F(v+k'), w)` — what `v` says under the
//! converse kind is part of the answer, for every candidate `v`. Nothing is
//! added to the data; the one operator is read in both directions.

use super::inherit::{context, predictions, Inherit};
use super::score::{tally, Score};
use super::Triple;
use crate::joint::Joint;
use crate::{cosine, mix64, Adam, Fno, Function};
use std::collections::HashMap;

/// Declared converse kinds (both directions) and the weight μ of the
/// converse reading.
pub struct Converse {
    pub of: HashMap<usize, usize>,
    pub mu: f64,
}

impl Converse {
    pub fn none() -> Self {
        Self { of: HashMap::new(), mu: 0.0 }
    }

    /// `pairs` of kinds `(k, k')`, each read as the other's converse.
    pub fn declared(pairs: &[(usize, usize)], mu: f64) -> Self {
        let mut of = HashMap::new();
        for &(a, b) in pairs {
            of.insert(a, b);
            of.insert(b, a);
        }
        Self { of, mu }
    }
}

/// One epoch with the declared laws in the forward pass; hinge against a
/// decoy. Rows whose kind has a converse are batched together, after the
/// rest, so each batch reads one way or both.
#[allow(clippy::too_many_arguments)]
pub fn laws_epoch(joint: &mut Joint, told: &[Triple], inh: Option<&Inherit>, conv: &Converse, pool: &[usize], adam_op: &mut Adam, adam_table: &mut Adam, seed: u64) -> f64 {
    let params = joint.fno.params();
    let table = joint.table;
    let margin = joint.fno.config.margin;
    let mut order: Vec<usize> = (0..told.len()).collect();
    order.sort_by_key(|i| mix64(*i as u64 ^ seed));
    let (one, both): (Vec<usize>, Vec<usize>) = order.into_iter().partition(|&i| !conv.of.contains_key(&told[i].kind));
    let (mut total, mut n) = (0.0, 0usize);
    for chunk in one.chunks(32).chain(both.chunks(32)) {
        joint.fno.reset();
        let rows: Vec<&Triple> = chunk.iter().map(|&i| &told[i]).collect();
        let w: Vec<usize> = rows.iter().map(|t| t.about).collect();
        let k: Vec<usize> = rows.iter().map(|t| t.kind).collect();
        let target: Vec<usize> = rows.iter().map(|t| t.other).collect();
        let decoy: Vec<usize> = chunk.iter().enumerate().map(|(j, &i)| phistd::decoy(pool, told[i].other, mix64(seed ^ ((i as u64) << 20) ^ j as u64))).collect();
        let mut read = |ids: &[usize], kinds: &[usize]| {
            let t = &mut joint.fno.tape;
            let (a, b) = (t.gather(table, ids), t.gather(table, kinds));
            let s = t.add(a, b);
            let x = t.unit(s);
            joint.fno.forward(x)
        };
        let y = read(&w, &k);
        let yp = inh.map(|inh| {
            let p: Vec<usize> = chunk
                .iter()
                .zip(&rows)
                .map(|(&i, t)| {
                    let ps = inh.of(t);
                    if ps.is_empty() { t.about } else { ps[(mix64(seed ^ i as u64) % ps.len() as u64) as usize] }
                })
                .collect();
            (read(&p, &k), inh.lambda)
        });
        let back = conv.of.contains_key(&k[0]).then(|| {
            let kc: Vec<usize> = k.iter().map(|x| conv.of[x]).collect();
            (read(&target, &kc), read(&decoy, &kc))
        });
        let t = &mut joint.fno.tape;
        let y = match yp {
            Some((yp, lambda)) => {
                let yp = t.scale(yp, lambda);
                t.add(y, yp)
            }
            None => y,
        };
        let (tv, dv) = (t.gather(table, &target), t.gather(table, &decoy));
        let (mut sp, mut sn) = (t.cosine(y, tv), t.cosine(y, dv));
        if let Some((bt, bd)) = back {
            let wv = t.gather(table, &w);
            let (cp, cn) = (t.cosine(bt, wv), t.cosine(bd, wv));
            let (cp, cn) = (t.scale(cp, conv.mu), t.scale(cn, conv.mu));
            sp = t.add(sp, cp);
            sn = t.add(sn, cn);
        }
        let loss = t.hinge(sp, sn, margin);
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

/// The reading under the declared laws: inheritance in the prediction, and
/// for a kind with a converse, every candidate's converse reading added.
pub fn score_laws(joint: &mut Joint, rows: &[Triple], inh: Option<&Inherit>, conv: &Converse, pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>) -> Score {
    let table = joint.functions();
    let preds = predictions(joint, &table, rows, inh);
    let mut back: HashMap<usize, HashMap<usize, Function>> = HashMap::new();
    for r in rows {
        let Some(&kc) = conv.of.get(&r.kind) else { continue };
        if back.contains_key(&kc) {
            continue;
        }
        let ys = joint.fno.apply(&pool.iter().map(|&v| context(&table, v, kc)).collect::<Vec<_>>());
        back.insert(kc, pool.iter().copied().zip(ys).collect());
    }
    tally(rows, pool, by_kind, |i, v| {
        let r = &rows[i];
        let own = cosine(&preds[i], &table[v]);
        match conv.of.get(&r.kind).and_then(|kc| back[kc].get(&v)) {
            Some(b) => own + conv.mu * cosine(b, &table[r.about]),
            None => own,
        }
    })
}
