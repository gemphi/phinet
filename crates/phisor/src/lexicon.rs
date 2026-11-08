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
//! its text once; the reader queries return `None`/`false` until primed, so
//! phisor's own tests (which do not ship the file) use the reader's built-in
//! rules unchanged. Only the foundation lexicon is a source here - never the
//! gold - so priming does not train the reader on its own yardstick.

pub mod agreement;
pub mod auxiliary;
pub mod contraction;
pub mod irregular;
pub mod lists;
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

/// Whether a foundation lexicon has been primed.
pub fn is_primed() -> bool {
    TAGS.get().is_some()
}

/// The raw tag set for a word, or `None` when unprimed or unknown.
pub fn tag(word: &str) -> Option<&'static str> {
    TAGS.get().and_then(|m| m.get(&word.to_lowercase()).map(String::as_str))
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
    let set: std::collections::HashSet<String> =
        text.split_whitespace().map(|w| w.trim().to_lowercase()).filter(|w| !w.is_empty()).collect();
    let _ = FUNCTION.set(set);
}

/// Whether a word is a closed-class function word, never inflected.
pub fn function_word(word: &str) -> bool {
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

/// An irregular verb's forms, as the Guide's Appendix 1.4 lists them. A form
/// may have alternatives ("got/gotten", "shown/showed"); the first is the
/// one the realiser writes, and every one is recognised on reading.
#[derive(Clone, Debug, PartialEq)]
pub struct IrregularForms {
    pub past: Vec<String>,
    pub participle: Vec<String>,
}

/// The irregular verbs, with the reverse maps the reader needs.
#[derive(Clone, Debug, Default)]
pub struct Irregulars {
    by_lemma: HashMap<String, IrregularForms>,
    past_to_lemma: HashMap<String, String>,
    participle_to_lemma: HashMap<String, String>,
}

impl Irregulars {
    /// How many irregular verbs were read.
    pub fn len(&self) -> usize {
        self.by_lemma.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_lemma.is_empty()
    }

    pub fn forms(&self, lemma: &str) -> Option<&IrregularForms> {
        self.by_lemma.get(lemma)
    }

    pub fn lemma_of_past(&self, word: &str) -> Option<&str> {
        self.past_to_lemma.get(word).map(String::as_str)
    }

    pub fn lemma_of_participle(&self, word: &str) -> Option<&str> {
        self.participle_to_lemma.get(word).map(String::as_str)
    }
}

/// Read the Guide's table of irregular verbs (Appendix 1.4) out of its text.
/// The table is two blocks of `infinitive  past simple  past participle` side
/// by side, so a line carries one or two verbs; a pronunciation note in
/// brackets or starred is dropped, and the table ends at the `*` footnote.
/// `be` is skipped: the reader and realiser have `be` forms of their own.
/// Pure, so it can be tested on a snippet without priming anything.
pub fn parse_irregulars(text: &str) -> Irregulars {
    let mut out = Irregulars::default();
    let mut lines = text.lines();
    // The table sits after its own heading; find the column header after it.
    let mut in_table = false;
    let mut seen_heading = false;
    let mut rows: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();
    for line in lines.by_ref() {
        let trimmed = line.trim();
        if !seen_heading {
            seen_heading = trimmed.contains("List of irregular verbs");
            continue;
        }
        if !in_table {
            in_table = trimmed.starts_with("infinitive") && trimmed.contains("past participle");
            continue;
        }
        if trimmed.starts_with('*') {
            break;
        }
        let tokens: Vec<&str> = trimmed
            .split_whitespace()
            .filter(|t| !t.starts_with('[') && !t.contains('*'))
            .collect();
        if tokens.is_empty() || tokens.len() % 3 != 0 || tokens.len() > 6 {
            continue;
        }
        for chunk in tokens.chunks(3) {
            let split = |s: &str| -> Vec<String> {
                s.split('/').map(|a| a.trim().to_lowercase()).filter(|a| !a.is_empty()).collect()
            };
            rows.push((chunk[0].to_lowercase(), split(chunk[1]), split(chunk[2])));
        }
    }
    rows.retain(|(lemma, _, _)| lemma != "be");
    let lemmas: std::collections::HashSet<String> = rows.iter().map(|(l, _, _)| l.clone()).collect();
    for (lemma, past, participle) in rows {
        for form in &past {
            // A form that is itself another verb's base ("lay", past of "lie")
            // reads as that base, not as the past.
            if !lemmas.contains(form) || *form == lemma {
                out.past_to_lemma.entry(form.clone()).or_insert_with(|| lemma.clone());
            }
        }
        for form in &participle {
            if !lemmas.contains(form) || *form == lemma {
                out.participle_to_lemma.entry(form.clone()).or_insert_with(|| lemma.clone());
            }
        }
        out.by_lemma.insert(lemma, IrregularForms { past, participle });
    }
    out
}

/// The irregular verbs primed from the Guide's Appendix 1.4.
static IRREGULAR: OnceLock<Irregulars> = OnceLock::new();

/// Prime the reader and realiser with the Guide's irregular verbs, from the
/// Guide's own text. Idempotent: the first prime wins.
pub fn prime_irregular(guide_text: &str) {
    let _ = IRREGULAR.set(parse_irregulars(guide_text));
}

/// The simple past of an irregular verb, or `None` when unprimed or regular.
pub fn irregular_past(lemma: &str) -> Option<String> {
    IRREGULAR.get()?.forms(lemma)?.past.first().cloned()
}

/// The past participle of an irregular verb, or `None` when unprimed or regular.
pub fn irregular_participle(lemma: &str) -> Option<String> {
    IRREGULAR.get()?.forms(lemma)?.participle.first().cloned()
}

/// The verb a form is the simple past of ("met" -> "meet"), or `None`.
pub fn lemma_of_past(word: &str) -> Option<String> {
    IRREGULAR.get()?.lemma_of_past(word).map(str::to_string)
}

/// The verb a form is the past participle of ("gone" -> "go"), or `None`.
pub fn lemma_of_participle(word: &str) -> Option<String> {
    IRREGULAR.get()?.lemma_of_participle(word).map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_primed_lexicon_keeps_the_tag_set_and_answers_by_ambiguity() {
        prime("arrive\tv\nwater\tnv\nthing\tn\nblue\ta\n");
        // The OnceLock is process-global and first-write-wins; another test in
        // this binary may have primed the real lexicon first. Only assert on
        // the synthetic data when it is the one that got primed.
        if tag("arrive") != Some("v") || tag("water") != Some("nv") {
            return;
        }
        // "arrive" is verb-only: an imperative may generalise to it.
        assert!(verb_only("arrive"));
        assert!(verb_capable("arrive"));
        // "water" is noun-and-verb: verb-capable but NOT verb-only, so the
        // imperative gate does not fire on it.
        assert!(!verb_only("water"));
        assert!(verb_capable("water"));
        assert!(noun_capable("water"));
        // "thing" is a noun only.
        assert!(!verb_only("thing"));
        assert!(!verb_capable("thing"));
        assert!(noun_capable("thing"));
    }

    #[test]
    fn the_appendix_table_parses_both_column_blocks_and_alternatives() {
        let text = "List of irregular verbs\n\
            infinitive        past simple   past participle   infinitive   past simple   past participle\n\
            be                was/were      been              light        lit           lit\n\
            get               got           got/gotten        lie          lay           lain\n\
            lay               laid          laid              read         read [red]*   read [red]*\n\
            * pronunciation\n\
            pay               paid          paid\n";
        let t = parse_irregulars(text);
        // be is skipped; the rest of both blocks are read; the footnote ends it.
        assert!(t.forms("be").is_none());
        assert_eq!(t.forms("light").map(|f| f.past.clone()), Some(vec!["lit".to_string()]));
        assert_eq!(t.forms("get").map(|f| f.participle.clone()), Some(vec!["got".to_string(), "gotten".to_string()]));
        assert_eq!(t.forms("read").map(|f| f.past.clone()), Some(vec!["read".to_string()]));
        assert!(t.forms("pay").is_none(), "rows after the footnote are not part of the table");
        // "lay" is the past of "lie" but is also a verb of its own, so it reads
        // as its own base and not as the past of "lie".
        assert_eq!(t.lemma_of_past("lay"), None);
        assert_eq!(t.lemma_of_past("laid"), Some("lay"));
        assert_eq!(t.lemma_of_participle("gotten"), Some("get"));
    }
}
