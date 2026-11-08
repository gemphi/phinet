//! Property 6: `suffix.tsv` gives the forms the `-s/-ing/-ed` code gives.
//!
//! The model is the code in `ladder/act/inflect.rs`, which stays until the
//! callers switch to `lexicon::suffix::inflect`.

use super::*;
use crate::lexicon::lists::{lex_dir, load_plain};
use crate::lexicon::suffix::{inflect, Suffix};

/// Every word of the lexicon (`pos.tsv`) and of the irregular-plural list.
fn vocabulary() -> Vec<String> {
    let text = std::fs::read_to_string(lex_dir().join("pos.tsv")).unwrap_or_default();
    let mut words: Vec<String> = text.lines().filter_map(|l| l.split('\t').next()).map(str::to_string).collect();
    words.extend(load_plain("doubling_stressed.txt").iter().cloned());
    words.extend(crate::lexicon::lists::irregular_plurals().iter().flat_map(|(s, p)| [s.clone(), p.clone()]));
    words
}

/// Stems built from every ending class the rules test: a vowel, `e`, `y`,
/// a sibilant, `o`, and a consonant pair, after each possible letter.
fn generated() -> Vec<String> {
    let letters = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let ends = ["a", "e", "i", "o", "u", "y", "ee", "ss", "ll", "tt", "ch", "sh", "x", "qu", "ey", "ay"];
    let mut out = vec![String::new(), "y".into(), "e".into(), "qu".into()];
    for l in &letters {
        for m in &letters {
            for end in ends {
                out.push(format!("{l}{m}{end}"));
                out.push(format!("{m}{end}"));
            }
        }
    }
    out
}

fn assert_same(word: &str) {
    assert_eq!(inflect(word, Suffix::THIRD), third_s(word), "third {word}");
    assert_eq!(inflect(word, Suffix::PLURAL), pluralise(word), "plural {word}");
    assert_eq!(inflect(word, Suffix::ING), ing(word), "ing {word}");
    assert_eq!(inflect(word, Suffix::PAST), past(word), "past {word}");
    assert_eq!(crate::lexicon::suffix::doubles_final(word), doubles_final(word), "doubles {word}");
}

#[test]
fn the_rule_file_inflects_every_lexicon_word_as_the_code_does() {
    let words = vocabulary();
    if words.is_empty() {
        return;
    }
    for w in &words {
        assert_same(w);
    }
}

#[test]
fn the_rule_file_inflects_every_generated_stem_as_the_code_does() {
    for w in generated() {
        assert_same(&w);
    }
}

#[test]
fn the_rule_file_gives_the_book_examples() {
    assert_eq!(inflect("go", Suffix::THIRD), "goes");
    assert_eq!(inflect("fly", Suffix::THIRD), "flies");
    assert_eq!(inflect("make", Suffix::ING), "making");
    assert_eq!(inflect("agree", Suffix::ING), "agreeing");
    assert_eq!(inflect("stop", Suffix::PAST), "stopped");
    assert_eq!(inflect("monday", Suffix::PLURAL), "mondays");
    assert_eq!(inflect("quit", Suffix::ING), "quitting");
}
