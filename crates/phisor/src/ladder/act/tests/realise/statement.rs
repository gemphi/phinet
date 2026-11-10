//! Realising statements: `never` and proper-name case.

use super::*;

#[test]
fn never_is_carried_through_the_frame_and_written_back() {
    use crate::ir::Polarity;
    // Task 7.3: `never` is read as a negative frame that remembers its word,
    // and the Realiser writes it where English puts it.
    for said in [
        "she never eats fish.",
        "he has never finished the work.",
        "they will never come.",
        "i was never late.",
    ] {
        let act = read(said).unwrap_or_else(|| panic!("{said} is covered"));
        assert_eq!(act.frame.polarity, Polarity::Negative, "{said}");
        assert!(act.frame.never, "{said}");
        assert_eq!(realise(&act), said);
    }
    // A plain `not` stays `not`.
    let act = read("she does not eat fish.").expect("covered");
    assert!(!act.frame.never);
    assert_eq!(realise(&act), "she does not eat fish.");
}

#[test]
fn a_proper_name_keeps_its_case_through_the_frame_and_is_written_back() {
    use crate::ir::Filler;
    // Task 7.4: the raw sentence spells the name, the Reader keeps it as a
    // `Filler::Name`, and the Realiser writes it back with its capital.
    for (said, name) in [
        ("She lives in London.", "London"),
        ("They play tennis on Sunday.", "Sunday"),
    ] {
        let act = read(said).unwrap_or_else(|| panic!("{said} is covered"));
        let held: Vec<_> = act.frame.slots.iter().filter(|(_, f)| matches!(f, Filler::Name(n) if n == name)).collect();
        assert_eq!(held.len(), 1, "{said}");
        assert_eq!(realise_cased(&act), said.replacen(&said[..1], &said[..1].to_lowercase(), 1));
        // The folded text is unchanged: the gold and Canonical_Form compare lower case.
        assert_eq!(realise(&act), said.to_lowercase());
    }
    // Lower case stays a common noun; a capital that only opens the sentence is not a name.
    let act = read("she lives in london.").expect("covered");
    assert!(!act.frame.slots.iter().any(|(_, f)| matches!(f, Filler::Name(_))));
    let act = read("The dog sleeps.").expect("covered");
    assert!(!act.frame.slots.iter().any(|(_, f)| matches!(f, Filler::Name(_))));
}
