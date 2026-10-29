//! Multi-path reasoning: explores multiple reasoning paths from different sectors.
//! Also provides depth-controlled reasoning with effort levels.

mod effort;
mod solve;
#[allow(unused_imports)]
use effort::*;
#[allow(unused_imports)]
use solve::*;

use phiton::units::TWO_PI;
use phiecs::phasor::facet::Facet;
use phient::generate::ContextWaveBuffer;
use crate::eval::Diagnostic;
use crate::pathfinding::{ReasoningChain, ReasoningEngine, ReasoningStep, REASONING_MAX_STEPS};
use phiton::p64::words::Tokenizer;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum EffortLevel {
    Instant,
    Quick,
    Standard,
    Deep,
    Exhaustive,
}

#[derive(Debug, Default)]
pub struct MultiPath;
