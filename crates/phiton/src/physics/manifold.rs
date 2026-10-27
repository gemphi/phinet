//! color-space-time manifold — the geometric space where words live.
//!
//! Words are mapped onto a continuous manifold that unifies color
//! (wavelength) and time (frequency). The manifold is a torus T²
//! parameterized by (phase, band_n×α), where the fine-structure
//! constant α provides the quantum sub-band spacing.
//!
//! The manifold supports:
//! - **Geodesic distance**: shortest path between two words in color-space-time
//! - **Gradient flow**: direction of steepest semantic change
//! - **Standing wave nodes**: stable equilibrium positions (word meanings)

use super::ColorSpaceTimePoint;
use crate::units::{ALPHA, PHI, TWO_PI};
use std::f64::consts::PI;

/// The color-space-time manifold T².
///
/// A 2-torus where:
/// - Axis 1: phase angle θ ∈ [0, 2π) — the color wheel
/// - Axis 2: sub-band energy n·α — the quantum spectrum
///
/// Words occupy positions on this torus. Their semantic similarity
/// is measured by geodesic distance on the manifold surface.
pub struct ColorSpaceTimeManifold;

impl ColorSpaceTimeManifold {
    /// Computes the geodesic distance between two points on the manifold.
    ///
    /// The geodesic accounts for the toroidal topology (phase wraps
    /// around at 2π) and the golden-ratio-weighted sub-band spacing.
    pub fn geodesic(a: &ColorSpaceTimePoint, b: &ColorSpaceTimePoint) -> f64 {
        let mut phase_delta = (a.effective_phase - b.effective_phase).abs();
        if phase_delta > PI {
            phase_delta = TWO_PI - phase_delta;
        }

        let band_delta = (a.band_n as f64 - b.band_n as f64) * ALPHA;

        (phase_delta * phase_delta + PHI * band_delta * band_delta).sqrt()
    }

    /// Computes the gradient direction from point a toward point b.
    ///
    /// Returns a unit vector (dθ, dn) in the manifold's tangent space,
    /// pointing in the direction of steepest descent of the geodesic
    /// distance. Used for Kuramoto-style phase relaxation.
    pub fn gradient(a: &ColorSpaceTimePoint, b: &ColorSpaceTimePoint) -> (f64, f64) {
        let mut d_phase = b.effective_phase - a.effective_phase;
        if d_phase > PI { d_phase -= TWO_PI; }
        if d_phase < -PI { d_phase += TWO_PI; }

        let d_band = (b.band_n as f64 - a.band_n as f64) * ALPHA;

        let norm = (d_phase * d_phase + d_band * d_band).sqrt();
        if norm < 1e-12 {
            return (0.0, 0.0);
        }
        (d_phase / norm, d_band / norm)
    }

    /// Finds the standing wave nodes (stable equilibrium positions).
    ///
    /// Nodes occur where the phiton (roll) and chiton (wave) interfere
    /// constructively. These are the positions where words settle into
    /// stable meanings — the "attractors" of the linguistic model.
    pub fn is_standing_wave_node(point: &ColorSpaceTimePoint) -> bool {
        // The previous version sampled the standing wave at x = 0, t = 0,
        // where cos(0)·cos(0) = 1 for every wavelength, so every point was a
        // node. A node of the *manifold* is a phase that lands on the golden-
        // angle lattice `PhysicsOp::bind_to_node` snaps to: the point is a
        // node when snapping does not move it by more than one channel
        // quantum (2π/256).
        let snapped = super::PhysicsOp::bind_to_node(point.effective_phase, point.band_n);
        let mut d = (snapped - point.effective_phase).abs();
        if d > PI {
            d = TWO_PI - d;
        }
        d <= TWO_PI / crate::units::CHANNEL_QUANTA
    }

    /// Computes the manifold curvature at a point.
    ///
    /// The curvature is determined by the fine-structure constant
    /// and the band level. Higher band levels create tighter curvature,
    /// corresponding to more specific/nuanced word meanings.
    pub fn curvature(point: &ColorSpaceTimePoint) -> f64 {
        ALPHA * (point.band_n as f64 + 1.0) / PHI
    }

    /// Maps a sector index to a color-space-time point.
    ///
    /// This is the primary entry point for connecting the phase-space
    /// sector model to the color-space-time manifold.
    pub fn sector_to_point(sector: u16, sector_count: u16, band_n: u32) -> ColorSpaceTimePoint {
        let phase = (sector as f64 / sector_count as f64) * TWO_PI;
        ColorSpaceTimePoint::from_phase(phase, band_n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_are_not_everywhere() {
        let on = ColorSpaceTimePoint::from_phase(phistd::GOLDEN_ANGLE * 2.0, 0);
        let off = ColorSpaceTimePoint::from_phase(phistd::GOLDEN_ANGLE * 2.0 + 0.3, 0);
        assert!(ColorSpaceTimeManifold::is_standing_wave_node(&on));
        assert!(!ColorSpaceTimeManifold::is_standing_wave_node(&off));
    }
}
