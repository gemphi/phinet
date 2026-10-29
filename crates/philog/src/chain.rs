//! A chain of steps toward an answer, whatever one step records: phiano's
//! chain of thought (`core::searle`) holds its `Thought`s, the pathfinding
//! here its phase steps (`ReasoningChain`). One chain, two kinds of step.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Chain<S> {
    pub problem: String,
    pub steps: Vec<S>,
    pub converged: bool,
    pub final_answer: String,
}
