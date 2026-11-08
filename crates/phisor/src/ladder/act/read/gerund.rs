//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Whether `inner` is a gerund complement of `matrix`: the same agent and a
/// plain present active positive progressive, with no modal. The `-ing` form is
/// kept as progressive aspect, which a control infinitive (simple aspect) never
/// has, so the two are told apart.
pub(in crate::ladder::act) fn is_gerund_complement(matrix: &Frame, inner: &Frame) -> bool {
    let same_subject = matches!(
        (matrix.get(&Role::Agent), inner.get(&Role::Agent)),
        (Some(a), Some(b)) if a == b
    );
    same_subject
        && inner.modal.is_none()
        && inner.tense == Tense::Present
        && inner.aspect == Aspect::Progressive
        && inner.voice == Voice::Active
        && inner.polarity == Polarity::Positive
}

/// The gerund reading of a predicate tail: a gerund verb followed by an `-ing`
/// form, which opens a clause sharing the matrix agent and read to the end of
/// the sentence. Stored as the theme, with progressive aspect. Returns `None`
/// unless the whole tail is accounted for - never a partial frame (R3.4). A
/// blanked exercise stem has no verb after the `-ing` word and does not reach
/// here: Well_Formed removes those, not the Reader (R7.5).
pub(in crate::ladder::act) fn finish_with_gerund(mut frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    // `read_verb` undoes a regular `-ed` by guessing the lemma ("enjoyed" ->
    // "enjoye", "admitted" -> "admitt"). The gerund verbs are a closed list, so
    // name the lemma whose own past re-realises the surface word.
    if let Some(surface) = from.checked_sub(1).and_then(|i| words.get(i)) {
        if !lists::gerund_verbs().contains(&frame.pred.as_str()) {
            if let Some(g) = lists::gerund_verbs().iter().find(|g| inflect(g, Suffix::PAST) == *surface || inflect(g, Suffix::THIRD) == *surface) {
                frame.pred = (*g).to_string();
            }
        }
    }
    if frame.voice != Voice::Active
        || frame.aspect != Aspect::Simple
        || frame.modal.is_some()
        || !(lists::gerund_verbs().contains(&frame.pred.as_str()) || lists::control_verbs().contains(&frame.pred.as_str()))
        || frame.get(&Role::Theme).is_some()
    {
        return None;
    }
    let agent = frame.get(&Role::Agent)?.clone();
    let word = words.get(from)?;
    if is_aux(word) {
        return None;
    }
    // A host ending in `-ing` ("something", "nothing") is never a gerund.
    if lists::to_modified_hosts().contains(&word.as_str()) {
        return None;
    }
    let lemma = read_ing(word)?;
    let inner = Frame::new(&lemma, Tense::Present).with(Role::Agent, agent).aspect(Aspect::Progressive);
    let inner = finish_predicate(inner, words, from + 1, Force::Assert)?;
    Some(Act::new(force, frame.with(Role::Theme, Filler::clause(inner.frame))))
}

/// Read a to-infinitive complement clause: the words after "to", with the
/// controlling subject copied in as the clause's agent (control). The inner
/// verb is a bare lemma ("to go", "to pay"); a nested "to" recurses, so
/// "promise to try to pay the money" composes. Returns the inner frame, or
/// `None` when the complement does not read cleanly (the matrix verb then
/// abstains rather than guess).
pub(in crate::ladder::act) fn read_infinitive(words: &[String], subject: &Filler) -> Option<Act> {
    let verb = words.first()?;
    // "to be a teacher": the copula infinitive takes the controlling subject
    // as its theme and the rest as its complement, as a finite copula does.
    if crate::lexicon::lists::is_role(verb, "copula_lemma") {
        let rest = &words[1..];
        let mut frame = Frame::new("be", Tense::Present).with(Role::Theme, subject.clone());
        if rest.first().is_some_and(|w| is_preposition(w)) {
            let (preps, used) = read_prepositions(rest);
            if preps.is_empty() || used != rest.len() {
                return None;
            }
            for (role, filler) in preps {
                frame = frame.with(role, filler);
            }
        } else {
            frame = frame.with(Role::As, complement_filler(rest)?);
        }
        return Some(Act::new(Force::Assert, frame));
    }
    if is_aux(verb) || crate::lexicon::lists::is_role(verb, "infinitive_marker") {
        return None;
    }
    // The infinitive verb is a bare lemma; read it as a present-simple clause.
    let (lemma, _) = read_verb(verb);
    let mut frame = Frame::new(&lemma, Tense::Present).with(Role::Agent, subject.clone());
    // A nested to-infinitive ("to try to pay"): the inner clause is the theme.
    // A nested "to" is another infinitive only after a control verb; after any
    // other it is a preposition ("go to london").
    if lists::control_verbs().contains(&lemma.as_str()) && words.get(1).map(String::as_str) == Some("to") {
        let inner = read_infinitive(&words[2..], subject)?;
        frame = frame.with(Role::Theme, Filler::clause(inner.frame));
        return Some(Act::new(Force::Assert, frame));
    }
    // Otherwise the usual complements, read from just past the inner verb.
    finish_predicate(frame, words, 1, Force::Assert)
}

/// Whether `tail` opens with `to` and a bare verb directly after `host`, a word
/// from `to_modified_host.txt`: the infinitive then modifies the host.
pub(in crate::ladder::act) fn infinitive_after_host(host: &str, tail: &[String]) -> bool {
    lists::to_modified_hosts().contains(&host)
        && tail.first().is_some_and(|w| crate::lexicon::lists::is_role(w, "infinitive_marker"))
        && tail.get(1).is_some_and(|w| is_purpose_verb(w))
}

/// A theme that is a host ("something", "anywhere") followed by `to` + a bare
/// verb takes the infinitive as its own `to` attribute: "has something to
/// read" is `something ~to [read]`, with `read` a verb, never the noun of a
/// `to` phrase. The infinitive shares the matrix agent (no agent, no rule), is
/// said without a subject, and must read to the end of `tail`. Returns the theme and the words consumed past it (0 when
/// the rule does not apply, leaving the theme as read).
pub(in crate::ladder::act) fn with_host_infinitive(theme: Filler, tail: &[String], agent: Option<&Filler>) -> (Filler, usize) {
    let Some(agent) = agent else { return (theme, 0) };
    let head = match &theme {
        Filler::Thing(t) if t.det.is_none() && t.attrs.is_empty() && !t.plural => t.head.clone(),
        Filler::Pronoun(p) => p.clone(),
        _ => return (theme, 0),
    };
    if !infinitive_after_host(&head, tail) {
        return (theme, 0);
    }
    let lemma = tail[1].clone();
    let inner = Frame::new(&lemma, Tense::Present).with(Role::Agent, agent.clone());
    let Some(inner) = finish_predicate(inner, tail, 2, Force::Assert) else {
        return (theme, 0);
    };
    let host = crate::ir::Thing::new(&head).with(crate::ir::Attr::Via("to".into()), Filler::clause(inner.frame));
    (Filler::Thing(host), tail.len())
}

