//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// The imperative: no subject, the verb as a bare lemma, then its objects and
/// adjuncts. Negation is `do not` before the verb.
pub(in crate::ladder::act) fn imperative(frame: &Frame) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    if frame.polarity == Polarity::Negative {
        if !frame.never {
            words.push("do".into());
        }
        words.push(negation_word(frame).into());
    }
    words.push(frame.pred.clone());
    words.extend(complements(frame, true));
    words.extend(adjuncts(frame, None));
    words
}

