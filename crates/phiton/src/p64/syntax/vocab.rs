//! The given dictionary: each part of speech's words, read from a file
//! under `data/lex/` (the ruling of 2026-11-07: no data in code). The words
//! were constants here until then; they are the same words, one per line,
//! and a run that learns a word appends it to the file so the next run
//! knows it ([`PosDictionary::learn`]).
//!
//! | file | what |
//! |---|---|
//! | `pronoun.txt` | 31 words |
//! | `determiner.txt` | 12 words |
//! | `auxiliary.txt` | 24 words |
//! | `preposition.txt` | 35 words |
//! | `conjunction.txt` | 15 words |
//! | `interjection.txt` | 12 words |
//! | `adverb.txt` | 28 words |
//! | `adjective.txt` | 28 words |
//! | `verb.txt` | 53 words |

use std::path::PathBuf;
use std::sync::OnceLock;

/// Where the files live: the crate's own `data/lex/`, or `$PHITON_LEX`.
pub fn lex_dir() -> PathBuf {
    match std::env::var_os("PHITON_LEX") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/data/lex")),
    }
}

/// The file a part of speech's words are read from and appended to.
pub fn lex_file(name: &str) -> PathBuf {
    lex_dir().join(format!("{name}.txt"))
}

/// The words of a part of speech, read once from its file.
pub fn keys(name: &'static str) -> &'static [String] {
    static LISTS: OnceLock<std::sync::Mutex<std::collections::HashMap<&'static str, &'static [String]>>> = OnceLock::new();
    let lists = LISTS.get_or_init(Default::default);
    let mut lists = lists.lock().unwrap_or_else(|e| e.into_inner());
    lists.entry(name).or_insert_with(|| Box::leak(phistd::lexfile::read(lex_file(name)).into_boxed_slice()))
}

/// The nine given classes, in the order they are registered.
pub const CLASSES: &[&str] = &["pronoun", "determiner", "auxiliary", "preposition", "conjunction", "interjection", "adverb", "adjective", "verb"];
