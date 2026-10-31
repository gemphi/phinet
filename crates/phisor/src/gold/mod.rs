//! `gold` — the 150 frozen gold frames: for each of 150 English sentences,
//! the [`Act`] that is the right reading of it (R5.11, R10.10).
//!
//! The frames are the act height's yardstick, so they are written and
//! fingerprinted *before* the learner first runs on them. A per-field score
//! measured against frames written after the learner saw them is not a
//! measurement, and the freeze is what makes it one: [`read`] asserts the
//! count at [`COUNT`] and the file's fingerprint at [`FINGERPRINT`], so
//! editing a row, adding one or dropping one fails the load rather than
//! quietly moving the yardstick.
//!
//! | file | what |
//! |---|---|
//! | `mod` | [`Gold`], [`Entry`], [`Flaw`], the count, the fingerprint, [`read`] |
//! | `read` | the row grammar and the filler grammar of the frozen file |
//!
//! The frozen file is `phisor/tests/gold/frames.tsv`: nine tab-separated
//! columns, described in full in its own header. It sits under `tests/`
//! because it is a frozen input rather than source, and is read here with
//! `include_str!` so the fingerprint is over the bytes the build saw.
//!
//! This is the design's `GoldFrames` under the workspace's naming rule,
//! which wants a type name singular: [`Gold`] is the set, [`Entry`] one
//! frame of it.

mod read;
#[cfg(test)]
mod tests;

use crate::ir::Act;

/// Where the frozen file lives, as a failure must name it.
pub const PATH: &str = "phinet/crates/phisor/tests/gold/frames.tsv";

/// The declared count (R5.11): 150, neither more nor fewer.
pub const COUNT: usize = 150;

/// The pinned fingerprint of the frozen file, from [`fingerprint`].
pub const FINGERPRINT: u64 = 0xba68_8be5_a28b_730c;

/// The frozen file's bytes, as the build saw them.
const TEXT: &str = include_str!("../../tests/gold/frames.tsv");

/// One gold frame: the English it is gold for, and the Act that reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    /// The row's place in the file, 1 to 150: a failure names it.
    pub id: usize,
    /// The sentence, normalised — lower case, words and the terminating
    /// mark, which R10.11 keeps.
    pub sentence: String,
    /// The right reading: a Force over a Frame, every field given.
    pub act: Act,
}

/// The frozen set: its 150 entries and the fingerprint they were read at.
#[derive(Clone, Debug, PartialEq)]
pub struct Gold {
    pub entries: Vec<Entry>,
    pub fingerprint: u64,
}

/// Why the frozen set did not load. The first two are the freeze itself —
/// each names what was read beside what is declared — and the third names
/// the row, the column and the cell that would not read.
#[derive(Clone, Debug, PartialEq)]
pub enum Flaw {
    Count { read: usize, declared: usize },
    Fingerprint { read: u64, declared: u64 },
    Row { id: usize, column: &'static str, cell: String, why: String },
}

/// Read the frozen set, asserting the count and the fingerprint.
pub fn read() -> Result<Gold, Flaw> {
    of(TEXT)
}

/// The same loader over given text, so a test can show what a changed row,
/// a dropped row or an unreadable cell does without editing the frozen
/// file.
pub fn of(text: &str) -> Result<Gold, Flaw> {
    let entries = read::all(text)?;
    match entries.len() {
        COUNT => {}
        n => return Err(Flaw::Count { read: n, declared: COUNT }),
    }
    let fingerprint = fingerprint(text);
    match fingerprint {
        FINGERPRINT => Ok(Gold { entries, fingerprint }),
        n => Err(Flaw::Fingerprint { read: n, declared: FINGERPRINT }),
    }
}

/// A frozen file's fingerprint: `phistd::mix64` folded over its bytes with
/// carriage returns dropped, so the line ending a checkout happened to
/// choose cannot move the number.
pub fn fingerprint(text: &str) -> u64 {
    text.bytes().filter(|b| *b != b'\r').fold(0, |h, b| phistd::mix64(h ^ u64::from(b)))
}

impl Gold {
    /// The Acts alone, in file order: what R10.10 reads exhaustively.
    pub fn acts(&self) -> impl Iterator<Item = &Act> {
        self.entries.iter().map(|e| &e.act)
    }

    /// Each Act beside the sentence it is gold for, in file order.
    pub fn pairs(&self) -> impl Iterator<Item = (&Act, &str)> {
        self.entries.iter().map(|e| (&e.act, e.sentence.as_str()))
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl std::fmt::Display for Flaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flaw::Count { read, declared } => {
                write!(f, "{PATH} holds {read} frames, not the declared {declared}")
            }
            Flaw::Fingerprint { read, declared } => {
                write!(f, "{PATH} fingerprints {read:#018x}, not the pinned {declared:#018x}")
            }
            Flaw::Row { id, column, cell, why } => {
                write!(f, "{PATH} row {id}, column {column}: {cell:?} {why}")
            }
        }
    }
}
