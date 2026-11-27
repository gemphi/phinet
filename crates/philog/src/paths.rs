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
use crate::eval::Diagnostic;
use crate::pathfinding::{ReasoningChain, ReasoningEngine, REASONING_MAX_STEPS};
use serde::Serialize;

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
