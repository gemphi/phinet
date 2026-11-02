//! The act height, measured honestly on the frozen gold set.

use super::*;
use crate::gold;

#[test]
fn a_simple_sentence_reads_and_realises_back() {
    let act = read("they play tennis.").expect("a simple declarative is covered");
    assert_eq!(act.force, crate::ir::Force::Assert);
    assert_eq!(act.frame.pred, "play");
    assert_eq!(realise(&act), "they play tennis.");
}

#[test]
fn negation_is_read_and_realised() {
    let act = read("he does not smoke.").expect("negation is covered");
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "he does not smoke.");
}

#[test]
fn a_copula_now_reads_into_a_be_frame() {
    // The extended reader handles the copula shape `X is Y`.
    let act = read("her name is helen.").expect("a copula is now covered");
    assert_eq!(act.frame.pred, "be");
    assert_eq!(act.force, crate::ir::Force::Assert);
}

#[test]
fn a_participle_copula_stays_uncovered_not_a_guess() {
    // `is boiling` is a progressive, not a copula+complement: the reader
    // abstains rather than read `boiling` as a predicate complement.
    assert!(read("the water is boiling.").is_none());
    // `is repaired` is a passive: likewise abstained.
    assert!(read("the car is repaired.").is_none());
}

#[test]
fn the_gold_round_trip_is_measured_and_reported() {
    let gold = gold::read().expect("the frozen gold set loads");
    let (matched, total, _failures) = round_trip(gold.pairs());
    assert_eq!(total, gold::COUNT);
    let rate = 100.0 * matched as f64 / total as f64;
    println!("gold realisation round-trip: {matched}/{total} = {rate:.1}%");
    assert!(matched > 0, "the realiser round-trips no gold frame at all");
}

#[test]
fn the_read_realise_rate_is_measured_over_the_gold_sentences() {
    let gold = gold::read().expect("the frozen gold set loads");
    let sentences: Vec<&str> = gold.entries.iter().map(|e| e.sentence.as_str()).collect();
    let (matched, covered, total) = read_realise_rate(sentences);
    let cov = 100.0 * covered as f64 / total as f64;
    let acc = if covered > 0 { 100.0 * matched as f64 / covered as f64 } else { 0.0 };
    println!("read coverage {covered}/{total} = {cov:.1}%; round-trip on covered {matched}/{covered} = {acc:.1}%");
    assert_eq!(total, gold::COUNT);
    assert!(covered > 0, "the reader covers no gold sentence");
}
/// A diagnostic, not a gate: it prints the gold frames the realiser does not
/// reproduce, each the name of a lesson the generator is still missing.
/// `#[ignore]` so it runs only when asked: `cargo test -p phisor
/// gold_realisation_failures -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: prints the gold frames the realiser misses"]
fn gold_realisation_failures() {
    let gold = gold::read().expect("the frozen gold set loads");
    let mut shown = 0;
    for (act, sentence) in gold.pairs() {
        let got = realise(act);
        let want = crate::guide::holdout::normalise(sentence);
        if got != want && shown < 40 {
            shown += 1;
            println!("WANT: {want}");
            println!("GOT:  {got}");
            println!("ACT:  {act:?}");
            println!("---");
        }
    }
    println!("shown {shown} of the realiser's gold misses");
}

/// A diagnostic, not a gate: for every gold sentence it reports whether the
/// reader abstained (returned None - an uncovered shape) or read a wrong Act
/// (a soundness concern), so the two kinds of gap can be told apart.
/// `#[ignore]` so it runs only when asked: `cargo test -p phisor
/// gold_reading_failures -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: prints the gold sentences the reader abstains on or mis-reads"]
fn gold_reading_failures() {
    let gold = gold::read().expect("the frozen gold set loads");
    let (mut abstained, mut wrong, mut correct) = (0usize, 0usize, 0usize);
    let mut shown_abstain = 0;
    let mut shown_wrong = 0;
    for (want, sentence) in gold.pairs() {
        match read(sentence) {
            None => {
                abstained += 1;
                if shown_abstain < 40 {
                    shown_abstain += 1;
                    println!("ABSTAIN: {sentence}");
                }
            }
            Some(got) if &got == want => correct += 1,
            Some(got) => {
                wrong += 1;
                if shown_wrong < 40 {
                    shown_wrong += 1;
                    println!("WRONG:   {sentence}");
                    println!("  want:  {want:?}");
                    println!("  got:   {got:?}");
                }
            }
        }
    }
    println!(
        "reader over {} gold: {correct} correct, {wrong} wrong, {abstained} abstained",
        gold::COUNT
    );
}

#[test]
fn an_imperative_reads_with_the_direct_force() {
    // A base verb with no subject is a command (Murphy Unit 35), with its
    // object in place and the Direct force recorded.
    let act = read("close the window.").expect("an imperative is covered");
    assert_eq!(act.force, crate::ir::Force::Direct);
    assert_eq!(act.frame.pred, "close");
    assert_eq!(realise(&act), "close the window.");
}

#[test]
fn a_ditransitive_command_names_the_recipient_first() {
    // "tell" takes a recipient; a lone object pronoun after it is the
    // recipient, not the theme.
    let act = read("tell me the truth.").expect("a ditransitive command is covered");
    assert_eq!(act.frame.get(&crate::ir::Role::Recipient), Some(&crate::ir::Filler::pronoun("me")));
}

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
fn a_lone_object_pronoun_after_a_plain_verb_is_the_theme() {
    // "how did you do it?" - "it" is the theme of "do", not a recipient.
    let act = read("how did you do it?").expect("covered");
    assert_eq!(act.frame.get(&crate::ir::Role::Theme), Some(&crate::ir::Filler::pronoun("it")));
}