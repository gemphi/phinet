//! Abstraction extraction: finds common phase patterns across examples.
//! Implements Ch 14.4's abstraction through analogy-making.

use phiton::units::TWO_PI;
use phiecs::phasor::facet::Facet;
use phiton::p64::words::Tokenizer;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize)]
pub struct Abstraction {
    pub centroid_phase: f64,
    pub common_relations: Vec<f64>,
    pub member_words: Vec<String>,
    pub coherence: f64,
}

#[derive(Debug, Default)]
pub struct AbstractionExtractor;

impl AbstractionExtractor {
    /// Extracts a common abstraction from multiple example sentences.
    /// Finds the centroid phase and common phase deltas.
    pub fn extract(facet: &Facet, examples: &[String]) -> Abstraction {
        if examples.is_empty() {
            return Abstraction {
                centroid_phase: 0.0,
                common_relations: Vec::new(),
                member_words: Vec::new(),
                coherence: 0.0,
            };
        }

        let mut all_phases: Vec<f64> = Vec::new();
        let mut all_deltas: Vec<Vec<f64>> = Vec::new();
        let mut member_words = Vec::new();

        for example in examples {
            let tokens = Tokenizer::tokenize(example);
            let mut example_phases = Vec::new();

            for token in &tokens {
                if let Some(p) = facet.lexicon.get(token) {
                    all_phases.push(p.phase);
                    example_phases.push(p.phase);
                    if !member_words.contains(token) {
                        member_words.push(token.clone());
                    }
                }
            }

            let mut deltas = Vec::new();
            for i in 1..example_phases.len() {
                let mut d = (example_phases[i] - example_phases[i - 1]).abs();
                if d > PI {
                    d = TWO_PI - d;
                }
                deltas.push(d);
            }
            all_deltas.push(deltas);
        }

        let centroid_phase = if all_phases.is_empty() {
            0.0
        } else {
            let sum_x: f64 = all_phases.iter().map(|&p| p.cos()).sum();
            let sum_y: f64 = all_phases.iter().map(|&p| p.sin()).sum();
            let phase = sum_y.atan2(sum_x);
            if phase < 0.0 { phase + TWO_PI } else { phase }
        };

        let common_relations = Self::find_common_deltas(&all_deltas);

        let coherence = if all_phases.is_empty() {
            0.0
        } else {
            let mut total_align = 0.0;
            for &p in &all_phases {
                let mut diff = (p - centroid_phase).abs();
                if diff > PI {
                    diff = TWO_PI - diff;
                }
                total_align += 1.0 - diff / PI;
            }
            total_align / all_phases.len() as f64
        };

        Abstraction {
            centroid_phase,
            common_relations,
            member_words,
            coherence,
        }
    }

    /// Finds phase deltas that appear across multiple examples.
    fn find_common_deltas(all_deltas: &[Vec<f64>]) -> Vec<f64> {
        if all_deltas.is_empty() {
            return Vec::new();
        }

        let mut common: Vec<f64> = all_deltas[0].clone();
        for deltas in all_deltas.iter().skip(1) {
            common.retain(|c| {
                deltas.iter().any(|d| (c - d).abs() < 0.15)
            });
        }
        common
    }
}
