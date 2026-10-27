//! Topology evaluation — assessing geometric properties of linguistic shapes.
//!
//! This is how we "evaluate what we've about these layers and their
//! relates" — by measuring the topological invariants of the shapes
//! that words, phrases, sentences, stories, and concepts form on the
//! color-space-time manifold.
//!
//! The evaluator works without data concreteness — it understands
//! building blocks through their geometric relationships, not through
//! specific word content.

use super::topology::{Edge, Path, Region, Surface, Vertex};
use crate::units::ALPHA;
use crate::physics::ColorSpaceTimePoint;

/// Topology evaluator — assesses the geometric properties of linguistic shapes.
pub struct Topology;

impl Topology {
    /// Evaluates a word as a vertex on the manifold.
    pub fn evaluate_word(point: ColorSpaceTimePoint, word_hash: u64) -> Vertex {
        Vertex::new(point, word_hash)
    }

    /// Evaluates a phrase (two-word relation) as an edge.
    pub fn evaluate_phrase(a: Vertex, b: Vertex) -> Edge {
        Edge::new(a, b)
    }

    /// Evaluates a sentence as a path through the manifold.
    pub fn evaluate_sentence(vertices: Vec<Vertex>) -> Path {
        let edges: Vec<Edge> = vertices.windows(2)
            .map(|w| Edge::new(w[0], w[1]))
            .collect();
        Path::from_edges(edges)
    }

    /// Evaluates a story as a surface (2-complex of connected sentences).
    pub fn evaluate_story(sentences: Vec<Path>) -> Surface {
        Surface::from_paths(sentences)
    }

    /// Evaluates a concept as a region (attractor basin of related words).
    pub fn evaluate_concept(vertices: Vec<Vertex>) -> Region {
        Region::from_vertices(vertices)
    }

    /// Computes the layer relationship between two topological shapes.
    ///
    /// Returns a score [0, 1] indicating how strongly two shapes
    /// are related on the manifold, regardless of their specific content.
    /// Uses coherence product, winding number difference, and length ratio.
    pub fn layer_relation(a: &Path, b: &Path) -> f64 {
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }
        let coherence_product = a.coherence * b.coherence;
        let winding_diff = (a.winding - b.winding).abs();
        let length_ratio = a.total_length.min(b.total_length)
            / a.total_length.max(b.total_length).max(1e-10);
        coherence_product * length_ratio / (1.0 + winding_diff * ALPHA)
    }

    /// Computes the structural complexity of a story (surface).
    ///
    /// Higher genus means more topological "holes" — more complex
    /// narrative structure with interwoven themes.
    pub fn story_complexity(surface: &Surface) -> u32 {
        surface.genus
    }

    /// Computes the semantic coherence of a sentence (path).
    ///
    /// High coherence means the words form a tight, well-coupled
    /// sequence on the manifold.
    pub fn sentence_coherence(path: &Path) -> f64 {
        path.coherence
    }

    /// Computes the conceptual density of a concept (region).
    ///
    /// Dense concepts have words tightly clustered on the manifold.
    pub fn concept_density(region: &Region) -> f64 {
        region.density
    }
}
