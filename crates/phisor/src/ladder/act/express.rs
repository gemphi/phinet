//! Express frames said as a fixed phrase: "thank you for your help",
//! "welcome to london". The opening words come from `expressive_phrase.tsv`.

use super::{adjuncts, say_filler};
use crate::ir::{Aspect, Force, Frame, Polarity, Role, Tense, Voice};
use crate::lexicon::lists;

/// The sentence of an Express frame whose predicate has a fixed phrase and
/// that is a plain present positive active: the phrase, the theme when the
/// phrase takes one, then the adjuncts. The agent is the speaker and is not
/// said. `None` leaves the frame to the ordinary statement.
pub(super) fn phrase(force: Force, frame: &Frame) -> Option<Vec<String>> {
    let plain = force == Force::Express
        && frame.tense == Tense::Present
        && frame.aspect == Aspect::Simple
        && frame.voice == Voice::Active
        && frame.polarity == Polarity::Positive
        && frame.modal.is_none();
    if !plain {
        return None;
    }
    let entry = lists::expressive_phrases().iter().find(|e| e.pred == frame.pred)?;
    let mut words = entry.opening.clone();
    if entry.theme {
        words.extend(say_filler(frame.get(&Role::Theme)?));
    }
    words.extend(adjuncts(frame, None));
    Some(words)
}
