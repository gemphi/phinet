use super::*;
use crate::ground::ground;
use crate::ir::{Filler, Force, Frame, Role, Tense, Wh};

/// Three told sentences as acts built by hand - "sarah is driving to
/// work", "tom was reading a book", "a girl was wearing a blue suit" - so
/// these tests do not depend on the reader, whose lexicon other tests prime
/// process-wide.
fn told() -> Vec<phiecs::Row> {
    let acts = [
        Frame::new("drive", Tense::Present).with(Role::Agent, Filler::Name("sarah".into())).with(Role::named("to"), Filler::Word("work".into())),
        Frame::new("read", Tense::Past).with(Role::Agent, Filler::Name("tom".into())).with(Role::Theme, Filler::Word("book".into())),
        Frame::new("wear", Tense::Past).with(Role::Agent, Filler::Word("girl".into())).with(Role::Theme, Filler::Word("suit".into())),
    ];
    acts.into_iter().enumerate().flat_map(|(i, frame)| ground(&Act::new(Force::Assert, frame), &format!("e{}", i + 1))).collect()
}

/// "what was <agent> <pred>ing?" as an act.
fn what_did(agent: &str, pred: &str) -> Act {
    Act::new(Force::Ask, Frame::new(pred, Tense::Past).with(Role::Agent, Filler::Word(agent.into())).with(Role::Theme, Filler::Hole(Wh::What)))
}

#[test]
fn the_trace_numbers_every_column_fetched_and_operation_performed() {
    let rows = told();
    let act = what_did("tom", "read");
    let (answered, trace) = answer_traced(&act, &rows).expect("answered");
    assert_eq!(answered.members, ["book"]);
    assert!(!trace.is_empty(), "a run leaves a trace");
    assert!(trace.iter().enumerate().all(|(i, s)| s.n == i + 1), "numbered from 1 in order: {trace:?}");
    // The fixed roles and the hole are each a column fetched: assert, agent, theme.
    let fetched: Vec<&str> = trace.iter().filter(|s| s.operation == "rows").flat_map(|s| s.read.iter().map(String::as_str)).collect();
    for kind in ["assert", "agent", "theme"] {
        assert!(fetched.contains(&kind), "{kind} fetched: {fetched:?}");
    }
    assert!(trace.iter().all(|s| s.result.ends_with(" rows") || !s.result.is_empty()));
}

#[test]
fn the_engine_arranges_by_economy_among_cards_that_agree() {
    let rows = told();
    let act = what_did("girl", "wear");
    let shipped = "(define told-answer (lambda (roles rels) (image (union-all roles) (events rels))))";
    // The same answer, one column fetched twice: a longer trace.
    let wasteful = "(define told-answer (lambda (roles rels) (image (union (phisor-rows \"theme\") (union-all roles)) (events rels))))";
    // A different answer: reads the agent off the events instead.
    let wrong = "(define told-answer (lambda (roles rels) (image (phisor-rows \"agent\") (events rels))))";
    let a = arrange(&act, &rows, &[("wasteful", wasteful), ("shipped", shipped), ("wrong", wrong)]).expect("arranged");
    assert_eq!(a.answer, ["suit"]);
    assert_eq!(a.chosen, "shipped", "{a:?}");
    assert_eq!(a.disagreed, ["wrong"]);
    let steps = |n: &str| a.tried.iter().find(|t| t.name == n).map(|t| t.steps).unwrap();
    assert!(steps("wasteful") > steps("shipped"), "{a:?}");
    assert!(arrange(&act, &rows, &[]).is_err(), "nothing to arrange is an error, not a choice");
}
