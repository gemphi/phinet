//! Reversible text for an [`Act`].
//!
//! The surface stays the IR's readable notation:
//! `ASK(wait(agent: you, in: rain), past, progressive)`. Where two Rust
//! values would otherwise print alike, the canonical writer adds a small
//! marker: `*France` is a name, `^you` a pronoun and `'blue` a bare word;
//! determiners are retained on things. This is deliberately stricter than
//! [`Act`]'s presentation [`Display`](std::fmt::Display), which predates the
//! reader and omits distinctions that a reversible form cannot omit.
//!
//! [`write`] and [`read`] are the paired operations. [`round_trip`] reports
//! their field-for-field success share against Requirement 10.10's 100% Bar.

mod parse;
mod print;
#[cfg(test)]
mod property;
mod scan;
#[cfg(test)]
mod tests;

use super::Act;
use std::fmt;

/// The Recorder key for the Act text round trip.
pub const ROUND_TRIP_NAME: &str = "ladder.act.round_trip";
/// Requirement 10.10 permits no loss.
pub const ROUND_TRIP_BAR: f64 = 100.0;

/// A failure to read the reversible Act notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// UTF-8 byte offset at which reading stopped.
    pub offset: usize,
    /// What made the text unreadable.
    pub message: String,
}

impl ParseError {
    pub(super) fn new(offset: usize, message: impl Into<String>) -> Self {
        Self {
            offset,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at byte {}: {}", self.offset, self.message)
    }
}

impl std::error::Error for ParseError {}

/// The named round-trip number, its denominator and its 100% judgement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundTrip {
    pub name: &'static str,
    pub matched: usize,
    pub total: usize,
    /// `matched / total`, as a percentage. An empty input reports `0.0`.
    pub share: f64,
    pub bar: f64,
    /// The distance below the Bar in percentage points.
    pub shortfall: f64,
}

impl RoundTrip {
    pub fn meets_bar(self) -> bool {
        self.total > 0 && self.share >= self.bar
    }
}

/// Write one Act in the readable, reversible notation.
pub fn write(act: &Act) -> String {
    print::act(act)
}

/// Read exactly one Act, refusing trailing or structurally incomplete text.
pub fn read(text: &str) -> Result<Act, ParseError> {
    parse::act(text)
}

/// Measure exact field preservation over the supplied Acts.
///
/// Equality is stated field by field here rather than delegated to a future
/// change in `PartialEq`: force, predicate, role/filler slots and all five
/// frame features must each survive.
pub fn round_trip<'a>(acts: impl IntoIterator<Item = &'a Act>) -> RoundTrip {
    let (mut matched, mut total) = (0usize, 0usize);
    for original in acts {
        total += 1;
        let restored = read(&write(original));
        if restored
            .as_ref()
            .is_ok_and(|read| same_fields(original, read))
        {
            matched += 1;
        }
    }
    let share = match total {
        0 => 0.0,
        _ => matched as f64 * 100.0 / total as f64,
    };
    RoundTrip {
        name: ROUND_TRIP_NAME,
        matched,
        total,
        share,
        bar: ROUND_TRIP_BAR,
        shortfall: (ROUND_TRIP_BAR - share).max(0.0),
    }
}

fn same_fields(a: &Act, b: &Act) -> bool {
    a.force == b.force
        && a.frame.pred == b.frame.pred
        && a.frame.slots == b.frame.slots
        && a.frame.tense == b.frame.tense
        && a.frame.aspect == b.frame.aspect
        && a.frame.modal == b.frame.modal
        && a.frame.voice == b.frame.voice
        && a.frame.polarity == b.frame.polarity
}

impl std::str::FromStr for Act {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        read(text)
    }
}
