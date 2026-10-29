//! The counts the routine moves: each unit's pairs (who follows it, whom
//! it follows), the class table they sum to, and one unit's counts against
//! the classes. Classes are dense ids here (`0..k`), whatever their codes.
//! A move's gain and the move itself are this module's parent's
//! (`move_gain`, `apply_move`). From phisor's `exchange`, 2026-10-29.

use std::collections::HashMap;

/// Each unit's pairs, sparse: `out[w][v]` counts `w` then `v`, and
/// `inn[w][v]` counts `v` then `w`.
pub struct Pairs {
    pub out: Vec<HashMap<usize, f64>>,
    pub inn: Vec<HashMap<usize, f64>>,
    /// How often each unit stands in a pair, either side.
    pub told: Vec<f64>,
    /// How many pairs there are.
    pub total: f64,
}

impl Pairs {
    pub fn count(units: usize, sequences: &[Vec<usize>]) -> Self {
        let mut out = vec![HashMap::new(); units];
        let mut inn = vec![HashMap::new(); units];
        let mut told = vec![0.0; units];
        let mut total = 0.0;
        for s in sequences {
            for w in s.windows(2) {
                *out[w[0]].entry(w[1]).or_insert(0.0) += 1.0;
                *inn[w[1]].entry(w[0]).or_insert(0.0) += 1.0;
                told[w[0]] += 1.0;
                told[w[1]] += 1.0;
                total += 1.0;
            }
        }
        Pairs { out, inn, told, total }
    }
}

/// The class table: `pairs[a * k + b]` counts a unit of class `a` then one of
/// class `b`; `left` and `right` are its row and column sums.
pub struct Table {
    pub k: usize,
    pub pairs: Vec<f64>,
    pub left: Vec<f64>,
    pub right: Vec<f64>,
}

/// One unit's counts against the classes, its pairs with itself apart.
pub struct Against {
    pub out: Vec<f64>,
    pub inn: Vec<f64>,
    pub own: f64,
}

impl Table {
    /// The table of `pairs` with each unit in class `ids[unit]`.
    pub fn build(pairs: &Pairs, ids: &[usize], k: usize) -> Self {
        let mut t = Table { k, pairs: vec![0.0; k * k], left: vec![0.0; k], right: vec![0.0; k] };
        for (w, next) in pairs.out.iter().enumerate() {
            for (&v, &n) in next {
                let (a, b) = (ids[w], ids[v]);
                t.pairs[a * k + b] += n;
                t.left[a] += n;
                t.right[b] += n;
            }
        }
        t
    }

    /// Unit `w`'s counts against the classes as they stand.
    pub fn against(&self, pairs: &Pairs, ids: &[usize], w: usize) -> Against {
        let mut a = Against { out: vec![0.0; self.k], inn: vec![0.0; self.k], own: 0.0 };
        for (&v, &n) in &pairs.out[w] {
            match v == w {
                true => a.own += n,
                false => a.out[ids[v]] += n,
            }
        }
        for (&v, &n) in &pairs.inn[w] {
            match v == w {
                true => {}
                false => a.inn[ids[v]] += n,
            }
        }
        a
    }
}
