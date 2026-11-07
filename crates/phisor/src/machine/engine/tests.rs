use super::*;
use crate::ground::ground;
use crate::ladder::read;

fn told() -> Vec<phiecs::Row> {
    let mut rows = Vec::new();
    for (i, s) in ["sarah is driving to work.", "tom was reading a book.", "a girl was wearing a blue suit."].iter().enumerate() {
        let act = read(s).unwrap_or_else(|| panic!("the reader reads {s}"));
        rows.extend(ground(&act, &format!("e{}", i + 1)));
    }
    rows
}

#[test]
fn the_trace_numbers_every_column_fetched_and_operation_performed() {
    let rows = told();
    let act = read("what was tom reading?").expect("the reader reads the question");
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
    let act = read("what was the girl wearing?").expect("the reader reads the question");
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
