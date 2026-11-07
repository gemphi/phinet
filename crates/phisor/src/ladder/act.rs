//! The act height: an honest first English reader and realiser (R5.11,
//! R10.10, R10.11, task 13.7).
//!
//! This reads a training sentence into an [`Act`] and realises an Act back to
//! a sentence, for the gold-frame shapes it genuinely handles: simple
//! declaratives and copulas, present and past, the four aspects, active and
//! passive, positive and negative, with subject, object, a `be`-complement,
//! and prepositional phrases. It does not claim to parse all English. What it
//! cannot read it reports as uncovered, and the four act-height numbers are
//! measured honestly over what it can - a recorded miss is the truth when the
//! reader falls short of a Bar, which is better than leaving the number
//! unjudged.
//!
//! The reader and the realiser are inverse over the shapes they share, which
//! is what [`round_trip`] measures: a sentence read to an Act and realised
//! back equals the original under [`normalise`](crate::guide::holdout::normalise),
//! the terminating mark kept (R10.11).

use crate::guide::holdout::normalise;
use crate::lexicon::auxiliary::{Entry, Family};
use crate::lexicon::lists;
use crate::ir::{Act, Aspect, Filler, Force, Frame, Polarity, Role, Tense, Thing, Voice, Wh};

mod express;
mod names;
mod wish;
#[cfg(test)]
mod tests;

