//! A trailing time phrase opened by a word of `time_opener.txt` ("... last week").
use crate::ladder::act::*;
/// The time-phrase reading of a predicate tail: the tail ends in an opener
/// (`last`) and a one-word noun phrase. The words before it must read whole on
/// their own; the noun is stored under `Via(opener)` and the Realiser says the
/// opener then the noun. Returns `None` unless the whole tail is accounted for
/// - never a partial frame (R3.4).
pub(in crate::ladder::act) fn finish_with_time_phrase(frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    let n = words.len();
    if n < from + 2 || !list::time_openers().contains(&words[n - 2].as_str()) {
        return None;
    }
    let (phrase, end) = read_noun_phrase(&words[n - 1..])?;
    if end != 1 {
        return None;
    }
    let head = finish_plain(frame, &words[..n - 2], from, force)?;
    Some(Act::new(force, head.frame.with(Role::Via(words[n - 2].clone()), phrase)))
}
