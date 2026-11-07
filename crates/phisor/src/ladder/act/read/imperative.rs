//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Read an imperative, or `None` if the sentence does not open as a command.
/// The shape is `[do not] VERB [recipient] [object] [prepositions]`, with the
/// subject suppressed - the Direct force the gold records. The reader only
/// treats a leading word as a command verb when it is in the known set, so a
/// subject noun is never mistaken for an instruction.
pub(in crate::ladder::act) fn read_imperative(words: &[String]) -> Option<Act> {
    let mut i = 0;
    let mut neg = false;
    if words.get(0).map(String::as_str) == Some("do") && words.get(1).map(String::as_str) == Some("not") {
        neg = true;
        i = 2;
    }
    let verb = words.get(i)?;
    if !lists::imperative_verbs().contains(&verb.as_str()) {
        return None;
    }
    let mut frame = Frame::new(verb, Tense::Present);
    if neg {
        frame = frame.negative();
    }
    let rest = &words[i + 1..];
    // A ditransitive command names the recipient first ("tell me the truth",
    // "give her the letter"): a leading object pronoun is the recipient, and
    // the next noun phrase is the theme.
    let mut cursor = 0;
    if leading_recipient(verb, rest) {
        frame = frame.with(Role::Recipient, Filler::pronoun(&canon_pronoun(&rest[0])));
        cursor = 1;
    }
    let body = &rest[cursor..];
    let mut consumed = 0;
    if !body.is_empty() && !is_preposition(&body[0]) {
        if let Some((role, filler, end)) = read_theme(body) {
            frame = frame.with(role, filler);
            consumed = end;
        }
    }
    let tail = &body[consumed..];
    let (preps, prep_words) = read_prepositions(tail);
    for (role, filler) in preps {
        frame = frame.with(role, filler);
    }
    if consumed + prep_words != body.len() {
        return None; // trailing words unaccounted for: abstain, not a guess
    }
    Some(Act::new(Force::Direct, frame))
}

/// Whether a leading object pronoun is a ditransitive recipient rather than
/// the sole object. It is a recipient when another object follows ("give her
/// the letter") or when the verb inherently takes a recipient ("tell you"),
/// and the theme when it stands alone after a plain transitive verb ("do it").
/// The valency is the verb's own, which the Guide teaches per verb.
pub(in crate::ladder::act) fn leading_recipient(verb: &str, rest: &[String]) -> bool {
    if rest.first().map(|w| is_object_pronoun(w)) != Some(true) {
        return false;
    }
    if lists::ditransitive_verbs().contains(&verb) {
        return true;
    }
    match rest.get(1) {
        None => false,
        Some(w) => !is_preposition(w),
    }
}

