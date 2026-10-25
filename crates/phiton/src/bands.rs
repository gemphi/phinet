/// Spectral bands — discrete wavelength regions of the visible spectrum.
///
/// Each band corresponds to a range of wavelengths and has a human-readable
/// name. The 16 bands map proportionally to the phase circle, replacing
/// the hard-coded color array.

use serde::Serialize;

/// A spectral band within the visible electromagnetic spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct SpectralBand {
    /// Minimum wavelength (nm) of this band.
    pub min_nm: f64,
    /// Maximum wavelength (nm) of this band.
    pub max_nm: f64,
    /// Human-readable color name.
    pub name: &'static str,
}

impl SpectralBand {
    /// The 16 spectral bands of the visible spectrum.
    ///
    /// Ordered from shortest wavelength (violet) to longest (red),
    /// distributed proportionally across [380, 750] nm.
    pub const BANDS: [SpectralBand; 16] = [
        SpectralBand { min_nm: 380.0, max_nm: 405.6, name: "violet" },
        SpectralBand { min_nm: 405.6, max_nm: 431.2, name: "indigo" },
        SpectralBand { min_nm: 431.2, max_nm: 456.8, name: "blue" },
        SpectralBand { min_nm: 456.8, max_nm: 482.5, name: "azure" },
        SpectralBand { min_nm: 482.5, max_nm: 508.1, name: "cyan" },
        SpectralBand { min_nm: 508.1, max_nm: 533.7, name: "teal" },
        SpectralBand { min_nm: 533.7, max_nm: 559.3, name: "emerald" },
        SpectralBand { min_nm: 559.3, max_nm: 585.0, name: "green" },
        SpectralBand { min_nm: 585.0, max_nm: 610.6, name: "lime" },
        SpectralBand { min_nm: 610.6, max_nm: 636.2, name: "yellow" },
        SpectralBand { min_nm: 636.2, max_nm: 661.8, name: "gold" },
        SpectralBand { min_nm: 661.8, max_nm: 687.5, name: "amber" },
        SpectralBand { min_nm: 687.5, max_nm: 713.1, name: "orange" },
        SpectralBand { min_nm: 713.1, max_nm: 738.7, name: "scarlet" },
        SpectralBand { min_nm: 738.7, max_nm: 749.0, name: "red" },
        SpectralBand { min_nm: 749.0, max_nm: 750.0, name: "crimson" },
    ];

    /// Finds the spectral band containing a given wavelength.
    pub fn from_wavelength(wavelength_nm: f64) -> Self {
        let clamped = wavelength_nm.clamp(380.0, 750.0);
        Self::BANDS
            .iter()
            .find(|b| clamped >= b.min_nm && clamped < b.max_nm)
            .copied()
            .unwrap_or(Self::BANDS[0])
    }

    /// Returns the band at a proportional index [0, 16).
    pub fn at_index(index: usize) -> Self {
        Self::BANDS[index % Self::BANDS.len()]
    }

    /// Returns the band index for a given wavelength.
    pub fn index_of(wavelength_nm: f64) -> usize {
        let clamped = wavelength_nm.clamp(380.0, 750.0);
        Self::BANDS
            .iter()
            .position(|b| clamped >= b.min_nm && clamped < b.max_nm)
            .unwrap_or(0)
    }

    /// Returns the center wavelength (nm) of this band.
    pub fn center_nm(&self) -> f64 {
        (self.min_nm + self.max_nm) / 2.0
    }

    /// Returns true if this band is in the warm / long-wavelength spectrum
    /// (gold, amber, orange, scarlet, red, crimson: >= 636.2 nm).
    pub fn is_warm(&self) -> bool {
        self.min_nm >= 636.0
    }

    /// Returns true if this band is in the cool / short-wavelength spectrum
    /// (violet, indigo, blue, azure, cyan: <= 508.1 nm).
    pub fn is_cool(&self) -> bool {
        self.max_nm <= 508.5
    }

    /// Returns true if this band is in the green / balanced mid-wavelength spectrum
    /// (teal, emerald, green, lime, yellow: 508.1 - 636.2 nm).
    pub fn is_green(&self) -> bool {
        self.min_nm >= 508.0 && self.max_nm <= 636.5
    }
}
