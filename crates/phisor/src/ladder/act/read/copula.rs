//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// The complement after a copula, as one filler: a single pronoun/word, or a
/// noun phrase that spans the rest. Abstains when the rest is more than one
/// clean phrase, so a copula with trailing prepositions is left uncovered.
pub(in crate::ladder::act) fn complement_filler(rest: &[String]) -> Option<Filler> {
    if rest.len() == 1 {
        return Some(subject_filler(&rest[0]));
    }
    match read_noun_phrase(rest) {
        Some((thing, end)) if end == rest.len() => Some(thing),
        _ => None,
    }
}

/// Read a copula question: "what is your name?" (wh fills `as`), "where were
/// the keys?" (wh fills its preposition role). The subject is the noun phrase
/// after the copula.
pub(in crate::ladder::act) fn read_copula_question(words: &[String], op_at: usize, wh: Option<Wh>) -> Option<Act> {
    let op = words.get(op_at)?;
    let tense = if aux_entry(op).is_some_and(|e| e.family == Family::Be && e.tense == Tense::Past) { Tense::Past } else { Tense::Present };
    let rest = &words[op_at + 1..];
    let (subject, end) = read_subject(rest)?;
    let mut tail = &rest[end..];
    let mut frame = Frame::new("be", tense).with(Role::Theme, subject);
    // "is she not here?" - `not` after the subject negates the copula.
    if wh.is_none() && tail.first().map(String::as_str) == Some("not") {
        frame = frame.negative();
        tail = &tail[1..];
    }
    match wh {
        Some(wh) => {
            if !tail.is_empty() {
                return None; // trailing words beyond the subject: abstain
            }
            frame = match wh {
                // "what is your name?" - the wh is the predicate complement.
                Wh::What | Wh::Who | Wh::Which => frame.with(Role::As, Filler::Hole(wh)),
                // "where were the keys?" - the wh is a place, stored under its prep.
                _ => frame.with(wh_role(wh), Filler::Hole(wh)),
            };
        }
        // A yes/no question: "was the weather good?", "is he at home?". What
        // follows the subject is the complement - a prepositional phrase, or a
        // word or noun phrase in the `as` role.
        None => {
            if tail.is_empty() {
                return None;
            }
            if is_preposition(&tail[0]) {
                let (preps, used) = read_prepositions(tail);
                if preps.is_empty() || used != tail.len() {
                    return None;
                }
                for (role, filler) in preps {
                    frame = frame.with(role, filler);
                }
            } else {
                frame = frame.with(Role::As, complement_filler(tail)?);
            }
        }
    }
    Some(Act::new(Force::Ask, frame))
}

/// Read an aspect/passive question: "has she arrived?", "what has happened?",
/// "what are you doing?". The inverted operator at `op_at` is lifted back onto
/// the words after the subject to reconstruct the verb group, which the shared
/// [`read_verb_group`] decomposes; the subject and the wh are then placed as
/// in a plain question. Returns `None` when the chain does not read as an
/// aspect (so a `be`-form copula question falls through to its own reader).
pub(in crate::ladder::act) fn read_aspect_question(words: &[String], op_at: usize, wh: Option<Wh>) -> Option<Act> {
    let operator = words.get(op_at)?.clone();

    // A subject wh ("what has happened?") is itself the subject: the verb
    // group is the operator plus everything after it, with no interposed
    // subject, and the wh fills the subject role as a Hole.
    if matches!(wh, Some(Wh::What) | Some(Wh::Who) | Some(Wh::Which)) {
        let mut chain = vec![operator.clone()];
        chain.extend_from_slice(&words[op_at + 1..]);
        if let Some((lemma, tense, aspect, voice, neg, consumed)) = read_verb_group(&chain, 0) {
            // Only a clean, whole verb group (no leftover object) is a subject
            // wh; anything trailing means there is a real subject instead.
            if consumed == chain.len() {
                let mut frame = Frame::new(&lemma, tense).aspect(aspect);
                if voice == Voice::Passive {
                    frame = frame.passive();
                }
                if neg {
                    frame = frame.negative();
                }
                // A subject wh of an aspect clause fills the theme in the gold
                // ("what has happened?" -> theme=?what).
                frame = frame.with(Role::Theme, Filler::Hole(wh?));
                return Some(Act::new(Force::Ask, frame));
            }
        }
    }

    let (subject, after) = read_subject(&words[op_at + 1..])?;
    let subject_end = op_at + 1 + after;

    // Reconstruct the contiguous verb group: the operator, then the words
    // after the subject (been/participle/-ing), and read it as one chain.
    let mut chain = vec![operator];
    chain.extend_from_slice(&words[subject_end..]);
    let (lemma, tense, aspect, voice, neg, consumed) = read_verb_group(&chain, 0)?;

    let mut frame = Frame::new(&lemma, tense).aspect(aspect);
    if voice == Voice::Passive {
        frame = frame.passive();
    }
    if neg {
        frame = frame.negative();
    }
    // The subject is the agent in the active (the theme in the passive, and
    // when the wh is the agent); the wh is placed into its role afterwards.
    let subject_is_agent = wh != Some(Wh::Who) && voice != Voice::Passive;
    let subject_role = if subject_is_agent { Role::Agent } else { Role::Theme };
    if !is_expletive_subject(&lemma, &subject) {
        frame = frame.with(subject_role, subject);
    }

    // The words the verb group consumed beyond the operator sit after the
    // subject; the remainder is objects/prepositions/by-agent.
    let rest_start = subject_end + (consumed - 1);
    frame = match finish_predicate(frame, words, rest_start, Force::Ask) {
        Some(act) => act.frame,
        None => return None,
    };

    if let Some(wh) = wh {
        frame = place_wh(frame, wh, subject_is_agent);
    }
    Some(Act::new(Force::Ask, frame))
}

