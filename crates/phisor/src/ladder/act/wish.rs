//! Wish frames said in the subjunctive: "i wish she were here", "i wish we did
//! not have to go". The verb group comes from `wish_form.tsv`.
use super::{adjuncts, complements, read_inner, say_filler, subject_of};
use crate::ir::{Act, Aspect, Frame, Modal, Polarity, Tense, Voice};
use crate::lexicon::list;

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
        None if list::is_role(&frame.pred, "copula_lemma") => list::word_of("copula_lemma"),
        None => return None,
    };
    let form = list::wish_form(key, frame.polarity == Polarity::Negative)?;
    let mut words = subject_of(frame).map(say_filler).unwrap_or_default();
    words.extend(form.iter().cloned());
    if frame.modal.is_some() {
        words.push(frame.pred.clone());
    }
    words.extend(complements(frame, false));
    words.extend(adjuncts(frame, None));
    Some(words)
}

/// The inverse of [`content`] for a modal row: the wished-for words are a
/// subject, a verb group `wish_form.tsv` gives for a modal, then the rest. The
/// group is put back to the modal's present form (`had to` to `must`, `did not
/// have to` to `must not`), the clause is read, and its tense is set back to
/// the past the Realiser started from. `None` when no row matches at a subject
/// boundary, or the clause does not read.
pub(super) fn read_clause(words: &[String]) -> Option<Act> {
    for (key, negative, form) in list::wish_forms() {
        let Some(modal) = Modal::named(key) else { continue };
        for at in 1..words.len().saturating_sub(form.len()) {
            if words[at..at + form.len()] != form[..] {
                continue;
            }
            let mut plain = words[..at].to_vec();
            plain.push(modal.name().to_string());
            if *negative {
                plain.push("not".to_string());
            }
            plain.extend_from_slice(&words[at + form.len()..]);
            let mut act = read_inner("x.", plain)?;
            let frame = &mut act.frame;
            let fits = frame.modal == Some(modal)
                && frame.tense == Tense::Present
                && frame.aspect == Aspect::Simple
                && frame.voice == Voice::Active
                && (frame.polarity == Polarity::Negative) == *negative;
            if !fits {
                return None;
            }
            frame.tense = Tense::Past;
            return Some(act);
        }
    }
    None
}