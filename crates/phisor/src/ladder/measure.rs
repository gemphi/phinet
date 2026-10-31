//! Measurements at the Mode, Variation and Kind heights (R5.6–R5.10,
//! R5.13–R5.14).
//!
//! Learning and scoring stay separate. [`KindReport`] contains no labels;
//! the label scorers accept a [`HoldoutGuard`] and record each access in
//! [`Phase::ScoreNumber`]. A missed Bar is [`Judged::Miss`] data, not an
//! error, so recording it cannot stop a later height.

mod counts;
mod labels;
mod river;
#[cfg(test)]
#[path = "measure/tests.rs"]
mod tests;
pub use counts::{counts, counts_agree, grown, CountReport, GrownCounts, ModeCount};
pub use labels::{
    function_gap, function_purity, terminator_purity, BestKindF1, FunctionReport, GapReport,
    TerminatorReport,
};
pub use river::{river_z, ArmZ, RiverReport, SHUFFLES};

use super::Height;
use crate::guide::holdout::Breach;

/// The declared function-purity Bar: the 51.7% majority baseline plus five
/// percentage points (R5.6).
pub const FUNCTION_BAR: f64 = 56.7;
/// The learned-minus-Hash_Null Bar at an equal Kind count (R5.7).
pub const GAP_BAR: f64 = 5.0;
/// The held-out River z-score Bar (R5.9).
pub const RIVER_BAR: f64 = 3.0;

/// The unit in which a measured value, Bar and shortfall are recorded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Percentage,
    PercentagePoints,
    ZScore,
}

/// A declared lower Bar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bar {
    pub value: f64,
    pub unit: Unit,
}

/// A Bar verdict. A Miss repeats the reported value and Bar beside the
/// shortfall, so none of the three can be lost by a later height (R5.14).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Judged {
    Pass,
    Miss {
        value: f64,
        bar: f64,
        shortfall: f64,
    },
}

/// One named number judged against an at-least Bar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Measured {
    pub name: &'static str,
    pub value: f64,
    pub bar: Bar,
    pub verdict: Judged,
}

impl Measured {
    pub fn at_least(name: &'static str, value: f64, bar: f64, unit: Unit) -> Self {
        let verdict = match value.is_finite() && value >= bar {
            true => Judged::Pass,
            false => Judged::Miss {
                value,
                bar,
                shortfall: match value.is_finite() {
                    true => bar - value,
                    false => f64::INFINITY,
                },
            },
        };
        Self {
            name,
            value,
            bar: Bar { value: bar, unit },
            verdict,
        }
    }
}

/// A sequence the Recorder can append to. [`record`](Self::record) returns
/// no verdict to branch on: a Miss is retained and the next height runs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeasureLog {
    pub entries: Vec<(Height, Measured)>,
}

impl MeasureLog {
    pub fn record(&mut self, height: Height, measured: Measured) {
        self.entries.push((height, measured));
    }
}

/// A scoring error, distinct from a measured Miss. The gap is never scored
/// unless both arms have the same Mode and occupied Kind count (R5.7).
#[derive(Clone, Debug, PartialEq)]
pub enum MeasureError {
    Holdout(Breach),
    ModeMismatch { learned: u8, hash_null: u8 },
    UnequalKindCount { learned: usize, hash_null: usize },
}

impl From<Breach> for MeasureError {
    fn from(value: Breach) -> Self {
        Self::Holdout(value)
    }
}

pub(super) fn one_decimal(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}
