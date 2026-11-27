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
        crate::walk::walk(facet, problem, max_steps, crate::walk::Aim::Context)
    }

    /// Solves starting from a specific sector offset, so a second walk
    /// over the same problem goes another way.
    pub(super) fn solve_from_sector(facet: &Facet, problem: &str, phase_offset: f64) -> ReasoningChain {
        crate::walk::walk(facet, problem, REASONING_MAX_STEPS, crate::walk::Aim::Sector(phase_offset))
    }
}
