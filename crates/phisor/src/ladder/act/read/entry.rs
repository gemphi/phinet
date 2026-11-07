//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Read a normalised English sentence into an Act, or `None` when the
/// sentence is a shape this first reader does not handle. The caller counts a
/// `None` as uncovered - never guesses.
pub fn read(sentence: &str) -> Option<Act> {
    let norm = normalise(sentence);
    let body: String = norm.trim_end_matches(|c| MARKS.contains(&c)).trim().to_string();
    let raw: Vec<String> = body.split_whitespace().map(str::to_string).collect();
    let expanded = expand_contractions(&raw);
    // Set aside the manner/frequency adverbs the IR has no slot for, as the
    // gold does; "never" is also read as negation.
    let (words, never) = strip_adverbs(&expanded);
    // A fixed phrase of thanks, apology, congratulation or welcome is an Express
    // when it is the whole sentence, never a clause inside one ("kevin never
    // says thank you"). Only a full stop: the Realiser says the phrase with one,
    // so an exclamation would not round-trip.
    let phrase = norm.ends_with('.').then(|| express::read(&words)).flatten();
    // A politeness marker ahead of an imperative ("please close the window")
    // is a request: the marker has no slot, the rest is the command.
    let polite = (phrase.is_none() && !norm.ends_with('?'))
        .then(|| words.split_first())
        .flatten()
        .filter(|(m, _)| lists::politeness_markers().contains(&m.as_str()))
        .and_then(|(_, rest)| read_imperative(rest));
    let mut act = match phrase.or(polite) {
        Some(act) => act,
        None => read_inner(&norm, words)?,
    };
    // "i promise ..." is an explicit commissive: the verb itself says so.
    if act.force == Force::Assert
        && act.frame.pred == "promise"
        && act.frame.tense == Tense::Present
        && matches!(act.frame.get(&Role::Agent), Some(Filler::Pronoun(p)) if p == "I")
    {
        act.force = Force::Commit;
    }
    // A first-person present statement of a psychological-state predicate is
    // an expressive (Searle 1979; `expressive_phrase.tsv`).
    if act.force == Force::Assert
        && act.frame.tense == Tense::Present
        && act.frame.modal.is_none()
        && act.frame.voice == Voice::Active
        && lists::expressive_predicates().contains(&act.frame.pred)
        && is_first_person_agent(&act.frame)
    {
        act.force = Force::Express;
    }
    if never {
        act.frame.polarity = Polarity::Negative;
        act.frame.never = true;
    }
    // `normalise` lowered the names; the raw sentence still spells them.
    names::restore(&mut act.frame, &names::spelled(sentence));
    Some(act)
}

/// The reader over a sentence already expanded and stripped of its set-aside
/// adverbs. A sentence is read whole when it reads whole; only when it does not
/// is it tried as two clauses joined by a conjunction, so a sentence that reads
/// today is never disturbed by the split.
pub(in crate::ladder::act) fn read_inner(norm: &str, words: Vec<String>) -> Option<Act> {
    if let Some(act) = read_plain(norm, words.clone()) {
        return Some(act);
    }
    if norm.ends_with('?') {
        return None;
    }
    read_compound(&words)
}

