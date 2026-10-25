//! Inheritance as structure: a word's answer under an inheriting kind is its
//! own reading plus its kinds' readings — computed that way in training and
//! in reading, not rewarded (phiano docs/97 §1).
//!
//! `has(cat) = own_has(cat) ⊕ λ · has(mammal)`: the operator's prediction
//! for `cat · has` is added to its prediction for each told `cat is-a p`
//! parent under `has`, so what a mammal has reaches a cat by construction and
//! the operator only has to learn what a cat has *of its own*. A word with no
//! told parent reads exactly as before (its own prediction, scaled — cosine
//! does not see the scale).

use super::score::{tally, Score};
use super::Triple;
use crate::joint::Joint;
use crate::{c64, cosine, Adam, Function};
use std::collections::HashMap;

/// Which kinds inherit, along which kind, and each item's told parents.
/// Inheritance runs along `is-a`; a transitive kind runs along itself
/// (`is-a(cat) = own ⊕ λ · is-a(mammal)`), so both are one structure.
pub struct Inherit {
    /// Kind ⇒ the kind whose told rows give the parents.
    pub via: HashMap<usize, usize>,
    /// `(via kind, item)` ⇒ the item's told parents under that kind.
    pub parents: HashMap<(usize, usize), Vec<usize>>,
    /// Weight of the inherited reading.
    pub lambda: f64,
}

impl Inherit {
    /// `kinds` inherit along the told `is-a` rows (kind row `isa`).
    pub fn from_told(told: &[Triple], isa: usize, kinds: Vec<usize>, lambda: f64) -> Self {
        let mut me = Self { via: HashMap::new(), parents: HashMap::new(), lambda };
        me.add(told, isa, &kinds);
        me
    }

    /// Declares `kinds` transitive: each also reads its own told parents.
    pub fn with_transitive(mut self, told: &[Triple], kinds: &[usize]) -> Self {
        for &k in kinds {
            self.add(told, k, &[k]);
        }
        self
    }

    fn add(&mut self, told: &[Triple], via: usize, kinds: &[usize]) {
        for &k in kinds {
            self.via.insert(k, via);
        }
        for t in told.iter().filter(|t| t.kind == via && t.about != t.other) {
            let v = self.parents.entry((via, t.about)).or_default();
            if !v.contains(&t.other) {
                v.push(t.other);
            }
        }
    }

    /// The parents a row's reading adds: none for a kind that does not
    /// inherit or a word with no told parent.
    pub fn of(&self, t: &Triple) -> &[usize] {
        match self.via.get(&t.kind) {
            Some(&v) => self.parents.get(&(v, t.about)).map_or(&[], Vec::as_slice),
            None => &[],
        }
    }
}

/// One epoch with inheritance in the forward pass; hinge against a decoy.
/// Each example adds one of its parents (drawn by `seed`), or itself.
pub fn inherit_epoch(joint: &mut Joint, told: &[Triple], inh: &Inherit, pool: &[usize], adam_op: &mut Adam, adam_table: &mut Adam, seed: u64) -> f64 {
    super::structure::laws_epoch(joint, told, Some(inh), &super::structure::Converse::none(), pool, adam_op, adam_table, seed)
}

pub(crate) fn context(table: &[Function], about: usize, kind: usize) -> Function {
    let mut f: Function = table[about].iter().zip(&table[kind]).map(|(x, y)| x + y).collect();
    crate::normalise(&mut f);
    f
}

/// Each row's prediction: own reading plus λ times the mean of its
/// parents' readings under the row's kind (own only without `inh`).
pub(crate) fn predictions(joint: &mut Joint, table: &[Function], rows: &[Triple], inh: Option<&Inherit>) -> Vec<Function> {
    let own: Vec<Function> = joint.fno.apply(&rows.iter().map(|r| context(table, r.about, r.kind)).collect::<Vec<_>>());
    let Some(inh) = inh else { return own };
    let mut preds = Vec::with_capacity(rows.len());
    for (r, y) in rows.iter().zip(own) {
        let ps = inh.of(r);
        if ps.is_empty() {
            preds.push(y);
            continue;
        }
        let ys = joint.fno.apply(&ps.iter().map(|&p| context(table, p, r.kind)).collect::<Vec<_>>());
        let m = inh.lambda / ps.len() as f64;
        preds.push(y.iter().enumerate().map(|(i, z)| z + ys.iter().map(|q| q[i]).sum::<c64>() * m).collect());
    }
    preds
}

/// The reading with inheritance: own prediction plus λ times the mean of
/// the parents' predictions under the row's kind.
pub fn score_inherit(joint: &mut Joint, rows: &[Triple], inh: &Inherit, pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>) -> Score {
    let table = joint.functions();
    let preds = predictions(joint, &table, rows, Some(inh));
    tally(rows, pool, by_kind, |i, w| cosine(&preds[i], &table[w]))
}