/// The terminating marks a sentence may keep (R10.11).
const MARKS: [char; 3] = ['.', '?', '!'];

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
fn is_existential(frame: &Frame) -> bool {
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
fn theme_is_singular(theme: &Filler) -> bool {
    match theme {
        Filler::Thing(t) => !t.plural,
        _ => true,
    }
}

fn statement(frame: &Frame) -> Vec<String> {
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
fn weather_gerund(frame: &Frame) -> Option<&Frame> {
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

/// The imperative: no subject, the verb as a bare lemma, then its objects and
/// adjuncts. Negation is `do not` before the verb.
fn imperative(frame: &Frame) -> Vec<String> {
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

/// A question: front the wh-word (if any), then invert the operator with the
/// subject, then the remainder of the verb group, objects and adjuncts. A
/// yes/no question has no wh and simply inverts. When the wh is the subject
/// (or a modifier of it) there is no inversion - the wh stays in place and
/// the verb stays in statement order ("how many people came?").
fn question(frame: &Frame) -> Vec<String> {
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

/// The subject filler for a frame, voice-aware: active uses the agent (a
/// copula the theme); passive promotes the theme.
fn subject_of(frame: &Frame) -> Option<&Filler> {
    if frame.voice == Voice::Passive {
        frame.get(&Role::Theme).or_else(|| frame.get(&Role::Agent))
    } else {
        frame.get(&Role::Agent).or_else(|| frame.get(&Role::Theme))
    }
}

fn aux_entry(word: &str) -> Option<Entry> {
    crate::lexicon::auxiliary::entry(word)
}

/// Whether the word is an auxiliary of the given family (from `auxiliary.tsv`).
fn aux_is(word: &str, family: Family) -> bool {
    crate::lexicon::auxiliary::is_family(word, family)
}

/// Whether the frame has no argument at all and so needs an expletive `it`
/// subject (a weather verb: rain, snow, ...).
fn needs_expletive(frame: &Frame) -> bool {
    frame.slots.is_empty() && lists::weather_verbs().contains(&frame.pred)
}

/// The objects in English order: recipient then theme in the active, the
/// be-complement in the `as` role. Passive puts neither after the verb.
fn complements(frame: &Frame, subject_suppressed: bool) -> Vec<String> {
    complements_skipping(frame, None, subject_suppressed)
}

/// Say a matrix verb's theme. A clause that shares the matrix subject and is
/// otherwise a plain present active positive ("i want [i go to london]") is a
/// control infinitive and is said "to go to london", the clause's own
/// complements following and a nested infinitive composing ("promise to try
/// to pay"). Any other clause - a finite one such as "i wish i could sing" -
/// is said as before.
fn say_theme(matrix: &Frame, theme: &Filler) -> Vec<String> {
    if let Filler::Clause(inner) = theme {
        if lists::control_verbs().contains(&matrix.pred.as_str()) && is_control_infinitive(matrix, inner) {
            let mut out = vec!["to".to_string(), inner.pred.clone()];
            out.extend(complements_skipping(inner, None, true));
            out.extend(adjuncts(inner, None));
            return out;
        }
        // A gerund complement: the `-ing` form, then the clause's complements.
        if (lists::gerund_verbs().contains(&matrix.pred.as_str()) || lists::control_verbs().contains(&matrix.pred.as_str()))
            && is_gerund_complement(matrix, inner)
        {
            let mut out = vec![ing(&inner.pred)];
            out.extend(complements_skipping(inner, None, true));
            out.extend(adjuncts(inner, None));
            return out;
        }
        // Any other clause is finite: its own subject, verb group and complements.
        return statement(inner);
    }
    say_filler(theme)
}

/// Whether `inner` is a control infinitive of `matrix`: the same agent, and no
/// tense, modal, aspect, voice or polarity of its own to say.
fn is_control_infinitive(matrix: &Frame, inner: &Frame) -> bool {
    let same_subject = matches!(
        (matrix.get(&Role::Agent), inner.get(&Role::Agent)),
        (Some(a), Some(b)) if a == b
    );
    same_subject
        && inner.modal.is_none()
        && inner.tense == Tense::Present
        && inner.aspect == Aspect::Simple
        && inner.voice == Voice::Active
        && inner.polarity == Polarity::Positive
}

/// As [`complements`], but omit the slot a fronted wh already consumed.
fn complements_skipping(frame: &Frame, skip: Option<&Role>, subject_suppressed: bool) -> Vec<String> {
    let mut out = Vec::new();
    let passive = frame.voice == Voice::Passive;
    let drop = |r: &Role| skip == Some(r);
    // The theme is a post-verbal object only when it is not the surface
    // subject: an active frame with an agent, or an imperative whose subject
    // is suppressed. An agentless active statement (a copula, an unaccusative)
    // has the theme as its subject, so it is not repeated here.
    if !passive && (frame.get(&Role::Agent).is_some() || subject_suppressed) {
        if let Some(recipient) = frame.get(&Role::Recipient) {
            if !drop(&Role::Recipient) {
                out.extend(say_filler(recipient));
            }
        }
        if let Some(theme) = frame.get(&Role::Theme) {
            if !drop(&Role::Theme) {
                out.extend(say_theme(frame, theme));
            }
        }
    }
    if let Some(c) = frame.get(&Role::As) {
        if !drop(&Role::As) {
            out.extend(say_filler(c));
        }
    }
    out
}

/// The prepositional and `by`-agent adjuncts in slot order, skipping a
/// fronted wh's slot. A bare time/place adverb (tomorrow, here, later) drops
/// its preposition: the gold stores "on=tomorrow" but English says
/// "tomorrow", not "on tomorrow".
fn adjuncts(frame: &Frame, skip: Option<&Role>) -> Vec<String> {
    let mut out = Vec::new();
    let passive = frame.voice == Voice::Passive;
    for (role, filler) in &frame.slots {
        if skip == Some(role) {
            // "who was the window broken by?" - the passive agent is the
            // fronted wh, but English strands the "by" at the end.
            if passive && role == &Role::Agent && matches!(filler, Filler::Hole(_)) {
                out.push("by".into());
            }
            continue;
        }
        match role {
            Role::Via(prep) => {
                if let Filler::Hole(_) = filler {
                    continue; // a wh already fronted, or an unfronted hole dropped
                }
                // A conjunction and its clause: "... and he stays", "... if i have time".
                if let Filler::Clause(inner) = filler {
                    out.push(prep.clone());
                    // A purpose infinitive ("... to buy bread"): `to` over a
                    // clause that shares the matrix agent is said as `to` + the
                    // bare verb and its complements, not as a finite statement.
                    if prep == "to" && is_control_infinitive(frame, inner) {
                        out.push(inner.pred.clone());
                        out.extend(complements_skipping(inner, None, true));
                        out.extend(adjuncts(inner, None));
                    } else {
                        out.extend(statement(inner));
                    }
                    continue;
                }
                if is_bare_adverb(filler) {
                    out.extend(say_filler(filler));
                } else {
                    out.push(prep.clone());
                    out.extend(say_filler(filler));
                }
            }
            Role::Agent if passive && frame.get(&Role::Theme).is_some() => {
                out.push("by".into());
                out.extend(say_filler(filler));
            }
            _ => {}
        }
    }
    out
}

/// Whether a Via filler is a bare adverb that English says without its
/// stored preposition: tomorrow, today, yesterday, here, there, now, later.
fn is_bare_adverb(filler: &Filler) -> bool {
    matches!(
        filler,
        Filler::Thing(t) if t.det.is_none() && t.attrs.is_empty()
            && (is_time_adverb(&t.head) || is_place_adverb(&t.head))
    )
}

/// Split the first word of a verb group off as the inversion operator. A
/// plain simple-active lexical verb (no modal, no aux) has no operator, so
/// do-support supplies one and the verb drops back to its lemma: "you live"
/// -> operator "do", rest "live"; "she arrived" -> "did" + "arrive".
fn split_operator(frame: &Frame, mut group: Vec<String>) -> (String, Vec<String>) {
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
fn wh_surface(wh: Wh) -> String {
    match wh {
        Wh::HowMany => "how many".into(),
        _ => wh.name().to_string(),
    }
}
/// Find a wh-hole anywhere in the frame's slots, returning its role and the
/// Wh. The first hole wins (the gold frames carry at most one).
fn find_wh(frame: &Frame) -> Option<(Role, Wh)> {
    frame.slots.iter().find_map(|(role, filler)| match filler {
        Filler::Hole(wh) => Some((role.clone(), *wh)),
        _ => None,
    })
}

/// Whether a role is the frame's surface subject, so a wh in it does not
/// trigger inversion.
fn is_subject_role(frame: &Frame, role: &Role) -> bool {
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
fn subject_has_wh(frame: &Frame) -> bool {
    matches!(
        subject_of(frame),
        Some(Filler::Thing(t)) if t.attrs.iter().any(|(_, f)| matches!(f, Filler::Hole(_)))
    )
}

/// The verb group for a frame: the auxiliaries its aspect, voice, modality
/// and polarity imply, then the predicate in the form the aspect needs. This
/// is deliberately a small, honest English generator over the closed verb
/// stratum, not a full morphology.
fn verb_group(frame: &Frame) -> Vec<String> {
    let third = third_singular(frame);
    let neg = frame.polarity == Polarity::Negative;
    // `never` is the negation word when the frame carries it, else `not`.
    let neg_w = neg.then(|| negation_word(frame));
    // A modal (`will`, `can`): the lexical verb stays a bare lemma after it,
    // negation is `modal not` (`modal never`).
    if let Some(modal) = frame.modal {
        let word = modal_word(modal, frame.tense);
        let mut out = Vec::new();
        if neg && word == "can" && !frame.never {
            out.push("cannot".into()); // the book writes it as one word
        } else {
            out.push(word);
            if let Some(w) = neg_w { out.push(w.into()); }
        }
        // After a modal the rest is tenseless: `be` and `have` are bare, so a
        // modal composes with the aspect and the voice (Murphy Unit 21-30):
        // "will be finished", "may have been a mistake", "will not be coming".
        out.extend(modal_tail(frame));
        return out;
    }
    match (frame.aspect, frame.voice) {
        (Aspect::Simple, Voice::Active) => simple_active(frame, third, neg),
        (Aspect::Simple, Voice::Passive) => {
            // `be` + past participle: "the window was broken", "is closed".
            let be = be_form(frame.tense, third, is_first_person(frame));
            with_not(vec![be], neg_w, participle(&frame.pred))
        }
        (Aspect::Progressive, Voice::Active) => {
            // `be` + -ing: "a girl is wearing".
            let be = be_form(frame.tense, third, is_first_person(frame));
            with_not(vec![be], neg_w, ing(&frame.pred))
        }
        (Aspect::Progressive, Voice::Passive) => {
            // `be being` + past participle: "the car is being repaired".
            let be = be_form(frame.tense, third, is_first_person(frame));
            with_not(vec![be, "being".into()], neg_w, participle(&frame.pred))
        }
        (Aspect::Perfect, Voice::Active) => {
            // `have` + past participle: "she has worn", "they have not finished".
            let have = have_form(frame.tense, third);
            with_not(vec![have], neg_w, participle(&frame.pred))
        }
        (Aspect::Perfect, Voice::Passive) => {
            // `have been` + past participle: "the house has been sold".
            let have = have_form(frame.tense, third);
            with_not(vec![have, "been".into()], neg_w, participle(&frame.pred))
        }
        (Aspect::PerfectProgressive, _) => {
            // `have been` + -ing: "she has been wearing".
            let have = have_form(frame.tense, third);
            with_not(vec![have, "been".into()], neg_w, ing(&frame.pred))
        }
    }
}

/// The word that writes a frame's negation: `never` when the Reader read it,
/// otherwise `not`.
fn negation_word(frame: &Frame) -> &'static str {
    if frame.never { "never" } else { "not" }
}

/// Prepend the auxiliaries, insert `not` after the first when negative, then
/// the main form: `has not finished`, `is not working`.
fn with_not(mut aux: Vec<String>, neg: Option<&str>, main: String) -> Vec<String> {
    if let Some(word) = neg {
        aux.insert(1.min(aux.len()), word.into());
    }
    aux.push(main);
    aux
}

/// A modal's surface form, in the past where the frame is past: `can` ->
/// `could`, `will` -> `would`, `may` -> `might`, `shall` -> `should`. The
/// book's Unit 19-25 modal table.
fn modal_word(modal: crate::ir::Modal, tense: Tense) -> String {
    use crate::ir::Modal;
    match (modal, tense) {
        (Modal::Can, Tense::Past) => "could".into(),
        (Modal::Will, Tense::Past) => "would".into(),
        (Modal::May, Tense::Past) => "might".into(),
        (Modal::Shall, Tense::Past) => "should".into(),
        _ => modal.name().to_string(),
    }
}

/// The copula `be` surface form for a tense and number: is/are/am, was/were.
fn be_form(tense: Tense, third: bool, first: bool) -> String {
    match (tense, third, first) {
        (Tense::Present, _, true) => "am".into(),
        (Tense::Present, true, _) => "is".into(),
        (Tense::Present, false, _) => "are".into(),
        (Tense::Past, _, true) => "was".into(),
        (Tense::Past, true, _) => "was".into(),
        (Tense::Past, false, _) => "were".into(),
    }
}

/// The `have` surface form: has/have (present), had (past).
fn have_form(tense: Tense, third: bool) -> String {
    match (tense, third) {
        (Tense::Present, true) => "has".into(),
        (Tense::Present, false) => "have".into(),
        (Tense::Past, _) => "had".into(),
    }
}

/// The verb forms after a modal, which are all tenseless (bare `be`, bare
/// `have`), composed from the aspect and voice: the book's modal + perfect /
/// progressive / passive combinations.
fn modal_tail(frame: &Frame) -> Vec<String> {
    match (frame.aspect, frame.voice) {
        (Aspect::Simple, Voice::Active) => vec![frame.pred.clone()],
        (Aspect::Simple, Voice::Passive) => vec!["be".into(), participle(&frame.pred)],
        (Aspect::Progressive, Voice::Active) => vec!["be".into(), ing(&frame.pred)],
        (Aspect::Progressive, Voice::Passive) => {
            vec!["be".into(), "being".into(), participle(&frame.pred)]
        }
        (Aspect::Perfect, Voice::Active) => vec!["have".into(), participle(&frame.pred)],
        // `be` under modal+perfect is just "have been": the participle of be
        // is been, and there is no second main verb.
        (Aspect::Perfect, Voice::Passive) => {
            vec!["have".into(), "been".into(), participle(&frame.pred)]
        }
        (Aspect::PerfectProgressive, _) => {
            vec!["have".into(), "been".into(), ing(&frame.pred)]
        }
    }
}

/// The simple active present/past: `do/does/did not` for negation, else the
/// verb with its present `-s` (third singular) or past form.
fn simple_active(frame: &Frame, third: bool, neg: bool) -> Vec<String> {
    // `be` is the copula: it has its own forms (is/are/am/was/were) and takes
    // `not` directly, never do-support.
    if frame.pred == "be" {
        let be = be_form(frame.tense, third, is_first_person(frame));
        return if neg { vec![be, negation_word(frame).into()] } else { vec![be] };
    }
    let mut out = Vec::new();
    // `never` sits before a lexical verb that keeps its own tense: "she never
    // eats fish", no do-support.
    if neg && frame.never {
        out.push("never".into());
    } else if neg {
        out.push(match frame.tense {
            Tense::Past => "did".into(),
            Tense::Present => if third { "does".into() } else { "do".into() },
        });
        out.push("not".into());
        out.push(frame.pred.clone()); // after do-support the verb is bare
        return out;
    }
    out.push(match frame.tense {
        Tense::Past => past(&frame.pred),
        Tense::Present => if third { third_s(&frame.pred) } else { frame.pred.clone() },
    });
    out
}

/// The present third-person `-s` form: `boil` -> `boils`, `go` -> `goes`,
/// `fly` -> `flies`. The closed rule Murphy Unit 2 teaches.
fn third_s(verb: &str) -> String {
    if verb == "have" {
        return "has".into();
    }
    if verb.ends_with('s') || verb.ends_with('x') || verb.ends_with("ch") || verb.ends_with("sh") || verb.ends_with('o') {
        format!("{verb}es")
    } else if let Some(stem) = verb.strip_suffix('y') {
        if stem.chars().last().is_some_and(|c| !"aeiou".contains(c)) {
            return format!("{stem}ies");
        }
        format!("{verb}s")
    } else {
        format!("{verb}s")
    }
}

/// The `-ing` present participle: `wear` -> `wearing`, `wait` -> `waiting`,
/// `rain` -> `raining`, a silent-e dropped (`make` -> `making`).
fn ing(verb: &str) -> String {
    if let Some(form) = crate::lexicon::irregular::ing(verb) {
        return form;
    }
    if doubles_final(verb) {
        return format!("{verb}{}ing", verb.chars().last().unwrap_or_default());
    }
    if let Some(stem) = verb.strip_suffix('e') {
        if !stem.is_empty() && !verb.ends_with("ee") {
            return format!("{stem}ing");
        }
    }
    format!("{verb}ing")
}

/// Whether `-ed`/`-ing` double the verb's final consonant: a one-syllable
/// lemma that ends consonant + single vowel + consonant (stop, sit, run,
/// plan), or a named stressed multi-syllable verb. A final `w`, `x` or `y`
/// never doubles, and `qu` counts as a consonant (quit -> quitting). The
/// irregular table overrides this for the past (`past` consults it first).
fn doubles_final(verb: &str) -> bool {
    if lists::doubling_stressed().contains(&verb) {
        return true;
    }
    let masked = verb.replace("qu", "kw");
    let chars: Vec<char> = masked.chars().collect();
    let n = chars.len();
    let vowel = |c: char| "aeiou".contains(c);
    if n < 3 || !chars.iter().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    let groups = chars.windows(2).filter(|w| vowel(w[1]) && !vowel(w[0])).count() + usize::from(vowel(chars[0]));
    !vowel(chars[n - 1])
        && !"wxy".contains(chars[n - 1])
        && vowel(chars[n - 2])
        && !vowel(chars[n - 3])
        && groups == 1
}

/// Undo the doubling in a stem read off `-ed`/`-ing` (`stopp` -> `stop`). The
/// bare stem wins when the lexicon knows it as a verb ("add", "kiss", "purr");
/// `s`, `f` and `z` never undouble, and a lemma under three letters is not
/// proposed. Otherwise the result is a lemma that would double again.
fn undouble(stem: &str) -> Option<String> {
    let mut it = stem.chars().rev();
    let (last, prev) = (it.next()?, it.next()?);
    if last != prev || "aeiouyswfz".contains(last) || crate::lexicon::verb_capable(stem) {
        return None;
    }
    let lemma = &stem[..stem.len() - last.len_utf8()];
    let named = lists::doubling_stressed().contains(&lemma);
    if (last == 'l' && !named) || lemma.len() < 3 || !doubles_final(lemma) {
        return None;
    }
    Some(lemma.to_string())
}

/// The past participle used by the perfect and the passive. Irregulars the
/// gold set uses are named; the rest take `-ed`. These are the book's own
/// irregular list, learned once.
fn participle(verb: &str) -> String {
    crate::lexicon::irregular_participle(verb)
        .or_else(|| crate::lexicon::irregular::participle(verb))
        .unwrap_or_else(|| past(verb))
}

/// Read a surface past participle back to its lemma, the inverse of
/// [`participle`]. An irregular is recognised by trying each known lemma and
/// confirming it re-realises to the surface; a regular `-ed` form is inverted
/// by the silent-e and consonant rules, again confirmed by re-realising.
/// Returns `None` when no lemma reproduces the surface, so an ambiguous or
/// unknown word makes the whole verb group abstain rather than guess.
fn read_participle(word: &str) -> Option<String> {
    // Irregulars: a known lemma whose participle is this surface word.
    for lemma in crate::lexicon::irregular::lemmas().iter() {
        if participle(lemma) == word {
            return Some(lemma.to_string());
        }
    }
    if let Some(lemma) = crate::lexicon::lemma_of_participle(word) {
        return Some(lemma);
    }
    // Regular `-ed`: the candidate lemmas are the bare stem and the stem with
    // a restored silent e (preferred for the known silent-e verbs); the
    // consonant+y rule restores `y` from `ied`. Confirmed by re-realising.
    let candidates = regular_ed_candidates(word);
    for candidate in &candidates {
        if lists::silent_e_lemmas().contains(&candidate.as_str()) && participle(candidate) == word {
            return Some(candidate.clone());
        }
    }
    for candidate in &candidates {
        if participle(candidate) == word {
            return Some(candidate.clone());
        }
    }
    None
}

/// Read a surface `-ing` present participle back to its lemma, the inverse of
/// [`ing`]. The `-ing` suffix loses whether a silent e was dropped ("living"
/// could be `liv` or `live`), so the reader restores the e only for the known
/// silent-e verbs the Guide uses and otherwise keeps the bare stem; each
/// candidate is confirmed by re-realising, and an unresolved form abstains.
fn read_ing(word: &str) -> Option<String> {
    let stem = word.strip_suffix("ing")?;
    if stem.is_empty() {
        return None;
    }
    // Prefer the silent-e restoration for the closed set of e-dropping verbs
    // the Guide uses; otherwise the bare stem. Confirm by re-realising so a
    // mismatch abstains rather than guesses.
    if let Some(lemma) = undouble(stem).filter(|l| ing(l) == word) {
        return Some(lemma);
    }
    let with_e = format!("{stem}e");
    if lists::silent_e_lemmas().contains(&with_e.as_str()) && ing(&with_e) == word {
        return Some(with_e);
    }
    if ing(stem) == word {
        return Some(stem.to_string());
    }
    if ing(&with_e) == word {
        return Some(with_e);
    }
    None
}

/// The lemma candidates for a regular `-ed` participle: the bare stem, the
/// stem with a restored silent e, and the consonant+y restoration from `ied`.
fn regular_ed_candidates(word: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(stem) = word.strip_suffix("ied") {
        out.push(format!("{stem}y"));
    }
    if let Some(stem) = word.strip_suffix("ed") {
        out.extend(undouble(stem));
        out.push(stem.to_string());
        out.push(format!("{stem}e"));
    }
    out
}

/// The simple past form. Irregulars the gold set uses are named; the rest
/// take `-ed` (with the silent-e and consonant rules the book gives).
fn past(verb: &str) -> String {
    if let Some(p) = crate::lexicon::irregular_past(verb).or_else(|| crate::lexicon::irregular::past(verb)) {
        return p;
    }
    if doubles_final(verb) {
        return format!("{verb}{}ed", verb.chars().last().unwrap_or_default());
    }
    if let Some(stem) = verb.strip_suffix('e') {
        if !stem.is_empty() {
            return format!("{verb}d");
        }
    }
    if let Some(stem) = verb.strip_suffix('y') {
        if stem.chars().last().is_some_and(|c| !"aeiou".contains(c)) {
            return format!("{stem}ied");
        }
    }
    format!("{verb}ed")
}

/// Whether the subject is third-person singular (a noun, or he/she/it), so
/// the present verb takes `-s` and negation uses `does`. The first reader's
/// gold shapes are third-person; a fuller reader would track number.
fn third_singular(frame: &Frame) -> bool {
    match frame.get(&Role::Agent).or_else(|| frame.get(&Role::Theme)) {
        Some(Filler::Pronoun(p)) => lists::third_singular_pronouns().contains(p.to_lowercase()),
        Some(Filler::Thing(t)) => !t.plural,
        _ => true,
    }
}

/// Whether the subject is first person (`I`/`we`), so the present copula is
/// `am`/`are` and a fuller agreement can be read off it.
fn is_first_person(frame: &Frame) -> bool {
    match frame.get(&Role::Agent).or_else(|| frame.get(&Role::Theme)) {
        Some(Filler::Pronoun(p)) => {
            lists::first_person_pronouns().contains(p.to_lowercase())
        }
        _ => false,
    }
}

/// Say a filler as words: a name/pronoun/word as itself, a thing as its
/// determiner, attributes and head.
fn say_filler(filler: &Filler) -> Vec<String> {
    match filler {
        Filler::Name(n) | Filler::Pronoun(n) | Filler::Word(n) => vec![n.clone()],
        Filler::Hole(wh) => vec![wh_surface(*wh)],
        Filler::Thing(thing) => say_thing(thing),
        Filler::Clause(frame) => verb_group(frame),
    }
}

/// Say a thing: determiner, modifiers, head (pluralised crudely with `s`).
fn say_thing(thing: &Thing) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(det) = &thing.det {
        if let Some(mut word) = determiner_word(det, thing.plural) {
            // `a` -> `an` before a vowel sound (the head, or an adjective if
            // one precedes it). A shallow vowel-letter test, which the gold
            // set's words obey.
            if word == "a" {
                let next = thing.attrs.iter().find_map(|(_, f)| match f {
                    Filler::Word(w) => Some(w.clone()),
                    _ => None,
                }).unwrap_or_else(|| thing.head.clone());
                if starts_vowel_sound(&next) {
                    word = "an".into();
                }
            }
            out.push(word);
        }
    }
    // The attributes the book teaches: an adjective (mod or a known kind), an
    // ordinal, a count. They sit before the head: "a blue suit", "cold
    // climates", "one hundred degrees". A prepositional attribute (`~of`)
    // follows the head.
    let mut post: Vec<String> = Vec::new();
    for (attr, value) in &thing.attrs {
        use crate::ir::Attr;
        match attr {
            Attr::Via(prep) => {
                post.push(prep.clone());
                post.extend(say_filler(value));
            }
            _ => out.extend(say_filler(value)),
        }
    }
    // Never pluralise a function word that slipped into a Thing head (a
    // mis-segmented "we"/"at"): the foundation lexicon keeps it whole.
    let head = if thing.plural && crate::lexicon::inflectable(&thing.head) {
        pluralise(&thing.head)
    } else {
        thing.head.clone()
    };
    out.push(head);
    out.extend(post);
    out
}

/// Whether a word starts with a vowel *sound*, so the indefinite article is
/// `an`: the vowel letters, plus silent-h words (hour, honest, honour), minus
/// the `u`/`eu`-as-consonant words (university, european) the book notes.
fn starts_vowel_sound(word: &str) -> bool {
    let w = word.to_lowercase();
    if let Some((_, article)) = lists::article_exceptions().iter().find(|(prefix, _)| w.starts_with(prefix.as_str())) {
        return article == "an";
    }
    w.chars().next().is_some_and(|c| "aeiou".contains(c))
}
/// The determiner surface form, agreeing with number: `this`/`these`,
/// `that`/`those`.
fn determiner_word(det: &crate::ir::Det, plural: bool) -> Option<String> {
    use crate::ir::Det;
    Some(match det {
        Det::Definite => "the".into(),
        Det::Indefinite => "a".into(),
        Det::Near => if plural { "these".into() } else { "this".into() },
        Det::Far => if plural { "those".into() } else { "that".into() },
        Det::Quant(q) => q.clone(),
        Det::Owner(o) => o.clone(),
    })
}

/// The plural surface form. Irregulars the gold set uses are named; a noun
/// ending vowel+y takes `-s` (monday -> mondays), consonant+y takes `-ies`,
/// a sibilant `-es`, the rest `-s`.
fn pluralise(head: &str) -> String {
    if let Some((_, plural)) = lists::irregular_plurals().iter().find(|(singular, _)| singular == head) {
        return plural.clone();
    }
    if head.ends_with('s') || head.ends_with('x') || head.ends_with("ch") || head.ends_with("sh") {
        format!("{head}es")
    } else if let Some(stem) = head.strip_suffix('y') {
        if stem.chars().last().is_some_and(|c| "aeiou".contains(c)) {
            format!("{head}s")
        } else {
            format!("{stem}ies")
        }
    } else {
        format!("{head}s")
    }
}
// ----- The reader: English -> Act, for the shapes it handles -----

/// Expand contractions into the words they stand for, so the reader meets "do
/// not" where the text wrote "don't" (phiecs tokenises a contraction as one
/// word). The expansion is rule-driven, from data files (see
/// `crate::lexicon::contraction`): an irregular whole form (`won't`, `can't`)
/// first, then a clitic suffix (`n't`, `'re`, `'ve`, `'ll`, `'m`) on any token.
/// The genuinely ambiguous `'s` (is/has/possessive) is resolved by the word
/// that follows it, and only after he/she/it; `'d` (would/had) is left as read,
/// so the reader abstains on them rather than guess. This directly lifts
/// coverage on real text, which writes its auxiliaries contracted.
fn expand_contractions(words: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for (idx, w) in words.iter().enumerate() {
        // The rule-driven contractions (data/lex/contraction_*.tsv): an
        // irregular whole form first, then any clitic suffix on any token.
        if let Some(parts) = crate::lexicon::contraction::expand(w) {
            out.extend(parts);
            continue;
        }
        // Ambiguous: 's (is/has/possessive) is left as the surface word unless
        // its host is a pronoun that takes 's as a verb (he, she, it), so the
        // reader abstains rather than guess. "he's", "she's", "it's" is "is" or
        // "has", and what follows says which: a cue word ("been", "got") means
        // has; an -ing form, or a word that cannot be a participle, means is
        // ("it's raining", "he's a nice person"). An -ed or irregular
        // participle ("he's tired") stays ambiguous, so it is left as read.
        match w.strip_suffix("'s").filter(|b| lists::s_hosts().contains(b)) {
            Some(base) => match following_word(words, idx) {
                Some(next) if lists::s_has_cues().contains(next) => {
                    out.extend([base.to_string(), "has".to_string()])
                }
                Some(next) if read_ing(next).is_some() || read_participle(next).is_none() => {
                    out.extend([base.to_string(), "is".to_string()])
                }
                _ => out.push(w.clone()),
            },
            None => out.push(w.clone()),
        }
    }
    out
}
/// The word after position `idx`, skipping the adverbs the reader sets aside,
/// so a contraction can be resolved by the word that really follows it.
fn following_word(words: &[String], idx: usize) -> Option<&str> {
    words[idx + 1..].iter().map(String::as_str).find(|w| !lists::set_aside_adverbs().contains(w))
}

/// Whether the verb of a clause read as a complement is plausibly a verb. The
/// reader takes the word after a subject as its verb without checking, so a
/// preposition could be read as one; when the foundation lexicon is primed it
/// is asked, and the copula and auxiliaries are always accepted.
fn clause_verb_is_verbal(pred: &str) -> bool {
    lists::aux_lemmas().contains(pred)
        || !crate::lexicon::is_primed()
        || crate::lexicon::verb_capable(pred)
}


/// Remove the set-aside adverb phrases, then the single set-aside adverbs, from
/// a word list. Returns the remaining words and whether "never" was among them
/// (it carries negation). Phrases are matched first so a word inside a kept
/// phrase is not stripped on its own.
fn strip_adverbs(words: &[String]) -> (Vec<String>, bool) {
    strip_adverbs_keeping(words, false)
}

/// As [`strip_adverbs`], optionally leaving "never" in the words, so the
/// round-trip comparison checks where it is written.
fn strip_adverbs_keeping(words: &[String], keep_never: bool) -> (Vec<String>, bool) {
    let without_phrases = strip_adverb_phrases(words);
    let mut never = false;
    let kept = without_phrases
        .iter()
        .filter(|w| {
            let is_adverb = lists::set_aside_adverbs().contains(&w.as_str());
            if is_adverb && w.as_str() == "never" {
                never = true;
            }
            !is_adverb || (keep_never && w.as_str() == "never")
        })
        .cloned()
        .collect();
    (kept, never)
}

/// Drop every contiguous run of words that equals a [`SET_ASIDE_ADVERB_PHRASES`]
/// entry. Phrases do not overlap: once a run matches it is consumed whole.
fn strip_adverb_phrases(words: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(words.len());
    let mut i = 0;
    while i < words.len() {
        let matched = lists::set_aside_adverb_phrases()
            .iter()
            .filter(|phrase| phrase.len() <= words.len() - i)
            .find(|phrase| phrase.iter().zip(&words[i..]).all(|(p, w)| p == w));
        match matched {
            Some(phrase) => i += phrase.len(),
            None => {
                out.push(words[i].clone());
                i += 1;
            }
        }
    }
    out
}

/// The closed word lists the Reader consults, each as a named list of
/// lower-case words, as loaded from their files under data/lex. Exposed so
/// the harness can audit how much held-out text each covers and how much would change if the list were
/// learned instead (R5.2, R5.3). The names match the lists' identifiers in
/// this module. This is a read-only view: it does not change the Reader.
pub fn word_lists() -> Vec<(&'static str, &'static [String])> {
    vec![
        ("control_verbs", lists::control_verbs().as_slice()),
        ("gerund_verbs", lists::gerund_verbs().as_slice()),
        ("complement_verbs", lists::complement_verbs().as_slice()),
        ("performatives", lists::performatives().as_slice()),
        ("set_aside_adverbs", lists::set_aside_adverbs().as_slice()),
        ("clause_conjunctions", lists::clause_conjunctions().as_slice()),
    ]
}

/// Read a normalised English sentence into an Act, or `None` when the
/// sentence is a shape this first reader does not handle. The caller counts a
/// `None` as uncovered - never guesses.
pub fn read(sentence: &str) -> Option<Act> {
    let norm = normalise(sentence);
    let body: String = norm.trim_end_matches(|c| MARKS.contains(&c)).trim().to_string();
    let raw: Vec<String> = body.split_whitespace().map(str::to_string).collect();
    let expanded = expand_contractions(&raw);
    // Set aside the manner/frequency adverbs the IR has no slot for, as the
    // gold does; "never" is also read as negation.
    let (words, never) = strip_adverbs(&expanded);
    let mut act = read_inner(&norm, words)?;
    // "i promise ..." is an explicit commissive: the verb itself says so.
    if act.force == Force::Assert
        && act.frame.pred == "promise"
        && act.frame.tense == Tense::Present
        && matches!(act.frame.get(&Role::Agent), Some(Filler::Pronoun(p)) if p == "I")
    {
        act.force = Force::Commit;
    }
    if never {
        act.frame.polarity = Polarity::Negative;
        act.frame.never = true;
    }
    // `normalise` lowered the names; the raw sentence still spells them.
    names::restore(&mut act.frame, &names::spelled(sentence));
    Some(act)
}

/// The reader over a sentence already expanded and stripped of its set-aside
/// adverbs. A sentence is read whole when it reads whole; only when it does not
/// is it tried as two clauses joined by a conjunction, so a sentence that reads
/// today is never disturbed by the split.
fn read_inner(norm: &str, words: Vec<String>) -> Option<Act> {
    if let Some(act) = read_plain(norm, words.clone()) {
        return Some(act);
    }
    if norm.ends_with('?') {
        return None;
    }
    read_compound(&words)
}

/// Read "A <conjunction> B" where A and B each read as a clause of their own.
/// A conjunction between two nouns ("karen and joe played tennis") is not a
/// split, because the words before it are not a clause; so this only fires
/// when both sides read, and both are plain assertions.
fn read_compound(words: &[String]) -> Option<Act> {
    if words.len() < 5 {
        return None;
    }
    // Find the first conjunction whose left side is a clause of its own: that
    // clause is the head the trailing clauses hang under. A conjunction between
    // two nouns ("karen and joe played tennis") has no clause on its left, so
    // it is skipped and the sentence stays uncovered.
    for k in 2..=(words.len() - 3) {
        if !lists::clause_conjunctions().contains(&words[k].as_str()) {
            continue;
        }
        let Some(head) = read_plain_clause(&words[..k]) else { continue };
        // Attach every trailing "<conjunction> <clause>" in turn, looping past
        // the first conjunction so a sentence with three or more clauses reads
        // whole. Each trailing clause is a plain verbal assertion delimited by
        // the next clause conjunction (or the end); any gap that is not such a
        // clause means the whole sentence is uncovered - never a partial frame.
        if let Some(act) = read_trailing_clauses(head, words, k) {
            return Some(act);
        }
    }
    None
}

/// A span of words read as a plain verbal assertion: the unit a compound hangs
/// its clauses on. `None` unless it reads whole as an `Assert` over a verbal
/// predicate, so a bare noun phrase or a question is never taken for a clause.
fn read_plain_clause(words: &[String]) -> Option<Frame> {
    let act = read_inner("x.", words.to_vec())?;
    (act.force == Force::Assert && clause_verb_is_verbal(&act.frame.pred)).then_some(act.frame)
}

/// Attach each trailing "<conjunction> <clause>" onto `head`, starting at the
/// conjunction `first`. Loops over every following clause conjunction, reading
/// the span up to the next one (or the sentence end) as a plain clause. Returns
/// the whole frame only when every span reads; otherwise `None`, never a frame
/// missing any trailing words.
fn read_trailing_clauses(head: Frame, words: &[String], first: usize) -> Option<Act> {
    let mut frame = head;
    let mut conj = first;
    loop {
        let start = conj + 1;
        // The next clause conjunction after this one bounds the current clause;
        // with none left, the clause runs to the end of the sentence.
        let next = (start + 1..words.len())
            .find(|&i| lists::clause_conjunctions().contains(&words[i].as_str()));
        let end = next.unwrap_or(words.len());
        let clause = read_plain_clause(&words[start..end])?;
        frame = frame.with(Role::named(&words[conj]), Filler::clause(clause));
        match next {
            Some(n) => conj = n,
            None => return Some(Act::new(Force::Assert, frame)),
        }
    }
}

/// The reader proper, over a sentence already expanded and stripped of its
/// set-aside adverbs. `norm` is the normalised sentence, kept for its mark.
fn read_plain(norm: &str, words: Vec<String>) -> Option<Act> {
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
                if end == rest.len() {
                    let mut frame = Frame::new("be", tense).with(Role::Theme, theme);
                    if neg {
                        frame = frame.negative();
                    }
                    return Some(Act::new(Force::Assert, frame));
                }
            }
        }
    }

    let (subject, after_subject) = read_subject(&words)?;
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
    if lemma == "wish" && tense == Tense::Present && !neg && matches!(&subject, Filler::Pronoun(p) if p == "I") {
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

/// Whether `inner` is a gerund complement of `matrix`: the same agent and a
/// plain present active positive progressive, with no modal. The `-ing` form is
/// kept as progressive aspect, which a control infinitive (simple aspect) never
/// has, so the two are told apart.
fn is_gerund_complement(matrix: &Frame, inner: &Frame) -> bool {
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
fn finish_with_gerund(mut frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    // `read_verb` undoes a regular `-ed` by guessing the lemma ("enjoyed" ->
    // "enjoye", "admitted" -> "admitt"). The gerund verbs are a closed list, so
    // name the lemma whose own past re-realises the surface word.
    if let Some(surface) = from.checked_sub(1).and_then(|i| words.get(i)) {
        if !lists::gerund_verbs().contains(&frame.pred.as_str()) {
            if let Some(g) = lists::gerund_verbs().iter().find(|g| past(g) == *surface || third_s(g) == *surface) {
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
fn read_infinitive(words: &[String], subject: &Filler) -> Option<Act> {
    let verb = words.first()?;
    if is_aux(verb) || verb == "to" {
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

/// Read the words after the verb group into the frame: an optional object
/// (theme), trailing prepositional phrases, and - in the passive - a trailing
/// `by <np>` demoted agent (Murphy Unit 42-44). Returns the Act, or `None`
/// when any word is left unaccounted for, so a partial parse abstains rather
/// than dropping material it cannot place.
///
/// A sentence that reads whole today is never disturbed: the plain reading is
/// tried first, and only when it abstains is a trailing purpose infinitive
/// ("went to the shop to buy bread") peeled off and read (R3.3, task 6.3).
fn finish_predicate(frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    finish_plain(frame.clone(), words, from, force)
        .or_else(|| finish_with_purpose(frame, words, from, force))
}

/// Whether `word` can open a purpose infinitive: a bare lexical verb. The
/// primed foundation lexicon decides (verb-capable, not a function word);
/// unprimed, only the reader's closed instructional verbs count, so the rule
/// stays sound without the lexicon. An inflected form ("buying", "buys") is
/// not a bare verb and does not qualify.
fn is_purpose_verb(word: &str) -> bool {
    if is_aux(word) || is_determiner(word) || is_preposition(word) || lists::pronouns().contains(&word) {
        return false;
    }
    let known = if crate::lexicon::is_primed() {
        crate::lexicon::verb_capable(word) && !crate::lexicon::function_word(word)
    } else {
        lists::imperative_verbs().contains(&word)
    };
    known && read_verb(word).0 == word
}

/// The purpose-infinitive reading of a predicate tail: the first `to` followed
/// by a bare verb splits the tail in two. The words before it must read whole
/// on their own (object and prepositional phrases), and the words after it read
/// as an infinitive clause sharing the matrix agent. The clause is stored under
/// `Via("to")`, the way a prepositional `to` phrase is. Returns `None` unless
/// the whole tail is accounted for - never a partial frame (R3.4).
///
/// Not read: a passive or agentless frame (no controlling subject), and a
/// control verb, whose `to` is its own complement and is read by the control
/// branch, so a control infinitive is never read twice.
fn finish_with_purpose(frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    if frame.voice != Voice::Active || lists::control_verbs().contains(&frame.pred.as_str()) {
        return None;
    }
    let agent = frame.get(&Role::Agent)?.clone();
    for k in from..words.len().saturating_sub(1) {
        if words[k] != "to" || !is_purpose_verb(&words[k + 1]) {
            continue;
        }
        if k > 0 && lists::to_modified_hosts().contains(&words[k - 1].as_str()) {
            continue;
        }
        let Some(head) = finish_plain(frame.clone(), &words[..k], from, force) else { continue };
        let Some(inner) = read_infinitive(&words[k + 1..], &agent) else { continue };
        let frame = head.frame.with(Role::Via("to".into()), Filler::clause(inner.frame));
        return Some(Act::new(force, frame));
    }
    None
}

/// The plain reading of the words after the verb group (see
/// [`finish_predicate`]): objects and prepositional phrases only.
fn finish_plain(mut frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    let passive = frame.voice == Voice::Passive;
    let pred = frame.pred.clone();
    let rest = &words[from..];
    let mut i = 0;

    // The active voice takes post-verbal objects: a ditransitive recipient
    // first ("could have told you"), then the theme. The passive has promoted
    // its theme to subject, so it reads no object - but a bare time/place
    // adverb that reads under a preposition is still an adjunct in either
    // voice ("will be finished tomorrow").
    if !passive {
        if leading_recipient(&pred, &rest[i..]) {
            frame = frame.with(Role::Recipient, Filler::pronoun(&canon_pronoun(&rest[i])));
            i += 1;
        }
        let here = &rest[i..];
        if !here.is_empty() && !is_preposition(&here[0]) && !is_by_agent(here) {
            if let Some((role, filler, end)) = read_theme(here) {
                frame = frame.with(role, filler);
                i += end;
            }
        }
    } else {
        // In the passive, read only a leading bare adverb as its adjunct
        // (not a theme); other objects are ungrammatical here.
        let here = &rest[i..];
        if here.first().is_some_and(|w| is_time_adverb(w) || is_place_adverb(w)) {
            if let Some((role, filler, end)) = read_theme(here) {
                frame = frame.with(role, filler);
                i += end;
            }
        }
    }

    let tail = &rest[i..];
    // The passive demoted agent: a trailing `by <np>` is the agent, not a
    // `Via("by")` adjunct ("has been repaired by a mechanic").
    let (by_words, after_by) = if passive && is_by_agent(tail) {
        match read_noun_phrase(&tail[1..]) {
            Some((agent, end)) => {
                frame = frame.with(Role::Agent, agent);
                (1 + end, &tail[1 + end..])
            }
            None => (0, tail),
        }
    } else {
        (0, tail)
    };
    let (preps, prep_words) = read_prepositions(after_by);
    for (role, filler) in preps {
        frame = frame.with(role, filler);
    }
    if i + by_words + prep_words != rest.len() {
        // Trailing words the reader cannot account for: abstain rather than
        // drop them silently and claim a partial read.
        return None;
    }
    Some(Act::new(force, frame))
}

/// Whether the words begin a passive `by <noun phrase>` demoted agent, as
/// opposed to a `by` adjunct of means. A shallow test: `by` followed by a
/// noun phrase, used only in the passive where the gold stores the agent.
fn is_by_agent(words: &[String]) -> bool {
    words.first().map(String::as_str) == Some("by") && words.len() > 1
}

/// Read a post-verbal object into its role, filler and the words it spans. A
/// pronoun object stays a `Pronoun` ("do it"); a bare time adverb is stored
/// under `on` as the Guide does ("come tomorrow" -> on=tomorrow); otherwise a
/// noun phrase fills the theme. Returns `None` when nothing reads cleanly.
fn read_theme(words: &[String]) -> Option<(Role, Filler, usize)> {
    let first = words.first()?;
    if is_object_pronoun(first) || lists::pronouns().contains(&first.as_str()) {
        return Some((Role::Theme, Filler::pronoun(&canon_pronoun(first)), 1));
    }
    if is_time_adverb(first) {
        return Some((Role::named("on"), Filler::thing(first), 1));
    }
    if is_place_adverb(first) {
        return Some((Role::named("in"), Filler::thing(first), 1));
    }
    let (thing, end) = read_noun_phrase(words)?;
    Some((Role::Theme, thing, end))
}

/// A bare time adverb the Guide stores under a preposition: "tomorrow" ->
/// on=tomorrow, mirroring the realiser's bare-adverb handling.
fn is_time_adverb(word: &str) -> bool {
    lists::time_adverbs().contains(word)
}

/// A bare place adverb the Guide stores under `in`: "here" -> in=here,
/// mirroring the realiser's bare-adverb handling.
fn is_place_adverb(word: &str) -> bool {
    lists::place_adverbs().contains(word)
}

/// Whether a read subject is the empty expletive `it` of a weather verb, so
/// the reader drops it: "it has been raining" has no argument, matching the
/// realiser's `needs_expletive`.
fn is_expletive_subject(lemma: &str, subject: &Filler) -> bool {
    matches!(subject, Filler::Pronoun(p) if p.eq_ignore_ascii_case("it"))
        && lists::weather_verbs().contains(lemma)
}
/// Read the subject noun phrase from the front of the sentence, returning
/// the filler and the index just past it. A pronoun is one word; a
/// determiner-led or bare noun phrase is read by [`read_noun_phrase`]. The
/// reader abstains (None) when it cannot find a head, so it never treats a
/// verb as a subject.
fn read_subject(words: &[String]) -> Option<(Filler, usize)> {
    if lists::pronouns().contains(&words[0].as_str()) {
        return Some((subject_filler(&words[0]), 1));
    }
    let (thing, end) = read_noun_phrase(words)?;
    // A subject noun phrase must leave at least one word for the verb.
    if end >= words.len() {
        return None;
    }
    Some((thing, end))
}

/// Read a noun phrase from the front of `words`: an optional determiner, an
/// optional possessive/ordinal modifier, then a head noun. Returns the
/// `Filler::Thing` and the index just past the phrase. Deliberately shallow:
/// it stops at the first word that reads as a head, so `the first king of` is
/// read as `the king` and the `of` phrase is left to the caller or dropped,
/// rather than mis-attached.
fn read_noun_phrase(words: &[String]) -> Option<(Filler, usize)> {
    let mut i = 0;
    let mut det = None;
    if i < words.len() && is_determiner(&words[i]) {
        det = determiner_of(&words[i]);
        i += 1;
    }
    // One modifier before the head: an ordinal or number ("the first king",
    // "two cars"), or an adjective the foundation lexicon names ("a hot day",
    // "a blue suit"). An adjective is kept as a Mod attribute so it survives
    // the round trip; an ordinal/number modifier is skipped as before.
    let mut adjective: Option<String> = None;
    let mut count: Option<String> = None;
    if i + 1 < words.len() {
        if is_cardinal(&words[i]) {
            count = Some(words[i].clone());
            i += 1;
        } else if is_modifier(&words[i]) {
            i += 1;
        } else if is_adjective_word(&words[i]) {
            adjective = Some(words[i].clone());
            i += 1;
        }
    }
    let head = words.get(i)?;
    // A conjunction ("and", "but") joins; it is never the head of a noun phrase.
    if is_determiner(head) || is_aux(head) || is_copula(head) || lists::clause_conjunctions().contains(&head.as_str()) {
        return None;
    }
    let (singular, plural) = lemma_number(head);
    let mut thing = Thing::new(&singular);
    if let Some(adj) = adjective {
        thing = thing.with(crate::ir::Attr::Mod, Filler::word(&adj));
    }
    if let Some(c) = count {
        thing = thing.with(crate::ir::Attr::Count, Filler::word(&c)).plural();
    }
    if let Some(d) = det {
        thing = thing.det(d);
    }
    if plural {
        thing = thing.plural();
    }
    Some((Filler::Thing(thing), i + 1))
}

/// The determiner a word denotes, for the reader (inverse of
/// [`determiner_word`]). Only the closed set the gold frames use.
fn determiner_of(word: &str) -> Option<crate::ir::Det> {
    lists::determiner(word)
}

/// Whether a word reads as an adjective modifying the head that follows it.
/// The foundation lexicon decides when primed - adjective-capable and not a
/// noun - so the reader does not need an adjective list; unprimed, only a
/// small set the gold uses is treated as one, to stay sound.
fn is_adjective_word(word: &str) -> bool {
    if crate::lexicon::is_primed() {
        return matches!(crate::lexicon::tag(word), Some(t) if t.contains('a') && !t.contains('n') && !t.contains('v'))
            && !crate::lexicon::function_word(word);
    }
    lists::fallback_adjectives().contains(word)
}

/// Whether a word is a cardinal number that counts the head it precedes
/// ("four languages", "ten people", "20 years"): a number word the Guide uses
/// or a run of digits. Stored as a Count attribute, which the realiser writes
/// back before the head.
fn is_cardinal(word: &str) -> bool {
    lists::cardinals().contains(word) || (word.len() <= 4 && word.chars().all(|c| c.is_ascii_digit()))
}

/// A shallow ordinal/number modifier the noun phrase may carry before its
/// head.
fn is_modifier(word: &str) -> bool {
    lists::modifiers().contains(word)
}

/// Reduce a surface head to `(singular, is_plural)` by the crude inverse of
/// [`pluralise`]. Honest and shallow: irregular plurals (child/children) are
/// left as read, so the reader does not claim a lemma it cannot justify.
fn lemma_number(head: &str) -> (String, bool) {
    // Irregular plurals the realiser's `pluralise` names, inverted so the
    // reader recovers the singular lemma: "children" -> (child, plural).
    if let Some((singular, _)) = lists::irregular_plurals().iter().find(|(_, plural)| plural == head) {
        return (singular.clone(), true);
    }
    if let Some(stem) = head.strip_suffix("ies") {
        return (format!("{stem}y"), true);
    }
    if let Some(stem) = head.strip_suffix("es") {
        if (stem.ends_with('s') || stem.ends_with('x') || stem.ends_with("ch")) && pluralise(stem) == head {
            return (stem.to_string(), true);
        }
    }
    // A singular noun that happens to end in `s` is not a plural: the Guide's
    // closed set of these is kept whole ("bus", not "bu").
    if lists::singular_s_nouns().contains(&head) {
        return (head.to_string(), false);
    }
    // A plain `-s`: a plural when re-pluralising the stem reproduces the
    // surface.
    if let Some(stem) = head.strip_suffix('s') {
        if stem.len() >= 2 && pluralise(stem) == head {
            return (stem.to_string(), true);
        }
    }
    (head.to_string(), false)
}

/// The tense of a copula form the reader builds a `be` frame from: present for
/// is/are/am, past for was/were. `None` for any other word.
fn copula_tense(word: &str) -> Option<Tense> {
    aux_entry(word).filter(|e| e.family == Family::Be).map(|e| e.tense)
}

/// A present-tense copula the reader builds a `be` frame from.
fn is_copula(word: &str) -> bool {
    lists::copulas().contains(word)
}



/// The complement after a copula, as one filler: a single pronoun/word, or a
/// noun phrase that spans the rest. Abstains when the rest is more than one
/// clean phrase, so a copula with trailing prepositions is left uncovered.
fn complement_filler(rest: &[String]) -> Option<Filler> {
    if rest.len() == 1 {
        return Some(subject_filler(&rest[0]));
    }
    match read_noun_phrase(rest) {
        Some((thing, end)) if end == rest.len() => Some(thing),
        _ => None,
    }
}

/// Negation detection starting the verb search at `from` (just past the
/// subject), so a determiner-led subject does not shift the verb index.
fn detect_negation_from(words: &[String], from: usize) -> (bool, usize) {
    for i in from..words.len().saturating_sub(1) {
        if aux_is(&words[i], Family::Do) && words[i + 1] == "not" {
            return (true, i + 2);
        }
    }
    (false, from)
}

/// The closed auxiliaries the first reader declines to parse.
/// The determiners the first reader sees at the head of an unsegmented
/// subject noun phrase; their presence makes it abstain.
fn is_determiner(word: &str) -> bool {
    lists::determiners().contains(word)
}

fn is_aux(word: &str) -> bool {
    lists::auxiliaries().contains(word)
}

/// A subject word as a filler: a pronoun kept nominative, else a bare word.
fn subject_filler(word: &str) -> Filler {
    if lists::pronouns().contains(&word) {
        Filler::pronoun(&canon_pronoun(word))
    } else {
        Filler::word(word)
    }
}

/// Canonicalise a pronoun to the form the IR stores: the first person is the
/// capital `I`, matching the gold frames; the rest stay as read.
fn canon_pronoun(word: &str) -> String {
    if word == "i" { "I".into() } else { word.to_string() }
}

/// Read a surface verb into its lemma and the tense it carries, the inverse
/// of the realiser's morphology. Irregular pasts the gold set uses are named;
/// a regular `-ed` or third-person `-s` is undone by rule. A bare lemma reads
/// as the present. This lets the reader recover "boils" -> (boil, Present) and
/// "wore" -> (wear, Past) instead of storing the surface string.
fn read_verb(word: &str) -> (String, Tense) {
    if word == "has" {
        return ("have".into(), Tense::Present);
    }
    if let Some(lemma) = crate::lexicon::lemma_of_past(word) {
        // A form identical to its base ("put", "cut") is the present.
        if lemma != word {
            return (lemma, Tense::Past);
        }
    }
    // Irregular past -> lemma from the file (the inverse of the `past` table).
    if let Some(lemma) = crate::lexicon::irregular::lemma_of_past(word) {
        return (lemma, Tense::Past);
    }
    // Regular past `-ed`: "discovered" -> discover, "arrived" -> arrive,
    // "studied" -> study. Undo the doubling the realiser never adds for the
    // gold set, then the silent-e and the consonant+y rules.
    if let Some(stem) = word.strip_suffix("ied") {
        return (format!("{stem}y"), Tense::Past);
    }
    if let Some(stem) = word.strip_suffix("ed") {
        // "arrived" lost a silent e: if adding it back yields a known shape we
        // cannot tell, so prefer the e-form when the stem ends in a consonant
        // that the realiser would have written with `d` only. The gold's
        // regular pasts are e-verbs (arrive, like) or plain (-ed) verbs; the
        // reader keeps the lemma that re-realises, which `past` decides.
        if let Some(lemma) = undouble(stem).filter(|l| past(l) == word) {
            return (lemma, Tense::Past);
        }
        let with_e = format!("{stem}e");
        if past(&with_e) == word {
            return (with_e, Tense::Past);
        }
        if past(stem) == word {
            return (stem.to_string(), Tense::Past);
        }
        // Fall through: treat as past with the bare stem.
        return (stem.to_string(), Tense::Past);
    }
    // Third-person present `-s`: "boils" -> boil, "goes" -> go, "flies" ->
    // fly. Only undo it when re-realising the lemma reproduces the surface,
    // so a genuine noun-like verb is not mangled.
    if let Some(stem) = word.strip_suffix("ies") {
        let lemma = format!("{stem}y");
        if third_s(&lemma) == word {
            return (lemma, Tense::Present);
        }
    }
    if let Some(stem) = word.strip_suffix("es") {
        if third_s(stem) == word {
            return (stem.to_string(), Tense::Present);
        }
    }
    if let Some(stem) = word.strip_suffix('s') {
        if !stem.is_empty() && third_s(stem) == word {
            return (stem.to_string(), Tense::Present);
        }
    }
    (word.to_string(), Tense::Present)
}

/// Read an auxiliary verb group starting at `words[from]` into its lemma,
/// tense, aspect, voice, polarity and the number of words it spans - the
/// inverse of the realiser's [`verb_group`]. It recognises the perfect
/// (`have`+participle), the progressive (`be`+`-ing`), the perfect
/// progressive (`have been`+`-ing`), the passive (`be`+participle), and their
/// combinations, with an optional `not` after the first auxiliary. It returns
/// `None` the moment a word does not read as the auxiliary or participle the
/// chain needs, so an ambiguous group abstains rather than guesses. A modal
/// group is left to a later reader (returns `None`), as are plain simple
/// verbs (which the caller reads directly).
fn read_verb_group(words: &[String], from: usize) -> Option<(String, Tense, Aspect, Voice, bool, usize)> {
    let mut i = from;
    let head = words.get(i)?;
    // The first auxiliary fixes tense and the family. `have`/`has`/`had` open
    // a perfect; `be`-forms open a progressive or a passive.
    let (tense, family) = match aux_entry(head) {
        Some(Entry { tense, family: Family::Have, .. }) => (tense, Aux::Have),
        Some(Entry { tense, family: Family::Be, .. }) => (tense, Aux::Be),
        _ => return None, // a modal or a plain verb: not this reader's job
    };
    i += 1;

    // Optional `not` right after the first auxiliary: "have not finished",
    // "was not broken".
    let mut neg = false;
    if words.get(i).map(String::as_str) == Some("not") {
        neg = true;
        i += 1;
    }

    match family {
        Aux::Have => {
            // Perfect family. A bare `been` continues to a perfect progressive
            // (`been` + -ing) or a perfect passive (`been` + participle);
            // otherwise the next word is the participle of a perfect active.
            if words.get(i).map(String::as_str) == Some("been") {
                i += 1;
                let next = words.get(i)?;
                if let Some(lemma) = read_ing(next) {
                    return Some((lemma, tense, Aspect::PerfectProgressive, Voice::Active, neg, i + 1 - from));
                }
                let lemma = read_participle(next)?;
                return Some((lemma, tense, Aspect::Perfect, Voice::Passive, neg, i + 1 - from));
            }
            let lemma = read_participle(words.get(i)?)?;
            Some((lemma, tense, Aspect::Perfect, Voice::Active, neg, i + 1 - from))
        }
        Aux::Be => {
            // Progressive or passive family. `being` + participle is a
            // progressive passive; a bare `-ing` is a progressive active; a
            // bare participle is a simple passive. A word that is neither
            // abstains (it is a copula complement, handled by the caller).
            if words.get(i).map(String::as_str) == Some("being") {
                i += 1;
                let lemma = read_participle(words.get(i)?)?;
                return Some((lemma, tense, Aspect::Progressive, Voice::Passive, neg, i + 1 - from));
            }
            let next = words.get(i)?;
            if let Some(lemma) = read_ing(next) {
                return Some((lemma, tense, Aspect::Progressive, Voice::Active, neg, i + 1 - from));
            }
            let lemma = read_participle(next)?;
            Some((lemma, tense, Aspect::Simple, Voice::Passive, neg, i + 1 - from))
        }
    }
}

/// Read a surface modal word into its sense and tense, the inverse of
/// [`modal_word`]: `could` is `can` in the past, `would` is `will`, and so on.
/// `cannot` reads as `can` (its negation is handled by the caller). Returns
/// `None` for a non-modal word.
fn read_modal(word: &str) -> Option<(crate::ir::Modal, Tense)> {
    match aux_entry(word)? {
        Entry { tense, family: Family::Modal(modal), .. } => Some((modal, tense)),
        _ => None,
    }
}

/// Read the tenseless tail after a modal into its lemma, aspect, voice and the
/// number of words it spans - the inverse of [`modal_tail`]. After a modal the
/// auxiliaries are bare (`be`, `have`, not `is`/`has`), so this recognises
/// `have been`+-ing (perfect progressive), `have been`+participle (perfect
/// passive), `have`+participle (perfect), `be being`+participle (progressive
/// passive), `be`+-ing (progressive), `be`+participle (simple passive), and a
/// bare lemma (simple active). Abstains on anything else.
fn read_modal_tail(words: &[String], from: usize) -> Option<(String, Aspect, Voice, usize)> {
    let first = words.get(from)?;
    match first.as_str() {
        "have" => {
            let mut i = from + 1;
            if words.get(i).map(String::as_str) == Some("been") {
                i += 1;
                let next = words.get(i)?;
                if let Some(lemma) = read_ing(next) {
                    return Some((lemma, Aspect::PerfectProgressive, Voice::Active, i + 1 - from));
                }
                let lemma = read_participle(next)?;
                return Some((lemma, Aspect::Perfect, Voice::Passive, i + 1 - from));
            }
            let lemma = read_participle(words.get(i)?)?;
            Some((lemma, Aspect::Perfect, Voice::Active, i + 1 - from))
        }
        "be" => {
            let mut i = from + 1;
            if words.get(i).map(String::as_str) == Some("being") {
                i += 1;
                let lemma = read_participle(words.get(i)?)?;
                return Some((lemma, Aspect::Progressive, Voice::Passive, i + 1 - from));
            }
            let next = words.get(i)?;
            if let Some(lemma) = read_ing(next) {
                return Some((lemma, Aspect::Progressive, Voice::Active, i + 1 - from));
            }
            let lemma = read_participle(next)?;
            Some((lemma, Aspect::Simple, Voice::Passive, i + 1 - from))
        }
        // A bare lemma: simple active. The modal is the only auxiliary, so the
        // tail is a single word - the predicate as read, not an aspect chain.
        _ => {
            if is_aux(first) {
                return None; // an unexpected auxiliary after the modal: abstain
            }
            Some((first.clone(), Aspect::Simple, Voice::Active, 1))
        }
    }
}
/// The two auxiliary families the verb-group reader opens on.
enum Aux {
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
fn read_question(words: &[String]) -> Option<Act> {
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
fn lift_operator_negation(words: &[String], op_at: usize) -> Vec<String> {
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

/// Read a copula question: "what is your name?" (wh fills `as`), "where were
/// the keys?" (wh fills its preposition role). The subject is the noun phrase
/// after the copula.
fn read_copula_question(words: &[String], op_at: usize, wh: Option<Wh>) -> Option<Act> {
    let op = words.get(op_at)?;
    let tense = if aux_entry(op).is_some_and(|e| e.family == Family::Be && e.tense == Tense::Past) { Tense::Past } else { Tense::Present };
    let rest = &words[op_at + 1..];
    let (subject, end) = read_subject(rest)?;
    let mut tail = &rest[end..];
    let mut frame = Frame::new("be", tense).with(Role::Theme, subject);
    // "is she not here?" - `not` after the subject negates the copula.
    if wh.is_none() && tail.first().map(String::as_str) == Some("not") {
        frame = frame.negative();
        tail = &tail[1..];
    }
    match wh {
        Some(wh) => {
            if !tail.is_empty() {
                return None; // trailing words beyond the subject: abstain
            }
            frame = match wh {
                // "what is your name?" - the wh is the predicate complement.
                Wh::What | Wh::Who | Wh::Which => frame.with(Role::As, Filler::Hole(wh)),
                // "where were the keys?" - the wh is a place, stored under its prep.
                _ => frame.with(wh_role(wh), Filler::Hole(wh)),
            };
        }
        // A yes/no question: "was the weather good?", "is he at home?". What
        // follows the subject is the complement - a prepositional phrase, or a
        // word or noun phrase in the `as` role.
        None => {
            if tail.is_empty() {
                return None;
            }
            if is_preposition(&tail[0]) {
                let (preps, used) = read_prepositions(tail);
                if preps.is_empty() || used != tail.len() {
                    return None;
                }
                for (role, filler) in preps {
                    frame = frame.with(role, filler);
                }
            } else {
                frame = frame.with(Role::As, complement_filler(tail)?);
            }
        }
    }
    Some(Act::new(Force::Ask, frame))
}

/// Read an aspect/passive question: "has she arrived?", "what has happened?",
/// "what are you doing?". The inverted operator at `op_at` is lifted back onto
/// the words after the subject to reconstruct the verb group, which the shared
/// [`read_verb_group`] decomposes; the subject and the wh are then placed as
/// in a plain question. Returns `None` when the chain does not read as an
/// aspect (so a `be`-form copula question falls through to its own reader).
fn read_aspect_question(words: &[String], op_at: usize, wh: Option<Wh>) -> Option<Act> {
    let operator = words.get(op_at)?.clone();

    // A subject wh ("what has happened?") is itself the subject: the verb
    // group is the operator plus everything after it, with no interposed
    // subject, and the wh fills the subject role as a Hole.
    if matches!(wh, Some(Wh::What) | Some(Wh::Who) | Some(Wh::Which)) {
        let mut chain = vec![operator.clone()];
        chain.extend_from_slice(&words[op_at + 1..]);
        if let Some((lemma, tense, aspect, voice, neg, consumed)) = read_verb_group(&chain, 0) {
            // Only a clean, whole verb group (no leftover object) is a subject
            // wh; anything trailing means there is a real subject instead.
            if consumed == chain.len() {
                let mut frame = Frame::new(&lemma, tense).aspect(aspect);
                if voice == Voice::Passive {
                    frame = frame.passive();
                }
                if neg {
                    frame = frame.negative();
                }
                // A subject wh of an aspect clause fills the theme in the gold
                // ("what has happened?" -> theme=?what).
                frame = frame.with(Role::Theme, Filler::Hole(wh?));
                return Some(Act::new(Force::Ask, frame));
            }
        }
    }

    let (subject, after) = read_subject(&words[op_at + 1..])?;
    let subject_end = op_at + 1 + after;

    // Reconstruct the contiguous verb group: the operator, then the words
    // after the subject (been/participle/-ing), and read it as one chain.
    let mut chain = vec![operator];
    chain.extend_from_slice(&words[subject_end..]);
    let (lemma, tense, aspect, voice, neg, consumed) = read_verb_group(&chain, 0)?;

    let mut frame = Frame::new(&lemma, tense).aspect(aspect);
    if voice == Voice::Passive {
        frame = frame.passive();
    }
    if neg {
        frame = frame.negative();
    }
    // The subject is the agent in the active (the theme in the passive, and
    // when the wh is the agent); the wh is placed into its role afterwards.
    let subject_is_agent = wh != Some(Wh::Who) && voice != Voice::Passive;
    let subject_role = if subject_is_agent { Role::Agent } else { Role::Theme };
    if !is_expletive_subject(&lemma, &subject) {
        frame = frame.with(subject_role, subject);
    }

    // The words the verb group consumed beyond the operator sit after the
    // subject; the remainder is objects/prepositions/by-agent.
    let rest_start = subject_end + (consumed - 1);
    frame = match finish_predicate(frame, words, rest_start, Force::Ask) {
        Some(act) => act.frame,
        None => return None,
    };

    if let Some(wh) = wh {
        frame = place_wh(frame, wh, subject_is_agent);
    }
    Some(Act::new(Force::Ask, frame))
}

/// Place a wh as a Hole in the role the Guide stores for it: who->agent (or
/// theme when the subject already took the agent), what/which->theme,
/// where/when/why/how->their preposition role.
fn place_wh(frame: Frame, wh: Wh, subject_is_agent: bool) -> Frame {
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
fn wh_role(wh: Wh) -> Role {
    match wh {
        Wh::Where => Role::named("in"),
        Wh::When => Role::named("at"),
        Wh::Why => Role::named("for"),
        Wh::How => Role::named("by"),
        _ => Role::named("by"),
    }
}

/// Read a question operator into `(tense, modal)`: do-support drops and
/// carries tense; a modal keeps its sense and the tense its past form shows.
/// Returns `None` for an aspect/passive operator this reader does not build.
fn read_operator(word: &str) -> Option<(Tense, Option<crate::ir::Modal>)> {
    match aux_entry(word)? {
        Entry { tense, family: Family::Do, operator: true } => Some((tense, None)),
        Entry { tense, family: Family::Modal(modal), operator: true } => Some((tense, Some(modal))),
        _ => None,
    }
}
/// Read an imperative, or `None` if the sentence does not open as a command.
/// The shape is `[do not] VERB [recipient] [object] [prepositions]`, with the
/// subject suppressed - the Direct force the gold records. The reader only
/// treats a leading word as a command verb when it is in the known set, so a
/// subject noun is never mistaken for an instruction.
fn read_imperative(words: &[String]) -> Option<Act> {
    let mut i = 0;
    let mut neg = false;
    if words.get(0).map(String::as_str) == Some("do") && words.get(1).map(String::as_str) == Some("not") {
        neg = true;
        i = 2;
    }
    let verb = words.get(i)?;
    if !lists::imperative_verbs().contains(&verb.as_str()) {
        return None;
    }
    let mut frame = Frame::new(verb, Tense::Present);
    if neg {
        frame = frame.negative();
    }
    let rest = &words[i + 1..];
    // A ditransitive command names the recipient first ("tell me the truth",
    // "give her the letter"): a leading object pronoun is the recipient, and
    // the next noun phrase is the theme.
    let mut cursor = 0;
    if leading_recipient(verb, rest) {
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
        return None; // trailing words unaccounted for: abstain, not a guess
    }
    Some(Act::new(Force::Direct, frame))
}

/// Whether a leading object pronoun is a ditransitive recipient rather than
/// the sole object. It is a recipient when another object follows ("give her
/// the letter") or when the verb inherently takes a recipient ("tell you"),
/// and the theme when it stands alone after a plain transitive verb ("do it").
/// The valency is the verb's own, which the Guide teaches per verb.
fn leading_recipient(verb: &str, rest: &[String]) -> bool {
    if rest.first().map(|w| is_object_pronoun(w)) != Some(true) {
        return false;
    }
    if lists::ditransitive_verbs().contains(&verb) {
        return true;
    }
    match rest.get(1) {
        None => false,
        Some(w) => !is_preposition(w),
    }
}

/// The object-form pronouns that read as a recipient at the head of a
/// ditransitive command.
fn is_object_pronoun(word: &str) -> bool {
    lists::object_pronouns().contains(word)
}
/// Read trailing prepositional phrases into `Via` slots: "in a bank", "on
/// time", "at eight". Each is a known preposition followed by a noun phrase
/// the reader can segment; it stops (returning what it has) at the first word
/// that is not a clean preposition+phrase, so it never mis-attaches.
fn read_prepositions(words: &[String]) -> (Vec<(Role, Filler)>, usize) {
    let mut out = Vec::new();
    let mut i = 0;
    while i < words.len() {
        if !is_preposition(&words[i]) {
            break;
        }
        let prep = words[i].clone();
        let rest = &words[i + 1..];
        match read_noun_phrase(rest) {
            Some((thing, end)) => {
                out.push((Role::named(&prep), thing));
                i += 1 + end;
            }
            None => break,
        }
    }
    (out, i)
}

/// The closed set of prepositions the reader recognises at the head of an
/// adjunct, from the gold frames.
fn is_preposition(word: &str) -> bool {
    lists::prepositions().contains(word)
}
// ----- The round-trip measure -----

/// The realisation round-trip over a set of `(sentence, act)` pairs: the
/// share whose realised Act equals the sentence under `normalise`, with the
/// mark kept. Returns `(matched, total, first failures up to 20)`.
pub fn round_trip<'a>(pairs: impl IntoIterator<Item = (&'a Act, &'a str)>) -> (usize, usize, Vec<String>) {
    let (mut matched, mut total) = (0usize, 0usize);
    let mut failures = Vec::new();
    for (act, sentence) in pairs {
        total += 1;
        let want = normalise(sentence);
        if realise(act) == want {
            matched += 1;
        } else if failures.len() < 20 {
            failures.push(format!("{want} != {}", realise(act)));
        }
    }
    (matched, total, failures)
}

/// The canonical realisation round-trip: as [`round_trip`], but both sides go
/// through [`canonical`] first (contractions expanded, "going to" folded to
/// "will", set-aside adverbs dropped). The strict [`round_trip`] stays the
/// declared measure; this one is reported beside it. Returns
/// `(matched, total, first failures up to 20)`.
pub fn round_trip_canonical<'a>(
    pairs: impl IntoIterator<Item = (&'a Act, &'a str)>,
) -> (usize, usize, Vec<String>) {
    let (mut matched, mut total) = (0usize, 0usize);
    let mut failures = Vec::new();
    for (act, sentence) in pairs {
        total += 1;
        let want = canonical(&normalise(sentence));
        let got = canonical(&realise(act));
        if got == want {
            matched += 1;
        } else if failures.len() < 20 {
            failures.push(format!("{want} != {got}"));
        }
    }
    (matched, total, failures)
}

/// Force purity over `(sentence, gold act)` pairs: the share whose read Force
/// equals the gold Force, among the sentences the reader covers. Returns
/// `(matched, covered)`. A sentence the reader abstains on is not counted -
/// force purity is a statement about what it reads, not what it skips.
pub fn force_purity<'a>(pairs: impl IntoIterator<Item = (&'a Act, &'a str)>) -> (usize, usize) {
    let (mut matched, mut covered) = (0usize, 0usize);
    for (gold, sentence) in pairs {
        let Some(read_act) = read(sentence) else { continue };
        covered += 1;
        if read_act.force == gold.force {
            matched += 1;
        }
    }
    (matched, covered)
}

/// Fold "am/is/are/was/were going to" down to "will", so the near-future reads
/// the same whether it was written "going to" or "will". Applied on both sides
/// of the round-trip comparison, never to the Act itself.
fn fold_going_to(words: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < words.len() {
        if aux_is(&words[i], Family::Be)
            && words.get(i + 1).map(String::as_str) == Some("going")
            && words.get(i + 2).map(String::as_str) == Some("to")
        {
            out.push("will".into());
            i += 3;
        } else {
            out.push(words[i].clone());
            i += 1;
        }
    }
    out
}

/// A sentence in contraction-canonical form: normalised, then every known
/// contraction expanded to the words it stands for. Used only to compare a
/// realised sentence with its original, so "i'm" and "i am" count the same.
fn canonical(s: &str) -> String {
    // The terminating mark rides on the last word; lift it off so a set-aside
    // adverb that ends the sentence ("... very often.") is still recognised,
    // then put it back.
    let mark: String = s.chars().last().filter(|c| MARKS.contains(c)).map(String::from).unwrap_or_default();
    let bare = s.trim_end_matches(|c| MARKS.contains(&c));
    let words: Vec<String> = bare.split_whitespace().map(str::to_string).collect();
    let words = fold_going_to(&words);
    let (kept, _) = strip_adverbs_keeping(&expand_contractions(&words), true);
    format!("{}{}", kept.join(" "), mark)
}

/// The read-and-realise round-trip: a sentence read to an Act and realised
/// back equals the original, over the sentences this reader covers. A
/// sentence it cannot read is counted as uncovered, never as a pass.
pub fn read_realise_rate<'a>(sentences: impl IntoIterator<Item = &'a str>) -> (usize, usize, usize) {
    let (mut matched, mut covered, mut total) = (0usize, 0usize, 0usize);
    for sentence in sentences {
        total += 1;
        let Some(act) = read(sentence) else { continue };
        covered += 1;
        if canonical(&realise(&act)) == canonical(&normalise(sentence))
            && names::kept(&act.frame, &realise_cased(&act), sentence)
        {
            matched += 1;
        }
    }
    (matched, covered, total)
}
