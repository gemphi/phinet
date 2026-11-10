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

#[test]
fn a_wh_question_on_a_control_verb_puts_the_hole_in_the_infinitive() {
    use crate::ir::{Filler, Role, Wh};
    // "where does sarah want to go?" asks about the going, not the wanting:
    // the infinitive is the theme clause and the hole sits inside it.
    let act = read("where does sarah want to go?").expect("read");
    assert_eq!(act.frame.pred, "want");
    let Some(Filler::Clause(inner)) = act.frame.get(&Role::Theme) else { panic!("the infinitive is the theme: {act:?}") };
    assert_eq!(inner.pred, "go");
    assert!(inner.slots.iter().any(|(_, f)| matches!(f, Filler::Hole(Wh::Where))), "the hole is in the clause: {inner:?}");
    assert!(!act.frame.slots.iter().any(|(_, f)| matches!(f, Filler::Hole(_))), "and not on want: {act:?}");
    // The yes/no shape is unchanged.
    let yes_no = read("does sarah want to go home?").expect("read");
    assert!(matches!(yes_no.frame.get(&Role::Theme), Some(Filler::Clause(c)) if c.pred == "go"));
}

#[test]
fn a_subject_wh_question_reads_in_statement_order_with_the_hole_as_subject() {
    // "who lives in london?" has no operator to invert: the wh is the
    // subject, the rest a statement's predicate, and it says itself back.
    let act = read("who lives in london?").expect("read");
    assert_eq!(act.force, crate::ir::Force::Ask);
    assert_eq!(act.frame.pred, "live");
    assert_eq!(act.frame.get(&Role::Agent), Some(&Filler::Hole(Wh::Who)));
    assert_eq!(realise(&act), "who lives in london?");
    // With an auxiliary group and an object left over, which the aspect
    // reader's whole-chain subject wh does not take.
    let act = read("who was reading a book?").expect("read");
    assert_eq!(act.frame.pred, "read");
    assert_eq!(act.frame.aspect, crate::ir::Aspect::Progressive);
    assert_eq!(act.frame.get(&Role::Agent), Some(&Filler::Hole(Wh::Who)));
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Thing(t)) if t.head == "book"), "{act:?}");
    assert_eq!(realise(&act), "who was reading a book?");
    // A where is never a subject, and a statement is untouched.
    assert!(read("where in london?").is_none());
}
