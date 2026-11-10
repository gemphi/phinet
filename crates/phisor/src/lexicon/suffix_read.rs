//! `lexicon::suffix_read` - the lemma a stem read off `-ed`/`-ing` came from.
//!
//! The reading side of `suffix`: the letter classes it tests live in
//! `data/lex/letter.tsv`, the same file the writing rules use.
use super::list;
use super::suffix::{doubles_final, letter_is};

/// Undo the doubling in a stem read off `-ed`/`-ing` (`stopp` -> `stop`). The
/// bare stem wins when the lexicon knows it as a verb ("add", "kiss", "purr");
/// a `no_undouble` letter never undoubles, an `l` only when the lemma is named
/// in `doubling_stressed.txt`, and a lemma under three letters is not
/// proposed. Otherwise the result is a lemma that would double again.
pub fn undouble(stem: &str) -> Option<String> {
    let mut it = stem.chars().rev();
    let (last, prev) = (it.next()?, it.next()?);
    if last != prev || letter_is(last, "vowel") || letter_is(last, "no_undouble") || super::verb_capable(stem) {
        return None;
    }
    let lemma = &stem[..stem.len() - last.len_utf8()];
    let named = list::doubling_stressed().contains(&lemma);
    if (letter_is(last, "named_double") && !named) || lemma.len() < 3 || !doubles_final(lemma) {
        return None;
    }
    Some(lemma.to_string())
}

/// The lemma candidates for a regular `-ed` participle: the bare stem, the
/// stem with a restored silent e, and the consonant+y restoration from `ied`.
pub fn regular_ed_candidates(word: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(stem) = word.strip_suffix("ied") {
        out.push(format!("{stem}y"));
    }
    if let Some(stem) = word.strip_suffix("ed") {
        out.extend(undouble(stem));
        out.push(stem.to_string());
        out.push(format!("{stem}e"));
    }
    out
}
