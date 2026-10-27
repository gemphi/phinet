#[allow(unused_imports)]
use super::*;

impl Physics {
    /// The coupling constant between color and time.
    ///
    /// α governs how much the sub-band energy level shifts both the
    /// color (wavelength) and the temporal frequency of a word's
    /// oscillation. This is where the fine-structure constant "shines".
    pub const COUPLING: f64 = ALPHA;

    /// The natural scaling ratio for color-space-time mapping.
    ///
    /// φ (golden ratio) provides the most uniform distribution of
    /// semantic points across the color-space-time manifold, just as it
    /// does for sunflower seeds on a disk.
    pub const NATURAL_RATIO: f64 = PHI;

    /// Computes the color-space-time phase of a word.
    ///
    /// The effective phase combines the word's base phase with its
    /// sub-band energy correction: φ_eff = φ + n·α.
    /// This single value determines both the word's color (via wavelength)
    /// and its temporal oscillation (via frequency).
    pub fn effective_phase(phase: f64, band_n: u32) -> f64 {
        crate::units::effective_phase(phase, band_n)
    }

    /// The sub-band quantum number a raw `band_n` stands for.
    ///
    /// `SpectralPhasor::band_n` is a convergence *counter* — the trainer
    /// increments it every time a word's channel-0 step falls under the
    /// convergence threshold, and on the shipped v1 manifold it reaches 1.8
    /// million. The quantum number is that counter clamped to
    /// `BAND_N_EFFECTIVE_MAX` (13: 2π/64 ÷ α, one sector of fine-structure
    /// shift), exactly as `SpectralPhasor::effective_phase` reads it. Every
    /// Physics formula goes through here, so a counter can never walk a word
    /// around the colour wheel.
    #[inline]
    pub fn quantum(band_n: u32) -> u32 {
        crate::units::quantum(band_n)
    }

    /// Computes the temporal frequency (THz) from an effective phase.
    ///
    /// Maps the phase to a wavelength, then converts to frequency
    /// using ν = c/λ. This is the "time" aspect of phical.
    pub fn phase_to_frequency_thz(effective_phase: f64) -> f64 {
        let wavelength = crate::Phiton::phase_to_wavelength(effective_phase, 0);
        crate::Phiton::frequency_thz(wavelength)
    }

    /// Computes the color-space-time distance between two words.
    ///
    /// Combines spectral distance (color) and frequency distance (time)
    /// into a single metric, weighted by the golden ratio:
    ///   d = √(Δλ² + φ·Δν²)
    pub fn color_space_time_distance(
        phase_a: f64,
        band_a: u32,
        phase_b: f64,
        band_b: u32,
    ) -> f64 {
        let wa = crate::Phiton::phase_to_wavelength(phase_a, Self::quantum(band_a));
        let wb = crate::Phiton::phase_to_wavelength(phase_b, Self::quantum(band_b));
        let fa = crate::Phiton::frequency_thz(wa);
        let fb = crate::Phiton::frequency_thz(wb);

        let delta_lambda = wa - wb;
        let delta_freq = fa - fb;
        (delta_lambda * delta_lambda + PHI * delta_freq * delta_freq).sqrt()
    }

    /// Computes the quantum oscillation energy of a word.
    ///
    /// E = n·α·h·ν, where n is the band level, α is the fine-structure
    /// constant, h is Planck's constant, and ν is the frequency.
    /// This energy determines how strongly a word participates in
    /// the standing wave patterns of the color-space-time manifold.
    pub fn oscillation_energy(phase: f64, band_n: u32) -> f64 {
        let n = Self::quantum(band_n);
        let wavelength = crate::Phiton::phase_to_wavelength(phase, n);
        let freq = crate::Phiton::frequency_thz(wavelength) * 1e12;
        (n as f64 + 1.0) * ALPHA * crate::Phiton::H * freq
    }
}

impl ColorSpaceTimePoint {
    /// Creates a color-space-time point from a phase and band level.
    pub fn from_phase(phase: f64, band_n: u32) -> Self {
        let band_n = Physics::quantum(band_n);
        let eff = Physics::effective_phase(phase, band_n);
        let wavelength = crate::Phiton::phase_to_wavelength(eff, 0);
        let frequency = crate::Phiton::frequency_thz(wavelength);
        Self {
            wavelength_nm: wavelength,
            frequency_thz: frequency,
            effective_phase: eff,
            band_n,
        }
    }

    /// Returns the spectral band of this point.
    pub fn band(&self) -> crate::SpectralBand {
        crate::SpectralBand::from_wavelength(self.wavelength_nm)
    }

    /// Returns the color of this point.
    pub fn color(&self) -> crate::PhitonColor {
        crate::PhitonColor::new(self.wavelength_nm, self.band())
    }
}
