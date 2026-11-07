//! `lexicon::lists` - the reader's closed word lists, read from data files.
//!
//! No language data is written in Rust. Every list the reader and the
//! realiser consult lives in a plain-text file under `data/lex/` (beside
//! `pos.tsv` and `function.txt`): one entry per line, `#` starts a comment,
//! and a mapping or a tagged list is a `.tsv`. Each list is read lazily, once,
//! the first time it is asked for, from [`lex_dir`]; a file that is missing
//! reads as an empty list, so the reader abstains rather than guesses. A list
//! learned later is appended to its file and picked up by the next run.
//!
//! Tagged lists share one file per concept: `pronoun.tsv` carries a word and a
//! comma-separated set of the roles it plays (`subject`, `object`, ...), and
//! each caller asks for the tag it needs, so no word is written twice.
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Where the lexicon data files live: `$PHISOR_LEX_DIR` when set, otherwise
/// the foundation's `data/lex` beside this crate. The one path helper every
/// list, and every caller that reads a lexicon file, goes through.
pub fn lex_dir() -> PathBuf {
    match std::env::var_os("PHISOR_LEX_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data/lex")),
    }
}

/// The text of a lexicon data file, or empty when it cannot be read.
fn read_file(file: &str) -> String {
    std::fs::read_to_string(lex_dir().join(file)).unwrap_or_default()
}

/// The data lines of a file: trimmed, blank lines and `#` comments dropped
/// (the format and parser are `phistd::lexfile`, shared by every crate).
pub fn data_lines(text: &str) -> Vec<String> {
    phistd::lexfile::parse(text).into_iter().map(|l| l.trim().to_string()).collect()
}

/// The tab-separated rows of a file's data lines, each field trimmed.
pub fn data_rows(text: &str) -> Vec<Vec<String>> {
    data_lines(text).iter().map(|l| l.split('\t').map(|f| f.trim().to_string()).collect()).collect()
}

/// An ordered word list with constant-time membership.
#[derive(Debug, Default)]
pub struct Words {
    items: Vec<String>,
    set: HashSet<String>,
}

impl Words {
    pub fn new(items: Vec<String>) -> Words {
        let set = items.iter().cloned().collect();
        Words { items, set }
    }
    /// Whether the list holds the word exactly.
    pub fn contains(&self, word: impl AsRef<str>) -> bool {
        self.set.contains(word.as_ref())
    }
    /// The entries in file order.
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.items.iter()
    }
    pub fn as_slice(&self) -> &[String] {
        &self.items
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Read a plain list: one entry per data line.
pub fn load_plain(file: &str) -> Words {
    Words::new(data_lines(&read_file(file)))
}

/// Read a tagged list: the first field of each row of `file`, for the rows
/// whose second field (a comma-separated tag set) holds `tag`.
pub fn load_tagged(file: &str, tag: &str) -> Words {
    let rows = data_rows(&read_file(file));
    let pick = |r: &Vec<String>| r.get(1).is_some_and(|t| t.split(',').any(|x| x.trim() == tag));
    Words::new(rows.iter().filter(|r| pick(r)).map(|r| r[0].clone()).collect())
}

/// Read a phrase list: one phrase per data line, its words space-separated.
pub fn load_phrases(file: &str) -> Vec<Vec<String>> {
    data_lines(&read_file(file)).iter().map(|l| l.split_whitespace().map(str::to_string).collect()).collect()
}

/// Declare an accessor for a plain list file, read once on first use.
macro_rules! plain_list {
    ($(#[$doc:meta])* $name:ident, $file:literal) => {
        $(#[$doc])*
        pub fn $name() -> &'static Words {
            static CELL: OnceLock<Words> = OnceLock::new();
            CELL.get_or_init(|| load_plain($file))
        }
    };
}

/// Declare an accessor for one tag of a tagged list file.
macro_rules! tagged_list {
    ($(#[$doc:meta])* $name:ident, $file:literal, $tag:literal) => {
        $(#[$doc])*
        pub fn $name() -> &'static Words {
            static CELL: OnceLock<Words> = OnceLock::new();
            CELL.get_or_init(|| load_tagged($file, $tag))
        }
    };
}

plain_list!(
    /// Stressed multi-syllable verbs that double their final consonant.
    doubling_stressed,
    "doubling_stressed.txt"
);
plain_list!(
    /// The irregular lemmas the reader inverts a surface form against.
    irregular_lemmas,
    "irregular_lemma.txt"
);
plain_list!(
    /// Silent-e verbs whose `-ing`/`-ed` drops the e.
    silent_e_lemmas,
    "silent_e_lemma.txt"
);
plain_list!(
    /// Nouns ending in `s` that are singular.
    singular_s_nouns,
    "singular_s_noun.txt"
);

tagged_list!(
    /// Pronouns the reader keeps as filler words, never as verbs.
    pronouns,
    "pronoun.tsv",
    "reader"
);
tagged_list!(
    /// Subject pronouns, which can open a clause.
    subject_pronouns,
    "pronoun.tsv",
    "subject"
);
tagged_list!(
    /// Object pronouns, which read as a recipient after a ditransitive verb.
    object_pronouns,
    "pronoun.tsv",
    "object"
);

tagged_list!(
    /// Conjunctions that join two clauses.
    clause_conjunctions,
    "conjunction.tsv",
    "clause"
);
tagged_list!(
    /// Words that join or open a clause, for the abstention buckets.
    clause_openers,
    "conjunction.tsv",
    "opener"
);

plain_list!(
    /// Verbs that take a to-infinitive with the same subject (control verbs).
    control_verbs,
    "control_verb.txt"
);
plain_list!(
    /// Verbs that take a -ing complement.
    gerund_verbs,
    "gerund_verb.txt"
);
plain_list!(
    /// Verbs that take a finite clause as their complement.
    complement_verbs,
    "complement_verb.txt"
);
plain_list!(
    /// Explicit performative verbs.
    performatives,
    "performative.txt"
);
plain_list!(
    /// Adverbs the IR has no slot for, set aside on reading.
    set_aside_adverbs,
    "set_aside_adverb.txt"
);
plain_list!(
    /// Base-form verbs that give an instruction.
    imperative_verbs,
    "imperative_verb.txt"
);
plain_list!(
    /// Verbs that take a recipient as their first object.
    ditransitive_verbs,
    "ditransitive_verb.txt"
);
plain_list!(
    /// Pronoun hosts whose following 	o + verb modifies them.
    to_modified_hosts,
    "to_modified_host.txt"
);

/// Multi-word adverbials set aside as a unit, one phrase (a word list) each.
pub fn set_aside_adverb_phrases() -> &'static Vec<Vec<String>> {
    static CELL: OnceLock<Vec<Vec<String>>> = OnceLock::new();
    CELL.get_or_init(|| load_phrases("set_aside_adverb_phrase.txt"))
}

tagged_list!(
    /// Pronouns that take 's as is/has (he's), never as a possessive.
    s_hosts,
    "pronoun.tsv",
    "s_host"
);
tagged_list!(
    /// Words after 's that make it has (he's been).
    s_has_cues,
    "contraction_s.tsv",
    "has"
);
