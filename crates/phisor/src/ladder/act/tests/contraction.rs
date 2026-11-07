//! Contraction expansion.

use super::*;


#[test]
fn contractions_expand_to_their_words() {
    use crate::ir::Polarity;
    // "i don't go" reads as a negated present: the n't expands to "do not".
    let act = read("i don't go away.").expect("a contraction is covered");
    assert_eq!(act.frame.pred, "go");
    assert_eq!(act.frame.polarity, Polarity::Negative);
    // "i'm" expands to "i am", so a progressive copula reads.
    let w = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    assert_eq!(expand_contractions(&w("i don't")), vec!["i", "do", "not"]);
    assert_eq!(expand_contractions(&w("i'm you're we've")), vec!["i", "am", "you", "are", "we", "have"]);
    assert_eq!(expand_contractions(&w("can't won't")), vec!["cannot", "will", "not"]);
    // Ambiguous forms are left as read.
    assert_eq!(expand_contractions(&w("he's tired i'd")), vec!["he's", "tired", "i'd"]);
}

#[test]
fn an_s_contraction_is_resolved_by_the_word_that_follows() {
    let w = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    // been and got mean "has"; an -ing form and a non-participle mean "is".
    assert_eq!(expand_contractions(&w("it's been raining")), vec!["it", "has", "been", "raining"]);
    assert_eq!(expand_contractions(&w("he's got a boat")), vec!["he", "has", "got", "a", "boat"]);
    assert_eq!(expand_contractions(&w("it's raining")), vec!["it", "is", "raining"]);
    assert_eq!(expand_contractions(&w("he's a nice person")), vec!["he", "is", "a", "nice", "person"]);
    // A set-aside adverb does not hide the word that settles it.
    assert_eq!(expand_contractions(&w("it's never been")), vec!["it", "has", "never", "been"]);
    // An -ed participle could be either, so it is left for the reader to refuse.
    assert_eq!(expand_contractions(&w("he's tired")), vec!["he's", "tired"]);
}
