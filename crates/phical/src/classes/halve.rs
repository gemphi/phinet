//! Halving: depth by depth, every class still growing splits in two and
//! every unit may move to its sibling, so the code stays nested (its first
//! four bits are the code at 16). A unit starts in the half its rank
//! alternates it into, most told first, and then the passes run. Each pass
//! tries every unit, most told first, and keeps a move only if it is
//! better: a positive gain. The passes stop when one gains less than a
//! thousandth of the first (Babbage: the difference that no longer pays),
//! or after `MAX_PASSES`.
//!
//! How deep: `Grow::Fixed(d)` splits every class to depth `d` (the first
//! slice). `Grow::Paid` keeps a class's split only if it pays for itself
//! (`pay`: the likelihood it gains against `split_cost`), so the
//! data says how many classes there are. A class whose split does not pay
//! stops growing; the code stops when no split pays, or at `MAX_DEPTH`.

use super::pay::settle;
use super::table::{Pairs, Table};
use super::{apply_move, likelihood, move_gain, WordCounts};
use std::collections::{HashMap, HashSet};

/// What the routine left: each unit's code, and the record of its growth.
#[derive(Clone, Debug, Default)]
pub struct Halving {
    /// Each unit's code, `depth` bits; a class that stopped early is padded with 0 bits.
    pub codes: Vec<u16>,
    /// How many bits the code grew to.
    pub depth: u32,
    /// How many classes the code ends with: the number the data asked for.
    pub classes: usize,
    /// `passes[d]` is depth `d + 1`'s gain, pass by pass (Babbage: the differences the stop reads).
    pub passes: Vec<Vec<f64>>,
    /// The likelihood at each depth once its passes and splits are settled.
    pub likelihood: Vec<f64>,
    /// How many splits each depth kept.
    pub kept: Vec<usize>,
}

impl Halving {
    /// A unit's class with the code cut to `bits` bits (all of it when the
    /// code is shorter).
    pub fn cut(&self, unit: usize, bits: u32) -> u16 {
        let c = self.codes[unit];
        match self.depth > bits {
            true => c >> (self.depth - bits),
            false => c,
        }
    }
}

/// The most passes at one depth.
pub const MAX_PASSES: usize = 20;
/// A pass that gains less than this share of the first pass's gain stops the depth.
pub const STOP: f64 = 1e-3;
/// The deepest a code may grow: a bound for safety, reported when reached.
pub const MAX_DEPTH: u32 = 12;

/// How deep the code grows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grow {
    /// Every class splits to this depth.
    Fixed(u32),
    /// A class splits only while the split pays, up to `MAX_DEPTH`.
    Paid,
}

/// Each code its dense class id, in the order of the codes.
pub(super) fn dense(codes: &[u16]) -> (HashMap<u16, usize>, Vec<usize>) {
    let mut sorted: Vec<u16> = codes.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let map: HashMap<u16, usize> = sorted.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let ids = codes.iter().map(|c| map[c]).collect();
    (map, ids)
}

/// Grow a code for `units` units from the pairs in `sequences`.
pub fn halve(units: usize, sequences: &[Vec<usize>], grow: Grow) -> Halving {
    let pairs = Pairs::count(units, sequences);
    let mut order: Vec<usize> = (0..units).collect();
    order.sort_by(|&a, &b| pairs.told[b].total_cmp(&pairs.told[a]).then(a.cmp(&b)));
    let limit = match grow {
        Grow::Fixed(d) => d.min(MAX_DEPTH),
        Grow::Paid => MAX_DEPTH,
    };
    let mut codes = vec![0u16; units];
    // The leaves that have stopped growing, as codes at the current depth: a
    // unit whose code is one of them does not move.
    let mut stopped: HashSet<u16> = HashSet::new();
    let mut out = Halving::default();
    for _ in 1..=limit {
        let before = dense(&codes).0.len();
        stopped = stopped.iter().map(|s| s * 2).collect();
        let mut seen: HashMap<u16, usize> = HashMap::new();
        for &w in &order {
            let parent = codes[w];
            codes[w] = match stopped.contains(&(parent * 2)) {
                true => parent * 2,
                false => {
                    let s = seen.entry(parent).or_default();
                    *s += 1;
                    parent * 2 + ((*s - 1) % 2) as u16
                }
            };
        }
        let (map, mut ids) = dense(&codes);
        let k = map.len();
        let mut table = Table::build(&pairs, &ids, k);
        let mut gains = Vec::new();
        for _ in 0..MAX_PASSES {
            let mut gained = 0.0;
            for &w in &order {
                let sibling = codes[w] ^ 1;
                let (from, to) = match (stopped.contains(&codes[w]), map.get(&sibling)) {
                    (false, Some(&to)) => (ids[w], to),
                    _ => continue,
                };
                let a = table.against(&pairs, &ids, w);
                let counts = WordCounts { out: &a.out, inn: &a.inn, own: a.own };
                let gain = move_gain(&table.pairs, &table.left, &table.right, k, &counts, from, to);
                match gain > 0.0 {
                    true => {
                        apply_move(&mut table.pairs, &mut table.left, &mut table.right, k, &counts, from, to);
                        codes[w] = sibling;
                        ids[w] = to;
                        gained += gain;
                    }
                    false => {}
                }
            }
            gains.push(gained);
            match gained <= gains[0] * STOP {
                true => break,
                false => {}
            }
        }
        let kept = match grow {
            Grow::Fixed(_) => k - before,
            Grow::Paid => settle(&pairs, &mut codes, &mut ids, &map, &mut table, &mut stopped),
        };
        out.passes.push(gains);
        out.likelihood.push(likelihood(&table.pairs, &table.left, &table.right));
        out.kept.push(kept);
        out.depth += 1;
        match (grow, kept) {
            (Grow::Paid, 0) => {
                // Nothing paid at this depth: take the bit back.
                for c in codes.iter_mut() {
                    *c >>= 1;
                }
                out.depth -= 1;
                break;
            }
            _ => {}
        }
    }
    out.classes = dense(&codes).0.len();
    out.codes = codes;
    out
}
