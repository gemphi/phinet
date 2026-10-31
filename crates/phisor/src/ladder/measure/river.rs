//! Held-out Kind-to-Kind mutual information and its shuffled null (R5.9).

use super::{MeasureError, Measured, Unit, RIVER_BAR};
use crate::guide::{Part, Split};
use crate::ladder::{KindReport, Ladder};
use phistd::random::xorshift64;
use phistd::stats::{mutual_information, z_score};

/// The exact null size required by R5.9.
pub const SHUFFLES: usize = 1_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArmZ {
    pub information: f64,
    pub z: f64,
    pub null_draws: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RiverReport {
    /// The held-out sentence and its immediate predecessor. Both arms are
    /// evaluated on this exact index list.
    pub sentence_pairs: Vec<(usize, usize)>,
    pub shuffles: usize,
    pub learned: ArmZ,
    pub hash_null: ArmZ,
    pub against_three: Measured,
    pub against_hash: Measured,
}

/// Score Learned and Hash_Null on the same held-out adjacent sentence pairs.
/// The predecessor need not itself be held out: each pair is held out by its
/// second sentence, matching the guide's established River reading.
pub fn river_z(
    learned: &KindReport,
    hash: &KindReport,
    split: &Split,
    seed: u64,
) -> Result<RiverReport, MeasureError> {
    if learned.level != hash.level {
        return Err(MeasureError::ModeMismatch {
            learned: learned.level.bits() as u8,
            hash_null: hash.level.bits() as u8,
        });
    }
    let sentence_pairs: Vec<(usize, usize)> = split
        .of(Part::Test)
        .into_iter()
        .filter_map(|after| {
            let before = split.before(after)?;
            [learned, hash]
                .iter()
                .all(|report| report.of(before).is_some() && report.of(after).is_some())
                .then_some((before, after))
        })
        .collect();
    let learned_pairs = kind_pairs(learned, &sentence_pairs);
    let hash_pairs = kind_pairs(hash, &sentence_pairs);
    let learned_z = arm_z(&learned_pairs, learned.occupied, seed);
    let hash_z = arm_z(&hash_pairs, hash.occupied, seed);
    Ok(RiverReport {
        sentence_pairs,
        shuffles: SHUFFLES,
        learned: learned_z,
        hash_null: hash_z,
        against_three: Measured::at_least(
            "ladder.river_z.learned",
            learned_z.z,
            RIVER_BAR,
            Unit::ZScore,
        ),
        against_hash: Measured::at_least(
            "ladder.river_z.above_hash",
            learned_z.z,
            hash_z.z,
            Unit::ZScore,
        ),
    })
}

fn kind_pairs(report: &KindReport, indices: &[(usize, usize)]) -> Vec<(usize, usize)> {
    indices
        .iter()
        .map(|&(before, after)| (report.of(before).unwrap(), report.of(after).unwrap()))
        .collect()
}

fn arm_z(pairs: &[(usize, usize)], kinds: usize, seed: u64) -> ArmZ {
    let information = mutual_information(&pair_table(pairs, kinds));
    let null = shuffled_information(pairs, kinds, seed);
    ArmZ {
        information,
        z: z_score(information, &null),
        null_draws: null.len(),
    }
}

fn pair_table(pairs: &[(usize, usize)], kinds: usize) -> Vec<Vec<f64>> {
    let mut table = vec![vec![0.0; kinds]; kinds];
    for &(before, after) in pairs {
        table[before][after] += 1.0;
    }
    table
}

/// Shuffle the predecessor side only, preserving both marginals. The loop
/// has no caller-supplied count: every score therefore uses exactly 1,000
/// draws rather than merely defaulting to that many.
fn shuffled_information(pairs: &[(usize, usize)], kinds: usize, seed: u64) -> Vec<f64> {
    let mut state = phistd::mix64(seed) | 1;
    let mut before: Vec<usize> = pairs.iter().map(|pair| pair.0).collect();
    (0..SHUFFLES)
        .map(|_| {
            for i in (1..before.len()).rev() {
                state = xorshift64(state);
                before.swap(i, (state % (i as u64 + 1)) as usize);
            }
            let shuffled: Vec<(usize, usize)> = before
                .iter()
                .zip(pairs)
                .map(|(&left, &(_, right))| (left, right))
                .collect();
            mutual_information(&pair_table(&shuffled, kinds))
        })
        .collect()
}

impl Ladder<'_> {
    pub fn river_z(
        &self,
        learned: &KindReport,
        hash: &KindReport,
    ) -> Result<RiverReport, MeasureError> {
        river_z(learned, hash, self.split(), self.seeds().shuffles)
    }
}
