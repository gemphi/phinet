//! Family resemblance (Wittgenstein, Investigations §66–67), counted:
//! within one kind, does every variation share one feature, or do they hang
//! together by overlapping ones? A feature is a class at a place (the first,
//! the second, the last). The count is the most variations any one feature
//! holds, and how many connected pieces the variations fall into when two
//! are joined by any shared feature.

use std::collections::HashMap;

/// What a kind's variations hold in common.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resemblance {
    pub variations: usize,
    /// The most variations one feature is in.
    pub widest: usize,
    /// The connected pieces of the overlap graph: 1 is one family.
    pub pieces: usize,
}

/// A variation's features: each class with its place (`c9 c2 … c11` gives
/// `0:c9`, `1:c2`, `last:c11`; `… c1 c11` gives `-2:c1`, `last:c11`).
fn features(v: &str) -> Vec<String> {
    let parts: Vec<&str> = v.split(' ').collect();
    let dots = parts.iter().position(|p| *p == "…");
    parts
        .iter()
        .enumerate()
        .filter(|(_, p)| **p != "…")
        .map(|(i, p)| match (dots, i) {
            (Some(d), i) if i > d && i + 1 == parts.len() => format!("last:{p}"),
            (Some(d), i) if i > d => format!("-2:{p}"),
            (_, i) => format!("{i}:{p}"),
        })
        .collect()
}

pub fn resemblance(variations: &[&str]) -> Resemblance {
    let n = variations.len();
    let mut holders: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, v) in variations.iter().enumerate() {
        for f in features(v) {
            holders.entry(f).or_default().push(i);
        }
    }
    let widest = holders.values().map(Vec::len).max().unwrap_or(0);
    // Union-find over the variations, joined through every shared feature.
    let mut root: Vec<usize> = (0..n).collect();
    fn find(root: &mut [usize], x: usize) -> usize {
        let mut r = x;
        while root[r] != r {
            r = root[r];
        }
        root[x] = r;
        r
    }
    for members in holders.values() {
        for w in members.windows(2) {
            let (a, b) = (find(&mut root, w[0]), find(&mut root, w[1]));
            root[a] = b;
        }
    }
    let pieces = (0..n).filter(|&i| find(&mut root, i) == i).count();
    Resemblance { variations: n, widest, pieces }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_family_hangs_together_by_overlaps_not_by_one_feature() {
        // ab, bc, cd: no feature in all three, yet one family.
        let r = resemblance(&["c1 c2 … c3", "c4 c2 … c5", "c4 c6 … c7"]);
        assert_eq!(r, Resemblance { variations: 3, widest: 2, pieces: 1 });
        // Nothing shared: three pieces.
        let r = resemblance(&["c1 c2 … c3", "c4 c5 … c6", "c7 c8 … c9"]);
        assert_eq!(r.pieces, 3);
        assert_eq!(features("… c1 c11"), ["-2:c1", "last:c11"]);
    }
}
