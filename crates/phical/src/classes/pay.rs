//! Whether a split pays: the data, not a number chosen beforehand, says how
//! many classes there are. After a depth's passes, each class that split
//! is merged back for a moment. The likelihood its split gained is read
//! against what one more class costs to describe (`split_cost`,
//! minimum description length), and the split stays only if it gains more.
//! A class whose split does not pay is merged and stops growing.

use super::table::{Pairs, Table};
use super::{apply_move, likelihood, split_cost, WordCounts};
use std::collections::{HashMap, HashSet};

/// Move every unit in `movers` from class `from` to class `to`.
fn carry(
    pairs: &Pairs,
    table: &mut Table,
    ids: &mut [usize],
    movers: &[usize],
    from: usize,
    to: usize,
) {
    let k = table.k;
    for &w in movers {
        let a = table.against(pairs, ids, w);
        let counts = WordCounts {
            out: &a.out,
            inn: &a.inn,
            own: a.own,
        };
        apply_move(
            &mut table.pairs,
            &mut table.left,
            &mut table.right,
            k,
            &counts,
            from,
            to,
        );
        ids[w] = to;
    }
}

/// Keep each split of this depth that pays; merge and stop the rest.
/// Returns how many splits were kept.
pub fn settle(
    pairs: &Pairs,
    codes: &mut [u16],
    ids: &mut [usize],
    map: &HashMap<u16, usize>,
    table: &mut Table,
    stopped: &mut HashSet<u16>,
) -> usize {
    let mut members: HashMap<u16, Vec<usize>> = HashMap::new();
    for (w, &c) in codes.iter().enumerate() {
        members.entry(c).or_default().push(w);
    }
    let mut parents: Vec<u16> = map
        .keys()
        .filter(|&&c| c & 1 == 0 && map.contains_key(&(c | 1)) && !stopped.contains(&c))
        .copied()
        .collect();
    parents.sort_unstable();
    // A class with one non-empty half did not split: it stops.
    for &c in map.keys().filter(|&&c| !map.contains_key(&(c ^ 1))) {
        stopped.insert(c);
    }
    let mut leaves = map.len();
    let mut kept = 0;
    for zero in parents {
        let one = zero | 1;
        let movers = members.get(&one).cloned().unwrap_or_default();
        let (from, to) = (map[&one], map[&zero]);
        let split = likelihood(&table.pairs, &table.left, &table.right);
        carry(pairs, table, ids, &movers, from, to);
        let merged = likelihood(&table.pairs, &table.left, &table.right);
        match split - merged > split_cost(leaves - 1, pairs.total) {
            true => {
                carry(pairs, table, ids, &movers, to, from);
                kept += 1;
            }
            false => {
                for &w in &movers {
                    codes[w] = zero;
                }
                stopped.insert(zero);
                leaves -= 1;
            }
        }
    }
    kept
}
