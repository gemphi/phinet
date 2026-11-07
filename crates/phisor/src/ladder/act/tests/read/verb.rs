//! Reading the verb group: aspect, voice and modals.

use super::*;

#[test]
fn read_verb_group_reads_active_aspects() {
    use crate::ir::{Aspect, Tense, Voice};
    let w = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();

    // Perfect active: "has arrived".
    let (lemma, tense, aspect, voice, neg, consumed) =
        read_verb_group(&w("has arrived"), 0).expect("perfect active reads");
    assert_eq!((lemma.as_str(), tense, aspect, voice, neg, consumed),
               ("arrive", Tense::Present, Aspect::Perfect, Voice::Active, false, 2));

    // Perfect progressive: "had been working".
    let (lemma, tense, aspect, voice, neg, consumed) =
        read_verb_group(&w("had been working"), 0).expect("perfect progressive reads");
    assert_eq!((lemma.as_str(), tense, aspect, voice, neg, consumed),
               ("work", Tense::Past, Aspect::PerfectProgressive, Voice::Active, false, 3));

    // Negated perfect: "have not finished".
    let (lemma, tense, aspect, voice, neg, consumed) =
        read_verb_group(&w("have not finished"), 0).expect("negated perfect reads");
    assert_eq!((lemma.as_str(), tense, aspect, voice, neg, consumed),
               ("finish", Tense::Present, Aspect::Perfect, Voice::Active, true, 3));

    // Progressive active: "is boiling".
    let (lemma, _t, aspect, voice, _n, _c) =
        read_verb_group(&w("is boiling"), 0).expect("progressive active reads");
    assert_eq!((lemma.as_str(), aspect, voice), ("boil", Aspect::Progressive, Voice::Active));

    // A modal head is not this reader's job.
    assert!(read_verb_group(&w("will come"), 0).is_none());
}

#[test]
fn a_perfect_active_reads_to_the_gold_act_and_round_trips() {
    use crate::ir::{Aspect, Role, Tense};
    // "i have lost my keys." - present perfect, agent I, theme my keys.
    let act = read("i have lost my keys.").expect("perfect active is covered");
    assert_eq!(act.frame.pred, "lose");
    assert_eq!(act.frame.tense, Tense::Present);
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Agent), Some(&crate::ir::Filler::pronoun("I")));
    assert_eq!(realise(&act), "i have lost my keys.");
}

#[test]
fn a_perfect_progressive_reads_to_the_gold_act_and_round_trips() {
    use crate::ir::Aspect;
    // "the children have been reading my books." - present perfect progressive.
    let act = read("the children have been reading my books.").expect("perfect progressive is covered");
    assert_eq!(act.frame.pred, "read");
    assert_eq!(act.frame.aspect, Aspect::PerfectProgressive);
    assert_eq!(realise(&act), "the children have been reading my books.");
}

#[test]
fn a_simple_passive_promotes_the_theme_to_subject() {
    use crate::ir::{Aspect, Role, Tense, Voice};
    // "the window was broken." - simple past passive, theme the window.
    let act = read("the window was broken.").expect("simple passive is covered");
    assert_eq!(act.frame.pred, "break");
    assert_eq!(act.frame.tense, Tense::Past);
    assert_eq!(act.frame.aspect, Aspect::Simple);
    assert_eq!(act.frame.voice, Voice::Passive);
    assert!(matches!(act.frame.get(&Role::Theme), Some(crate::ir::Filler::Thing(t)) if t.head == "window"));
    assert_eq!(realise(&act), "the window was broken.");
}

#[test]
fn a_perfect_passive_reads_and_round_trips() {
    use crate::ir::Voice;
    // "the house has been sold." - present perfect passive.
    let act = read("the house has been sold.").expect("perfect passive is covered");
    assert_eq!(act.frame.pred, "sell");
    assert_eq!(act.frame.voice, Voice::Passive);
    assert_eq!(realise(&act), "the house has been sold.");
}

