//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// The two auxiliary families the verb-group reader opens on.
pub(in crate::ladder::act) enum Aux {
    Have,
    Be,
}

/// Read a question into an Act, or `None` when the shape is one this reader
/// does not yet parse soundly (an aspect, a passive, a proper-name object).
/// The shape is `[wh] OPERATOR subject [not] verb [recipient] [object]
/// [prepositions]?`, the inverse of the question realiser: the operator is
/// do-support or a modal, the wh becomes a Hole in the role the Guide stores
/// for it. Yes/no questions carry no wh. Anything past the shapes below is
/// abstained on, never guessed.
pub(in crate::ladder::act) fn read_question(words: &[String]) -> Option<Act> {
    let mut i = 0;
    // A leading wh-word and the role it fills. The Guide fixes each wh to one
    // role (where->in, when->at, why->for, how->by, who->agent, what/which->
    // theme); the reader mirrors those choices.
    let wh = words.first().and_then(|w| Wh::named(w));
    if wh.is_some() {
        i = 1;
    }
    // "don't you have?" expands to "do not you have": put the `not` back
    // after the subject, where the rest of this reader expects it.
    let lifted = lift_operator_negation(words, i);
    let words = lifted.as_slice();
    // "how many people came?" - the wh modifies the subject, so it is not an
    // operator-fronted question. Left to a later reader; abstain for now.
    if wh == Some(Wh::HowMany) {
        return None;
    }

    // An aspect/passive question: the inverted operator is `has`/`have`/`had`
    // (always a perfect) or a `be`-form followed, past the subject, by a
    // participle or `-ing`. The verb group is reconstructed with the subject
    // lifted out and read by the shared verb-group reader. A `be`-form that
    // is not followed by a participle is a copula question instead.
    if let Some(op) = words.get(i) {
        if aux_is(op, Family::Have) {
            return read_aspect_question(words, i, wh);
        }
        if aux_is(op, Family::Be) {
            // "are you going to eat?" - a near-future question: the subject
            // after the copula, then "going to V", as a will-modal.
            if wh.is_none() {
                let after = &words[i + 1..];
                if let Some((subject, end)) = read_subject(after) {
                    if after.get(end).map(String::as_str) == Some("going")
                        && after.get(end + 1).map(String::as_str) == Some("to")
                    {
                        if let Some(v) = after.get(end + 2).filter(|v| !is_aux(v)) {
                            let (lemma, _) = read_verb(v);
                            let frame = Frame::new(&lemma, Tense::Present)
                                .modal(crate::ir::Modal::Will)
                                .with(Role::Agent, subject);
                            return finish_predicate(frame, after, end + 3, Force::Ask);
                        }
                    }
                }
            }
            if let Some(act) = read_aspect_question(words, i, wh) {
                return Some(act);
            }
            return read_copula_question(words, i, wh);
        }
    }

    let op = words.get(i)?;
    // The operator is do-support (dropped, carrying tense) or a modal.
    let (tense, modal) = read_operator(op)?;
    i += 1;

    // The subject.
    let (subject, after) = read_subject(&words[i..])?;
    i += after;

    // Optional `not` after the subject: "did they not tell you?".
    let mut neg = false;
    if words.get(i).map(String::as_str) == Some("not") {
        neg = true;
        i += 1;
    }

    // The main verb. After do-support or a modal it is a bare lemma; the
    // reader reads it as-is (abstaining if it is itself an auxiliary, an
    // aspect this path does not build).
    let verb = words.get(i)?;
    if is_aux(verb) {
        return None;
    }
    let (lemma, _) = read_verb(verb);
    i += 1;

    let mut frame = Frame::new(&lemma, tense);
    if let Some(m) = modal {
        frame = frame.modal(m);
    }
    if neg {
        frame = frame.negative();
    }

    // The subject's role: a wh=who takes the agent, so the read subject is the
    // theme; otherwise the subject is the agent (the theme when there is no
    // agent is handled by placing the wh).
    let subject_is_agent = wh != Some(Wh::Who);
    if subject_is_agent {
        frame = frame.with(Role::Agent, subject);
    } else {
        frame = frame.with(Role::Theme, subject);
    }

    // A control verb with its own-subject infinitive ("do you want to eat
    // now?"): the inner clause is the theme, as in the declarative.
    if wh.is_none() && lists::control_verbs().contains(&lemma.as_str()) && words.get(i).map(String::as_str) == Some("to") {
        if let Some(inner) = read_infinitive(&words[i + 1..], frame.get(&Role::Agent)?) {
            return Some(Act::new(Force::Ask, frame.with(Role::Theme, Filler::clause(inner.frame))));
        }
    }

    // A leading object pronoun after the verb is a recipient ("who gave her
    // the book"); then the object noun phrase, then prepositions.
    let rest = &words[i..];
    let mut cursor = 0;
    if leading_recipient(&lemma, rest) {
        frame = frame.with(Role::Recipient, Filler::pronoun(&canon_pronoun(&rest[0])));
        cursor = 1;
    }
    let body = &rest[cursor..];
    let mut consumed = 0;
    if !body.is_empty() && !is_preposition(&body[0]) {
        if let Some((role, filler, end)) = read_theme(body) {
            frame = frame.with(role, filler);
            consumed = end;
        }
    }
    let tail = &body[consumed..];
    let (preps, prep_words) = read_prepositions(tail);
    for (role, filler) in preps {
        frame = frame.with(role, filler);
    }
    if consumed + prep_words != body.len() {
        return None;
    }

    // Place the wh as a Hole in its role.
    if let Some(wh) = wh {
        frame = place_wh(frame, wh, subject_is_agent);
    }
    Some(Act::new(Force::Ask, frame))
}

