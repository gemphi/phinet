//! Reading noun phrases: cardinals and heads.

use super::*;

#[test]
fn a_cardinal_number_counts_the_head_and_round_trips() {
    use crate::ir::{Attr, Filler, Role};
    let act = read("she speaks four languages.").expect("a cardinal count is covered");
    assert_eq!(act.frame.pred, "speak");
    let Some(Filler::Thing(t)) = act.frame.get(&Role::Theme) else { panic!("theme is not a thing") };
    assert_eq!(t.head, "language");
    assert!(t.plural);
    assert!(t.attrs.iter().any(|(a, f)| *a == Attr::Count && matches!(f, Filler::Word(w) if w == "four")));
    assert_eq!(realise(&act), "she speaks four languages.");
    // A number subject too.
    let act = read("three people came.").expect("a cardinal subject is covered");
    assert_eq!(realise(&act), "three people came.");
}

#[test]
fn and_is_not_a_noun_phrase_head() {
    assert!(super::read_noun_phrase(&["and".to_string()]).is_none());
    assert!(super::read_noun_phrase(&["the".to_string(), "and".to_string()]).is_none());
    assert!(read("he reads and.").is_none());
}

#[test]
fn an_adjective_is_a_modifier_only_before_a_head() {
    use crate::ir::{Attr, Filler, Role};
    // "a white cap": the adjective is kept as the head's modifier.
    let act = read("a boy was wearing a white cap.").expect("read");
    let Some(Filler::Thing(t)) = act.frame.get(&Role::Theme) else { panic!("theme is not a thing: {act:?}") };
    assert_eq!(t.head, "cap");
    assert!(t.attrs.iter().any(|(a, f)| *a == Attr::Mod && matches!(f, Filler::Word(w) if w == "white")), "{t:?}");
    assert_eq!(realise(&act), "a boy was wearing a white cap.");
    // The same word with nothing after it, or with a word that cannot head
    // a phrase after it, is the head itself.
    let words = |ws: &[&str]| ws.iter().map(|w| w.to_string()).collect::<Vec<_>>();
    let (filler, end) = super::read_noun_phrase(&words(&["the", "blue"])).expect("read");
    assert!(matches!(&filler, Filler::Thing(t) if t.head == "blue" && t.attrs.is_empty()), "{filler:?}");
    assert_eq!(end, 2);
    let (filler, end) = super::read_noun_phrase(&words(&["the", "blue", "is"])).expect("read");
    assert!(matches!(&filler, Filler::Thing(t) if t.head == "blue" && t.attrs.is_empty()), "{filler:?}");
    assert_eq!(end, 2);
}
