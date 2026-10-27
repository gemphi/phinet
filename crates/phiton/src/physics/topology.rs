//! Topology of the color-space-time manifold — abstract shapes for
//! linguistic building blocks.
//!
//! Words, phrases, sentences, stories, and concepts are not just points
//! on the manifold — they have **shape**. This module defines the
//! topological structures that emerge when linguistic units are mapped
//! onto the color-space-time manifold as graphs.
//!
//! # Building Blocks
//!
//! ```text
//!   Word       → Vertex (0-simplex)     — a single manifold point
//!   Phrase     → Edge (1-simplex)       — two words + their coupling
//!   Sentence   → Path (1-chain)         — ordered sequence of edges
//!   Story      → Surface (2-complex)    — network of connected sentences
//!   Concept    → Manifold region        — attractor basin of related words
//! ```
//!
//! Each shape is evaluated without data concreteness — we understand
//! the building blocks through their geometric relationships on the
//! manifold, not through specific word content.

mod vertex;
#[allow(unused_imports)]
use vertex::*;

use super::ColorSpaceTimePoint;
use super::ColorSpaceTimeManifold;
use crate::units::{PHI, TWO_PI};
use crate::chiton::Chiton;
use crate::types::LightQuantum;
use serde::Serialize;
use std::f64::consts::PI;

/// A vertex — the simplest topological shape: a single word on the manifold.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Vertex {
    pub point: ColorSpaceTimePoint,
    pub word_hash: u64,
}

/// An edge — connects two vertices. Represents a phrase (two-word relation).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Edge {
    pub from: Vertex,
    pub to: Vertex,
    /// Geodesic distance between the two vertices.
    pub length: f64,
    /// Coupling strength (interference × resonance).
    pub coupling: f64,
}

/// A path — ordered sequence of edges. Represents a sentence.
#[derive(Debug, Clone, Serialize)]
pub struct Path {
    pub edges: Vec<Edge>,
    /// Total path length (sum of edge geodesics).
    pub total_length: f64,
    /// Average coupling across all edges.
    pub coherence: f64,
    /// Phase winding number — how many times the path wraps around the manifold.
    pub winding: f64,
}

/// A surface — network of connected paths. Represents a story.
#[derive(Debug, Clone, Serialize)]
pub struct Surface {
    pub paths: Vec<Path>,
    /// Adjacency: which paths share vertices.
    pub adjacency: Vec<(usize, usize)>,
    /// Surface area — total semantic content.
    pub area: f64,
    /// Genus — topological complexity (number of "holes" in the story).
    pub genus: u32,
}

/// A region — an attractor basin of related vertices. Represents a concept.
#[derive(Debug, Clone, Serialize)]
pub struct Region {
    pub vertices: Vec<Vertex>,
    /// Centroid of the region in manifold space.
    pub centroid_phase: f64,
    /// Spread — how dispersed the concept is.
    pub spread: f64,
    /// Density — how tightly packed the vertices are.
    pub density: f64,
}
