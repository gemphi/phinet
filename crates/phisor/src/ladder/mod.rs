//! `ladder` — the Ladder's six heights, each run in all three arms against
//! a Bar fixed before the run (R5). This file is the first height, the
//! **word** height: a text written in a Mode is one class token per word,
//! structure only, with no word of the text in it (R5.1).
//!
//! | file | what |
//! |---|---|
//! | `mod` | [`Height`], [`Arm`], [`Seeds`], [`LadderError`] and [`Ladder`] itself |
//! | `mode` | the Mode gate and the write: [`mode`], [`named`], [`write`], [`show`] |
//! | `account` | the sentence accounting of R5.15: [`SHAPED`], [`shaped`], [`Account`] |
//! | `grow` | the code's growth and the Mode nesting: [`grow_code`], [`nesting`] |
//! | `kinds` | the Variation and Kind heights: `variation`, `kinds` (task 13.4) |
//! | `measure` | the height measures and their Bars (task 13.5) |
//! | `property` | Properties 21 through 25 and 52, at their respective heights |
//! | `tests` | the word height's examples, and the corpus the properties draw over |
//!
//! **State of this file.** The word, Mode, Variation and Kind heights have
//! landed. `measure.rs` lands beside them as task 13.5; the act height's
//! `act.rs` is task 13.7, which is skipped, so its four numbers stay
//! `unjudged`. [`Height`] names all six now because the Recorder keys its
//! report by height, and a height with nothing in it yet is reported as
//! having nothing rather than being absent.
//!
//! The three arms are the three the guide is read in, and [`Coder`] already
//! distinguishes them: phiton's `classify_str` is the Hash_Null, phiton's
//! `PosDictionary` the Given arm, and a code grown by `phical::classes` the
//! Learned arm. [`Ladder`] is the design's `Ladder_Runner` under the
//! workspace's naming rule, which wants a type name singular.

mod account;
mod grow;
mod kinds;
mod measure;
mod mode;
#[cfg(test)]
mod property;
#[cfg(test)]
mod tests;
pub use account::{shaped, Account, SHAPED};
pub use grow::{grow_code, nesting, CodeReport, DepthReport, Nesting};
pub use kinds::{kinds, river, variation, KindGroup, KindReport, Variation};
pub use measure::*;
pub use mode::{holds, mode, named, show, write, FEWEST, MODES, MOST};

use crate::code::{Code, Coder};
use crate::guide::{Sentence, Split};
use phical::classes::Grow;
use phiton::p64::variants::Level;

/// The six heights of R5. The Ladder climbs them in this order, and a
/// height that misses its Bar does not stop the ones above it (R5.14).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Height {
    Words,
    Mode,
    Variation,
    Kind,
    Act,
    Rows,
}

/// The three arms. The Hash_Null is the null every learned number is read
/// against: random buckets, told nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Arm {
    HashNull,
    Given,
    Learned,
}

/// Every random choice's seed, recorded so a run repeats (R9.15).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seeds {
    /// The exchange routine's move order: `Settings::move_order_seed`.
    pub move_order: u64,
    /// The river's shuffles, which the z-score of R5.9 counts 1,000 of.
    pub shuffles: u64,
    /// The phases a drawn case is seeded from.
    pub phases: u64,
}

/// Why a write did not happen. Each names what was given beside what is
/// accepted, and in both cases nothing is written (R5.16).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LadderError {
    /// The Mode named is not one of the three (R5.16).
    UnknownMode {
        given: String,
        accepted: [&'static str; 3],
    },
    /// The text holds no sentence, or more than [`MOST`] (R5.1).
    TextOutOfRange { sentences: usize },
}

/// The Ladder over one split, with the seeds its run repeats from. The
/// design's `Ladder_Runner`; the heights above the word height land in the
/// files named in this module's table.
pub struct Ladder<'a> {
    split: &'a Split,
    seeds: Seeds,
}

impl<'a> Ladder<'a> {
    pub fn new(split: &'a Split, seeds: Seeds) -> Self {
        Ladder { split, seeds }
    }

    /// The split the run reads: the training half the Learned arm's code is
    /// grown from, and the held-out half every number is scored on.
    pub fn split(&self) -> &Split {
        self.split
    }

