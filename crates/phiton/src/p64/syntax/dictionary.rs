//! Keyed Part-of-Speech dictionary and dynamic registry.

use super::vocab::{keys, lex_file};
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

    /// The given dictionary: every class read from its file under `data/lex/`.
    pub fn default_lexicon() -> Self {
        let mut dict = Self::new();
        for (name, pos) in Self::given() {
            dict.register_batch(keys(name), pos);
        }
        dict
    }

    /// The given classes and their files, open and closed alike.
    fn given() -> [(&'static str, PartOfSpeech); 9] {
        [
            ("pronoun", PartOfSpeech::Pronoun),
            ("determiner", PartOfSpeech::Determiner),
            ("auxiliary", PartOfSpeech::Auxiliary),
            ("preposition", PartOfSpeech::Preposition),
            ("conjunction", PartOfSpeech::Conjunction),
            ("interjection", PartOfSpeech::Interjection),
            ("adverb", PartOfSpeech::Adverb),
            ("adjective", PartOfSpeech::Adjective),
            ("verb", PartOfSpeech::Verb),
        ]
    }

    /// The closed classes: small, certain and structural, so they override
    /// a mined open-class tag.
    fn closed() -> [(&'static str, PartOfSpeech); 6] {
        [
            ("pronoun", PartOfSpeech::Pronoun),
            ("determiner", PartOfSpeech::Determiner),
            ("auxiliary", PartOfSpeech::Auxiliary),
            ("preposition", PartOfSpeech::Preposition),
            ("conjunction", PartOfSpeech::Conjunction),
            ("interjection", PartOfSpeech::Interjection),
        ]
    }

    /// Registers `word` under `pos` and appends it to that class's file, so
    /// the next run is born knowing it. Returns whether the file gained it.
    pub fn learn(&mut self, word: &str, pos: PartOfSpeech) -> std::io::Result<bool> {
        self.register(word, pos);
        let name = Self::given().iter().find(|(_, p)| *p == pos).map(|(n, _)| *n).unwrap_or("noun");
        phistd::lexfile::append(lex_file(name), &word.trim().to_lowercase())
    }

    /// Builds a dictionary from a mined `word\ttag` lexicon (WordNet codes:
    /// `n` noun, `v` verb, `a`/`s` adjective, `r` adverb), layered under the
    /// authoritative hand-keyed closed classes - a mined open-class entry
    /// never overrides a known pronoun, determiner, auxiliary or preposition,
    /// because the closed classes are small, certain and structural. This is
    /// how the reader draws its word classes from the foundation lexicon
    /// (data/lex/pos.tsv) instead of a compiled word list. Pure: the caller
    /// passes the file's text so the crate reads no path of its own.
    pub fn from_mined_tsv(text: &str) -> Self {
        let mut dict = Self::new();
        // Open-class mined entries first.
        for line in text.lines() {
            let mut it = line.split('\t');
            let (Some(word), Some(tag)) = (it.next(), it.next()) else { continue };
            // The mined tag is a WordNet code or a set of them. Map a word to
            // one class for the single-valued dictionary: a purely nominal
            // word is a noun, a purely verbal word a verb, and so on. An
            // ambiguous word (noun-and-verb, "nv") is left to the richer
            // phisor lexicon, which keeps the raw tag; here it is skipped so
            // the dictionary never claims a single class it cannot justify.
            let codes = tag.trim();
            let pos = match codes {
                "n" | "nr" => PartOfSpeech::Noun,
                "v" => PartOfSpeech::Verb,
                "a" | "s" => PartOfSpeech::Adjective,
                "r" => PartOfSpeech::Adverb,
                _ => continue,
            };
            dict.register(word, pos);
        }
        // The authoritative closed classes override, so a word the mined file
        // tagged open is still read as its structural class.
        for (name, pos) in Self::closed() {
            dict.register_batch(keys(name), pos);
        }
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
    pub fn register_batch<S: AsRef<str>>(&mut self, words: &[S], pos: PartOfSpeech) {
        for w in words {
            self.register(w.as_ref(), pos);
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
