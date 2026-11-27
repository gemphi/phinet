use crate::Chain;
use phiecs::phasor::facet::Facet;

pub const REASONING_MAX_STEPS: usize = 16;  // 2^4
pub const REASONING_CONVERGENCE: f64 = 0.01; // Phase angle delta threshold for convergence

#[derive(Debug, Clone, serde::Serialize)]
pub struct ReasoningStep {
    pub step_number: usize,
    pub focus_word: String,
    pub phase_angle: f64,
    pub phase_delta: f64,
    #[allow(dead_code)]
    pub sentence: String,
}

/// The pathfinding chain: the cognitive core's [`Chain`] of phase steps.
pub type ReasoningChain = Chain<ReasoningStep>;

#[derive(Debug, Default)]
pub struct ReasoningEngine;

impl ReasoningEngine {
    /// Solves a multi-step problem by walking the lexicon's phases
    /// (Phase-Space Pathfinding and Wave Convergence).
    pub fn solve(&self, facet: &Facet, problem: &str) -> ReasoningChain {
        crate::walk::walk(facet, problem, REASONING_MAX_STEPS, crate::walk::Aim::Context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_engine() {
        let mut facet = Facet::new();
        facet.get_or_init("ownership");
        facet.get_or_init("borrowing");
        facet.get_or_init("lifetime");
        facet.get_or_init("checker");

        let engine = ReasoningEngine;
        let chain = engine.solve(&facet, "ownership borrowing lifetime");

        assert!(!chain.steps.is_empty());
        assert!(!chain.steps[0].sentence.is_empty());
        assert_eq!(chain.problem, "ownership borrowing lifetime");
    }
}
