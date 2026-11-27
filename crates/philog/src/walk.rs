//! One walk through the lexicon, and the ways of taking it.
//!
//! A walk steps from word to word by phase: at each step it takes the
//! unvisited word nearest what it is aiming at, tells that step to the
//! context buffer and records it, and stops when the context has stopped
//! moving. The engine's walk, the walk under an effort limit and the walk
//! swept from a sector were three copies of it, differing in where they
//! aim and in the words they say themselves in - which is all [`Aim`]
//! holds (docs/241's duplicate ratchet named them).

use crate::pathfinding::{REASONING_CONVERGENCE, ReasoningChain, ReasoningStep};
use phiecs::phasor::facet::Facet;
use phient::generate::ContextWaveBuffer;
use phiton::p64::words::Tokenizer;
use phiton::units::TWO_PI;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64::consts::PI;

/// How far round the circle a sector walk sweeps with each step.
const SWEEP: f64 = 0.15;

/// What a walk aims at, and the words it says itself in.
pub enum Aim {
    /// The context's own phase: the walk follows where it has got to.
    Context,
    /// A sector of the circle, started at an offset and swept from there,
    /// so a second walk over the same problem goes another way.
    Sector(f64),
}

impl Aim {
    /// The phase the walk starts from, the context having got to `at`.
    fn start(&self, at: f64) -> f64 {
        match self {
            Aim::Context => at,
            Aim::Sector(offset) => (at + offset).rem_euclid(TWO_PI),
        }
    }

    /// What step `step` aims at, the context having got to `at`.
    fn at(&self, start: f64, step: usize, at: f64) -> f64 {
        match self {
            Aim::Context => at,
            Aim::Sector(_) => (start + step as f64 * SWEEP).rem_euclid(TWO_PI),
        }
    }

    /// How the walk says a step it has just taken.
    fn said(&self, step: usize, word: &str) -> String {
        match self {
            Aim::Context => format!("Step {step}: resonate with {word}"),
            Aim::Sector(offset) => format!("Step {step}: explore {word} from sector {offset:.1}"),
        }
    }

    /// The answer the walk ends on, from the words it went through.
    fn answer(&self, words: &[String]) -> String {
        match (self, words.len()) {
            (Aim::Context, 0) => "No clear answer found.".to_string(),
            (Aim::Context, 1) => format!("The answer centers on {}.", words[0]),
            (Aim::Context, 2) => format!("{} relates to {}.", words[0], words[1]),
            (Aim::Context, _) => format!("{} relates to {}, which connects to {}.", words[0], words[1], words[2]),
            (Aim::Sector(_), 0) => "No path found.".to_string(),
            (Aim::Sector(_), 1) => format!("Path focuses on {}.", words[0]),
            (Aim::Sector(_), _) => format!("{} connects to {}.", words[0], words[1]),
        }
    }
}

/// Walks the lexicon from `problem`, at most `limit` steps, and answers
/// with the chain of them.
pub fn walk(facet: &Facet, problem: &str, limit: usize, aim: Aim) -> ReasoningChain {
    let mut context = ContextWaveBuffer::new(4096);
    context.push_turn(facet, problem);
    let start = aim.start(context.context_phase());
    let mut visited: HashSet<String> = Tokenizer::tokenize(problem).into_iter().collect();
    let (mut steps, mut prev, mut converged) = (Vec::new(), start, false);
    for step in 1..=limit {
        let at = context.context_phase();
        let moved = nearer(at - prev);
        // The wave has settled: the context is where it was a step ago.
        match step > 1 && moved < REASONING_CONVERGENCE {
            true => {
                converged = true;
                break;
            }
            false => {}
        }
        let Some(word) = nearest(facet, &visited, aim.at(start, step, at)) else {
            break;
        };
        visited.insert(word.clone());
        // A step is told to the buffer in the walk's own words, then
        // recorded: the rule the three walks each wrote out.
        let sentence = aim.said(step, &word);
        context.push_turn(facet, &sentence);
        steps.push(ReasoningStep {
            step_number: step,
            focus_word: word,
            phase_angle: at,
            phase_delta: moved,
            sentence,
        });
        prev = at;
    }
    let words: Vec<String> = steps.iter().map(|s| s.focus_word.clone()).collect();
    ReasoningChain {
        problem: problem.to_string(),
        final_answer: aim.answer(&words),
        steps,
        converged,
    }
}

/// The shorter way round the circle between two phases.
fn nearer(d: f64) -> f64 {
    let d = d.abs().rem_euclid(TWO_PI);
    match d > PI {
        true => TWO_PI - d,
        false => d,
    }
}

/// The unvisited word nearest `aim` on the circle, if the lexicon holds one.
fn nearest(facet: &Facet, visited: &HashSet<String>, aim: f64) -> Option<String> {
    facet
        .lexicon
        .iter()
        .filter(|(w, _)| !visited.contains(*w))
        .min_by(|a, b| nearer(a.1.phase - aim).partial_cmp(&nearer(b.1.phase - aim)).unwrap_or(Ordering::Equal))
        .map(|(w, _)| w.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A facet of words at phases a sixth of the circle apart.
    fn lexicon() -> Facet {
        let mut facet = Facet::new();
        for (i, w) in ["ownership", "borrowing", "lifetime", "checker", "move", "copy"].iter().enumerate() {
            facet.get_or_init(w).phase = i as f64 * TWO_PI / 6.0;
        }
        facet
    }

    #[test]
    fn the_shorter_way_round_is_never_over_half_the_circle() {
        assert!((nearer(3.0 * PI / 2.0) - PI / 2.0).abs() < 1e-9);
        assert!((nearer(-PI / 4.0) - PI / 4.0).abs() < 1e-9);
        assert!(nearer(TWO_PI * 3.0) < 1e-9);
    }

    #[test]
    fn a_walk_goes_through_a_word_once() {
        let chain = walk(&lexicon(), "ownership borrowing", 16, Aim::Context);
        let mut seen: Vec<&str> = chain.steps.iter().map(|s| s.focus_word.as_str()).collect();
        let took = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), took);
        assert!(!seen.contains(&"ownership"), "the problem's own words are where it started");
    }

    #[test]
    fn a_walk_asked_for_no_steps_takes_none() {
        let chain = walk(&lexicon(), "ownership", 0, Aim::Context);
        assert!(chain.steps.is_empty());
        assert_eq!(chain.final_answer, "No clear answer found.");
        assert_eq!(chain.problem, "ownership");
    }

    #[test]
    fn a_walk_says_itself_in_its_aims_words() {
        let facet = lexicon();
        let context = walk(&facet, "ownership", 2, Aim::Context);
        let sector = walk(&facet, "ownership", 2, Aim::Sector(PI));
        assert!(context.steps[0].sentence.starts_with("Step 1: resonate with "));
        assert!(sector.steps[0].sentence.starts_with("Step 1: explore "));
        assert!(sector.steps[0].sentence.ends_with(" from sector 3.1"));
        assert!(context.final_answer.contains(" relates to "));
        assert!(sector.final_answer.contains(" connects to "));
    }

    #[test]
    fn a_walk_stops_at_the_limit_it_was_given() {
        let chain = walk(&lexicon(), "ownership", 2, Aim::Context);
        assert!(chain.steps.len() <= 2);
    }
}
