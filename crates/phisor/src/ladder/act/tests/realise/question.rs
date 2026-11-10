//! Realising questions: negative operators and wh phrases.

use super::*;

#[test]
fn a_negative_question_puts_nt_on_the_operator_and_reads_back() {
    use crate::ir::{Force, Polarity};
    // The contracted and the spaced `not` forms read to the same frame, and the
    // Realiser says the contracted form.
    for (said, spaced) in [
        ("don't you know the answer?", "do you not know the answer?"),
        ("didn't he go?", "did he not go?"),
        ("isn't she here?", "is she not here?"),
        ("won't they come?", "will they not come?"),
    ] {
        let a = read(said).unwrap_or_else(|| panic!("{said} is covered"));
        let b = read(spaced).unwrap_or_else(|| panic!("{spaced} is covered"));
        assert_eq!(a.force, Force::Ask);
        assert_eq!(a.frame.polarity, Polarity::Negative);
        assert_eq!(a, b);
        assert_eq!(realise(&a), spaced);
    }
    // `am` has no contraction, so `not` follows the subject.
    let act = read("am i not late?").expect("am i not is covered");
    assert_eq!(realise(&act), "am i not late?");
}

/// Task 2.3: a `how` hole in a `for` slot is said "how long"; in a `by` slot
/// it stays "how".
#[test]
fn a_how_hole_in_a_for_slot_is_said_how_long() {
    let gold = gold::read().expect("the frozen gold set loads");
    let pairs: Vec<_> = gold.pairs().collect();
    let (act, sentence) = pairs[89];
    assert_eq!(sentence, "how long have you been waiting?");
    assert_eq!(realise(act), "how long have you been waiting?");
    let (by, _) = pairs.iter().find(|(_, s)| *s == "how did you do it?").expect("the `by` how frame");
    assert_eq!(realise(by), "how did you do it?");
}
