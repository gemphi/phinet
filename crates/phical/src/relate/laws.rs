//! Laws counted in the told triples.
//!
//! A law `k₁ ⇒ k₂` says: when `w k₁ v`, `w` and `v` give the same `k₂`
//! answer. It is not written in; it is counted. Over the told `w k₁ v` where
//! both `w` and `v` have told `k₂` answers, `agree` is how often the two
//! answer sets meet. On phiano's synthetic quest `near ⇒ is-a` holds on every
//! family; `near ⇒ used-for` does not.

use super::Triple;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Law {
    pub k1: usize,
    pub k2: usize,
    pub support: usize,
    pub agree: usize,
}

impl Law {
    pub fn confidence(&self) -> f64 {
        self.agree as f64 / self.support.max(1) as f64
    }
}

/// Every ordered kind pair with at least `min_support` testable triples,
/// strongest first.
pub fn find_laws(told: &[Triple], min_support: usize) -> Vec<Law> {
    let mut answers: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    let mut kinds: Vec<usize> = Vec::new();
    for t in told {
        answers.entry((t.about, t.kind)).or_default().insert(t.other);
        if !kinds.contains(&t.kind) {
            kinds.push(t.kind);
        }
    }
    let mut laws = Vec::new();
    for &k1 in &kinds {
        for &k2 in &kinds {
            let (mut support, mut agree) = (0, 0);
            for t in told.iter().filter(|t| t.kind == k1 && t.about != t.other) {
                if let (Some(a), Some(b)) = (answers.get(&(t.about, k2)), answers.get(&(t.other, k2))) {
                    support += 1;
                    agree += !a.is_disjoint(b) as usize;
                }
            }
            if support >= min_support {
                laws.push(Law { k1, k2, support, agree });
            }
        }
    }
    laws.sort_by(|a, b| b.confidence().partial_cmp(&a.confidence()).unwrap().then(b.support.cmp(&a.support)));
    laws
}

/// An inverse law `k ⇐⇒ k'`: when `w k v` is told, so is `v k' w`. Counted
/// over told `w k v` where `v` has any told `k'`. On the synthetic quest
/// `unlike` is its own inverse (the word half a vocabulary away, both ways).
#[derive(Debug, Clone, PartialEq)]
pub struct Inverse {
    pub k: usize,
    pub k_inv: usize,
    pub support: usize,
    pub agree: usize,
}

impl Inverse {
    pub fn confidence(&self) -> f64 {
        self.agree as f64 / self.support.max(1) as f64
    }
}

/// Every inverse pair with at least `min_support` testable triples,
/// strongest first.
pub fn find_inverses(told: &[Triple], min_support: usize) -> Vec<Inverse> {
    let set: HashSet<(usize, usize, usize)> = told.iter().map(|t| (t.about, t.kind, t.other)).collect();
    let asked: HashSet<(usize, usize)> = told.iter().map(|t| (t.about, t.kind)).collect();
    let mut kinds: Vec<usize> = told.iter().map(|t| t.kind).collect();
    kinds.sort_unstable();
    kinds.dedup();
    let mut out = Vec::new();
    for &k in &kinds {
        for &k_inv in &kinds {
            let (mut support, mut agree) = (0, 0);
            for t in told.iter().filter(|t| t.kind == k && t.about != t.other && asked.contains(&(t.other, k_inv))) {
                support += 1;
                agree += set.contains(&(t.other, k_inv, t.about)) as usize;
            }
            if support >= min_support {
                out.push(Inverse { k, k_inv, support, agree });
            }
        }
    }
    out.sort_by(|a, b| b.confidence().partial_cmp(&a.confidence()).unwrap().then(b.support.cmp(&a.support)));
    out
}

/// The triples `inverses` imply that were not told: `w k v` for every told
/// `v k' w`. They train like told rows; they are the law's reading, counted
/// first, not written in.
pub fn implied(told: &[Triple], inverses: &[&Inverse]) -> Vec<Triple> {
    let set: HashSet<Triple> = told.iter().copied().collect();
    let mut out: Vec<Triple> = Vec::new();
    for inv in inverses {
        for t in told.iter().filter(|t| t.kind == inv.k_inv && t.about != t.other) {
            let x = Triple { about: t.other, kind: inv.k, other: t.about };
            if !set.contains(&x) && !out.contains(&x) {
                out.push(x);
            }
        }
    }
    out
}
