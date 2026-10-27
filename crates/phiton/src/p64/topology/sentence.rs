//! Sentence type classification for P64 language topology.

use super::super::config::Config;
use super::super::searle::SpeechAct;
use super::super::syntax::{PartOfSpeech, SyntaxKey, SyntaxParser};
use super::super::variants::{P16, P32, P64, Engine, Level, Variation};
use serde::{Deserialize, Serialize};

/// Sentence type — 16 base types expandable to 32 and 64.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SentenceType {
    pub base: u8,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}

impl SentenceType {
    /// Classifies a sentence into a sentence type.
    pub fn classify(sentence: &str) -> Self {
        let trimmed = sentence.trim();
        let key = SyntaxParser::parse(trimmed);
        let act = SpeechAct::classify(&key);

        let base = Self::base_type(trimmed, &key, act);
        let cfg = Config::global();
        let v16 = P16::classify_hash(cfg.hash_base(base as u64, Level::P16));
        let v32 = P32::classify_hash(cfg.hash_base(base as u64, Level::P32));
        let v64 = P64::classify_hash(cfg.hash_base(base as u64, Level::P64));

        Self { base, v16, v32, v64 }
    }

    fn base_type(text: &str, key: &SyntaxKey, act: SpeechAct) -> u8 {
        match (text.chars().last(), key.parts.first()) {
            (Some('?'), _) => 1, // Interrogative
            (Some('!'), _) => 3, // Exclamatory
            (_, Some(PartOfSpeech::Verb | PartOfSpeech::Auxiliary)) => 2, // Imperative
            _ => match act {
                SpeechAct::Assertive => 0,
                SpeechAct::Directive => 4,
                SpeechAct::Commissive => 5,
                SpeechAct::Expressive => 6,
                SpeechAct::Declaration => 7,
            },
        }
    }

    pub fn label(self) -> &'static str {
        match self.base {
            0 => "declarative",
            1 => "interrogative",
            2 => "imperative",
            3 => "exclamatory",
            4 => "directive",
            5 => "commissive",
            6 => "expressive",
            7 => "declaration",
            _ => "complex",
        }
    }
}
