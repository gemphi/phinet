//! Physics — physics-calculus abstraction of color space and time.
//!
//! Unifies the phiton (light particle) and chiton (light wave) into a
//! single mathematical framework that describes both color space and
//! temporal dynamics. This is where the fine-structure constant α
//! "shines" — it governs the quantum oscillations that connect
//! wavelength to phase to time.
//!
//! # Key Insight
//!
//! Light is governed by quantum oscillations. The fine-structure
//! constant α ≈ 1/137 controls the coupling between:
//! - **Color space**: phase → wavelength → spectral band → color
//! - **Time**: frequency → period → angular frequency → oscillation
//!
//! Physics provides the operators that let the linguistic model treat
//! words as standing waves in a coupled color-space-time manifold, making
//! intelligence plausible through natural linguistic functions.
//!
//! # Architecture
//!
//! ```text
//!   phiton (roll)  ──┐
//!                    ├── phical ─── word mapping (gemgum)
//!   chiton (wave)  ──┘
//! ```

mod phase;
#[cfg(test)]
mod tests;
#[allow(unused_imports)]
use phase::*;

pub mod evaluate;
pub mod manifold;
pub mod operators;
pub mod standing;
pub mod topology;

#[allow(unused_imports)]
pub use evaluate::Topology;
pub use manifold::ColorSpaceTimeManifold;
pub use operators::PhysicsOp;
#[allow(unused_imports)]
pub use topology::{Edge, Path, Region, Surface, Vertex};

use crate::units::{ALPHA, PHI};
use serde::{Deserialize, Serialize};

/// Physics — the physics-calculus engine for color-space-time abstraction.
///
/// Provides the mathematical bridge between the spectral domain
/// (wavelengths, colors, bands) and the temporal domain (frequencies,
/// periods, oscillations). Both are governed by the fine-structure
/// constant α and the golden ratio φ.
pub struct Physics;

/// A point in the color-space-time manifold.
///
/// Each word maps to a point in this 4D space: (wavelength, frequency,
/// phase, band_n). The manifold is continuous and differentiable,
/// enabling gradient-based optimization for the linguistic model.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorSpaceTimePoint {
    /// Wavelength (nm) — the color dimension.
    pub wavelength_nm: f64,
    /// Frequency (THz) — the time dimension.
    pub frequency_thz: f64,
    /// Effective phase [0, 2π) — the phase dimension.
    pub effective_phase: f64,
    /// Sub-band energy level — the quantum dimension.
    pub band_n: u32,
}
