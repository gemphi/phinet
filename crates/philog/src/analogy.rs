//! Value-centric analogy: compares words by continuous phase similarity.
//! This is the kind of abstraction deep learning already handles well (Ch 14.4).

use phiton::units::TWO_PI;
use phiecs::phasor::facet::Facet;
use phiton::physics::PhysicsOp;
use crate::program::ProgramAnalogy;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize)]
pub struct AnalogyResult {
    pub source: String,
    pub target: String,
    pub value_score: f64,
    pub program_score: f64,
    pub combined: f64,
}

#[derive(Debug, Default)]
pub struct Analogy;

impl Analogy {
    /// Compares two words by continuous phase similarity (value-centric analogy).
    pub fn value_centric(facet: &Facet, source: &str, target: &str) -> AnalogyResult {
        let value_score = match (facet.lexicon.get(source), facet.lexicon.get(target)) {
            (Some(p1), Some(p2)) => {
                let mut diff = (p1.phase - p2.phase).abs();
                if diff > PI {
                    diff = TWO_PI - diff;
                }
                1.0 - diff / PI
            }
            _ => 0.0,
        };

        let program_score = ProgramAnalogy::compare(facet, source, target);

        AnalogyResult {
            source: source.to_string(),
            target: target.to_string(),
            value_score,
            program_score: program_score.program_score,
            combined: Self::combine(value_score, program_score.program_score),
        }
    }

    /// Finds top-n words that are analogous to the given word in phase space.
    pub fn find(facet: &Facet, word: &str, n: usize) -> Vec<(String, f64)> {
        let target_phase = match facet.lexicon.get(word) {
            Some(p) => p.phase,
            None => return Vec::new(),
        };

        let mut candidates: Vec<(String, f64)> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| *w != word)
            .map(|(w, p)| {
                let mut diff = (p.phase - target_phase).abs();
                if diff > PI {
                    diff = TWO_PI - diff;
                }
                (w.clone(), 1.0 - diff / PI)
            })
            .collect();

        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(n);
        candidates
    }

    /// Weighted combination of value-centric and program-centric analogy scores.
    pub fn combine(value: f64, program: f64) -> f64 {
        0.5 * value + 0.5 * program
    }

    /// Light-centric analogy: compares words by phical interference.
    ///
    /// Uses the 2-in-1 phiton/chiton architecture to compute analogy
    /// through physical light interference. Words that constructively
    /// interfere in the color-space-time manifold are analogous.
    pub fn light_centric(facet: &Facet, source: &str, target: &str) -> f64 {
        match (facet.lexicon.get(source), facet.lexicon.get(target)) {
            (Some(p1), Some(p2)) => {
                let interference = PhysicsOp::interference(
                    p1.phase, p1.band_n,
                    p2.phase, p2.band_n,
                );
                let coupling = facet.word_coupling(source, target);
                (interference * coupling).clamp(0.0, 1.0)
            }
            _ => 0.0,
        }
    }
}

