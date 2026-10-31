//! Measures that turn "learned how to learn" into recorded comparisons.
//!
//! The word and sentence heights use the same six exchange [`Settings`].
//! Their values are shown side by side, and every drift is named. The
//! difference engine records every pass and stops at the first gain below
//! one thousandth of the first pass's gain, or at pass 100. [`twins`] reads
//! the backward mirror and compares probability addition with amplitude
//! addition through phical's Circle Operator.

#[cfg(test)]
mod property;
#[cfg(test)]
mod tests;
mod twins;

pub use phical::classes::Settings;
pub use twins::{
    combine_amplitudes, combine_probabilities, disagreements, disagreements_at, judge_twins,
    measure_mode, measure_modes, CombinationReport, Disagreement, KindDisagreement, ModeInput,
    ModeReport, TestReading, TwinPurity, BOTH_MARGIN, MAX_LISTED, TWIN_MODES,
};

/// The fixed fraction of the first pass's gain below which learning stops.
pub const STOP_FRACTION: f64 = 0.001;
/// The latest pass the difference engine runs.
pub const PASS_LIMIT: usize = 100;

/// Whether the sentence height learned without changing a setting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Judged {
    Pass,
    /// The setting names that drifted, in [`Settings::NAMES`] order. This is
    /// empty when the settings matched but the sentence height kept no move.
    Miss {
        differing: Vec<&'static str>,
    },
}

/// Pass only when all six settings match and the sentence height kept at
/// least one move (R9.2).
pub fn learned_how_to_learn(word: &Settings, sentence: &Settings, kept: usize) -> Judged {
    let differing = Settings::differing(word, sentence);
    match differing.is_empty() && kept >= 1 {
        true => Judged::Pass,
        false => Judged::Miss { differing },
    }
}

/// What one pass of the exchange routine did (R9.4).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pass {
    /// One-based pass index.
    pub index: usize,
    pub evaluated: usize,
    pub kept: usize,
    pub rejected: usize,
    pub gain: f64,
}

/// Which of the two declared conditions stopped the difference engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    GainBelowThousandth { at: usize },
    PassLimit,
}

/// Return the count of passes run and the first declared stop condition.
///
/// Pass indices are one-based. The gain condition is strict: a gain exactly
/// one thousandth of the first does not stop the routine. A complete record
/// that never crosses the threshold contains at least [`PASS_LIMIT`] gains;
/// for a shorter partial record, the returned count is the number supplied.
pub fn stop_at(gains: &[f64]) -> (usize, Stopped) {
    let Some(&first) = gains.first() else {
        return (0, Stopped::PassLimit);
    };
    let threshold = first * STOP_FRACTION;
    for (offset, &gain) in gains.iter().take(PASS_LIMIT).enumerate() {
        let index = offset + 1;
        if gain < threshold {
            return (index, Stopped::GainBelowThousandth { at: index });
        }
    }
    (gains.len().min(PASS_LIMIT), Stopped::PassLimit)
}
