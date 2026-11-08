//! Moved from `ladder/act.rs` unchanged.
use super::*;

/// Remove the set-aside adverb phrases, then the single set-aside adverbs, from
/// a word list. Returns the remaining words and whether "never" was among them
/// (it carries negation). Phrases are matched first so a word inside a kept
/// phrase is not stripped on its own.
pub(in crate::ladder::act) fn strip_adverbs(words: &[String]) -> (Vec<String>, bool) {
    strip_adverbs_keeping(words, false)
}

/// As [`strip_adverbs`], optionally leaving "never" in the words, so the
/// round-trip comparison checks where it is written.
pub(in crate::ladder::act) fn strip_adverbs_keeping(words: &[String], keep_never: bool) -> (Vec<String>, bool) {
    let without_phrases = strip_adverb_phrases(words);
    let mut never = false;
    let kept = without_phrases
        .iter()
        .filter(|w| {
            let is_adverb = list::set_aside_adverbs().contains(&w.as_str());
            if is_adverb && crate::lexicon::list::is_role(w, "negative_adverb") {
                never = true;
            }
            !is_adverb || (keep_never && crate::lexicon::list::is_role(w, "negative_adverb"))
        })
        .cloned()
        .collect();
    (kept, never)
}

/// Drop every contiguous run of words that equals a [`SET_ASIDE_ADVERB_PHRASES`]
/// entry. Phrases do not overlap: once a run matches it is consumed whole.
pub(in crate::ladder::act) fn strip_adverb_phrases(words: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(words.len());
    let mut i = 0;
    while i < words.len() {
        let matched = list::set_aside_adverb_phrases()
            .iter()
            .filter(|phrase| phrase.len() <= words.len() - i)
            .find(|phrase| phrase.iter().zip(&words[i..]).all(|(p, w)| p == w));
        match matched {
            Some(phrase) => i += phrase.len(),
            None => {
                out.push(words[i].clone());
                i += 1;
            }
        }
    }
    out
}

/// Fold "am/is/are/was/were going to" down to "will", so the near-future reads
/// the same whether it was written "going to" or "will". Applied on both sides
/// of the round-trip comparison, never to the Act itself.
pub(in crate::ladder::act) fn fold_going_to(words: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < words.len() {
        if aux_is(&words[i], Family::Be)
            && words.get(i + 1).map(String::as_str) == Some("going")
            && words.get(i + 2).map(String::as_str) == Some("to")
        {
            out.push("will".into());
            i += 3;
        } else {
            out.push(words[i].clone());
            i += 1;
        }
    }
    out
}

/// A sentence in contraction-canonical form: normalised, then every known
/// contraction expanded to the words it stands for. Used only to compare a
/// realised sentence with its original, so "i'm" and "i am" count the same.
pub(in crate::ladder::act) fn canonical(s: &str) -> String {
    // The terminating mark rides on the last word; lift it off so a set-aside
    // adverb that ends the sentence ("... very often.") is still recognised,
    // then put it back.
    let mark: String = s.chars().last().filter(|c| MARKS.contains(c)).map(String::from).unwrap_or_default();
    let bare = s.trim_end_matches(|c| MARKS.contains(&c));
    let words: Vec<String> = bare.split_whitespace().map(str::to_string).collect();
    let words = fold_going_to(&words);
    let (kept, _) = strip_adverbs_keeping(&expand_contractions(&words), true);
    format!("{}{}", kept.join(" "), mark)
}

