//! Paragraph and discourse type classification for P64 language topology.

use super::sentence::SentenceType;
use super::super::variants::{P16, P32, P64, Engine, Variation};
use serde::{Deserialize, Serialize};

/// Paragraph type — composed from sentence types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParagraphType {
    pub base: u8,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}

impl ParagraphType {
    /// Classifies a sequence of sentence types into a paragraph type.
    pub fn classify(sentence_types: &[SentenceType]) -> Self {
        if sentence_types.is_empty() {
            return Self {
                base: 0,
                v16: P16::classify_str(""),
                v32: P32::classify_str(""),
                v64: P64::classify_str(""),
            };
        }

        let base = Self::base_type(sentence_types);
        let hash: u64 = sentence_types
            .iter()
            .map(|st| st.base as u64)
            .fold(0u64, |acc, b| acc * 16 + b);
        let v16 = P16::classify_hash(hash);
        let v32 = P32::classify_hash(hash);
        let v64 = P64::classify_hash(hash);

        Self { base, v16, v32, v64 }
    }

    fn base_type(types: &[SentenceType]) -> u8 {
        // The share of the paragraph's sentences that are of one base type
        // (NaN for an empty paragraph, which no threshold passes: mixed).
        let n = types.len() as f64;
        let share = |base: u8| types.iter().filter(|t| t.base == base).count() as f64 / n;
        match (share(1) > 0.5, share(2) > 0.5, share(3) > 0.5, share(0) > 0.7) {
            (true, _, _, _) => 1,
            (_, true, _, _) => 2,
            (_, _, true, _) => 3,
            (_, _, _, true) => 0,
            _ => 4,
        }
    }

    pub fn label(self) -> &'static str {
        match self.base {
            0 => "narrative",
            1 => "interrogative",
            2 => "directive",
            3 => "expressive",
            4 => "mixed",
            _ => "complex",
        }
    }
}
