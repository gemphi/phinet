//! Relation operators: one operator for every kind of relation, learned
//! together with the items it relates, and the laws those kinds obey.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c). This module knows no words. An item is a row of a table of
//! functions; a kind is a row of the same table; a relation is a triple
//! `(about, kind, other)` of row indices. One [`Joint`] operator maps
//! `unit(about + kind)` to `other` for every kind at once — X ⇒ Y under c,
//! with c the kind — the weather models' way of learning a map rather than
//! storing each answer (phiano docs/91 §2.1).
//!
//! Measured (phiano docs/92): a shared operator learns each kind's *answer
//! type* but not the answer — held-out rows score at the prior. Inference
//! has to come from a law the operator is made to obey, which is what
//! [`laws`] counts in the told rows and [`closure`] trains.

pub mod closure;
pub mod inherit;
pub mod laws;
pub mod score;
pub mod structure;
#[cfg(test)]
mod tests;

pub use closure::{closure_epoch, closure_pairs};
pub use inherit::{inherit_epoch, score_inherit, Inherit};
pub use laws::{find_inverses, find_laws, implied, Inverse, Law};
pub use score::{chance, prior, score, two_hop, Score};
pub use structure::{laws_epoch, score_laws, Converse};

use crate::joint::IndexExample;

/// A told relation, by row index into the joint's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Triple {
    pub about: usize,
    pub kind: usize,
    pub other: usize,
}

/// The joint's example for a triple: context `(kind, about)`, target `other`.
pub fn example(t: &Triple) -> IndexExample {
    IndexExample { ctx: vec![(t.kind, 1.0), (t.about, 1.0)], target: t.other }
}

/// Every item named as an answer (the ranking pool), and per kind the items
/// named under it.
pub fn pools(triples: &[Triple]) -> (Vec<usize>, std::collections::HashMap<usize, Vec<usize>>) {
    let mut pool: Vec<usize> = triples.iter().map(|t| t.other).collect();
    pool.sort_unstable();
    pool.dedup();
    let mut by_kind: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    for t in triples {
        let v = by_kind.entry(t.kind).or_default();
        if !v.contains(&t.other) {
            v.push(t.other);
        }
    }
    (pool, by_kind)
}
