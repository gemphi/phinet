//! Reading questions: do-support, wh holes, operators.

use super::*;

#[test]
fn a_do_support_question_inverts_and_drops_do() {
    // "do you know the answer?" reads to a present ASK with the operator
    // dropped and the object in place.
    let act = read("do you know the answer?").expect("a yes/no question is covered");
    assert_eq!(act.force, crate::ir::Force::Ask);
    assert_eq!(act.frame.pred, "know");
    assert_eq!(act.frame.modal, None);
    assert_eq!(realise(&act), "do you know the answer?");
}

#[test]
fn a_wh_question_places_the_hole_in_its_role() {
    // "where do you live?" places the wh as a Hole under its preposition role
    // and reads the subject and tense.
    let act = read("where do you live?").expect("a wh-question is covered");
    assert_eq!(act.force, crate::ir::Force::Ask);
    assert_eq!(act.frame.pred, "live");
    assert_eq!(realise(&act), "where do you live?");
}

#[test]
fn a_modal_question_keeps_the_modal() {
    // "can you swim?" keeps the modal and inverts.
    let act = read("can you swim?").expect("a modal question is covered");
    assert_eq!(act.frame.modal, Some(crate::ir::Modal::Can));
    assert_eq!(realise(&act), "can you swim?");
}

#[test]
fn a_perfect_yes_no_question_reads() {
    use crate::ir::{Aspect, Force, Role};
    // "has she arrived?" - present perfect, theme she (unaccusative in gold),
    // Ask force. The verb group is read across the subject.
    let act = read("has she arrived?").expect("perfect question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "arrive");
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Agent), Some(&crate::ir::Filler::pronoun("she")));
}

#[test]
fn a_perfect_wh_question_places_the_hole() {
    use crate::ir::{Aspect, Force, Role, Wh};
    // "what has happened?" - the wh is the theme subject of an agentless
    // perfect; it is placed as a Hole and not duplicated.
    let act = read("what has happened?").expect("perfect wh-question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "happen");
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Theme), Some(&crate::ir::Filler::Hole(Wh::What)));
}

#[test]
fn a_progressive_wh_question_reads() {
    use crate::ir::{Aspect, Force};
    // "what are you doing?" - progressive active, wh theme.
    let act = read("what are you doing?").expect("progressive wh-question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "do");
    assert_eq!(act.frame.aspect, Aspect::Progressive);
}

#[test]
fn a_yes_no_copula_question_reads_and_round_trips() {
    use crate::ir::{Force, Role};
    let act = read("was the weather good?").expect("a yes/no copula question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "be");
    assert!(act.frame.get(&Role::As).is_some());
    assert_eq!(realise(&act), "was the weather good?");
    // A prepositional complement reads under its preposition.
    let act = read("is he at home?").expect("a located copula question is covered");
    assert_eq!(realise(&act), "is he at home?");
}

#[test]
fn a_going_to_question_reads() {
    use crate::ir::{Force, Modal};
    let act = read("are you going to eat anything?").expect("a going-to question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "eat");
    assert_eq!(act.frame.modal, Some(Modal::Will));
}
