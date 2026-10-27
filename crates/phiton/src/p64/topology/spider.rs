//! Keyed Language Spider-Net relational topology.

use super::paragraph::ParagraphType;
use super::sentence::SentenceType;
use super::super::iching::Hexagram;
use super::super::syntax::{SyntaxKey, SyntaxParser};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// The language spider net — captures language structure without storing examples.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpiderNet {
    /// Sentence type variation counts at each level.
    pub sentence_counts: HashMap<u64, u32>,
    /// Paragraph type variation counts at each level.
    pub paragraph_counts: HashMap<u64, u32>,
    /// Links between sentence types (transitions).
    pub type_links: HashMap<u64, u32>,
    /// Keyed map from structural syntax keys to their resonant I Ching Hexagram IDs (0..63).
    pub key_to_hexagrams: HashMap<String, HashSet<u8>>,
    /// Keyed map from Hexagram IDs (0..63) to sets of corresponding syntax keys.
    pub hexagram_to_keys: HashMap<u8, HashSet<String>>,
    /// Transitions between Hexagram harmonic states ("Song of Speech Shapes").
    pub hexagram_transitions: HashMap<u64, u32>,
    /// Speech part shape relations (e.g. "PRON" -> {"V", "AUX"}).
    pub pos_shape_relations: HashMap<String, HashSet<String>>,
    /// Total sentences processed.
    pub total_sentences: u32,
    /// Total paragraphs processed.
    pub total_paragraphs: u32,
}

impl SpiderNet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Processes a text, extracting syntactic keys and building the 64-hexagram spider net.
    pub fn process_text(&mut self, text: &str) -> ParagraphType {
        let sentences = crate::p64::words::Tokenizer::split_sentences(text);
        let mut stypes = Vec::new();
        let mut hexagrams = Vec::new();

        for s in &sentences {
            let st = SentenceType::classify(s);
            let key = SyntaxParser::parse(s);
            let hex = super::super::iching::of_syntax_key(&key);

            self.record_sentence(&st);
            self.record_syntax_key(&key, hex);

            stypes.push(st);
            hexagrams.push(hex);
        }

        for w in stypes.windows(2) {
            self.link_types(&w[0], &w[1]);
        }

        for w in hexagrams.windows(2) {
            self.link_hexagrams(w[0], w[1]);
        }

        let ptype = ParagraphType::classify(&stypes);
        self.record_paragraph(&ptype);
        ptype
    }

    fn record_sentence(&mut self, st: &SentenceType) {
        self.total_sentences += 1;
        let key = st.v64.index as u64;
        *self.sentence_counts.entry(key).or_insert(0) += 1;
    }

    fn record_paragraph(&mut self, pt: &ParagraphType) {
        self.total_paragraphs += 1;
        let key = pt.v64.index as u64;
        *self.paragraph_counts.entry(key).or_insert(0) += 1;
    }

    fn record_syntax_key(&mut self, key: &SyntaxKey, hex: Hexagram) {
        self.key_to_hexagrams
            .entry(key.key.clone())
            .or_default()
            .insert(hex.id);

        self.hexagram_to_keys
            .entry(hex.id)
            .or_default()
            .insert(key.key.clone());

        for window in key.parts.windows(2) {
            let from_code = window[0].code().to_string();
            let to_code = window[1].code().to_string();
            self.pos_shape_relations
                .entry(from_code)
                .or_default()
                .insert(to_code);
        }
    }

    fn link_types(&mut self, a: &SentenceType, b: &SentenceType) {
        let key = a.v64.index as u64 * 64 + b.v64.index as u64;
        *self.type_links.entry(key).or_insert(0) += 1;
    }

    fn link_hexagrams(&mut self, a: Hexagram, b: Hexagram) {
        let key = a.id as u64 * 64 + b.id as u64;
        *self.hexagram_transitions.entry(key).or_insert(0) += 1;
    }

    /// Spins a structural key by $\Delta\theta$ to find resonant alternative syntax forms.
    pub fn spin_structure(&self, syntax_key: &str, delta_phase: f64) -> Vec<String> {
        if let Some(hex_ids) = self.key_to_hexagrams.get(syntax_key) {
            let mut results = HashSet::new();
            for &id in hex_ids {
                let spun_hex = Hexagram::from_id(id).spin(delta_phase);
                if let Some(keys) = self.hexagram_to_keys.get(&spun_hex.id) {
                    for k in keys {
                        results.insert(k.clone());
                    }
                }
            }
            results.into_iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Returns the diversity of sentence types seen (0.0 to 1.0).
    pub fn sentence_diversity(&self) -> f64 {
        if self.total_sentences == 0 {
            return 0.0;
        }
        let unique = self.sentence_counts.len() as f64;
        unique / 64.0
    }

    /// Returns the diversity of paragraph types seen (0.0 to 1.0).
    pub fn paragraph_diversity(&self) -> f64 {
        if self.total_paragraphs == 0 {
            return 0.0;
        }
        let unique = self.paragraph_counts.len() as f64;
        unique / 64.0
    }
}
