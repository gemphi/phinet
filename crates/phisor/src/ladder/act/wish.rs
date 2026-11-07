//! Wish frames said in the subjunctive: "i wish she were here", "i wish we did
//! not have to go". The verb group comes from `wish_form.tsv`.
use super::{adjuncts, complements, say_filler, subject_of};
use crate::ir::{Aspect, Frame, Polarity, Tense, Voice};
use crate::lexicon::lists;

/// The wished-for clause when its simple past is replaced by a subjunctive
/// form: the subject, the form, the bare verb after a modal, then the
/// complements and adjuncts. `None` leaves the clause to the ordinary
/// statement, as when `wish_form.tsv` has no row for it.
pub(super) fn content(frame: &Frame) -> Option<Vec<String>> {
    let plain = frame.tense == Tense::Past
        && frame.aspect == Aspect::Simple
        && frame.voice == Voice::Active
        && !frame.never;
    if !plain {
        return None;
    }
    let key = match frame.modal {
        Some(modal) => modal.name(),
        None if frame.pred == "be" => "be",
        None => return None,
    };
    let form = lists::wish_form(key, frame.polarity == Polarity::Negative)?;
    let mut words = subject_of(frame).map(say_filler).unwrap_or_default();
    words.extend(form.iter().cloned());
    if frame.modal.is_some() {
        words.push(frame.pred.clone());
    }
    words.extend(complements(frame, false));
    words.extend(adjuncts(frame, None));
    Some(words)
}
