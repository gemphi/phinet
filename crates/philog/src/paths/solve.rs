#[allow(unused_imports)]
use super::*;

impl MultiPath {
    /// Solves with a specific effort level.
    pub fn solve_with_depth(facet: &Facet, problem: &str, effort: EffortLevel) -> ReasoningChain {
        let max_steps = effort.max_steps();
        let engine = ReasoningEngine;

        if effort.n_paths() > 1 {
            let paths = Self::solve_multi_path(facet, problem, effort.n_paths());
            Self::best_path(&paths).cloned().unwrap_or_else(|| engine.solve(facet, problem))
        } else {
            Self::solve_with_limit(facet, problem, max_steps)
        }
    }

    /// Explores n different reasoning paths from different starting sectors.
    pub fn solve_multi_path(facet: &Facet, problem: &str, n_paths: usize) -> Vec<ReasoningChain> {
        let engine = ReasoningEngine;
        let mut paths = Vec::with_capacity(n_paths);

        let sector_offset = TWO_PI / n_paths as f64;

        for path_idx in 0..n_paths {
            let chain = if path_idx == 0 {
                engine.solve(facet, problem)
            } else {
                Self::solve_from_sector(facet, problem, sector_offset * path_idx as f64)
            };
            paths.push(chain);
        }

        paths
    }

    /// Returns the best path by confidence score.
    pub fn best_path(paths: &[ReasoningChain]) -> Option<&ReasoningChain> {
        paths.iter().max_by(|a, b| {
            let ca = Diagnostic::confidence(a);
            let cb = Diagnostic::confidence(b);
            ca.partial_cmp(&cb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Solves with a custom step limit.
    pub(super) fn solve_with_limit(facet: &Facet, problem: &str, max_steps: usize) -> ReasoningChain {
    let mut context_buffer = ContextWaveBuffer::new(4096);
    context_buffer.push_turn(facet, problem);

    let mut steps = Vec::new();
    let mut prev_phase = context_buffer.context_phase();
    let mut converged = false;

    let tokens = Tokenizer::tokenize(problem);
    let mut visited: std::collections::HashSet<String> = tokens.iter().cloned().collect();

    for step in 1..=max_steps {
        let current_phase = context_buffer.context_phase();
        let phase_delta = ((current_phase - prev_phase).abs()).rem_euclid(TWO_PI);
        let normalized_delta = if phase_delta > PI { TWO_PI - phase_delta } else { phase_delta };

        if step > 1 && normalized_delta < 0.01 {
            converged = true;
            break;
        }

        let mut candidates: Vec<(String, f64)> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| !visited.contains(*w))
            .map(|(w, p)| {
                let mut diff = (p.phase - current_phase).abs();
                if diff > PI { diff = TWO_PI - diff; }
                (w.clone(), diff)
            })
            .collect();
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        if candidates.is_empty() { break; }

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

    let words: Vec<String> = steps.iter().map(|s| s.focus_word.clone()).collect();
    let final_answer = if words.len() >= 3 {
        format!("{} relates to {}, which connects to {}.", words[0], words[1], words[2])
    } else if words.len() == 2 {
        format!("{} relates to {}.", words[0], words[1])
    } else if words.len() == 1 {
        format!("The answer centers on {}.", words[0])
    } else {
        "No clear answer found.".to_string()
    };

        ReasoningChain { problem: problem.to_string(), steps, converged, final_answer }
    }

    /// Solves starting from a specific sector offset.
    pub(super) fn solve_from_sector(facet: &Facet, problem: &str, phase_offset: f64) -> ReasoningChain {
    let mut context_buffer = ContextWaveBuffer::new(4096);
    context_buffer.push_turn(facet, problem);

    let mut steps = Vec::new();
    let start_phase = (context_buffer.context_phase() + phase_offset).rem_euclid(TWO_PI);
    let mut prev_phase = start_phase;
    let mut converged = false;

    let tokens = Tokenizer::tokenize(problem);
    let mut visited: std::collections::HashSet<String> = tokens.iter().cloned().collect();

    for step in 1..=REASONING_MAX_STEPS {
        let current_phase = context_buffer.context_phase();
        let phase_delta = ((current_phase - prev_phase).abs()).rem_euclid(TWO_PI);
        let normalized_delta = if phase_delta > PI { TWO_PI - phase_delta } else { phase_delta };

        if step > 1 && normalized_delta < 0.01 {
            converged = true;
            break;
        }

        let target_phase = (start_phase + step as f64 * 0.15).rem_euclid(TWO_PI);

        let mut candidates: Vec<(String, f64)> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| !visited.contains(*w))
            .map(|(w, p)| {
                let mut diff = (p.phase - target_phase).abs();
                if diff > PI { diff = TWO_PI - diff; }
                (w.clone(), diff)
            })
            .collect();
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        if candidates.is_empty() { break; }

        let (next_word, _) = candidates[0].clone();
        visited.insert(next_word.clone());

        let step_text = format!("Step {}: explore {} from sector {:.1}", step, next_word, phase_offset);
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

    let words: Vec<String> = steps.iter().map(|s| s.focus_word.clone()).collect();
    let final_answer = if words.len() >= 2 {
        format!("{} connects to {}.", words[0], words[1])
    } else if words.len() == 1 {
        format!("Path focuses on {}.", words[0])
    } else {
        "No path found.".to_string()
    };

        ReasoningChain { problem: problem.to_string(), steps, converged, final_answer }
    }
}
