//! PhitonSpectrum — continuous spectral mapping across the phase circle.
//!
//! Provides the bridge between the phase-space model (sectors, phases)
//! and the physical light spectrum (wavelengths, frequencies, energies).
//! The spectrum is continuous, not discrete — any phase angle maps to
//! a precise wavelength, and the fine-structure constant provides
//! quantum sub-band corrections.

use super::bands::SpectralBand;
use super::types::{LightQuantum, PhitonColor};
use crate::units::{ALPHA, PHI, TWO_PI};

/// Continuous spectrum mapper — maps phase angles to the light spectrum.
pub struct PhitonSpectrum;

impl PhitonSpectrum {
    /// Maps a sector index to a [`PhitonColor`] for a given sector resolution.
    ///
    /// This is the primary replacement for the hard-coded color array.
    /// The sector is mapped proportionally across the full phase circle,
    /// then converted to a wavelength with fine-structure correction.
    pub fn sector_to_color(sector: u16, sector_count: u16, band_n: u32) -> PhitonColor {
        super::Phiton::sector_color(sector, sector_count, band_n)
    }

    /// Maps a raw phase angle [0, 2π) to a [`PhitonColor`].
    pub fn phase_to_color(phase: f64, band_n: u32) -> PhitonColor {
        let wavelength = super::Phiton::phase_to_wavelength(phase, band_n);
        let band = SpectralBand::from_wavelength(wavelength);
        PhitonColor::new(wavelength, band)
    }

    /// Returns all 16 band colors for a given sector resolution.
    ///
    /// Useful for rendering legends, UI palettes, or debugging.
    pub fn band_colors(sector_count: u16) -> Vec<PhitonColor> {
        let color_count = SpectralBand::BANDS.len() as u32;
        (0..color_count)
            .map(|i| {
                // In u32: `i * sector_count` passes u16::MAX once the count is above 4,369.
                let sector = (i * sector_count as u32 / color_count) as u16;
                Self::sector_to_color(sector, sector_count, 0)
            })
            .collect()
    }

    /// Computes the spectral distance between two phase angles.
    ///
    /// Returns the wavelength difference in nm, `|λa − λb|`, each wavelength
    /// carrying its sub-band shift. The spectrum is a line here, not a
    /// circle: violet and crimson are 370 nm apart, not neighbours.
    pub fn spectral_distance(phase_a: f64, band_a: u32, phase_b: f64, band_b: u32) -> f64 {
        let wa = super::Phiton::phase_to_wavelength(phase_a, band_a);
        let wb = super::Phiton::phase_to_wavelength(phase_b, band_b);
        (wa - wb).abs()
    }

    /// Computes the quantum coupling strength between two light quanta.
    ///
    /// Uses the fine-structure constant α as the coupling constant and
    /// the golden ratio φ for natural distance weighting:
    ///   coupling = α / (1 + φ · Δλ / 100), Δλ in nm
    pub fn coupling(a: &LightQuantum, b: &LightQuantum) -> f64 {
        let delta_lambda = (a.wavelength_nm - b.wavelength_nm).abs();
        ALPHA / (1.0 + PHI * delta_lambda / 100.0)
    }

    /// Returns the antipodal (complementary) color for a phase angle.
    ///
    /// The antipodal phase is φ + π, which maps to the opposite side
    /// of the color wheel (e.g. red ↔ green, blue ↔ orange).
    pub fn antipodal_color(phase: f64, band_n: u32) -> PhitonColor {
        let opposite_phase = (phase + std::f64::consts::PI).rem_euclid(TWO_PI);
        Self::phase_to_color(opposite_phase, band_n)
    }
}
