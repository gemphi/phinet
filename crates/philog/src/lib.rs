//! # philog — the logic of Phient systems
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle; meaning is a side effect of pairs of words moving under a condition, and they move by the error of what was expected — toward what came, away from what did not.
//!
//! Reasoning in phase space: pathfinding toward an answer, analogy and
//! program analogy, planning, abstraction, sorting, interjection (the
//! hypothetical: swap a word's phase and re-evaluate), the evaluation of a
//! chain's convergence, and the `Chain` of steps itself. It was phient's
//! `reasoning`; it reads phient's generator and evaluator, and its math
//! belongs in phical.
//!
#![allow(dead_code)]
//! Reasoning module: phase-space pathfinding, analogy, hybrid reasoning,
//! program synthesis, planning, and abstraction.
//!
//! Module structure:
//! - pathfinding.rs: Phase-space pathfinding (existing ReasoningEngine)
//! - analogy.rs: Value-centric and program-centric analogy
//! - program_analogy.rs: Structural/program-centric analogy
//! - hybrid.rs: Hybrid reasoner combining geometric + structural
//! - sorting.rs: Sorting as a reasoning test
//! - planning.rs: Multi-step planning
//! - abstraction.rs: Abstraction extraction from examples
//! - interject.rs: Hypothetical/counterfactual reasoning
//! - eval.rs: Convergence diagnostics
//! - multi_path.rs: Multi-path and depth-controlled reasoning
//! - comparison.rs: Reasoning comparison and step templating

mod chain;
pub mod pathfinding;
pub mod analogy;
pub mod program;
pub use self::program as program_analogy;
pub mod hybrid;
pub mod sorting;
pub mod planning;
pub mod abstraction;
pub mod interject;
pub mod eval;
pub mod paths;
pub mod comparison;

pub use chain::Chain;
pub use pathfinding::{ReasoningEngine, ReasoningChain};
pub use hybrid::HybridReasoner;
pub use paths::MultiPath;
pub use comparison::ReasoningComparison;
pub use eval::Diagnostic;
