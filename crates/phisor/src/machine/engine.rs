//! `engine` — Note G's two remaining ideas, made runnable: the **trace** and
//! the **arrangement**.
//!
//! Note G's Bernoulli table numbers every operation the mill performs, names
//! the variables it reads from the store, and shows the result — the first
//! execution trace. Here every call the Lisp makes on the host is one
//! [`Step`]: a store read (`phisor-rows`, the variable card: which column was
//! brought to the mill) or a mill operation (`phisor-apply`, the operation
//! card). [`answer_traced`] answers a question over what was told and hands
//! the trace back beside the answer.
//!
//! Note D says the same computation has many arrangements, and that
//! considerations of economy choose among them. [`arrange`] is that choice
//! made by the engine: given several cards for the same operation, it runs
//! each on the same question and the same store, keeps those that agree on
//! the answer, and chooses the one whose trace is shortest — the fewest
//! columns fetched, the fewest operations performed. The human still writes
//! the cards; the engine arranges, within her limit: it follows the analysis
//! it is given and anticipates nothing.

use phiclm::lisp::Value;

use super::ask::{AskError, Answered};
use super::told::answer_told_traced_kinds;
use super::KindRows;
use crate::ir::Act;

/// One numbered operation of a run, as Note G's table has it: what was
/// done, to what, with what result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    pub n: usize,
    /// `rows` for a store read, else the operator's name.
    pub operation: String,
    /// The variable card: the Kind read, or the operands given.
    pub read: Vec<String>,
    /// The result, written as the machine would print it.
    pub result: String,
}

impl Step {
    pub(super) fn store(n: usize, kind: &str, rows: usize) -> Self {
        Step { n, operation: "rows".into(), read: vec![kind.to_string()], result: format!("{rows} rows") }
    }

    pub(super) fn mill(n: usize, operation: &str, operands: &[Value], result: &Result<Value, String>) -> Self {
        let result = match result {
            Ok(v) => v.to_string(),
            Err(e) => format!("fault: {e}"),
        };
        Step { n, operation: operation.to_string(), read: operands.iter().map(Value::to_string).collect(), result }
    }
}

/// The answer and the numbered trace that produced it.
pub fn answer_traced(act: &Act, rows: &[phiecs::Row]) -> Result<(Answered, Vec<Step>), AskError> {
    answer_told_traced_kinds(act, KindRows::of(rows), None)
}

/// One candidate card tried in an [`arrange`]: its name, its steps, and
/// what it answered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tried {
    pub name: String,
    pub steps: usize,
    pub answer: Vec<String>,
}

/// The engine's choice among arrangements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arrangement {
    /// The card chosen: the shortest trace among those that agree.
    pub chosen: String,
    pub answer: Vec<String>,
    pub steps: usize,
    /// Every card tried, in the order given.
    pub tried: Vec<Tried>,
    /// Cards that answered differently from the agreed answer, by name.
    pub disagreed: Vec<String>,
}

/// Tries each `(name, card)` — a `(define told-answer (lambda …))` that
/// replaces the shipped card for that run — on `act` over `rows`, and
/// chooses the shortest trace among the cards that agree. Agreement is with
/// the answer most cards gave; a tie keeps the first.
pub fn arrange(act: &Act, rows: &[phiecs::Row], cards: &[(&str, &str)]) -> Result<Arrangement, AskError> {
    let mut tried = Vec::new();
    for (name, card) in cards {
        let (answered, trace) = answer_told_traced_kinds(act, KindRows::of(rows), Some(card))?;
        tried.push(Tried { name: name.to_string(), steps: trace.len(), answer: answered.members });
    }
    let Some(first) = tried.first() else {
        return Err(AskError::Machine("arrange: no cards to arrange".into()));
    };
    // The agreed answer: the one most cards gave, the first on a tie.
    let agreed = tried
        .iter()
        .map(|t| &t.answer)
        .max_by_key(|a| (tried.iter().filter(|t| &t.answer == *a).count(), std::cmp::Reverse(tried.iter().position(|t| &t.answer == *a))))
        .cloned()
        .unwrap_or_else(|| first.answer.clone());
    let disagreed = tried.iter().filter(|t| t.answer != agreed).map(|t| t.name.clone()).collect();
    let chosen = tried.iter().filter(|t| t.answer == agreed).min_by_key(|t| t.steps).cloned().unwrap_or_else(|| first.clone());
    Ok(Arrangement { chosen: chosen.name, answer: agreed, steps: chosen.steps, tried, disagreed })
}

#[cfg(test)]
mod tests;
