//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// The reader proper, over a sentence already expanded and stripped of its
/// set-aside adverbs. `norm` is the normalised sentence, kept for its mark.
pub(in crate::ladder::act) fn read_plain(norm: &str, words: Vec<String>) -> Option<Act> {
    if words.len() < 2 {
        return None;
    }
    // Segment a subject noun phrase, then read either a copula (`X is Y`)
    // into a `be` frame with the complement in the `as` role, or a simple
    // lexical verb with an optional one-word object. Anything it cannot
    // segment cleanly is uncovered, never mis-parsed.
    // An imperative (Murphy Unit 35): a statement that opens with a base
    // verb and has no subject. Recognised only for the closed set of
    // instructional verbs the Guide uses, so a subject noun is never
    // mistaken for a command.
    // A question (ends in ?): a wh-word and/or an inverted operator. Parsed
    // by the mirror of the question realiser; abstains on aspects, passives
    // and proper-name objects it cannot read soundly.
    if norm.ends_with('?') {
        return read_question(&words);
    }

    if !norm.ends_with('?') {
        if let Some(act) = read_imperative(&words) {
            return Some(act);
        }
    }

    // An existential: "there is a problem", "there are no free tables". The
    // dummy "there" and a copula open it; the rest is the theme, and a plural
    // theme is kept plural so the copula agrees on the way back out.
    if words.first().map(String::as_str) == Some("there") && !norm.ends_with('?') {
        if let Some(tense) = words.get(1).and_then(|w| copula_tense(w)) {
            let (neg, rest_at) = if words.get(2).map(String::as_str) == Some("not") {
                (true, 3)
            } else {
                (false, 2)
            };
            let rest = &words[rest_at..];
            if let Some((theme, end)) = read_noun_phrase(rest) {
                // Trailing prepositional phrases ("in the sky") are kept as Via slots.
                let (preps, used) = read_prepositions(&rest[end..]);
                if end + used == rest.len() {
                    let mut frame = Frame::new("be", tense).with(Role::Theme, theme);
                    for (role, filler) in preps {
                        frame = frame.with(role, filler);
                    }
                    if neg {
                        frame = frame.negative();
                    }
                    return Some(Act::new(Force::Assert, frame));
                }
            }
        }
    }

    let (subject, after_subject) = read_subject(&words)?;
    // "i wish <clause>" where the clause is a past-subjunctive modal group from
    // `wish_form.tsv` ("i wish we did not have to go"). Tried before negation is
    // looked for, because the clause's own "did not" is not the wish's negation.
    if matches!(&subject, Filler::Pronoun(p) if p == "I")
        && words.get(after_subject).is_some_and(|w| is_wish_present(w))
        && !norm.ends_with('?')
    {
        let after = &words[after_subject + 1..];
        let after = if after.first().map(String::as_str) == Some("that") { &after[1..] } else { after };
        if let Some(inner) = wish::read_clause(after) {
            return Some(Act::new(Force::Wish, inner.frame));
        }
    }
    let (neg, verb_at) = detect_negation_from(&words, after_subject);
    let verb = words.get(verb_at)?;
    let force = if norm.ends_with('?') { Force::Ask } else { Force::Assert };

    // The near-future "be going to V": a copula, then "going to", then a bare
    // verb. The Guide records it as a `will` modal over V, so the reader builds
    // that frame and the round trip treats "going to" and "will" alike.
    if copula_tense(verb).is_some()
        && words.get(verb_at + 1).map(String::as_str) == Some("going")
        && words.get(verb_at + 2).map(String::as_str) == Some("to")
    {
        if let Some(v) = words.get(verb_at + 3).filter(|v| !is_aux(v)) {
            let (lemma, _) = read_verb(v);
            let mut frame = Frame::new(&lemma, Tense::Present).modal(crate::ir::Modal::Will);
            if neg {
                frame = frame.negative();
            }
            frame = frame.with(Role::Agent, subject);
            return finish_predicate(frame, &words, verb_at + 4, force);
        }
    }

    // Present-tense copula: `is`, `are`, `am`. The complement is the rest of
    // the sentence as one filler, in the `as` role, matching the gold shape
    // `be  theme=<subject>  as=<complement>`. `is boiling` / `is repaired` is
    // a progressive or a passive, not a copula with a complement: when the
    // verb group reads as one (below), this branch defers to it rather than
    // reading the participle as a predicate complement.
    if let (Some(be_tense), true) = (copula_tense(verb), read_verb_group(&words, verb_at).is_none()) {
        let rest = &words[verb_at + 1..];
        if rest.is_empty() {
            return None;
        }
        let mut frame = Frame::new("be", be_tense);
        if neg {
            frame = frame.negative();
        }
        frame = frame.with(Role::Theme, subject);
        // The complement is a word or noun phrase in the `as` role ("is
        // empty", "was a hot day"), or a prepositional phrase under its own
        // preposition ("is from australia", "is for washing the dishes").
        if is_preposition(&rest[0]) {
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
        return Some(Act::new(force, frame));
    }

    // An auxiliary verb group (perfect, progressive, perfect progressive,
    // passive): read it into its lemma, tense, aspect and voice, the inverse
    // of the realiser's verb_group. The subject is the agent in the active
    // and the promoted theme in the passive (Murphy Unit 42-44).
    if let Some((lemma, tense, aspect, voice, vg_neg, consumed)) = read_verb_group(&words, verb_at) {
        let mut frame = Frame::new(&lemma, tense).aspect(aspect);
        if voice == Voice::Passive {
            frame = frame.passive();
        }
        if vg_neg || neg {
            frame = frame.negative();
        }
        // A weather verb's `it` is an empty expletive, not an argument: "it
        // has been raining" has no agent (Murphy Unit 84).
        if !is_expletive_subject(&lemma, &subject) {
            let subject_role = if voice == Voice::Passive { Role::Theme } else { Role::Agent };
            frame = frame.with(subject_role, subject);
        }
        return finish_predicate(frame, &words, verb_at + consumed, force);
    }

    // A modal verb group ("will arrive", "could not sing", "may have been",
    // "will be finished"): the modal fixes the sense and tense, then a
    // tenseless tail carries the aspect and voice - the inverse of the
    // realiser's modal branch. The subject is the agent in the active and
    // the promoted theme in the passive.
    if let Some((modal, m_tense)) = read_modal(verb) {
        let mut j = verb_at + 1;
        let mut m_neg = neg;
        if verb == "cannot" {
            m_neg = true;
        } else if words.get(j).map(String::as_str) == Some("not") {
            m_neg = true;
            j += 1;
        }
        if let Some((lemma, aspect, voice, tail)) = read_modal_tail(&words, j) {
            let mut frame = Frame::new(&lemma, m_tense).modal(modal).aspect(aspect);
            if voice == Voice::Passive {
                frame = frame.passive();
            }
            if m_neg {
                frame = frame.negative();
            }
            if !is_expletive_subject(&lemma, &subject) {
                let subject_role = if voice == Voice::Passive { Role::Theme } else { Role::Agent };
                frame = frame.with(subject_role, subject);
            }
            return finish_predicate(frame, &words, j + tail, force);
        }
        return None;
    }

    // A lexical verb: abstain if the verb slot is any other auxiliary (a
    // modal group the aspect reader leaves to a later reader).
    if is_aux(verb) && !aux_is(verb, Family::Have) {
        return None;
    }
    // A pronoun is never the verb: "she she went" has a second subject where
    // the verb should be (the list is data/lex/pronoun.tsv).
    if lists::pronouns().contains(&verb.as_str()) {
        return None;
    }
    // A lexical verb: read its lemma and the tense it carries, so "boils"
    // becomes (boil, Present) and "wore" becomes (wear, Past).
    let (lemma, tense) = read_verb(verb);
    let mut frame = Frame::new(&lemma, tense);
    if neg {
        // Do-support ("did not go") is read as a past the auxiliary carries,
        // not the bare lemma: "we did not go out" is past.
        if matches!(words.get(verb_at.wrapping_sub(2)).map(String::as_str), Some("did")) {
            frame = Frame::new(&lemma, Tense::Past);
        }
        frame = frame.negative();
    }
    // A catenative verb takes a to-infinitive complement ("i want to go to
    // london", "i promise to come"): the matrix verb's theme is the inner
    // clause, whose subject is the same as the matrix subject (control). The
    // inner clause is read by the reader itself, so it composes to any
    // depth ("promise to try to pay").
    if lists::control_verbs().contains(&lemma.as_str()) && words.get(verb_at + 1).map(String::as_str) == Some("to") {
        if let Some(inner) = read_infinitive(&words[verb_at + 2..], &subject) {
            frame = frame.with(Role::Agent, subject);
            frame = frame.with(Role::Theme, Filler::clause(inner.frame));
            return Some(Act::new(force, frame));
        }
    }

    // An explicit performative: first-person "i wish X" is a Wish force, and
    // the frame is the wished-for content X, as the gold records it.
    if lists::role_of(&lemma) == Some(Force::Wish) && tense == Tense::Present && !neg && matches!(&subject, Filler::Pronoun(p) if p == "I") {
        let after = &words[verb_at + 1..];
        let after = if after.first().map(String::as_str) == Some("that") { &after[1..] } else { after };
        if after.len() >= 2 {
            if let Some(inner) = read_inner("x.", after.to_vec()).filter(|a| clause_verb_is_verbal(&a.frame.pred)) {
                return Some(Act::new(Force::Wish, inner.frame));
            }
        }
    }

    // A verb that takes a finite clause ("i think she works in a shop"): the
    // rest of the sentence, less an optional "that", is read as a sentence of
    // its own and becomes the theme. If it does not read as a clause the verb
    // is read as before, so a plain object ("i know the answer") is untouched.
    if lists::complement_verbs().contains(&lemma.as_str()) {
        let after = &words[verb_at + 1..];
        let after = if after.first().map(String::as_str) == Some("that") { &after[1..] } else { after };
        // A clause opens with a subject, never an object pronoun ("you in your car").
        let opens_clean = after.first().is_some_and(|w| !is_object_pronoun(w) || w == "you");
        if after.len() >= 2 && opens_clean {
            if let Some(inner) = read_inner("x.", after.to_vec()).filter(|a| clause_verb_is_verbal(&a.frame.pred)) {
                frame = frame.with(Role::Agent, subject.clone());
                frame = frame.with(Role::Theme, Filler::clause(inner.frame));
                return Some(Act::new(force, frame));
            }
        }
    }

    frame = frame.with(Role::Agent, subject);
    // The plain reading first; only when it abstains is a gerund complement
    // ("i enjoy reading the book") tried, so nothing that read is changed.
    finish_predicate(frame.clone(), &words, verb_at + 1, force)
        .or_else(|| finish_with_gerund(frame, &words, verb_at + 1, force))
}


/// Whether the word reads as a present-tense verb that names the wish role.
fn is_wish_present(word: &str) -> bool {
    let (lemma, tense) = read_verb(word);
    tense == Tense::Present && lists::role_of(&lemma) == Some(Force::Wish)
}
