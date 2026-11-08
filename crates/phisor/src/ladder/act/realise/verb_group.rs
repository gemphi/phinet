//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;
use crate::lexicon::agreement::{aux_form, Number, Person};
use crate::lexicon::auxiliary::Family;

/// The verb group for a frame: the auxiliaries its aspect, voice, modality
/// and polarity imply, then the predicate in the form the aspect needs. This
/// is deliberately a small, honest English generator over the closed verb
/// stratum, not a full morphology.
pub(in crate::ladder::act) fn verb_group(frame: &Frame) -> Vec<String> {
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
pub(in crate::ladder::act) fn negation_word(frame: &Frame) -> &'static str {
    if frame.never { "never" } else { "not" }
}

/// Prepend the auxiliaries, insert `not` after the first when negative, then
/// the main form: `has not finished`, `is not working`.
pub(in crate::ladder::act) fn with_not(mut aux: Vec<String>, neg: Option<&str>, main: String) -> Vec<String> {
    if let Some(word) = neg {
        aux.insert(1.min(aux.len()), word.into());
    }
    aux.push(main);
    aux
}

/// A modal's surface form, in the past where the frame is past: `can` ->
/// `could`, `will` -> `would`, `may` -> `might`, `shall` -> `should`. The
/// book's Unit 19-25 modal table.
pub(in crate::ladder::act) fn modal_word(modal: crate::ir::Modal, tense: Tense) -> String {
    use crate::ir::Modal;
    match (modal, tense) {
        (Modal::Can, Tense::Past) => "could".into(),
        (Modal::Will, Tense::Past) => "would".into(),
        (Modal::May, Tense::Past) => "might".into(),
        (Modal::Shall, Tense::Past) => "should".into(),
        _ => modal.name().to_string(),
    }
}

/// The person and number the Realiser's `third`/`first` flags stand for;
/// anything else is the plural/second-person cell, which agrees alike.
fn agreement(third: bool, first: bool) -> (Person, Number) {
    match (third, first) {
        (true, _) => (Person::Third, Number::Singular),
        (_, true) => (Person::First, Number::Singular),
        _ => (Person::Second, Number::Plural),
    }
}
/// One auxiliary's surface form, read from the agreement data.
fn aux(family: Family, tense: Tense, third: bool, first: bool) -> String {
    let (person, number) = agreement(third, first);
    aux_form(family, tense, person, number).expect("auxiliary.tsv covers every cell").to_string()
}
/// The copula `be` surface form for a tense and number, from the data.
pub(in crate::ladder::act) fn be_form(tense: Tense, third: bool, first: bool) -> String {
    aux(Family::Be, tense, third, first)
}
/// The `have` surface form, from the data.
pub(in crate::ladder::act) fn have_form(tense: Tense, third: bool) -> String {
    aux(Family::Have, tense, third, false)
}
/// The `do` surface form (do-support), from the data.
pub(in crate::ladder::act) fn do_form(tense: Tense, third: bool) -> String {
    aux(Family::Do, tense, third, false)
}/// The verb forms after a modal, which are all tenseless (bare `be`, bare
/// `have`), composed from the aspect and voice: the book's modal + perfect /
/// progressive / passive combinations.
pub(in crate::ladder::act) fn modal_tail(frame: &Frame) -> Vec<String> {
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
pub(in crate::ladder::act) fn simple_active(frame: &Frame, third: bool, neg: bool) -> Vec<String> {
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
        out.push(do_form(frame.tense, third));
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

