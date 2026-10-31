//! `guide` — the English guide as phisor reads it (docs/phisor/guide.md):
//! every sentence of its units with what the layout says it is for, and
//! the Key's answers, which are never told.
//!
//! | file | what |
//! |---|---|
//! | `units` | the units' lines: an exercise line INSTRUCTS, the rest EXPLAINS (docs/201's reader), each piece with its mark |
//! | `key` | the Key at the back: every answer, unit by unit (phiano's workshop reader) |
//! | `split` | docs/203's rule: one in ten held out, one in twenty for dev; every answer held out |
//! | `holdout` | the Holdout_Guard: no held-out sentence and no Key line reaches a training input (requirements §10) |

pub mod holdout;
mod key;
mod split;
mod units;
#[cfg(test)]
mod tests;

pub use split::{Part, Split};

/// What the layout says a sentence is for (docs/201).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Function {
    Explain,
    Instruct,
    Answer,
}

/// How a sentence ended in the text: the mark is taken off the words, so a
/// reader can only sense it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mark {
    Stop,
    Question,
    Exclamation,
    /// The line ended first: the layout broke the sentence.
    Line,
}

/// One sentence of the guide: its words (lower case, words only), what it
/// is for, how it ended, the unit it is in, and where it stands in the run of
/// sentences it belongs to (a unit's text, or the Key).
#[derive(Clone, Debug, PartialEq)]
pub struct Sentence {
    pub words: Vec<String>,
    pub function: Function,
    pub mark: Mark,
    pub unit: usize,
    /// The run: 0 for a unit's text, 1 for the Key. Two sentences follow
    /// each other only within one run and one unit.
    pub run: u8,
}

/// The guide's sentences in their order: the units' text, then the Key.
pub fn read(text: &str, key: &str) -> Vec<Sentence> {
    let mut out = units::read(text);
    out.extend(key::read(key));
    out
}

impl Function {
    pub const ALL: [Function; 3] = [Function::Explain, Function::Instruct, Function::Answer];

    pub fn name(self) -> &'static str {
        match self {
            Function::Explain => "explain",
            Function::Instruct => "instruct",
            Function::Answer => "answer",
        }
    }
}

impl Mark {
    pub const ALL: [Mark; 4] = [Mark::Stop, Mark::Question, Mark::Exclamation, Mark::Line];

    pub fn name(self) -> &'static str {
        match self {
            Mark::Stop => "stop",
            Mark::Question => "question",
            Mark::Exclamation => "exclamation",
            Mark::Line => "line",
        }
    }

    /// The mark a terminator stands for; `None` when it is not one.
    pub fn of(c: char) -> Option<Mark> {
        match c {
            '.' => Some(Mark::Stop),
            '?' => Some(Mark::Question),
            '!' => Some(Mark::Exclamation),
            _ => None,
        }
    }
}

/// A piece's words: phiecs's tokens (lower case, the typographic apostrophe
/// held between letters), words only.
pub(crate) fn words(s: &str) -> Vec<String> {
    phiecs::speak::tokens(s).into_iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect()
}

/// The shortest sentence kept, in words, as docs/201 and docs/203 keep it.
pub(crate) const SHORTEST: usize = 4;
