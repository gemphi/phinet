//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Read a noun phrase from the front of `words`: an optional determiner, an
/// optional possessive/ordinal modifier, then a head noun. Returns the
/// `Filler::Thing` and the index just past the phrase. Deliberately shallow:
/// it stops at the first word that reads as a head, so `the first king of` is
/// read as `the king` and the `of` phrase is left to the caller or dropped,
/// rather than mis-attached.
pub(in crate::ladder::act) fn read_noun_phrase(words: &[String]) -> Option<(Filler, usize)> {
    let mut i = 0;
    let mut det = None;
    if i < words.len() && is_determiner(&words[i]) {
        det = determiner_of(&words[i]);
        i += 1;
    }
    // One modifier before the head: an ordinal or number ("the first king",
    // "two cars"), or an adjective the foundation lexicon names ("a hot day",
    // "a blue suit"). An adjective is kept as a Mod attribute so it survives
    // the round trip; an ordinal/number modifier is skipped as before.
    let mut adjective: Option<String> = None;
    let mut count: Option<String> = None;
    if i + 1 < words.len() {
        if is_cardinal(&words[i]) {
            count = Some(words[i].clone());
            i += 1;
        } else if is_modifier(&words[i]) {
            i += 1;
        } else if is_adjective_word(&words[i]) {
            adjective = Some(words[i].clone());
            i += 1;
        }
    }
    let head = words.get(i)?;
    // A conjunction ("and", "but") joins; it is never the head of a noun phrase.
    if is_determiner(head) || is_aux(head) || is_copula(head) || lists::clause_conjunctions().contains(&head.as_str()) {
        return None;
    }
    let (singular, plural) = lemma_number(head);
    let mut thing = Thing::new(&singular);
    if let Some(adj) = adjective {
        thing = thing.with(crate::ir::Attr::Mod, Filler::word(&adj));
    }
    if let Some(c) = count {
        thing = thing.with(crate::ir::Attr::Count, Filler::word(&c)).plural();
    }
    if let Some(d) = det {
        thing = thing.det(d);
    }
    if plural {
        thing = thing.plural();
    }
    Some((Filler::Thing(thing), i + 1))
}

/// Read trailing prepositional phrases into `Via` slots: "in a bank", "on
/// time", "at eight". Each is a known preposition followed by a noun phrase
/// the reader can segment; it stops (returning what it has) at the first word
/// that is not a clean preposition+phrase, so it never mis-attaches.
pub(in crate::ladder::act) fn read_prepositions(words: &[String]) -> (Vec<(Role, Filler)>, usize) {
    let mut out = Vec::new();
    let mut i = 0;
    while i < words.len() {
        if !is_preposition(&words[i]) {
            break;
        }
        let prep = words[i].clone();
        let rest = &words[i + 1..];
        match read_noun_phrase(rest) {
            Some((thing, end)) => {
                out.push((Role::named(&prep), thing));
                i += 1 + end;
            }
            None => break,
        }
    }
    (out, i)
}

