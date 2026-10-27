//! Sentence structure parser and keyed Part-of-Speech dictionary.

pub mod dictionary;
pub mod parser;
#[cfg(test)]
mod tests;
pub mod vocab;

pub use dictionary::PosDictionary;
pub use parser::{SyntaxKey, SyntaxParser};
use serde::{Deserialize, Serialize};

/// Part-of-speech tags used in structural keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartOfSpeech {
    Noun,
    Pronoun,
    Verb,
    Auxiliary,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Determiner,
    Interjection,
    Unknown,
}

impl PartOfSpeech {
    /// Returns the short code for this POS tag.
    pub fn code(self) -> &'static str {
        match self {
            Self::Noun => "N",
            Self::Pronoun => "PRON",
            Self::Verb => "V",
            Self::Auxiliary => "AUX",
            Self::Adjective => "ADJ",
            Self::Adverb => "ADV",
            Self::Preposition => "PREP",
            Self::Conjunction => "CONJ",
            Self::Determiner => "DET",
            Self::Interjection => "INTJ",
            Self::Unknown => "?",
        }
    }

    /// Classifies a word into a POS tag using the global keyed dictionary.
    pub fn classify(word: &str) -> Self {
        PosDictionary::global().classify(word)
    }
}
