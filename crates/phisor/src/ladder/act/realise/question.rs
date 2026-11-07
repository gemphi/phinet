//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// A question: front the wh-word (if any), then invert the operator with the
/// subject, then the remainder of the verb group, objects and adjuncts. A
/// yes/no question has no wh and simply inverts. When the wh is the subject
/// (or a modifier of it) there is no inversion - the wh stays in place and
/// the verb stays in statement order ("how many people came?").
pub(in crate::ladder::act) fn question(frame: &Frame) -> Vec<String> {
    let wh = find_wh(frame);
    // The wh is the subject itself: no inversion, statement order, the wh
    // standing where the subject would.
    if let Some((role, _)) = &wh {
        if is_subject_role(frame, role) {
            return statement(frame);
        }
    }
    // A wh on the subject's count ("how many people") also blocks inversion.
    if subject_has_wh(frame) {
        return statement(frame);
    }

    let mut words: Vec<String> = Vec::new();
    // Front the wh word, dropping any preposition it carried (the gold stores
    // "where" for "in=?where", not "in where").
    if let Some((role, w)) = &wh {
        match role {
            Role::Via(prep) if lists::wh_phrase(w.name(), prep).is_some() => {
                words.extend(lists::wh_phrase(w.name(), prep).into_iter().flatten().cloned());
            }
            _ => words.push(wh_surface(*w)),
        }
    }
    // The operator is the first word of the verb group; do-support supplies
    // one for a plain simple-active lexical verb that has none.
    let group = verb_group(frame);
    let (operator, rest) = split_operator(frame, group);
    words.push(operator);
    if let Some(filler) = subject_of(frame) {
        words.extend(say_filler(filler));
    }
    words.extend(rest);
    // Objects and adjuncts, skipping the slot the fronted wh came from.
    let skip = wh.as_ref().map(|(r, _)| r.clone());
    words.extend(complements_skipping(frame, skip.as_ref(), false));
    words.extend(adjuncts(frame, skip.as_ref()));
    words
}

/// Split the first word of a verb group off as the inversion operator. A
/// plain simple-active lexical verb (no modal, no aux) has no operator, so
/// do-support supplies one and the verb drops back to its lemma: "you live"
/// -> operator "do", rest "live"; "she arrived" -> "did" + "arrive".
pub(in crate::ladder::act) fn split_operator(frame: &Frame, mut group: Vec<String>) -> (String, Vec<String>) {
    let simple_lexical = frame.modal.is_none()
        && frame.aspect == Aspect::Simple
        && frame.voice == Voice::Active
        && frame.pred != "be";
    if simple_lexical {
        let third = third_singular(frame);
        let neg = frame.polarity == Polarity::Negative;
        let operator = match frame.tense {
            Tense::Past => "did".to_string(),
            Tense::Present => if third { "does".into() } else { "do".into() },
        };
        let mut rest = Vec::new();
        if neg {
            rest.push(negation_word(frame).into());
        }
        rest.push(frame.pred.clone());
        return (operator, rest);
    }
    // Otherwise the group already begins with an operator (a modal, be, or
    // have). "cannot" splits back into "can ... not" under inversion.
    if group.first().map(String::as_str) == Some("cannot") {
        group.remove(0);
        group.insert(0, "not".into());
        return ("can".into(), group);
    }
    let operator = group.remove(0);
    (operator, group)
}

/// The surface form of a wh-word: its stored name, with the one compound
/// "how-many" said as two words ("how many people came?").
pub(in crate::ladder::act) fn wh_surface(wh: Wh) -> String {
    match wh {
        Wh::HowMany => "how many".into(),
        _ => wh.name().to_string(),
    }
}
/// Find a wh-hole anywhere in the frame's slots, returning its role and the
/// Wh. The first hole wins (the gold frames carry at most one).
pub(in crate::ladder::act) fn find_wh(frame: &Frame) -> Option<(Role, Wh)> {
    frame.slots.iter().find_map(|(role, filler)| match filler {
        Filler::Hole(wh) => Some((role.clone(), *wh)),
        _ => None,
    })
}

/// Whether a role is the frame's surface subject, so a wh in it does not
/// trigger inversion.
pub(in crate::ladder::act) fn is_subject_role(frame: &Frame, role: &Role) -> bool {
    if frame.voice == Voice::Passive {
        role == &Role::Theme
    } else if frame.get(&Role::Agent).is_some() {
        role == &Role::Agent
    } else {
        role == &Role::Theme
    }
}

/// Whether the subject noun phrase carries a wh modifier ("how many people"),
/// which keeps statement order.
pub(in crate::ladder::act) fn subject_has_wh(frame: &Frame) -> bool {
    matches!(
        subject_of(frame),
        Some(Filler::Thing(t)) if t.attrs.iter().any(|(_, f)| matches!(f, Filler::Hole(_)))
    )
}

