//! Phiton — the particle/roll spin of light.
//!
//! Houses color types, spectral bands, and the electromagnetic spectrum
//! mapping that replaces hard-coded color names. Light is governed by
//! quantum oscillations of the fine-structure constant α ≈ 1/137.
//!
//! Phiton is the "roll" aspect of light — circular polarization, discrete
//! quanta, particle-like rotation. Its complement is [`chiton::Chiton`],
//! the "wave" aspect. Together they form a 2-in-1 architecture for
//! light's wave-particle duality, providing the physical basis for
//! linguistic word mapping and intelligence. Beside the chiton stands
//! [`conjugate::Mirror`], the phase-conjugate mirror: a relation as an
//! all-pass filter that returns a wave time-reversed (it was phical's).
//!
//! # Architecture
//!
//! ```text
//!   phase angle θ ──→ wavelength λ ──→ spectral band ──→ PhitonColor
//!        │                                              │
//!        │  + quantum(band_n)·α·10 nm (the shift)       │
//!        │                                              ▼
//!        │                                         gemgum coupling
//!        │                                              │
//!        ▼                                              ▼
//!   chiton (wave) ◄──── 2-in-1 ────►  phiton (roll) → word mapping
//!   frequency ν                                    color space
//!   period T
//! ```

pub mod bands;
pub mod chiton;
pub mod conjugate;
pub mod hrr;
pub mod p64;
pub mod physics;
pub mod spectrum;
pub mod types;
pub mod units;

pub use units::{effective_phase, quantum, ALPHA, BAND_N_EFFECTIVE_MAX, CHANNEL_QUANTA, PHASE_CHANNELS, PHI, PHI_CONJUGATE, TWO_PI};

#[allow(unused_imports)]
pub use bands::SpectralBand;
#[allow(unused_imports)]
pub use chiton::Chiton;
#[allow(unused_imports)]
pub use spectrum::PhitonSpectrum;
#[allow(unused_imports)]
pub use types::{LightQuantum, PhitonColor};


/// Phiton — the roll/particle spin of light.
///
/// Maps phase angles to wavelengths and spectral bands using the
/// fine-structure constant for quantum sub-band corrections.
pub struct Phiton;

impl Phiton {
    /// Speed of light in vacuum (m/s).
    pub const C: f64 = 299_792_458.0;

    /// Planck constant (J·s).
    pub const H: f64 = 6.626_070_15e-34;

    /// Visible spectrum lower bound (nm).
    pub const VISIBLE_MIN_NM: f64 = 380.0;

    /// Visible spectrum upper bound (nm).
    pub const VISIBLE_MAX_NM: f64 = 750.0;

    /// Maps a phase angle [0, 2π) to a wavelength in nanometers.
    ///
    /// The phase circle is mapped proportionally across the visible spectrum.
    /// The fine-structure constant α shifts the wavelength by sub-band energy
    /// level `band_n`, creating quantum spectral lines.
    pub fn phase_to_wavelength(phase: f64, band_n: u32) -> f64 {
        let normalized = phase.rem_euclid(TWO_PI) / TWO_PI;
        let base = Self::VISIBLE_MIN_NM + normalized * (Self::VISIBLE_MAX_NM - Self::VISIBLE_MIN_NM);
        // `band_n` is a convergence counter on the lexicon; the quantum
        // number is its clamp (see `Phical::quantum`). Without this the
        // shipped manifold's 1.8-million counts pushed every word to 750 nm.
        let quantum_shift = (crate::units::quantum(band_n) as f64 * ALPHA) * 10.0;
        (base + quantum_shift).min(Self::VISIBLE_MAX_NM).max(Self::VISIBLE_MIN_NM)
    }

    /// Maps a wavelength (nm) to a phase angle [0, 2π).
    pub fn wavelength_to_phase(wavelength_nm: f64) -> f64 {
        let clamped = wavelength_nm.clamp(Self::VISIBLE_MIN_NM, Self::VISIBLE_MAX_NM);
        let normalized = (clamped - Self::VISIBLE_MIN_NM) / (Self::VISIBLE_MAX_NM - Self::VISIBLE_MIN_NM);
        normalized * TWO_PI
    }

    /// Computes the frequency (THz) of a wavelength using ν = c/λ.
    pub fn frequency_thz(wavelength_nm: f64) -> f64 {
        let wavelength_m = wavelength_nm * 1e-9;
        (Self::C / wavelength_m) / 1e12
    }

    /// Computes the photon energy (eV) of a wavelength using E = hc/λ.
    pub fn energy_ev(wavelength_nm: f64) -> f64 {
        let wavelength_m = wavelength_nm * 1e-9;
        (Self::H * Self::C / wavelength_m) / 1.602_176_634e-19
    }

    /// Resolves a sector index to a [`PhitonColor`] using the full spectrum.
    ///
    /// This replaces the hard-coded color array in `SectorPalette`.
    pub fn sector_color(sector: u16, sector_count: u16, band_n: u32) -> PhitonColor {
        let normalized = (sector as f64 / sector_count as f64) * TWO_PI;
        let wavelength = Self::phase_to_wavelength(normalized, band_n);
        let band = SpectralBand::from_wavelength(wavelength);
        PhitonColor::new(wavelength, band)
    }
}

#[cfg(test)]
mod tests;
