//! `lexicon` - the reader's bridge to the mined foundation lexicon.
//!
//! The English reader (crate::ladder::act) decides word classes - is this a
//! verb, a noun? - to parse a sentence. On its own it knows only the small
//! closed sets its own rules name. This module lets the harness prime it with
//! the foundation's mined part-of-speech lexicon (data/lex/pos.tsv: tens of
//! thousands of words, each tagged with a set of WordNet codes learned over
//! the corpora), so the reader draws open-class knowledge from what was
//! learned rather than from a hand list.
//!
//! The mined tags are a *set* per word: "arrive" is "v" (verb only), "water"
//! is "nv" (a noun and a verb). Keeping the set - not flattening it - is what
//! lets the reader stay sound: an imperative generalises only to a word that
//! is unambiguously a verb ([`verb_only`]), so "paint" ("nv") does not turn
//! "water boils" into a command, while "arrive"/"sing" ("v") are recognised
//! beyond the reader's hand list.
//!
//! The crate stays pure: the harness reads the file and calls [`prime`] with
//! its text once; the reader queries return `None`/`false` until primed.
//! phisor's own tests read the primed reader, in any order (`prime`'s module
//! says why). Only the foundation lexicon is a source here - never the gold -
//! so priming does not train the reader on its own yardstick.

pub mod agreement;
pub mod auxiliary;
pub mod contraction;
pub mod irregular;
pub mod list;
mod prime;
pub mod suffix;
pub mod suffix_read;
#[cfg(test)]
mod tests;

pub use irregular::{parse_irregulars, IrregularForms, Irregulars};
pub use prime::prime_canonical;
use std::collections::HashMap;
use std::sync::OnceLock;

/// word -> the raw WordNet tag string (a set of codes, e.g. "v", "n", "nv").
static TAGS: OnceLock<HashMap<String, String>> = OnceLock::new();

/// The closed-class function words (data/lex/function.txt): pronouns,
/// determiners, prepositions, conjunctions, auxiliaries and the like. A
/// function word is never pluralised or conjugated, which stops the reader
/// mangling "his" to "hi" or the realiser making "we" into "wes".
static FUNCTION: OnceLock<std::collections::HashSet<String>> = OnceLock::new();

/// Prime the reader with the mined `word\ttag` lexicon text (data/lex/pos.tsv).
/// Idempotent: the first prime wins.
pub fn prime(pos_tsv: &str) {
    prime::ensure();
    set_tags(pos_tsv);
}

fn set_tags(pos_tsv: &str) {
    let mut map = HashMap::new();
    for line in pos_tsv.lines() {
        let mut it = line.split('\t');
        if let (Some(w), Some(tag)) = (it.next(), it.next()) {
            let w = w.trim();
            let tag = tag.trim();
            if !w.is_empty() && !tag.is_empty() {
                map.insert(w.to_lowercase(), tag.to_string());
            }
        }
    }
    let _ = TAGS.set(map);
}

/// The mined tags, once primed.
fn tag_table() -> Option<&'static HashMap<String, String>> {
    prime::ensure();
    TAGS.get()
}

/// Whether a foundation lexicon has been primed.
pub fn is_primed() -> bool {
    tag_table().is_some()
}

/// The raw tag set for a word, or `None` when unprimed or unknown.
pub fn tag(word: &str) -> Option<&'static str> {
    tag_table().and_then(|m| m.get(&word.to_lowercase()).map(String::as_str))
}

/// Whether the lexicon knows the word to be a verb and nothing else - the
/// unambiguous case an imperative may generalise to. `false` when unprimed,
/// unknown, or ambiguous (e.g. "nv"), so the reader never mistakes an
/// ambiguous noun/verb lead for a command.
pub fn verb_only(word: &str) -> bool {
    matches!(tag(word), Some(t) if t.contains('v') && !t.contains('n'))
}

/// Whether the word can be a verb at all (verb-only or verb-and-noun).
pub fn verb_capable(word: &str) -> bool {
    matches!(tag(word), Some(t) if t.contains('v'))
}

/// Whether the word can be a noun at all.
pub fn noun_capable(word: &str) -> bool {
    matches!(tag(word), Some(t) if t.contains('n'))
}

/// Prime the closed-class function words from data/lex/function.txt (words
/// separated by whitespace). Idempotent.
pub fn prime_function(text: &str) {
    prime::ensure();
    set_function(text);
}

fn set_function(text: &str) {
    let set: std::collections::HashSet<String> =
        text.split_whitespace().map(|w| w.trim().to_lowercase()).filter(|w| !w.is_empty()).collect();
    let _ = FUNCTION.set(set);
}

/// Whether a word is a closed-class function word, never inflected.
pub fn function_word(word: &str) -> bool {
    prime::ensure();
    FUNCTION.get().is_some_and(|s| s.contains(&word.to_lowercase()))
}

/// Whether it is safe to inflect a word (pluralise a noun, conjugate a
/// verb). A function word, or a word the mined tags call purely adverbial,
/// is never inflected; an unprimed lexicon returns true so the reader keeps
/// its own behaviour. This keeps real text's "always", "his", "we" whole.
pub fn inflectable(word: &str) -> bool {
    if function_word(word) {
        return false;
    }
    match tag(word) {
        Some(t) if !t.contains('n') && !t.contains('v') && t.contains('r') => false,
        _ => true,
    }
}

/// The irregular verbs primed from the Guide's Appendix 1.4.
static IRREGULAR: OnceLock<Irregulars> = OnceLock::new();

/// Prime the reader and realiser with the Guide's irregular verbs, from the
/// Guide's own text. Idempotent: the first prime wins.
pub fn prime_irregular(guide_text: &str) {
    prime::ensure();
    set_irregular(guide_text);
}

fn set_irregular(guide_text: &str) {
    let _ = IRREGULAR.set(parse_irregulars(guide_text));
}

/// The Guide's irregular verbs, once primed.
fn irregular_table() -> Option<&'static Irregulars> {
    prime::ensure();
    IRREGULAR.get()
}

/// The simple past of an irregular verb, or `None` when unprimed or regular.
pub fn irregular_past(lemma: &str) -> Option<String> {
    irregular_table()?.forms(lemma)?.past.first().cloned()
}

/// The past participle of an irregular verb, or `None` when unprimed or regular.
pub fn irregular_participle(lemma: &str) -> Option<String> {
    irregular_table()?.forms(lemma)?.participle.first().cloned()
}

/// The verb a form is the simple past of ("met" -> "meet"), or `None`.
pub fn lemma_of_past(word: &str) -> Option<String> {
    irregular_table()?.lemma_of_past(word).map(str::to_string)
}

/// The verb a form is the past participle of ("gone" -> "go"), or `None`.
pub fn lemma_of_participle(word: &str) -> Option<String> {
    irregular_table()?.lemma_of_participle(word).map(str::to_string)
}

