//! Force: express phrases, wishes, performatives and politeness.

use super::*;

#[test]
fn i_wish_is_a_wish_force_over_the_wished_for_content() {
    use crate::ir::{Force, Modal};
    let act = read("i wish i could sing.").expect("a wish is covered");
    assert_eq!(act.force, Force::Wish);
    // The frame is the content, not a nested "wish": the gold's convention.
    assert_eq!(act.frame.pred, "sing");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(realise(&act), "i wish i could sing.");
    let act = read("i wish i had known.").expect("a past-perfect wish is covered");
    assert_eq!(act.force, Force::Wish);
    assert_eq!(realise(&act), "i wish i had known.");
}

#[test]
fn i_promise_is_an_explicit_commissive() {
    use crate::ir::Force;
    let act = read("i promise to come.").expect("a promise is covered");
    assert_eq!(act.force, Force::Commit);
    assert_eq!(realise(&act), "i promise to come.");
}

#[test]
fn a_verb_that_announces_no_force_leaves_the_clause_an_assertion() {
    use crate::ir::Force;
    // Only the explicit performatives set a force; "think" does not, and a bare
    // "will" is never read as a promise.
    let act = read("i think i could sing.").expect("a complement clause is covered");
    assert_eq!(act.force, Force::Assert);
    let act = read("i will explain the rule.").expect("a bare will is covered");
    assert_eq!(act.force, Force::Assert);
}

/// Task 2.2: the three wish frames are said in the subjunctive, and a wish
/// whose verb has no row in `wish_form.tsv` stays a plain statement.
#[test]
fn wish_frames_are_said_in_the_subjunctive() {
    let gold = gold::read().expect("the frozen gold set loads");
    let pairs: Vec<_> = gold.pairs().collect();
    for i in [137, 138, 143] {
        let (act, sentence) = pairs[i];
        assert_eq!(realise(act), crate::guide::holdout::normalise(sentence), "gold frame {i}");
    }
    let (could, sentence) = pairs[135];
    assert_eq!(sentence, "i wish i could sing.");
    assert_eq!(realise(could), "i wish i could sing.");
}

/// The Reader reads a wished-for modal verb group back through `wish_form.tsv`,
/// for any subject and verb, not only the gold sentence.
#[test]
fn a_wish_with_a_past_subjunctive_modal_group_reads_as_a_wish() {
    use crate::ir::{Modal, Polarity, Tense};
    for (said, negative) in [
        ("i wish we did not have to go.", true),
        ("i wish they did not have to leave.", true),
        ("i wish she had to study.", false),
    ] {
        let act = read(said).unwrap_or_else(|| panic!("{said} reads"));
        assert_eq!(act.force, Force::Wish, "{said}");
        assert_eq!(act.frame.modal, Some(Modal::Must), "{said}");
        assert_eq!(act.frame.tense, Tense::Past, "{said}");
        assert_eq!(act.frame.polarity == Polarity::Negative, negative, "{said}");
        assert_eq!(realise(&act), said, "{said}");
    }
    // A group with no row stays an ordinary clause.
    assert_ne!(read("i wish they did not like it.").map(|a| a.frame.modal), Some(Some(Modal::Must)));
}
/// Task 2.1: the four Express frames are said as their fixed phrases, and an
/// Express frame with no phrase in `expressive_phrase.tsv` stays a statement.
#[test]
fn express_frames_are_said_as_their_fixed_phrases() {
    let gold = gold::read().expect("the frozen gold set loads");
    let pairs: Vec<_> = gold.pairs().collect();
    for i in [127, 128, 129, 131] {
        let (act, sentence) = pairs[i];
        assert_eq!(realise(act), crate::guide::holdout::normalise(sentence), "gold frame {i}");
    }
    let (mind, sentence) = pairs[130];
    assert_eq!(sentence, "i do not mind the noise.");
    assert_eq!(realise(mind), "i do not mind the noise.");
}

/// Task 4.2: the Reader reads `expressive_phrase.tsv` in reverse, so the four
/// gold fixed phrases come back as the frames the Realiser said, with the
/// unsaid speaker or addressee restored; a phrase inside a clause, an
/// exclamation and a phrase that loses a word stay with the ordinary reader.
#[test]
fn express_phrases_are_read_back_to_their_gold_frames() {
    let gold = gold::read().expect("the frozen gold set loads");
    let pairs: Vec<_> = gold.pairs().collect();
    for i in [127, 128, 129, 131] {
        let (act, sentence) = pairs[i];
        // The gold sentence is lower case, so a name is spelled from the original.
        let said = if i == 131 { "Welcome to London." } else { sentence };
        let read = read(said).unwrap_or_else(|| panic!("gold frame {i} reads"));
        assert_eq!(&read, act, "gold frame {i}");
    }
    assert!(read("kevin never says thank you.").is_none_or(|a| a.force != crate::ir::Force::Express));
    assert!(read("welcome to our country!").is_none_or(|a| a.force != crate::ir::Force::Express));
    assert!(read("congratulations on your new job.").is_none_or(|a| a.force != crate::ir::Force::Express));
}
/// Task 4.3: a first-person present psychological-state predicate is Express,
/// a third-person, past or future one stays Assert, and "please" ahead of an
/// imperative is a Direct.
#[test]
fn state_predicates_and_politeness_set_the_force() {
    use crate::ladder::read;
    let force = |s: &str| read(s).map(|a| a.force);
    assert_eq!(force("i enjoy the lesson."), Some(Force::Express));
    assert_eq!(force("she enjoys the lesson."), Some(Force::Assert));
    assert_eq!(force("i enjoyed the lesson."), Some(Force::Assert));
    assert_eq!(force("please close the window."), Some(Force::Direct));
}
