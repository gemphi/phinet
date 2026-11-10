//! Counts across Modes and the uncapped learned inventory (R5.10, R5.13).

use crate::ladder::{Arm, CodeReport, KindReport, Ladder};
use phical::classes::Settings;
use phiton::p64::variants::Level;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModeCount {
    pub level: Level,
    pub occupied: usize,
    pub variations_per_kind: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountReport {
    pub arm: Arm,
    pub modes: [ModeCount; 3],
    pub occupied: [usize; 3],
    pub agree: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrownCounts {
    pub templates: usize,
    pub word_classes: usize,
}

/// Agreement is inclusive at both 20% boundaries. With a zero mean the only
/// possible triple is `[0, 0, 0]`; every count is then at the mean and the
/// result is explicitly `true`.
///
/// Integer cross-products make the boundary exact:
/// `|count - mean| <= mean/5` iff `5 * |3*count - sum| <= sum`.
pub fn counts_agree(counts: [usize; 3]) -> bool {
    let values = counts.map(|count| count as u128);
    let sum = values.iter().sum::<u128>();
    if sum == 0 {
        return true;
    }
    values
        .into_iter()
        .all(|count| 5 * (3 * count).abs_diff(sum) <= sum)
}

/// Record occupied Kinds and every Kind's Variation count in Mode order.
pub fn counts(reports: &[KindReport; 3]) -> CountReport {
    let modes = std::array::from_fn(|i| ModeCount {
        level: reports[i].level,
        occupied: reports[i].occupied,
        variations_per_kind: reports[i]
            .kinds
            .iter()
            .map(|kind| kind.variations.len())
            .collect(),
    });
    let occupied = modes.each_ref().map(|mode| mode.occupied);
    CountReport {
        arm: reports[0].arm,
        modes,
        occupied,
        agree: counts_agree(occupied),
    }
}

/// The learned inventory is reported as it grew. Neither count is clamped
/// to 64 or to any other upper limit.
pub fn grown(kinds: &KindReport, code: &CodeReport) -> GrownCounts {
    GrownCounts {
        templates: kinds
            .kinds
            .iter()
            .map(|kind| kind.variations.len())
            .sum(),
        word_classes: code.halving.classes,
    }
}

impl Ladder<'_> {
    /// One arm's occupied and per-Kind Variation counts at Modes 16/32/64.
    pub fn counts(&self, arm: Arm, settings: &Settings) -> CountReport {
        let reports = crate::code::LEVELS.map(|level| self.kinds(arm, level, settings));
        counts(&reports)
    }

    /// All three arms use the same report path. In particular Hash_Null is
    /// subjected to the identical 20%-of-mean agreement predicate (R5.10).
    pub fn all_counts(&self, settings: &Settings) -> [CountReport; 3] {
        Arm::ALL.map(|arm| self.counts(arm, settings))
    }
}
