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
//!
//! Resuming: `halve_from` takes each unit's path from an earlier run
//! (`Halving::paths`). At each depth a unit starts in the half its path
//! names, and a unit with no path there (a new one, or one under a class
//! that had stopped) alternates as before; then the passes and the pay run
//! as always. So what was learned is where the next run starts, and a
//! class that stopped may split when the data grows to pay for it.

use super::pay::settle;
use super::table::{Pairs, Table};
use super::{apply_move, likelihood, move_gain, WordCounts};
use std::collections::{HashMap, HashSet};

mod halving;
mod settings;
#[cfg(test)]
mod tests;

pub use halving::Halving;
pub use settings::Settings;

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

/// Grow a code for `units` units from the pairs in `sequences`, at the
/// default settings.
pub fn halve(units: usize, sequences: &[Vec<usize>], grow: Grow) -> Halving {
    halve_with(units, sequences, grow, &Settings::default())
}

/// Grow a code for `units` units from the pairs in `sequences`, reading the
/// six settings. At `Settings::default()` this is byte-identical to the
/// routine before the struct existed; a non-default seed, frequency floor or
/// cost penalty changes nothing yet unless it leaves its no-op value.
pub fn halve_with(units: usize, sequences: &[Vec<usize>], grow: Grow, settings: &Settings) -> Halving {
    halve_from(units, sequences, grow, settings, &[])
}

/// Grow a code as `halve_with` does, each unit starting in the half its
/// `prior` path names at each depth it reaches (`Halving::paths` of an
/// earlier run). A unit beyond `prior`, or past its path's end, alternates
/// as a new unit does. With no prior this is `halve_with`.
pub fn halve_from(units: usize, sequences: &[Vec<usize>], grow: Grow, settings: &Settings, prior: &[Vec<bool>]) -> Halving {
    let pairs = Pairs::count(units, sequences);
    // Most told first, the seed naming no reorder at `0` (the default) so the
    // order stays the one the routine has always used.
    let mut order: Vec<usize> = (0..units).collect();
    order.sort_by(|&a, &b| pairs.told[b].total_cmp(&pairs.told[a]).then(a.cmp(&b)));
    // A unit told fewer than `min_unit_frequency` times does not move; at the
    // default floor of `0` every unit may move, so this set is empty.
    let frozen: HashSet<usize> = match settings.min_unit_frequency {
        0 => HashSet::new(),
        floor => (0..units)
            .filter(|&u| (pairs.told[u] as usize) < floor)
            .collect(),
    };
    let _ = (
        settings.target_classes,
        settings.split_cost_penalty,
        settings.move_order_seed,
    );
    let limit = match grow {
        Grow::Fixed(d) => d.min(MAX_DEPTH),
        Grow::Paid => MAX_DEPTH,
    };
    let mut codes = vec![0u16; units];
    // The leaves that have stopped growing, as codes at the current depth: a
    // unit whose code is one of them does not move.
    let mut stopped: HashSet<u16> = HashSet::new();
    let mut out = Halving::default();
    for depth in 1..=limit {
        let before = dense(&codes).0.len();
        stopped = stopped.iter().map(|s| s * 2).collect();
        let mut seen: HashMap<u16, usize> = HashMap::new();
        for &w in &order {
            let parent = codes[w];
            let resumed = prior.get(w).and_then(|p| p.get(depth as usize - 1));
            codes[w] = match (stopped.contains(&(parent * 2)), resumed) {
                (true, _) => parent * 2,
                (false, Some(&bit)) => parent * 2 + bit as u16,
                (false, None) => {
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
        for _ in 0..settings.max_passes {
            let mut gained = 0.0;
            for &w in &order {
                let sibling = codes[w] ^ 1;
                let (from, to) = match (
                    frozen.contains(&w),
                    stopped.contains(&codes[w]),
                    map.get(&sibling),
                ) {
                    (false, false, Some(&to)) => (ids[w], to),
                    _ => continue,
                };
                let a = table.against(&pairs, &ids, w);
                let counts = WordCounts {
                    out: &a.out,
                    inn: &a.inn,
                    own: a.own,
                };
                let gain = move_gain(
                    &table.pairs,
                    &table.left,
                    &table.right,
                    k,
                    &counts,
                    from,
                    to,
                );
                match gain > 0.0 {
                    true => {
                        apply_move(
                            &mut table.pairs,
                            &mut table.left,
                            &mut table.right,
                            k,
                            &counts,
                            from,
                            to,
                        );
                        codes[w] = sibling;
                        ids[w] = to;
                        gained += gain;
                    }
                    false => {}
                }
            }
            gains.push(gained);
            match gained <= gains[0] * settings.stop_fraction {
                true => break,
                false => {}
            }
        }
        let kept = match grow {
            Grow::Fixed(_) => k - before,
            Grow::Paid => settle(&pairs, &mut codes, &mut ids, &map, &mut table, &mut stopped),
        };
        out.passes.push(gains);
        out.likelihood
            .push(likelihood(&table.pairs, &table.left, &table.right));
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