    /// The seeds this run repeats from (R9.15).
    pub fn seeds(&self) -> Seeds {
        self.seeds
    }

    /// The arm's coder. The Hash_Null is phiton's `classify_str`, the Given
    /// arm phiton's `PosDictionary`, and the Learned arm a code grown by
    /// `phical::classes` over the split's **training** sentences alone, so
    /// nothing held out is told.
    pub fn coder(&self, arm: Arm, grow: Grow) -> Coder {
        match arm {
            Arm::HashNull => Coder::Hash,
            Arm::Given => Coder::Given,
            Arm::Learned => {
                let runs = self.split.training_runs();
                let told: Vec<&[String]> = runs
                    .iter()
                    .flatten()
                    .map(|&i| self.split.sentences[i].words.as_slice())
                    .collect();
                Coder::Learned(Code::learn(&told, grow))
            }
        }
    }

    /// One class token per word in the text's own word order, no word of the
    /// text emitted, a token count equal to the word count (R5.1). A text
    /// outside 1 to 10,000 sentences errors and writes nothing (R5.16);
    /// the Mode is gated by [`mode`] before it is a [`Level`].
    pub fn write_mode(
        &self,
        text: &[Sentence],
        level: Level,
        arm: Arm,
    ) -> Result<Vec<Vec<u16>>, LadderError> {
        write(text, level, &self.coder(arm, Grow::Paid))
    }

    /// R5.16's gate and R5.1's write as one act, which is how the criterion
    /// reads it: a text and a Mode as it is named. An unknown Mode errors
    /// before the arm's code is grown, so nothing is written.
    pub fn write_named(
        &self,
        text: &[Sentence],
        given: &str,
        arm: Arm,
    ) -> Result<Vec<Vec<u16>>, LadderError> {
        let level = mode(given)?;
        match holds(text) {
            true => self.write_mode(text, level, arm),
            false => Err(LadderError::TextOutOfRange {
                sentences: text.len(),
            }),
        }
    }

    /// The text's sentence accounting (R5.15).
    pub fn account(&self, text: &[Sentence]) -> Account {
        Account::of(text)
    }
}

impl Height {
    pub const ALL: [Height; 6] = [
        Height::Words,
        Height::Mode,
        Height::Variation,
        Height::Kind,
        Height::Act,
        Height::Rows,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Height::Words => "words",
            Height::Mode => "mode",
            Height::Variation => "variation",
            Height::Kind => "kind",
            Height::Act => "act",
            Height::Rows => "rows",
        }
    }

    /// The height a name stands for; `None` when it is not one of the six.
    pub fn of(name: &str) -> Option<Height> {
        Height::ALL.into_iter().find(|h| h.name() == name)
    }
}

impl Arm {
    pub const ALL: [Arm; 3] = [Arm::HashNull, Arm::Given, Arm::Learned];

    /// The arm's key in a report. [`Coder::name`] says `hash` for the same
    /// arm; here it is `hash_null`, because the null is what it is for.
    pub fn name(self) -> &'static str {
        match self {
            Arm::HashNull => "hash_null",
            Arm::Given => "given",
            Arm::Learned => "learned",
        }
    }
}

impl Seeds {
    /// The declared run's seeds. `move_order` is `0`, which is
    /// `phical::classes::Settings::default().move_order_seed`: the fixed
    /// most-told-first order the exchange routine has always used, so the
    /// declared run is the run as it stood before the seeds were named.
    pub const DECLARED: Seeds = Seeds {
        move_order: 0,
        shuffles: 0x51FF_F1E5,
        phases: 0x00F1_A5E5,
    };
}

impl Default for Seeds {
    fn default() -> Self {
        Seeds::DECLARED
    }
}

impl std::fmt::Display for LadderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LadderError::UnknownMode { given, accepted } => write!(
                f,
                "mode {given:?} is not one of {}, {} or {}",
                accepted[0], accepted[1], accepted[2]
            ),
            LadderError::TextOutOfRange { sentences } => write!(
                f,
                "a text of {sentences} sentences is outside {FEWEST} to {MOST}"
            ),
        }
    }
}
