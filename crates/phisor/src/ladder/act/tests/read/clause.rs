//! Reading complement, coordinated and subordinate clauses.

use super::*;

#[test]
fn a_finite_clause_is_not_mistaken_for_an_infinitive() {
    use crate::ir::{Filler, Force, Frame, Modal, Role, Tense};
    // "i wish [i could sing]": the inner clause has its own modal and tense, so
    // it is finite and must not be said as "to sing".
    let inner = Frame::new("sing", Tense::Past).modal(Modal::Can).with(Role::Agent, Filler::pronoun("I"));
    let matrix = Frame::new("wish", Tense::Present)
        .with(Role::Agent, Filler::pronoun("I"))
        .with(Role::Theme, Filler::clause(inner));
    let said = realise(&crate::ir::Act::new(Force::Wish, matrix));
    assert!(!said.contains(" to sing"), "a finite clause took a to: {said}");
}


#[test]
fn a_complement_clause_becomes_the_theme_and_round_trips() {
    use crate::ir::{Filler, Role};
    let act = read("i think she works in a shop.").expect("a complement clause is covered");
    assert_eq!(act.frame.pred, "think");
    let Some(Filler::Clause(inner)) = act.frame.get(&Role::Theme) else { panic!("theme is not a clause") };
    assert_eq!(inner.pred, "work");
    assert_eq!(realise(&act), "i think she works in a shop.");
    // A same-subject clause under a non-control verb stays finite.
    let act = read("i think i am late.").expect("a same-subject clause is covered");
    assert_eq!(realise(&act), "i think i am late.");
}

#[test]
fn a_negated_matrix_verb_keeps_its_clause() {
    // "don't" expands to "do not"; the round trip compares at the canonical level.
    let (matched, covered, _) = read_realise_rate(["i don't think she works in a shop."]);
    assert_eq!((matched, covered), (1, 1));
}

#[test]
fn a_plain_object_is_not_taken_for_a_clause() {
    use crate::ir::{Filler, Role};
    // "the answer" has no verb, so "know" keeps it as an ordinary theme.
    let act = read("i know the answer.").expect("a plain object is covered");
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Thing(_))));
    // "you in your car" is an object and a phrase, not a clause.
    let act = read("i saw you in your car.").expect("object and phrase are covered");
    assert!(!matches!(act.frame.get(&Role::Theme), Some(Filler::Clause(_))));
}

#[test]
fn to_is_an_infinitive_only_after_a_control_verb() {
    use crate::ir::{Filler, Role};
    // "want" controls an infinitive; "came to help" is a purpose phrase and is
    // not read as one.
    let act = read("i want to go.").expect("a control infinitive is covered");
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Clause(_))));
    let came = read("i came to help.");
    assert!(came.map_or(true, |a| !matches!(a.frame.get(&Role::Theme), Some(Filler::Clause(_)))));
}

#[test]
fn two_clauses_joined_by_a_conjunction_read_and_round_trip() {
    use crate::ir::{Filler, Role};
    let act = read("he works in a shop and she stays at home.").expect("coordinated clauses are covered");
    assert!(matches!(act.frame.get(&Role::named("and")), Some(Filler::Clause(_))));
    assert_eq!(realise(&act), "he works in a shop and she stays at home.");
    let act = read("i was tired so i went to bed.").expect("a past coordination is covered");
    assert_eq!(realise(&act), "i was tired so i went to bed.");
}

#[test]
fn a_subordinate_clause_is_an_adjunct_under_its_conjunction() {
    use crate::ir::{Filler, Role};
    let act = read("i will call you if i have time.").expect("a conditional is covered");
    assert!(matches!(act.frame.get(&Role::named("if")), Some(Filler::Clause(_))));
    assert_eq!(realise(&act), "i will call you if i have time.");
    // Contractions expand inside the clause, and the round trip compares canonically.
    let (matched, covered, _) = read_realise_rate(["i'll call you when i've finished the work."]);
    assert_eq!((matched, covered), (1, 1));
}

#[test]
fn a_conjunction_between_nouns_is_not_a_clause_split() {
    use crate::ir::Role;
    // "karen" is not a clause, so there is nothing to split at "and".
    let act = read("karen and joe played tennis.");
    assert!(act.map_or(true, |a| a.frame.get(&Role::named("and")).is_none()));
    // A noun pair as an object stays an object.
    let act = read("i like tea and coffee.");
    assert!(act.map_or(true, |a| a.frame.get(&Role::named("and")).is_none()));
}

#[test]
fn a_sentence_that_reads_whole_is_not_split() {
    use crate::ir::Role;
    let act = read("i know the answer.").expect("a plain sentence is covered");
    assert!(act.frame.get(&Role::named("and")).is_none());
    assert!(act.frame.get(&Role::named("so")).is_none());
}

#[test]
fn three_clauses_read_whole_and_round_trip() {
    use crate::ir::{Filler, Role};
    // read_compound loops past the first conjunction, so every trailing clause
    // is attached under its own conjunction and the sentence reads whole.
    let act =
        read("he works in a shop and she stays at home and they are happy.").expect("three clauses are covered");
    assert!(matches!(act.frame.get(&Role::named("and")), Some(Filler::Clause(_))));
    assert_eq!(realise(&act), "he works in a shop and she stays at home and they are happy.");
    // Mixed coordinators round-trip in order, including a subordinate tail.
    let act = read("i was tired so i went to bed and i slept well.").expect("a three-clause chain is covered");
    assert_eq!(realise(&act), "i was tired so i went to bed and i slept well.");
}

#[test]
fn a_trailing_clause_that_does_not_read_leaves_no_partial_frame() {
    // The last span ("and") is not a clause, so the whole sentence is
    // uncovered rather than read as the first two clauses only.
    assert!(read("he works in a shop and she stays at home and.").is_none());
}
