//! `the_numbers_on_the_guide_do_not_move` — the golden test over the two
//! numbers the Harness pins on The_Guide: the measured loss (the learner's
//! perplexity) and the score fingerprint (R11.9, R11.10).
//!
//! The yardstick is the same one `phisor::gold` and `phical`'s index table
//! use, read from the other side: a number is pinned here as a golden
//! constant, the measured side is folded to a fingerprint the same way, and
//! a drift from the pin fails loudly and names both numbers. This target
//! links none of the ten crates it polices, so the fold is the one written
//! out in `mod` (`crate::mix64`, phistd's `mix64`), not linked.
//!
//! What this file pins and what it does not. A full Guide training run is
//! heavy — epochs over 142k words — and does not belong in the structural
//! target, so the "measured" side is modelled: `measured_loss()` and
//! `measured_scores()` return a fixture that stands in for the Harness's
//! reading, pinned to match the golden so the drift-detection and the
//! re-recording logic are what is under test here, not a model's weights.
//! When the Harness runs for real it feeds its own reading through
//! [`drift`]; the shape of the judgement is the same.
//!
//! The re-recording path mirrors `phical::keyboard::index::Rerecord`: a
//! [`Rerecord`] carrying the previous values, the new values and the stated
//! reason, any one of the three absent making the verdict [`Verdict::Unjudged`]
//! and failing. Task 4.2's deliberate change to what is learned — the
//! move-keeping rule moving from `gain > 0.0` to `phical::liskov::staged`'s
//! better-and-not-worse — is a change to what the learner learns and so to
//! these numbers; it is accounted for here as a recorded re-recording rather
//! than left to fail as a silent drift.
//!
//! Validates: Requirements 11.9, 11.10

// ---------------------------------------------------------------------------
// The pinned golden values.
// ---------------------------------------------------------------------------

/// The measured loss pinned on The_Guide: the current learner's perplexity,
/// the number R4.3 writes every candidate beside (21.3% / 101.2). The loss
/// is the perplexity; a drift from it is the learner having moved.
const GOLDEN_LOSS: f64 = 101.2;

/// The score numbers pinned beside the loss, in a fixed order, so the
/// fingerprint over them is stable: top-1, perplexity, the trigram's top-1
/// and perplexity (R4.3). The fingerprint is what R11.9 names "the score
/// fingerprint" — one number standing for the whole score vector.
const GOLDEN_SCORES: &[f64] = &[21.3, 101.2, 24.8, 84.7];

/// A loss drift under this many units of perplexity is the same reading
/// twice, not a move: a loss is a float and a re-measured run need not
/// reproduce the last bit. One tenth matches the one-decimal reporting the
/// Harness writes (R4.2).
const LOSS_EPSILON: f64 = 0.1;

/// The golden score fingerprint: [`fingerprint`] over [`GOLDEN_SCORES`],
/// pinned so a single changed score moves it. Recorded from the fold below.
const GOLDEN_FINGERPRINT: u64 = 0x175f_27c5_c9c6_526d;

// ---------------------------------------------------------------------------
// The modelled "measured" side.
// ---------------------------------------------------------------------------

/// The measured loss the Harness reads on The_Guide. Modelled here as the
/// pinned value — the structural target does not train a learner — so the
/// pure drift logic is what the tests exercise. The real Harness replaces
/// this reading with its own run's perplexity and feeds it to [`drift`].
fn measured_loss() -> f64 {
    GOLDEN_LOSS
}

/// The measured score vector, modelled the same way and in the same order as
/// [`GOLDEN_SCORES`].
fn measured_scores() -> Vec<f64> {
    GOLDEN_SCORES.to_vec()
}

/// A score vector's fingerprint: `crate::mix64` (phistd's `mix64`) folded
/// over each score's bit pattern in order, so a changed score, a score added
/// or removed, or the same scores reordered all give another number. One
/// decimal of rounding is applied first so a last-bit wobble in a float does
/// not read as a drift.
fn fingerprint(scores: &[f64]) -> u64 {
    scores.iter().fold(0u64, |held, &score| {
        let rounded = ((score * 10.0).round() / 10.0).to_bits();
        crate::mix64(held ^ rounded)
    })
}

// ---------------------------------------------------------------------------
// The drift judgement (R11.9).
// ---------------------------------------------------------------------------

/// What a reading of the two numbers came to against the pin. `Held` is the
/// only outcome that does not fail; a `Miss` on either number names the
/// pinned value and the measured value side by side (R11.9).
#[derive(Clone, Debug, PartialEq)]
pub enum Drift {
    /// The loss is within [`LOSS_EPSILON`] of the pin and the fingerprint is
    /// the pinned one.
    Held,
    /// The measured loss moved. Both numbers are carried for the report.
    Loss { pinned: f64, measured: f64 },
    /// The score fingerprint moved, so some score is not what it was.
    Score { pinned: u64, measured: u64 },
}

impl Drift {
    /// Whether this drift fails the golden test and records a Miss (R11.9).
    pub fn is_miss(&self) -> bool {
        !matches!(self, Drift::Held)
    }

    /// The pinned value beside the measured value, as the record must show
    /// them (R11.9). `Held` has nothing to say.
    pub fn report(&self) -> String {
        match self {
            Drift::Held => "the numbers on The_Guide held".to_string(),
            Drift::Loss { pinned, measured } => format!(
                "the measured loss on The_Guide moved: pinned {pinned}, measured {measured} \
                 (re-record through Rerecord with the previous value, the new value and the \
                 reason, or revert the change)"
            ),
            Drift::Score { pinned, measured } => format!(
                "the score fingerprint on The_Guide moved: pinned {pinned:#018x}, measured \
                 {measured:#018x} (re-record or revert)"
            ),
        }
    }
}

/// Judge a measured loss and score vector against the pins: the loss first,
/// then the fingerprint, so a failure names the number that moved.
pub fn drift(loss: f64, scores: &[f64]) -> Drift {
    if (loss - GOLDEN_LOSS).abs() > LOSS_EPSILON {
        return Drift::Loss { pinned: GOLDEN_LOSS, measured: loss };
    }
    let measured = fingerprint(scores);
    if measured != GOLDEN_FINGERPRINT {
        return Drift::Score { pinned: GOLDEN_FINGERPRINT, measured };
    }
    Drift::Held
}

// ---------------------------------------------------------------------------
// The re-recording path (R11.10) lives in the single-word sibling `record`,
// which carries the three-part `Rerecord` rule and task 4.2's recorded
// change, keeping this file under the 250-line cap.
// ---------------------------------------------------------------------------

mod record;
pub use record::{task_4_2_rerecording, Part, Rerecord, Verdict};

#[cfg(test)]
mod tests;
