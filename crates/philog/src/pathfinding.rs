use crate::Chain;
use phiecs::phasor::facet::Facet;
use phient::generate::ContextWaveBuffer;
use phiton::p64::words::Tokenizer;
use std::f64::consts::PI;

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
    /// Solves multi-step problem using Phase-Space Pathfinding & Wave Convergence (Phase 6).
    pub fn solve(&self, facet: &Facet, problem: &str) -> ReasoningChain {
        let mut context_buffer = ContextWaveBuffer::new(4096);
        context_buffer.push_turn(facet, problem);

        let mut steps = Vec::new();
        let mut prev_phase = context_buffer.context_phase();
        let mut converged = false;

        let tokens = Tokenizer::tokenize(problem);
        let mut visited = std::collections::HashSet::new();
        for t in &tokens {
            visited.insert(t.clone());
        }

        for step in 1..=REASONING_MAX_STEPS {
            let current_phase = context_buffer.context_phase();
            let phase_delta = (current_phase - prev_phase).abs();
            let normalized_delta = if phase_delta > PI {
                2.0 * PI - phase_delta
            } else {
                phase_delta
            };

            // Check wave convergence
            if step > 1 && normalized_delta < REASONING_CONVERGENCE {
                converged = true;
                break;
            }

            // Find next resonant word along path
            let mut candidates: Vec<(String, f64)> = facet
                .lexicon
                .iter()
                .filter(|(w, _)| !visited.contains(*w))
                .map(|(w, p)| {
                    let mut diff = (p.phase - current_phase).abs();
                    if diff > PI {
                        diff = 2.0 * PI - diff;
                    }
                    (w.clone(), diff)
                })
                .collect();

            candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            if candidates.is_empty() {
                break;
            }

            let (next_word, _) = candidates[0].clone();
            visited.insert(next_word.clone());

            let step_text = format!("Step {}: resonate with {}", step, next_word);
            context_buffer.push_turn(facet, &step_text);

            steps.push(ReasoningStep {
                step_number: step,
                focus_word: next_word,
                phase_angle: current_phase,
                phase_delta: normalized_delta,
                sentence: step_text,
            });

            prev_phase = current_phase;
        }

        let final_words: Vec<String> = steps.iter().map(|s| s.focus_word.clone()).collect();
        let final_answer = if final_words.len() >= 3 {
            format!(
                "{} relates to {}, which connects to {}.",
                final_words[0], final_words[1], final_words[2]
            )
        } else if final_words.len() == 2 {
            format!("{} relates to {}.", final_words[0], final_words[1])
        } else if final_words.len() == 1 {
            format!("The answer centers on {}.", final_words[0])
        } else {
            "No clear answer found.".to_string()
        };

        ReasoningChain {
            problem: problem.to_string(),
            steps,
            converged,
            final_answer,
        }
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
