//! A preposition followed by an `-ing` clause ("thanked me for helping him").
use crate::ladder::act::*;

/// The prepositional-gerund reading of a predicate tail: the first word of
/// `gerund_preposition.txt` followed by an `-ing` verb splits the tail in two.
/// The words before it must read whole on their own, and the words after the
/// `-ing` verb read as its complements. The clause shares the matrix agent, is
/// kept as progressive aspect, and is stored under `Via(prep)` like any other
/// prepositional phrase. Returns `None` unless the whole tail is accounted
/// for - never a partial frame (R3.4).
pub(in crate::ladder::act) fn finish_with_prep_gerund(frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    if frame.voice != Voice::Active {
        return None;
    }
    let agent = frame.get(&Role::Agent)?.clone();
    for k in from..words.len().saturating_sub(1) {
        let prep = words[k].as_str();
        let word = &words[k + 1];
        if !list::gerund_prepositions().contains(&prep) || is_aux(word) || list::to_modified_hosts().contains(&word.as_str()) {
            continue;
        }
        let Some(lemma) = read_ing(word) else { continue };
        let Some(head) = finish_plain(frame.clone(), &words[..k], from, force) else { continue };
        let inner = Frame::new(&lemma, Tense::Present).with(Role::Agent, agent.clone()).aspect(Aspect::Progressive);
        let Some(inner) = finish_predicate(inner, words, k + 2, Force::Assert) else { continue };
        return Some(Act::new(force, head.frame.with(Role::Via(prep.to_string()), Filler::clause(inner.frame))));
    }
    None
}
