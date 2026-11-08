//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Whether a read subject is the empty expletive `it` of a weather verb, so
/// the reader drops it: "it has been raining" has no argument, matching the
/// realiser's `needs_expletive`.
pub(in crate::ladder::act) fn is_expletive_subject(lemma: &str, subject: &Filler) -> bool {
    matches!(subject, Filler::Pronoun(p) if crate::lexicon::lists::is_role(&p.to_ascii_lowercase(), "expletive_pronoun"))
        && lists::weather_verbs().contains(lemma)
}
/// Read the subject noun phrase from the front of the sentence, returning
/// the filler and the index just past it. A pronoun is one word; a
/// determiner-led or bare noun phrase is read by [`read_noun_phrase`]. The
/// reader abstains (None) when it cannot find a head, so it never treats a
/// verb as a subject.
pub(in crate::ladder::act) fn read_subject(words: &[String]) -> Option<(Filler, usize)> {
    let first = words.first()?;
    if lists::pronouns().contains(&first.as_str()) {
        return Some((subject_filler(first), 1));
    }
    let (thing, end) = read_noun_phrase(words)?;
    // A subject noun phrase must leave at least one word for the verb.
    if end >= words.len() {
        return None;
    }
    Some((thing, end))
}

/// A subject word as a filler: a pronoun kept nominative, else a bare word.
pub(in crate::ladder::act) fn subject_filler(word: &str) -> Filler {
    if lists::pronouns().contains(&word) {
        Filler::pronoun(&canon_pronoun(word))
    } else {
        Filler::word(word)
    }
}

/// Canonicalise a pronoun to the form the IR stores: the first person is the
/// capital `I`, matching the gold frames; the rest stay as read.
pub(in crate::ladder::act) fn canon_pronoun(word: &str) -> String {
    if word == "i" { "I".into() } else { word.to_string() }
}

