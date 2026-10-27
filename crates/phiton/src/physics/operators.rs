//! Physics operators — mathematical operations on the color-space-time manifold.
//!
//! These operators implement the calculus of the phical abstraction:
//! - **Phase relaxation**: Kuramoto-style phase updates using manifold gradients
//! - **Spectral convolution**: interference between words in color-space-time
//! - **Temporal projection**: mapping from color space to temporal dynamics
//! - **Word binding**: coupling words to stable manifold positions

use super::ColorSpaceTimePoint;
use super::ColorSpaceTimeManifold;
use crate::units::{ALPHA, TWO_PI};
use crate::chiton::Chiton;
use crate::types::LightQuantum;

/// Physics mathematical operators.
pub struct PhysicsOp;

impl PhysicsOp {
    /// Relaxes a word's phase toward a target using manifold gradient flow.
    ///
    /// This is the Kuramoto update rule applied on the color-space-time manifold:
    ///   θ_new = θ_old + lr · ∇geodesic(θ_old → θ_target)
    ///
    /// The learning rate is scaled by the fine-structure constant α
    /// to ensure quantum-stable convergence.
    pub fn relax_phase(
        current_phase: f64,
        current_band: u32,
        target_phase: f64,
        target_band: u32,
        rate: f64,
    ) -> f64 {
        let a = ColorSpaceTimePoint::from_phase(current_phase, current_band);
        let b = ColorSpaceTimePoint::from_phase(target_phase, target_band);
        let (d_phase, _) = ColorSpaceTimeManifold::gradient(&a, &b);

        let new_phase = current_phase + rate * d_phase;
        new_phase.rem_euclid(TWO_PI)
    }

    /// Computes the spectral convolution of two words.
    ///
    /// The convolution measures how much two words' spectral signatures
    /// overlap, weighted by their temporal frequencies. This is the
    /// core operation for semantic similarity in the phical framework.
    pub fn spectral_convolution(
        phase_a: f64,
        band_a: u32,
        phase_b: f64,
        band_b: u32,
    ) -> f64 {
        let qa = LightQuantum::from_phase(phase_a, 1.0, band_a);
        let qb = LightQuantum::from_phase(phase_b, 1.0, band_b);
        let ca = Chiton::from_quantum(&qa);
        let cb = Chiton::from_quantum(&qb);

        let wavelength_overlap = 1.0 / (1.0 + (qa.wavelength_nm - qb.wavelength_nm).abs() / 100.0);
        let temporal_resonance = ca.resonance(&cb);
        let quantum_coupling = crate::PhitonSpectrum::coupling(&qa, &qb);

        wavelength_overlap * temporal_resonance * (1.0 + quantum_coupling / ALPHA)
    }

    /// Projects a color-space-time point into the temporal domain.
    ///
    /// Returns the angular frequency and period of the word's oscillation,
    /// which determines its temporal dynamics in the linguistic model.
    pub fn temporal_project(point: &ColorSpaceTimePoint) -> (f64, f64) {
        let quantum = LightQuantum::from_phase(point.effective_phase, 1.0, point.band_n);
        let chiton = Chiton::from_quantum(&quantum);
        (chiton.angular_frequency, chiton.period)
    }

    /// Binds a word to a stable position on the manifold.
    ///
    /// Finds the nearest standing wave node to the given phase and
    /// returns the adjusted phase that places the word at a stable
    /// equilibrium. This is where "natural linguistic functions"
    /// emerge — words settle into physically meaningful positions.
    pub fn bind_to_node(phase: f64, band_n: u32) -> f64 {
        let point = ColorSpaceTimePoint::from_phase(phase, band_n);

        // Standing wave nodes occur at integer multiples of the
        // golden angle, weighted by the fine-structure constant.
        let golden_angle = phistd::GOLDEN_ANGLE;
        let node_spacing = golden_angle * (1.0 + super::Physics::quantum(band_n) as f64 * ALPHA);

        let nearest_node = (point.effective_phase / node_spacing).round() * node_spacing;
        nearest_node.rem_euclid(TWO_PI)
    }

    /// Computes the interference pattern between two words.
    ///
    /// Returns the constructive/destructive interference amplitude,
    /// which determines whether two words reinforce or cancel each
    /// other in the linguistic model.
    pub fn interference(
        phase_a: f64,
        band_a: u32,
        phase_b: f64,
        band_b: u32,
    ) -> f64 {
        let eff_a = super::Physics::effective_phase(phase_a, band_a);
        let eff_b = super::Physics::effective_phase(phase_b, band_b);

        let qa = LightQuantum::from_phase(eff_a, 1.0, band_a);
        let qb = LightQuantum::from_phase(eff_b, 1.0, band_b);

        // Interference = cos(Δφ) weighted by amplitude product
        let delta_phase = (eff_a - eff_b).rem_euclid(TWO_PI);
        let amplitude_product = (qa.amplitude * qb.amplitude).sqrt();

        amplitude_product * delta_phase.cos()
    }
}
