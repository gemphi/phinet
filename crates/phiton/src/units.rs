//! The constants PhiTon's units are built from, and the one clamp it needs.
//!
//! They were read from phiano's `config` and `phical` while PhiTon lived
//! inside phiano; phiano now reads them from here and a test there keeps the
//! two in step.

/// The fine-structure constant, α ≈ 1/137.
pub const ALPHA: f64 = 1.0 / 137.0;
/// The golden ratio.
pub const PHI: f64 = 1.6180339887498948;
/// The golden ratio's conjugate, 1/φ = φ − 1.
pub const PHI_CONJUGATE: f64 = 1.0 / PHI;
/// A full turn.
pub const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
/// The largest sub-band level a phase is shifted by.
pub const BAND_N_EFFECTIVE_MAX: u32 = 13;

/// P64: the phase channels a word carries, one circle each — the 64 the
/// hexagrams, the codons and the Q64 sectors all count.
pub const PHASE_CHANNELS: usize = 64;

/// The quanta of one channel: `θ_k = phases[k] · 2π / CHANNEL_QUANTA`.
pub const CHANNEL_QUANTA: f64 = 256.0;

/// The quantum number of a convergence counter: `band_n` clamped to
/// [`BAND_N_EFFECTIVE_MAX`]. The lexicon's counter runs into the millions;
/// without the clamp every word would sit at 750 nm.
pub fn quantum(band_n: u32) -> u32 {
    band_n.min(BAND_N_EFFECTIVE_MAX)
}

/// The fine-structure phase: `φ + quantum(n)·α`, on the circle. The colour,
/// the frequency and the spectral phasor's `effective_phase` all read this.
pub fn effective_phase(phase: f64, band_n: u32) -> f64 {
    (phase + quantum(band_n) as f64 * ALPHA).rem_euclid(TWO_PI)
}
