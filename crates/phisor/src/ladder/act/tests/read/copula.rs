//! Reading the copula, its complements and existentials.

use super::*;

#[test]
fn a_copula_now_reads_into_a_be_frame() {
    // The extended reader handles the copula shape `X is Y`.
    let act = read("her name is helen.").expect("a copula is now covered");
    assert_eq!(act.frame.pred, "be");
    assert_eq!(act.force, crate::ir::Force::Assert);
}

#[test]
fn a_participle_copula_reads_as_aspect_not_a_complement() {
    use crate::ir::{Aspect, Voice};
    // `is boiling` is a progressive, not a copula+complement: the reader now
    // reads the aspect rather than treating `boiling` as a predicate.
    let prog = read("the water is boiling.").expect("progressive is covered");
    assert_eq!(prog.frame.pred, "boil");
    assert_eq!(prog.frame.aspect, Aspect::Progressive);
    assert_eq!(prog.frame.voice, Voice::Active);
    // `is repaired` is a simple passive, the theme promoted to subject.
    let pass = read("the car is repaired.").expect("passive is covered");
    assert_eq!(pass.frame.pred, "repair");
    assert_eq!(pass.frame.voice, Voice::Passive);
}

#[test]
fn a_past_copula_and_a_past_matrix_read() {
    let act = read("i knew sarah was busy.").expect("a past clause is covered");
    assert_eq!(act.frame.pred, "know");
    assert_eq!(realise(&act), "i knew sarah was busy.");
    let act = read("they were students.").expect("a past copula is covered");
    assert_eq!(act.frame.pred, "be");
    assert_eq!(realise(&act), "they were students.");
}

#[test]
fn an_existential_reads_and_agrees_on_the_way_back() {
    use crate::ir::{Filler, Role};
    let act = read("there is a problem.").expect("an existential is covered");
    assert_eq!(act.frame.pred, "be");
    assert!(act.frame.get(&Role::Agent).is_none());
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Thing(_))));
    assert_eq!(realise(&act), "there is a problem.");
    // Plural theme: the copula agrees.
    let act = read("there are some tables.").expect("a plural existential is covered");
    assert_eq!(realise(&act), "there are some tables.");
    // Past and negative.
    let act = read("there was no milk.").expect("a negative past existential is covered");
    assert_eq!(realise(&act), "there was no milk.");
}

#[test]
fn there_as_a_place_is_not_mistaken_for_an_existential() {
    // "there" without a following copula is the ordinary adverb, not the dummy;
    // a sentence that does not fit the existential shape is read normally or
    // abstained, never forced into a be-frame.
    let act = read("i live there.");
    assert!(act.map_or(true, |a| a.frame.pred != "be"));
}

#[test]
fn a_copula_takes_an_adjective_prepositional_or_modified_complement() {
    use crate::ir::{Filler, Role};
    // A prepositional complement reads under its preposition.
    let act = read("rachel is from australia.").expect("a prepositional complement is covered");
    assert_eq!(act.frame.pred, "be");
    assert_eq!(realise(&act), "rachel is from australia.");
    // An adjective before the head is kept as a Mod and round-trips.
    let act = read("it was a hot day.").expect("an adjective-modified complement is covered");
    let Some(Filler::Thing(t)) = act.frame.get(&Role::As) else { panic!("as is not a thing") };
    assert_eq!(t.head, "day");
    assert_eq!(realise(&act), "it was a hot day.");
    // "both" is a quantifier determiner on the subject.
    let act = read("both restaurants are good.").expect("a both-subject is covered");
    assert_eq!(realise(&act), "both restaurants are good.");
}
