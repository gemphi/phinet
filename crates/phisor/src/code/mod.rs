//! `code` — a word written as its class at 16, 32 or 64 levels: the
//! modes, structure only (docs/phisor/code.md). Three arms write every
//! sentence in the same modes:
//!
//! | arm | what a class is |
//! |---|---|
//! | `Hash` | phiton's `classify_str`: the spelling's hash, modulo the level. Random buckets: the null |
//! | `Given` | phiton's `PosDictionary`: eleven parts of speech by hand, with a suffix fallback; one alphabet at every mode |
//! | `Learned` | a code per word, grown by halving with the exchange routine as deep as the data pays for; cut at 4, 5 or 6 bits. Nested: mode 16 is mode 64 coarsened |

mod learned;

pub use learned::{Code, BITS};

use phiton::p64::variants::{Engine, Level, P16, P32, P64};
use phiton::p64::PartOfSpeech;

/// Who writes a word as a class.
pub enum Coder {
    Hash,
    Given,
    Learned(Code),
}

/// The modes a run reads: 16, 32 and 64 levels.
pub const LEVELS: [Level; 3] = [Level::P16, Level::P32, Level::P64];

impl Coder {
    pub fn name(&self) -> &'static str {
        match self {
            Coder::Hash => "hash",
            Coder::Given => "given",
            Coder::Learned(_) => "learned",
        }
    }

    /// How many classes this arm writes at a level.
    pub fn alphabet(&self, level: Level) -> usize {
        match self {
            Coder::Given => PARTS,
            Coder::Hash | Coder::Learned(_) => level.count(),
        }
    }

    /// A word's class at a level.
    pub fn class(&self, word: &str, level: Level) -> u16 {
        match self {
            Coder::Hash => hash(word, level),
            Coder::Given => part(PartOfSpeech::classify(word)),
            Coder::Learned(code) => code.class(word, level.bits()),
        }
    }

    /// A sentence in a mode: each word its class.
    pub fn write(&self, words: &[String], level: Level) -> Vec<u16> {
        words.iter().map(|w| self.class(w, level)).collect()
    }
}

/// A sentence in a mode, as text: `c3 c7 c12 …`.
pub fn show(classes: &[u16]) -> String {
    classes.iter().map(|c| format!("c{c}")).collect::<Vec<_>>().join(" ")
}

fn hash(word: &str, level: Level) -> u16 {
    let v = match level {
        Level::P16 => P16::classify_str(word),
        Level::P32 => P32::classify_str(word),
        Level::P64 => P64::classify_str(word),
    };
    v.index as u16
}

/// The given alphabet's size: phiton's eleven parts of speech.
const PARTS: usize = 11;

fn part(p: PartOfSpeech) -> u16 {
    match p {
        PartOfSpeech::Noun => 0,
        PartOfSpeech::Pronoun => 1,
        PartOfSpeech::Verb => 2,
        PartOfSpeech::Auxiliary => 3,
        PartOfSpeech::Adjective => 4,
        PartOfSpeech::Adverb => 5,
        PartOfSpeech::Preposition => 6,
        PartOfSpeech::Conjunction => 7,
        PartOfSpeech::Determiner => 8,
        PartOfSpeech::Interjection => 9,
        PartOfSpeech::Unknown => 10,
    }
}