/// Move a `not` (or the `cannot` of "can't") that directly follows the
/// operator at `op_at` to just after the subject: `do not you have` ->
/// `do you not have`. Words are returned unchanged when no subject follows,
/// so a subject-less wh-question ("what isn't working?") is untouched.
pub(in crate::ladder::act) fn lift_operator_negation(words: &[String], op_at: usize) -> Vec<String> {
    let Some(op) = words.get(op_at) else { return words.to_vec() };
    let (operator, after) = if op == "cannot" {
        ("can".to_string(), op_at + 1)
    } else if words.get(op_at + 1).map(String::as_str) == Some("not") {
        (op.clone(), op_at + 2)
    } else {
        return words.to_vec();
    };
    if after >= words.len() {
        return words.to_vec();
    }
    let Some((_, end)) = read_subject(&words[after..]) else { return words.to_vec() };
    let mut out: Vec<String> = words[..op_at].to_vec();
    out.push(operator);
    out.extend_from_slice(&words[after..after + end]);
    out.push("not".into());
    out.extend_from_slice(&words[after + end..]);
    out
}

/// Place a wh as a Hole in the role the Guide stores for it: who->agent (or
/// theme when the subject already took the agent), what/which->theme,
/// where/when/why/how->their preposition role.
pub(in crate::ladder::act) fn place_wh(frame: Frame, wh: Wh, subject_is_agent: bool) -> Frame {
    match wh {
        Wh::Who => {
            if subject_is_agent {
                frame.with(Role::Theme, Filler::Hole(wh))
            } else {
                frame.with(Role::Agent, Filler::Hole(wh))
            }
        }
        Wh::What | Wh::Which => frame.with(Role::Theme, Filler::Hole(wh)),
        _ => frame.with(wh_role(wh), Filler::Hole(wh)),
    }
}

/// The preposition role the Guide stores an adjunct wh under.
pub(in crate::ladder::act) fn wh_role(wh: Wh) -> Role {
    Role::named(crate::lexicon::lists::wh_preposition(wh.name()).unwrap_or("by"))
}

/// Read a question operator into `(tense, modal)`: do-support drops and
/// carries tense; a modal keeps its sense and the tense its past form shows.
/// Returns `None` for an aspect/passive operator this reader does not build.
pub(in crate::ladder::act) fn read_operator(word: &str) -> Option<(Tense, Option<crate::ir::Modal>)> {
    match aux_entry(word)? {
        Entry { tense, family: Family::Do, operator: true } => Some((tense, None)),
        Entry { tense, family: Family::Modal(modal), operator: true } => Some((tense, Some(modal))),
        _ => None,
    }
}
