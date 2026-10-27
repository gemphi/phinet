//! Searle's speech act classification — 16/32/64 variants.
//!
//! John Searle's taxonomy of illocutionary acts provides the framework
//! for classifying *what a sentence does* (not just what it says).
//!
//! # Base 5 Categories (Searle)
//!
//! 1. **Assertive** — stating facts, claiming truth
//! 2. **Directive** — requesting, commanding, asking
//! 3. **Commissive** — promising, committing, obligating
//! 4. **Expressive** — expressing feelings, attitudes
//! 5. **Declaration** — bringing about a state by uttering
//!
//! # P64 Expansion
//!
//! - **P16**: 16 sub-categories (5 base × ~3 sub-types each)
//! - **P32**: 32 sub-categories (16 × 2 further refinements)
//! - **P64**: 64 sub-categories (32 × 2, full granularity)

use super::config::Config;
use super::syntax::SyntaxKey;
use super::variants::{P16, P32, P64, Engine, Level, Variation};
use serde::{Deserialize, Serialize};

/// The 5 base Searle speech act categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeechAct {
    /// Stating facts, claiming truth.
    Assertive,
    /// Requesting, commanding, asking.
    Directive,
    /// Promising, committing, obligating.
    Commissive,
    /// Expressing feelings, attitudes.
    Expressive,
    /// Bringing about a state by uttering.
    Declaration,
}

impl SpeechAct {
    /// Returns the label for this speech act.
    pub fn label(self) -> &'static str {
        match self {
            Self::Assertive => "assertive",
            Self::Directive => "directive",
            Self::Commissive => "commissive",
            Self::Expressive => "expressive",
            Self::Declaration => "declaration",
        }
    }

    /// Classifies a sentence into a speech act using structural heuristics.
    pub fn classify(key: &SyntaxKey) -> Self {
        let k = key.key.as_str();
        // A subject then its verb asserts when an object or a phrase follows,
        // and directs when the verb ends it.
        let asserts = k.contains("PREP") || k.ends_with("N") || k.ends_with("PRON");
        match () {
            _ if k.contains("INTJ") || k.contains("ADV+ADJ") => Self::Expressive,
            _ if (k.starts_with("PRON+V") || k.starts_with("N+V")) && asserts => Self::Assertive,
            _ if k.starts_with("PRON+V") || k.starts_with("N+V") => Self::Directive,
            _ if k.contains("V+PREP") || k.contains("AUX+V") => Self::Commissive,
            _ if k.contains("V+N") || k.ends_with("V") => Self::Directive,
            _ => Self::Assertive,
        }
    }

    /// Returns the 16-level sub-category index for this speech act.
    pub fn sub16(self, key: &SyntaxKey) -> Variation {
        let base = self as u8;
        let sub = (key.len() as u8) % 3;
        let idx = base * 3 + sub;
        let cfg = Config::global();
        P16::classify_hash(cfg.hash_base(idx as u64, Level::P16))
    }

    /// Returns the 32-level sub-category index for this speech act.
    pub fn sub32(self, key: &SyntaxKey) -> Variation {
        let base = self as u8;
        let sub = (key.len() as u8) % 6;
        let idx = base * 6 + sub;
        let cfg = Config::global();
        P32::classify_hash(cfg.hash_base(idx as u64, Level::P32))
    }

    /// Returns the 64-level sub-category index for this speech act.
    pub fn sub64(self, key: &SyntaxKey) -> Variation {
        let base = self as u8;
        let sub = (key.len() as u8) % 12;
        let idx = base * 12 + sub;
        let cfg = Config::global();
        P64::classify_hash(cfg.hash_base(idx as u64, Level::P64))
    }
}

/// Classifies sentences into Searle speech acts with P64 granularity.
pub struct SearleClassifier;

impl SearleClassifier {
    /// Classifies a single sentence.
    pub fn classify(sentence: &str) -> (SpeechAct, SyntaxKey) {
        let key = crate::p64::SyntaxParser::parse(sentence);
        let act = SpeechAct::classify(&key);
        (act, key)
    }

    /// Classifies a sentence at all three P64 levels.
    pub fn classify_full(sentence: &str) -> SearleClassification {
        let key = crate::p64::SyntaxParser::parse(sentence);
        let act = SpeechAct::classify(&key);
        SearleClassification {
            act,
            v16: act.sub16(&key),
            v32: act.sub32(&key),
            v64: act.sub64(&key),
            key,
        }
    }
}

/// Full Searle classification at all P64 levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearleClassification {
    pub act: SpeechAct,
    pub key: SyntaxKey,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}
