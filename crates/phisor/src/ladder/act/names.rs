//! Proper-name case through the Frame (spec phinet-english-reader-coverage,
//! task 7.4, R4.4).
//!
//! `normalise` lower-cases, so the Reader would lose `London` before it read
//! a word. The closest fit in the IR is [`Filler::Name`], which the gold
//! already uses for `*London`; no new `Role` or field is added, so the gold
//! format and its fingerprint are untouched.
//!
//! A name is found from the *shape of the raw sentence* alone: a word that
//! opens with a capital in the middle of the sentence, or at its start when
//! the foundation lexicon does not know the word as a common one. There is no
//! name list, and no Held_Out_Slice sentence is read to build one.
use super::say_thing;
use crate::ir::{Attr, Filler, Frame};
use crate::lexicon;

/// The proper names the raw sentence spells with a capital, as spelled.
pub(super) fn spelled(raw: &str) -> Vec<String> {
    raw.split_whitespace()
        .enumerate()
        .filter_map(|(i, token)| {
            let word = token.trim_matches(|c: char| !c.is_alphanumeric());
            looks_like_name(word, i == 0).then(|| word.to_string())
        })
        .collect()
}

/// Whether `word` is spelled like a proper name. A capital then lower-case
/// letters only: `I`, `TV` and `iPhone` are not names. At the start of the
/// sentence a capital says nothing, so the word must also be unknown to the
/// lexicon (or primed-and-untagged), never a function word.
fn looks_like_name(word: &str, first: bool) -> bool {
    let mut chars = word.chars();
    let (Some(head), rest) = (chars.next(), chars.as_str()) else { return false };
    if !head.is_uppercase() || rest.is_empty() || !rest.chars().all(char::is_lowercase) {
        return false;
    }
    if !first {
        return true;
    }
    let lower = word.to_lowercase();
    lexicon::is_primed() && lexicon::tag(&lower).is_none() && !lexicon::function_word(&lower)
}

/// Turn each bare thing whose surface is one of `names` (compared in lower
/// case) into a [`Filler::Name`] spelled as in the sentence. Things with a
/// determiner, a count or an attribute are left as they are.
pub(super) fn restore(frame: &mut Frame, names: &[String]) {
    if names.is_empty() {
        return;
    }
    for (_, filler) in frame.slots.iter_mut() {
        restore_filler(filler, names);
    }
}

fn restore_filler(filler: &mut Filler, names: &[String]) {
    match filler {
        Filler::Thing(thing) => {
            if thing.det.is_none() && thing.attrs.is_empty() {
                let said = say_thing(thing).join(" ");
                if let Some(name) = names.iter().find(|n| n.to_lowercase() == said) {
                    *filler = Filler::name(name);
                }
                return;
            }
            for (attr, value) in thing.attrs.iter_mut() {
                if matches!(attr, Attr::Via(_)) {
                    restore_filler(value, names);
                }
            }
        }
        Filler::Clause(inner) => restore(inner, names),
        _ => {}
    }
}

/// Every proper name a frame holds, in slot order.
pub(super) fn held(frame: &Frame) -> Vec<String> {
    let mut out = Vec::new();
    for (_, filler) in &frame.slots {
        collect(filler, &mut out);
    }
    out
}

fn collect(filler: &Filler, out: &mut Vec<String>) {
    match filler {
        Filler::Name(n) => out.push(n.clone()),
        Filler::Thing(thing) => thing.attrs.iter().for_each(|(_, v)| collect(v, out)),
        Filler::Clause(inner) => out.extend(held(inner)),
        _ => {}
    }
}

/// A lower-cased sentence with each word that is one of `names` put back in
/// the case the name is spelled in. The mark rides on the last word.
pub(super) fn recase(lowered: &str, names: &[String]) -> String {
    lowered
        .split(' ')
        .map(|token| {
            let bare = token.trim_end_matches(|c: char| !c.is_alphanumeric());
            match names.iter().find(|n| n.to_lowercase() == bare) {
                Some(n) => format!("{n}{}", &token[bare.len()..]),
                None => token.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Whether every name the frame holds is spelled in the realised sentence
/// and in the raw sentence just as it was read: case kept both ways.
pub(super) fn kept(frame: &Frame, realised: &str, raw: &str) -> bool {
    let spelled_in = |text: &str, name: &str| {
        text.split_whitespace().any(|t| t.trim_matches(|c: char| !c.is_alphanumeric()) == name)
    };
    held(frame).iter().all(|n| spelled_in(realised, n) && spelled_in(raw, n))
}
