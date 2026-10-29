//! Multi-step planning: uses phase-space traversal to plan sub-goals.
//! Each step moves the context wave toward the goal phase.

use phiton::units::TWO_PI;
use phiecs::phasor::facet::Facet;
use phient::generate::ContextWaveBuffer;
use phiton::p64::words::Tokenizer;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize)]
pub struct PlanStep {
    pub step: usize,
    pub sub_goal: String,
    pub target_phase: f64,
    pub current_phase: f64,
    pub phase_delta: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Plan {
    pub steps: Vec<PlanStep>,
    pub goal: String,
    pub goal_phase: f64,
    pub converged: bool,
}

#[derive(Debug, Default)]
pub struct Planner;

impl Planner {
    /// Plans a sequence of sub-goals to reach the goal's phase.
    pub fn plan(facet: &Facet, goal: &str, max_steps: usize) -> Plan {
    let mut context_buffer = ContextWaveBuffer::new(4096);
    context_buffer.push_turn(facet, goal);

    let goal_tokens = Tokenizer::tokenize(goal);
    let mut goal_phase = context_buffer.context_phase();
    for token in &goal_tokens {
        if let Some(p) = facet.lexicon.get(token) {
            goal_phase = p.phase;
            break;
        }
    }

    let mut steps = Vec::new();
    let mut visited = std::collections::HashSet::new();
    for t in &goal_tokens {
        visited.insert(t.clone());
    }

    let mut current_phase = context_buffer.context_phase();
    let mut converged = false;

    for step in 0..max_steps {
        let mut delta = (goal_phase - current_phase).abs();
        if delta > PI {
            delta = TWO_PI - delta;
        }

        if delta < 0.1 {
            converged = true;
            break;
        }

        let target_phase = current_phase + (goal_phase - current_phase) * 0.3;

        let mut candidates: Vec<(String, f64)> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| !visited.contains(*w))
            .map(|(w, p)| {
                let mut d = (p.phase - target_phase).abs();
                if d > PI {
                    d = TWO_PI - d;
                }
                (w.clone(), d)
            })
            .collect();
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        if candidates.is_empty() {
            break;
        }

        let (next_word, _) = candidates[0].clone();
        visited.insert(next_word.clone());

        steps.push(PlanStep {
            step,
            sub_goal: next_word.clone(),
            target_phase,
            current_phase,
            phase_delta: delta,
        });

        context_buffer.push_turn(facet, &next_word);
        current_phase = context_buffer.context_phase();
    }

    Plan {
        steps,
        goal: goal.to_string(),
        goal_phase,
        converged,
    }
    }
}
