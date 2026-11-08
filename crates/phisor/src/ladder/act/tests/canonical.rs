//! The canonical form: set-aside adverbs and the canonical round trip.

use super::*;

/// Feature: phinet-system-completion, Property 1: Strict round trip implies
/// canonical round trip.
///
/// **Validates: Requirements 1.1**
///
/// Every gold frame whose realised text equals the normalised sentence also
/// has equal canonical forms, so the canonical count is never below the strict
/// one. The input is the fixed 150-frame gold file, so it runs once over it.
#[test]
fn a_strict_gold_round_trip_is_also_a_canonical_round_trip() {
    let gold = gold::read().expect("the frozen gold set loads");
    for (i, (act, sentence)) in gold.pairs().enumerate() {
        let strict = realise(act) == crate::guide::holdout::normalise(sentence);
        let (canon, total, _) = round_trip_canonical([(act, sentence)]);
        assert_eq!(total, 1);
        assert!(!strict || canon == 1, "gold frame {i} round-trips strictly but not canonically: {sentence}");
    }
    let (strict, total, _) = round_trip(gold.pairs());
    let (canon, canon_total, _) = round_trip_canonical(gold.pairs());
    assert_eq!(total, canon_total);
    assert!(canon >= strict, "canonical {canon} < strict {strict}");
    println!("gold round trip: strict {strict}/{total}, canonical {canon}/{canon_total}");
}


#[test]
fn a_set_aside_adverb_is_understood_around_not_inside_the_frame() {
    use crate::ir::{Aspect, Polarity};
    // "he had already left." reads to the same frame as "he had left.": the
    // IR has no slot for "already", and the gold leaves it out too.
    let with = read("he had already left.").expect("adverb is set aside");
    let without = read("he had left.").expect("plain perfect reads");
    assert_eq!(with.frame, without.frame);
    assert_eq!(with.frame.aspect, Aspect::Perfect);
    // A final or degree adverb is set aside the same way.
    let often = read("i don't go away very often.").expect("degree adverbs are set aside");
    assert_eq!(often.frame.pred, "go");
    assert_eq!(often.frame.polarity, Polarity::Negative);
}

#[test]
fn round_trip_ignores_only_the_set_aside_adverbs() {
    // The same adverbs are set aside on both sides, so a sentence whose only
    // difference is such an adverb compares equal; a different content word
    // does not.
    assert_eq!(canonical("he had left."), canonical("he had already left."));
    assert_ne!(canonical("he had left."), canonical("she had left."));
}

#[test]
fn a_multi_word_adverb_is_set_aside_as_a_unit() {
    // "it's hard to find a job right now." has no IR slot for "right now"; the
    // phrase is dropped whole, so it reads to the same frame as without it and
    // round-trips. A trailing "right now" is the task's named example (R3.3).
    let with = read("i travel a lot these days.");
    let plain = read("i travel a lot.");
    assert_eq!(with, plain, "a trailing multi-word adverb is set aside as a unit");
    // The phrase is dropped on both sides of the round-trip comparison.
    assert_eq!(canonical("joe is away at the moment."), canonical("joe is away."));
    assert_eq!(canonical("it's hard right now."), canonical("it's hard."));
}

#[test]
fn a_multi_word_adverb_does_not_strip_its_words_singly() {
    let w = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    // The phrase is matched whole; its member words are ordinary outside it.
    // "the moment" and "days" survive when they are not part of the phrase.
    let kept = strip_adverb_phrases(&w("the moment has come these days"));
    assert_eq!(kept, vec!["the", "moment", "has", "come"]);
    // A noun phrase that merely contains a phrase word is untouched.
    assert_eq!(strip_adverb_phrases(&w("these books")), vec!["these", "books"]);
    assert_eq!(strip_adverb_phrases(&w("right turn")), vec!["right", "turn"]);
}

#[test]
fn every_set_aside_adverb_phrase_is_more_than_one_word() {
    // The phrase table is for multi-word adverbials only; single adverbs live
    // in set_aside_adverb.txt. A one-word entry here would double-strip.
    for phrase in crate::lexicon::list::set_aside_adverb_phrases() {
        assert!(phrase.len() >= 2, "{phrase:?} is a single word, not a phrase");
    }
}