#[test]
fn a_passive_by_agent_reads_the_demoted_agent() {
    use crate::ir::Role;
    // "my car has been repaired by a mechanic." - the by-phrase is the agent.
    let act = read("my car has been repaired by a mechanic.").expect("by-agent passive is covered");
    assert_eq!(act.frame.pred, "repair");
    assert!(matches!(act.frame.get(&Role::Agent), Some(crate::ir::Filler::Thing(t)) if t.head == "mechanic"));
    assert!(matches!(act.frame.get(&Role::Theme), Some(crate::ir::Filler::Thing(t)) if t.head == "car"));
    assert_eq!(realise(&act), "my car has been repaired by a mechanic.");
}


#[test]
fn a_simple_modal_reads_with_its_sense_and_tense() {
    use crate::ir::{Aspect, Modal, Role, Tense};
    // "i can swim." - present, modal can, simple active.
    let act = read("i can swim.").expect("simple modal is covered");
    assert_eq!(act.frame.pred, "swim");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.tense, Tense::Present);
    assert_eq!(act.frame.aspect, Aspect::Simple);
    assert_eq!(act.frame.get(&Role::Agent), Some(&crate::ir::Filler::pronoun("I")));
    assert_eq!(realise(&act), "i can swim.");
}

#[test]
fn a_past_modal_negative_reads() {
    use crate::ir::{Modal, Tense};
    // "he could not sing." - can in the past, negative.
    let act = read("he could not sing.").expect("past modal negative is covered");
    assert_eq!(act.frame.pred, "sing");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.tense, Tense::Past);
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "he could not sing.");
}

#[test]
fn a_modal_passive_reads() {
    use crate::ir::{Modal, Voice};
    // "the work will be finished tomorrow." - will + be + participle, passive.
    let act = read("the work will be finished tomorrow.").expect("modal passive is covered");
    assert_eq!(act.frame.pred, "finish");
    assert_eq!(act.frame.modal, Some(Modal::Will));
    assert_eq!(act.frame.voice, Voice::Passive);
    assert_eq!(realise(&act), "the work will be finished tomorrow.");
}

#[test]
fn a_cannot_passive_reads_the_negation() {
    use crate::ir::{Modal, Voice};
    // "this door cannot be opened." - cannot carries can + negative.
    let act = read("this door cannot be opened.").expect("cannot passive is covered");
    assert_eq!(act.frame.pred, "open");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.voice, Voice::Passive);
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "this door cannot be opened.");
}

#[test]
fn a_modal_perfect_reads() {
    use crate::ir::{Aspect, Modal, Role, Tense};
    // "i could have told you." - can/past + have + participle, perfect.
    let act = read("i could have told you.").expect("modal perfect is covered");
    assert_eq!(act.frame.pred, "tell");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.tense, Tense::Past);
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Recipient), Some(&crate::ir::Filler::pronoun("you")));
    assert_eq!(realise(&act), "i could have told you.");
}

#[test]
fn a_modal_progressive_negative_reads() {
    use crate::ir::{Aspect, Modal};
    // "he will not be coming." - will + be + -ing, progressive, negative.
    let act = read("he will not be coming.").expect("modal progressive is covered");
    assert_eq!(act.frame.pred, "come");
    assert_eq!(act.frame.modal, Some(Modal::Will));
    assert_eq!(act.frame.aspect, Aspect::Progressive);
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "he will not be coming.");
}


#[test]
fn be_going_to_reads_as_the_near_future() {
    use crate::ir::Modal;
    // The Guide records "be going to V" as a will-modal over V.
    let act = read("i am going to study french.").expect("be going to is covered");
    assert_eq!(act.frame.pred, "study");
    assert_eq!(act.frame.modal, Some(Modal::Will));
    // "going to" and "will" are the same under the round-trip comparison.
    let (matched, covered, _) = read_realise_rate(["i am going to study french."]);
    assert_eq!((matched, covered), (1, 1));
}
