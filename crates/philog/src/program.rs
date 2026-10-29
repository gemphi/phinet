//! Program-centric analogy: compares structural relationships between words.
//! Supports reasoning, planning, formal manipulation (Ch 14.4).

use phiton::units::TWO_PI;
use phiecs::phasor::facet::Facet;
use phiton::p64::words::Tokenizer;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize)]
pub struct PhaseRelation {
    pub from_phase: f64,
    pub to_phase: f64,
    pub delta: f64,
    pub sector_transition: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgramAnalogyResult {
    pub source: String,
    pub target: String,
    pub program_score: f64,
    pub shared_relations: usize,
}

#[derive(Debug, Default)]
pub struct ProgramAnalogy;

impl ProgramAnalogy {
    /// Extracts phase relations between consecutive words in a sentence.
    pub fn extract_structure(facet: &Facet, sentence: &str) -> Vec<PhaseRelation> {
        let tokens = Tokenizer::tokenize(sentence);
        let mut relations = Vec::new();

        for window in tokens.windows(2) {
            if let (Some(p1), Some(p2)) = (facet.lexicon.get(&window[0]), facet.lexicon.get(&window[1])) {
                let mut delta = (p2.phase - p1.phase).abs();
                if delta > PI {
                    delta = TWO_PI - delta;
                }
                let sector_width = TWO_PI / 64.0;
                let s1 = (p1.phase / sector_width).floor() as u16;
                let s2 = (p2.phase / sector_width).floor() as u16;
                relations.push(PhaseRelation {
                    from_phase: p1.phase,
                    to_phase: p2.phase,
                    delta,
                    sector_transition: ((s2 as i16) - (s1 as i16)).unsigned_abs(),
                });
            }
        }

        relations
    }

    /// Compares structural relationships between two words via their bigram patterns.
    pub fn compare(facet: &Facet, source: &str, target: &str) -> ProgramAnalogyResult {
        let source_rels = Self::extract_structure(facet, source);
        let target_rels = Self::extract_structure(facet, target);

        if source_rels.is_empty() || target_rels.is_empty() {
            return ProgramAnalogyResult {
                source: source.to_string(),
                target: target.to_string(),
                program_score: 0.0,
                shared_relations: 0,
            };
        }

        let mut shared = 0;
        for sr in &source_rels {
            for tr in &target_rels {
                if (sr.delta - tr.delta).abs() < 0.1 && sr.sector_transition == tr.sector_transition {
                    shared += 1;
                    break;
                }
            }
        }

        let score = shared as f64 / source_rels.len().max(target_rels.len()) as f64;

        ProgramAnalogyResult {
            source: source.to_string(),
            target: target.to_string(),
            program_score: score,
            shared_relations: shared,
        }
    }

    /// Detects if two sentences share the same structural form.
    pub fn shares_structure(facet: &Facet, s1: &str, s2: &str) -> bool {
        let r1 = Self::extract_structure(facet, s1);
        let r2 = Self::extract_structure(facet, s2);

        if r1.len() != r2.len() {
            return false;
        }

        for (a, b) in r1.iter().zip(r2.iter()) {
            if (a.delta - b.delta).abs() > 0.2 || a.sector_transition != b.sector_transition {
                return false;
            }
        }

        true
    }
}
