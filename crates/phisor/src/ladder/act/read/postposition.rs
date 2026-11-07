//! A noun phrase closed by a postposition ("the rain started two hours ago").
use crate::ladder::act::*;

/// The postposition reading of a predicate tail: the tail ends in a word of
/// `postposition.txt` preceded by a noun phrase that spans exactly the words
/// back to some start. The words before that phrase must read whole on their
/// own; the phrase is stored under `Via(word)`. Returns `None` unless the whole
/// tail is accounted for - never a partial frame (R3.4).
pub(in crate::ladder::act) fn finish_with_postposition(frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    let last = words.len().checked_sub(1)?;
    let post = words[last].as_str();
    if last <= from || !lists::postpositions().contains(&post) {
        return None;
    }
    for q in from..last {
        let Some((phrase, end)) = read_noun_phrase(&words[q..last]) else { continue };
        if q + end != last {
            continue;
        }
        let Some(head) = finish_plain(frame.clone(), &words[..q], from, force) else { continue };
        return Some(Act::new(force, head.frame.with(Role::Via(post.to_string()), phrase)));
    }
    None
}
