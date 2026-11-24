//! `lexicon::prime` - the reader's three process-wide tables primed in one
//! place, from the files the harness primes them from: the mined tags
//! (`lex/pos.tsv`), the closed-class function words (`lex/function.txt`),
//! and the irregular verbs of the Guide's Appendix 1.4 (`person/murphy.txt`,
//! beside the lexicon folder).
//!
//! Each table is a `OnceLock`: process-wide, and the first prime wins. A test
//! binary runs every test in one process, so the lexicon a test read used to
//! be whichever test primed first: the full files, a four-word table, or
//! nothing, and a test that passed alone could fail in the full run (seen
//! 2026-11-23 and 2026-11-24). Under `cfg(test)` every query and every prime
//! therefore primes the canonical files first, once, so every test reads the
//! reader the harness runs, in any order. Outside tests nothing is primed
//! but what a caller primes.

use super::list::{lex_dir, read_file};

/// Prime all three tables from the lexicon folder (`list::lex_dir`) and the
/// Guide beside it. A file that cannot be read, or is empty, leaves its
/// table unprimed rather than primed empty.
pub fn prime_canonical() {
    let guide = std::fs::read_to_string(lex_dir().join("../person/murphy.txt")).unwrap_or_default();
    let present = |text: &str| !text.trim().is_empty();
    let (tags, function) = (read_file("pos.tsv"), read_file("function.txt"));
    match present(&tags) {
        true => super::set_tags(&tags),
        false => {}
    }
    match present(&function) {
        true => super::set_function(&function),
        false => {}
    }
    match present(&guide) {
        true => super::set_irregular(&guide),
        false => {}
    }
}

/// Under `cfg(test)`, the canonical tables primed once, before anything
/// reads or primes them; outside tests, nothing.
pub(super) fn ensure() {
    #[cfg(test)]
    {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(prime_canonical);
    }
}
