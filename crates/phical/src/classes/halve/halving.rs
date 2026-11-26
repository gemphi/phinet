//! What the routine left: each unit's code, the record of its growth, and
//! each unit's path in the tree it grew, which a later run resumes from.

use std::collections::HashMap;

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

    /// Each unit's path in the tree: its code's bits down to where its class
    /// stopped growing, without the padding. A class stopped at depth `d`
    /// when no other class shares its first `d` bits, since a split kept
    /// below `d` would have left two. A later run resumes from these paths
    /// (`halve_from`), and a class that stopped may split there when the
    /// data grows to pay for it.
    pub fn paths(&self) -> Vec<Vec<bool>> {
        let depth = self.depth;
        let prefix = |c: u16, d: u32| match d {
            0 => 0,
            _ => c >> (depth - d),
        };
        let mut classes: Vec<u16> = self.codes.clone();
        classes.sort_unstable();
        classes.dedup();
        let mut shared: HashMap<(u32, u16), usize> = HashMap::new();
        for &c in &classes {
            for d in 0..=depth {
                *shared.entry((d, prefix(c, d))).or_default() += 1;
            }
        }
        let length = |c: u16| (0..=depth).find(|&d| shared[&(d, prefix(c, d))] == 1).unwrap_or(depth);
        self.codes
            .iter()
            .map(|&c| (0..length(c)).map(|i| (c >> (depth - 1 - i)) & 1 == 1).collect())
            .collect()
    }
}
