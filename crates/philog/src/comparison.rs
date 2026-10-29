//! Reasoning comparison and step templating.
//! Provides readable step output and side-by-side comparison of reasoning methods.

use phiecs::phasor::facet::Facet;
use crate::eval::Diagnostic;
use crate::hybrid::{HybridReasoner, HybridResult};
use crate::pathfinding::{ReasoningChain, ReasoningEngine, ReasoningStep};
use serde::Serialize;

/// Comparison of all reasoning approaches.
#[derive(Debug, Clone, Serialize)]
pub struct ReasoningComparison {
    pub pathfinding: ReasoningChain,
    pub hybrid: HybridResult,
    pub best_method: String,
}

#[derive(Debug, Default)]
pub struct StepTemplate;

impl StepTemplate {
    /// Templates a reasoning step into a readable sentence.
    pub fn step(step: &ReasoningStep, facet: &Facet) -> String {
        let word = &step.focus_word;
        let phase_deg = step.phase_angle.to_degrees();

        let nearby: Vec<String> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| *w != word)
            .map(|(w, p)| {
                let mut diff = (p.phase - step.phase_angle).abs();
                if diff > std::f64::consts::PI {
                    diff = 2.0 * std::f64::consts::PI - diff;
                }
                (w.clone(), diff)
            })
            .take(50)
            .filter(|(_, d)| *d < 0.3)
            .map(|(w, _)| w)
            .take(3)
            .collect();

        if nearby.is_empty() {
            format!("Step {}: Focusing on '{}' at {:.0}° — exploring this region of meaning.",
                step.step_number, word, phase_deg)
        } else {
            format!("Step {}: Focusing on '{}' at {:.0}° — nearby concepts: {}.",
                step.step_number, word, phase_deg, nearby.join(", "))
        }
    }

    /// Templates all steps in a chain into readable text.
    pub fn chain(chain: &ReasoningChain, facet: &Facet) -> Vec<String> {
        chain.steps.iter().map(|s| Self::step(s, facet)).collect()
    }
}

impl ReasoningComparison {
    /// Runs all reasoning methods and returns them side by side.
    pub fn compare(facet: &Facet, problem: &str) -> ReasoningComparison {
        let engine = ReasoningEngine;
        let pathfinding = engine.solve(facet, problem);

        let hybrid_reasoner = HybridReasoner::new();
        let hybrid = hybrid_reasoner.solve_hybrid(facet, problem);

        let pf_confidence = Diagnostic::confidence(&pathfinding);
        let hybrid_confidence = hybrid.confidence;

        let best_method = if hybrid_confidence > pf_confidence {
            "hybrid".to_string()
        } else {
            "pathfinding".to_string()
        };

        ReasoningComparison {
            pathfinding,
            hybrid,
            best_method,
        }
    }
}
