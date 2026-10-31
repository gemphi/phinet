//! The Mode gate and the write itself. A Mode is the one word code cut at
//! 4, 5 or 6 bits (R5.3), and a Mode that is not 16, 32 or 64 never reaches
//! the write: [`mode`] errors first, naming the Mode given and the three it
//! accepts, so nothing is written (R5.16).

use super::LadderError;
use crate::code::Coder;
use crate::guide::Sentence;
use phiton::p64::variants::Level;

/// The three Modes, as they are named.
pub const MODES: [&str; 3] = ["16", "32", "64"];

/// The fewest sentences a text may hold: one (R5.1).
pub const FEWEST: usize = 1;

/// The most sentences a text may hold: ten thousand (R5.1).
pub const MOST: usize = 10_000;

/// A named Mode as the level it cuts the code at; an unknown name errors,
/// naming the Mode given and the three accepted (R5.16). The bare number
/// and phiton's own `P16` spelling both read.
pub fn mode(given: &str) -> Result<Level, LadderError> {
    let bare = given.trim();
    let bare = bare.strip_prefix(['p', 'P']).unwrap_or(bare);
    match bare {
        "16" => Ok(Level::P16),
        "32" => Ok(Level::P32),
        "64" => Ok(Level::P64),
        _ => Err(LadderError::UnknownMode {
            given: given.to_string(),
            accepted: MODES,
        }),
    }
}

/// A Mode as it is named: the inverse of [`mode`], for a report's key.
pub fn named(level: Level) -> &'static str {
    match level {
        Level::P16 => MODES[0],
        Level::P32 => MODES[1],
        Level::P64 => MODES[2],
    }
}

/// Whether a text holds a writable count of sentences: 1 to 10,000 (R5.1).
pub fn holds(text: &[Sentence]) -> bool {
    (FEWEST..=MOST).contains(&text.len())
}

/// Write a text in a Mode with a coder a caller already holds, so a run
/// that writes one arm at three Modes grows its code once. One class token
/// per word, in the text's own word order; a text outside [`FEWEST`]`..=`
/// [`MOST`] errors and writes nothing (R5.1).
pub fn write(text: &[Sentence], level: Level, coder: &Coder) -> Result<Vec<Vec<u16>>, LadderError> {
    match holds(text) {
        true => Ok(text.iter().map(|s| coder.write(&s.words, level)).collect()),
        false => Err(LadderError::TextOutOfRange {
            sentences: text.len(),
        }),
    }
}

/// A written text as the class tokens it is, one line per sentence:
/// `c3 c7 c12 …`. The tokens are the whole of what is written — a word of
/// the text is never among them, however a word happens to be spelled
/// (R5.1).
pub fn show(written: &[Vec<u16>]) -> Vec<String> {
    written.iter().map(|s| crate::code::show(s)).collect()
}
