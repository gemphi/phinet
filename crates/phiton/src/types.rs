/// Core phiton types — light quanta and physics-derived colors.

use super::bands::SpectralBand;
use crate::units::PHI;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A physics-derived color from the electromagnetic spectrum.
///
/// Unlike hard-coded color names, this color is computed from a wavelength
/// using the fine-structure constant for quantum sub-band corrections.
/// The color carries both a human-readable name and an RGB approximation.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct PhitonColor {
    /// Wavelength in nanometers [380, 750].
    pub wavelength_nm: f64,
    /// The spectral band this color belongs to.
    pub band: SpectralBand,
    /// RGB approximation (0-255 each).
    pub rgb: (u8, u8, u8),
}

impl PhitonColor {
    /// Creates a color from a wavelength and pre-computed band.
    pub fn new(wavelength_nm: f64, band: SpectralBand) -> Self {
        let rgb = wavelength_to_rgb(wavelength_nm);
        Self { wavelength_nm, band, rgb }
    }

    /// Returns the human-readable color name.
    pub fn name(&self) -> &'static str {
        self.band.name
    }

    /// Returns the hex color string (e.g. "#FF5733").
    pub fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.rgb.0, self.rgb.1, self.rgb.2)
    }
}

impl fmt::Display for PhitonColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.band.name)
    }
}

/// A discrete light quantum — the "roll" (particle) aspect of light.
///
/// Represents light as a photon with wavelength, phase, and sub-band
/// energy level. The effective phase includes the fine-structure
/// correction: φ_eff = φ + n·α.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightQuantum {
    /// Phase angle on the unit circle [0, 2π).
    pub phase: f64,
    /// Amplitude / intensity.
    pub amplitude: f64,
    /// Quantized fine-structure energy sub-band level.
    pub band_n: u32,
    /// Wavelength in nanometers.
    pub wavelength_nm: f64,
}

impl LightQuantum {
    /// Creates a light quantum from phase, amplitude, and band level.
    pub fn from_phase(phase: f64, amplitude: f64, band_n: u32) -> Self {
        let wavelength_nm = super::Phiton::phase_to_wavelength(phase, band_n);
        Self { phase, amplitude, band_n, wavelength_nm }
    }

    /// Returns the frequency in THz.
    pub fn frequency_thz(&self) -> f64 {
        super::Phiton::frequency_thz(self.wavelength_nm)
    }

    /// Returns the photon energy in eV.
    pub fn energy_ev(&self) -> f64 {
        super::Phiton::energy_ev(self.wavelength_nm)
    }

    /// Resolves this quantum to a [`PhitonColor`].
    pub fn to_color(&self) -> PhitonColor {
        let band = SpectralBand::from_wavelength(self.wavelength_nm);
        PhitonColor::new(self.wavelength_nm, band)
    }

    /// Golden-ratio-weighted color position within the band.
    ///
    /// Returns [0, 1) indicating where within the band this wavelength falls,
    /// weighted by the golden ratio conjugate for natural distribution.
    pub fn band_position(&self) -> f64 {
        let pos = (self.wavelength_nm - self.to_color().band.min_nm)
            / (self.to_color().band.max_nm - self.to_color().band.min_nm);
        (pos * (1.0 / PHI)).rem_euclid(1.0)
    }
}

/// Converts a wavelength (nm) to an approximate RGB triple.
///
/// Uses the standard piecewise approximation based on the CIE 1931
/// color matching functions. Accurate enough for visualization.
fn wavelength_to_rgb(wavelength: f64) -> (u8, u8, u8) {
    let (r, g, b) = match wavelength {
        w if w >= 380.0 && w < 440.0 => {
            (-(w - 440.0) / (440.0 - 380.0), 0.0, 1.0)
        }
        w if w >= 440.0 && w < 490.0 => {
            (0.0, (w - 440.0) / (490.0 - 440.0), 1.0)
        }
        w if w >= 490.0 && w < 510.0 => {
            (0.0, 1.0, -(w - 510.0) / (510.0 - 490.0))
        }
        w if w >= 510.0 && w < 580.0 => {
            ((w - 510.0) / (580.0 - 510.0), 1.0, 0.0)
        }
        w if w >= 580.0 && w < 645.0 => {
            (1.0, -(w - 645.0) / (645.0 - 580.0), 0.0)
        }
        w if w >= 645.0 && w <= 750.0 => {
            (1.0, 0.0, 0.0)
        }
        _ => (0.0, 0.0, 0.0),
    };

    let factor = match wavelength {
        w if w >= 380.0 && w < 420.0 => 0.3 + 0.7 * (w - 380.0) / (420.0 - 380.0),
        w if w >= 420.0 && w < 700.0 => 1.0,
        w if w >= 700.0 && w <= 750.0 => 0.3 + 0.7 * (750.0 - w) / (750.0 - 700.0),
        _ => 0.0,
    };

    let to_byte = |c: f64| -> u8 {
        ((c * factor).clamp(0.0, 1.0) * 255.0).round() as u8
    };
    (to_byte(r), to_byte(g), to_byte(b))
}
