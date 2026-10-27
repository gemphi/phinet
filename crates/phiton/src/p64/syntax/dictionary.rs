//! Keyed Part-of-Speech dictionary and dynamic registry.

use super::vocab::*;
use super::PartOfSpeech;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

/// A keyed dictionary mapping words to POS tags and vice versa.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PosDictionary {
    /// Keyed forward map: word -> PartOfSpeech
    pub word_to_pos: HashMap<String, PartOfSpeech>,
    /// Keyed reverse map: PartOfSpeech -> Set of words
    pub pos_to_words: HashMap<PartOfSpeech, HashSet<String>>,
}

impl PosDictionary {
    /// Creates an empty dictionary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds the standard default keyed dictionary.
    pub fn default_lexicon() -> Self {
        let mut dict = Self::new();
        dict.register_batch(PRONOUN_KEYS, PartOfSpeech::Pronoun);
        dict.register_batch(DETERMINER_KEYS, PartOfSpeech::Determiner);
        dict.register_batch(AUXILIARY_KEYS, PartOfSpeech::Auxiliary);
        dict.register_batch(PREPOSITION_KEYS, PartOfSpeech::Preposition);
        dict.register_batch(CONJUNCTION_KEYS, PartOfSpeech::Conjunction);
        dict.register_batch(INTERJECTION_KEYS, PartOfSpeech::Interjection);
        dict.register_batch(ADVERB_KEYS, PartOfSpeech::Adverb);
        dict.register_batch(ADJECTIVE_KEYS, PartOfSpeech::Adjective);
        dict.register_batch(VERB_KEYS, PartOfSpeech::Verb);
        dict
    }

    /// Accesses the global static keyed dictionary singleton.
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<PosDictionary> = OnceLock::new();
        INSTANCE.get_or_init(Self::default_lexicon)
    }

    /// Registers a single word under a POS tag.
    pub fn register(&mut self, word: &str, pos: PartOfSpeech) {
        let w = word.trim().to_lowercase();
        if !w.is_empty() {
            self.word_to_pos.insert(w.clone(), pos);
            self.pos_to_words.entry(pos).or_default().insert(w);
        }
    }

    /// Registers a batch of words under a POS tag.
    pub fn register_batch(&mut self, words: &[&str], pos: PartOfSpeech) {
        for &w in words {
            self.register(w, pos);
        }
    }

    /// Queries the POS tag for a given word.
    pub fn lookup(&self, word: &str) -> Option<PartOfSpeech> {
        let w = word.trim().to_lowercase();
        self.word_to_pos.get(&w).copied()
    }

    /// Returns all registered words for a given POS category.
    pub fn words_for(&self, pos: PartOfSpeech) -> Vec<String> {
        self.pos_to_words
            .get(&pos)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Classifies a word using keyed lookup and fallback morphological heuristics.
    pub fn classify(&self, word: &str) -> PartOfSpeech {
        let w = word.trim().to_lowercase();
        if let Some(pos) = self.lookup(&w) {
            return pos;
        }

        if Self::looks_like_verb(&w) {
            PartOfSpeech::Verb
        } else {
            PartOfSpeech::Noun
        }
    }

    fn looks_like_verb(w: &str) -> bool {
        w.ends_with("ing") || w.ends_with("ed") || (w.ends_with('s') && w.len() > 3)
    }
}
