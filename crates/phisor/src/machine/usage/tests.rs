use super::super::{answer_usage, Usage};
use crate::ground::unfold;
use crate::ladder::read;

#[test]
fn a_question_answered_over_what_was_told_uses_primitives_and_cards_and_no_operator() {
    let rows = unfold(&read("sarah is driving to work.").unwrap(), "e1");
    let act = read("where is sarah driving?").unwrap();
    let run = answer_usage(&act, &rows).expect("answered");
    assert_eq!(run.answered.members, ["work"]);
    let u = &run.usage;
    assert_eq!(u.runs, 1);
    assert!(u.cards.contains_key("told-answer"), "the shipped card is what answers: {u:?}");
    assert!(u.primitives.contains_key("image"), "the hole is read with image: {u:?}");
    assert_eq!(u.host.get("phisor-rows").copied().unwrap_or(0), run.steps.iter().filter(|s| s.operation == "rows").count(), "every column fetched is a host call");
    assert!(u.operators.is_empty(), "answering English calls none of the 64 yet: {:?}", u.operators);
    assert!(!u.primitives.contains_key("told-answer") && !u.cards.contains_key("image"), "names are sorted by what they are");
    assert!(u.cards.contains_key("phisor-answer-told") && !u.host.contains_key("phisor-answer-told"), "the question's own lambda is a card, not a host call: {u:?}");
}

#[test]
fn usages_add_and_say_how_many_runs() {
    let rows = unfold(&read("tom was reading a book.").unwrap(), "e1");
    let one = answer_usage(&read("what was tom reading?").unwrap(), &rows).unwrap().usage;
    let mut sum = Usage::default();
    sum.add(&one);
    sum.add(&one);
    assert_eq!(sum.runs, 2);
    assert_eq!(sum.cards.get("told-answer"), Some(&2));
    let table = sum.to_string();
    assert!(table.contains("| `told-answer` | card | 2 | 1.0 |"), "{table}");
    assert!(table.ends_with("· 0 of 64 Operators · 1 cards") || table.contains("0 of 64 Operators"), "{table}");
}
