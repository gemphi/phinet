//! Convergence diagnostics for reasoning chains.
//! Detects oscillation, divergence, and stuck states.

use crate::pathfinding::ReasoningChain;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ConvergenceMode {
    Converged,
    Oscillating,
    Diverging,
    Stuck,
}

#[derive(Debug, Default)]
pub struct Diagnostic;

impl Diagnostic {
    /// Diagnoses the convergence mode of a reasoning chain.
    pub fn diagnose(chain: &ReasoningChain) -> ConvergenceMode {
        if chain.converged {
            return ConvergenceMode::Converged;
        }

        if chain.steps.len() < 2 {
            return ConvergenceMode::Stuck;
        }

        let deltas: Vec<f64> = chain.steps.iter().map(|s| s.phase_delta).collect();

        let last_delta = *deltas.last().unwrap_or(&0.0);
        let first_delta = deltas[0];

        if last_delta < 0.05 {
            return ConvergenceMode::Converged;
        }

        if Self::is_oscillating(&deltas) {
            return ConvergenceMode::Oscillating;
        }

        if last_delta > first_delta * 1.5 {
            return ConvergenceMode::Diverging;
        }

        ConvergenceMode::Stuck
    }

    /// Detects oscillation: deltas alternate between high and low.
    fn is_oscillating(deltas: &[f64]) -> bool {
        if deltas.len() < 4 {
            return false;
        }

        let mut alternations = 0;
        for i in 1..deltas.len() {
            let prev_high = deltas[i - 1] > 0.1;
            let curr_high = deltas[i] > 0.1;
            if prev_high != curr_high {
                alternations += 1;
            }
        }

        alternations > deltas.len() / 2
    }

    /// Computes a confidence score for a reasoning chain.
    pub fn confidence(chain: &ReasoningChain) -> f64 {
        let mode = Self::diagnose(chain);

        let convergence_score = match mode {
            ConvergenceMode::Converged => 1.0,
            ConvergenceMode::Oscillating => 0.3,
            ConvergenceMode::Diverging => 0.1,
            ConvergenceMode::Stuck => 0.2,
        };

        let n_steps = chain.steps.len() as f64;
        let length_penalty = (1.0 / (1.0 + n_steps * 0.05)).max(0.3);

        let avg_delta: f64 = if chain.steps.is_empty() {
            1.0
        } else {
            chain.steps.iter().map(|s| s.phase_delta).sum::<f64>() / n_steps
        };
        let coherence = (1.0 - avg_delta / PI).max(0.0);

        let novelty = if chain.steps.is_empty() {
            0.0
        } else {
            let unique: std::collections::HashSet<&String> =
                chain.steps.iter().map(|s| &s.focus_word).collect();
            unique.len() as f64 / n_steps
        };

        0.3 * convergence_score + 0.3 * coherence + 0.2 * length_penalty + 0.2 * novelty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phiecs::phasor::facet::Facet;
    use crate::pathfinding::ReasoningEngine;

    #[test]
    fn test_convergence_mode_converged() {
        let mut facet = Facet::new();
        facet.get_or_init("a");
        facet.get_or_init("b");
        let engine = ReasoningEngine;
        let chain = engine.solve(&facet, "a b");
        let mode = Diagnostic::diagnose(&chain);
        assert!(mode == ConvergenceMode::Converged || mode == ConvergenceMode::Stuck);
    }
}
