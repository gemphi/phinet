//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Realise an Act back to a normalised sentence, with the terminating mark
/// the Force implies: a question takes `?`, everything else `.`. The output
/// is already normalised - lower case, single-spaced - so it compares
/// directly against a read sentence.
pub fn realise(act: &Act) -> String {
    let frame = &act.frame;
    let words = match act.force {
        // An imperative drops its subject and fronts the bare verb: "close
        // the window", "do not open the door" (Murphy Unit 35).
        Force::Direct => imperative(frame),
        // A question fronts any wh-word and inverts the operator with the
        // subject (Murphy Unit 49-52).
        Force::Ask => question(frame),
        // A wish is announced by "i wish", then the wished-for content.
        Force::Wish => {
            let mut said = vec!["i".to_string(), "wish".to_string()];
            said.extend(wish::content(frame).unwrap_or_else(|| statement(frame)));
            said
        }
        // Everything else is a statement in subject-verb-object order.
        _ => express::phrase(act.force, frame).unwrap_or_else(|| statement(frame)),
    };
    let mark = if act.force == Force::Ask { '?' } else { '.' };
    let sentence = format!("{} {mark}", words.join(" "));
    normalise(&sentence.replace(" .", ".").replace(" ?", "?"))
}

/// [`realise`] with each proper name written back in its own case, as in
/// `she lives in London.` Everything else stays lower case.
pub fn realise_cased(act: &Act) -> String {
    names::recase(&realise(act), &names::held(&act.frame))
}
/// A plain statement: subject, verb group, objects, complement, adjuncts.
/// Whether a frame is an existential: `be`, active, no modal, no agent and no
/// `as`-complement, with a theme. "there is a problem" is read and written as
/// one of these.
pub(in crate::ladder::act) fn is_existential(frame: &Frame) -> bool {
    frame.pred == "be"
        && frame.voice == Voice::Active
        && frame.modal.is_none()
        && frame.aspect == Aspect::Simple
        // Exactly one slot, the theme: a bare "there is X". A copula with a
        // complement or a prepositional phrase has more, and is an ordinary copula.
        && frame.slots.len() == 1
        && frame.get(&Role::Theme).is_some()
}

/// Whether an existential theme takes a singular copula ("there is a problem"
/// vs "there are tables"). A pronoun or a singular thing is singular.
pub(in crate::ladder::act) fn theme_is_singular(theme: &Filler) -> bool {
    match theme {
        Filler::Thing(t) => !t.plural,
        _ => true,
    }
}

pub(in crate::ladder::act) fn statement(frame: &Frame) -> Vec<String> {
    // An existential: "there is a problem", "there were no tables". The frame
    // is `be` with a theme and nothing else; English fronts the dummy "there"
    // and the copula agrees with the theme that follows.
    if is_existential(frame) {
        if let Some(theme) = frame.get(&Role::Theme) {
            let third = theme_is_singular(theme);
            let be = be_form(frame.tense, third, false);
            let mut words = vec!["there".to_string(), be];
            if frame.polarity == Polarity::Negative {
                words.push(negation_word(frame).into());
            }
            words.extend(say_filler(theme));
            return words;
        }
    }
    // An aspectual verb over a weather clause: "it would stop raining". The
    // frame has no agent and its theme is the clause, which is said as the
    // gerund after the verb, with the clause's own expletive `it` before it.
    if let Some(inner) = weather_gerund(frame) {
        let mut words = vec!["it".to_string()];
        words.extend(verb_group(frame));
        words.push(ing(&inner.pred));
        words.extend(adjuncts(frame, None));
        return words;
    }
    let mut words: Vec<String> = Vec::new();
    if let Some(filler) = subject_of(frame) {
        words.extend(say_filler(filler));
    } else if needs_expletive(frame) {
        // A weather verb has no argument but English still needs a subject:
        // "it is raining" (Murphy Unit 84 on empty `it`).
        words.push("it".into());
    }
    words.extend(verb_group(frame));
    words.extend(complements(frame, false));
    words.extend(adjuncts(frame, None));
    words
}

/// The weather clause an agentless gerund verb takes as its theme ("stop
/// raining"): a plain present active positive progressive with no argument.
pub(in crate::ladder::act) fn weather_gerund(frame: &Frame) -> Option<&Frame> {
    let Some(Filler::Clause(inner)) = frame.get(&Role::Theme) else { return None };
    let fits = frame.get(&Role::Agent).is_none()
        && frame.voice == Voice::Active
        && lists::gerund_verbs().contains(&frame.pred)
        && needs_expletive(inner)
        && inner.aspect == Aspect::Progressive
        && inner.tense == Tense::Present
        && inner.voice == Voice::Active
        && inner.polarity == Polarity::Positive
        && inner.modal.is_none();
    fits.then_some(&**inner)
}

/// The subject filler for a frame, voice-aware: active uses the agent (a
/// copula the theme); passive promotes the theme.
pub(in crate::ladder::act) fn subject_of(frame: &Frame) -> Option<&Filler> {
    if frame.voice == Voice::Passive {
        frame.get(&Role::Theme).or_else(|| frame.get(&Role::Agent))
    } else {
        frame.get(&Role::Agent).or_else(|| frame.get(&Role::Theme))
    }
}

pub(in crate::ladder::act) fn aux_entry(word: &str) -> Option<Entry> {
    crate::lexicon::auxiliary::entry(word)
}

/// Whether the word is an auxiliary of the given family (from `auxiliary.tsv`).
pub(in crate::ladder::act) fn aux_is(word: &str, family: Family) -> bool {
    crate::lexicon::auxiliary::is_family(word, family)
}

/// Whether the frame has no argument at all and so needs an expletive `it`
/// subject (a weather verb: rain, snow, ...).
pub(in crate::ladder::act) fn needs_expletive(frame: &Frame) -> bool {
    frame.slots.is_empty() && lists::weather_verbs().contains(&frame.pred)
}

