/// Chiton — the wave/spin complement of light.
///
/// Where [`super::Phiton`] is the "roll" (particle, discrete quantum,
/// circular polarization), Chiton is the "wave" (continuous oscillation,
/// frequency, period). Together they form the 2-in-1 architecture of
/// light's wave-particle duality.
///
/// # The 2-in-1 Architecture
///
/// ```text
///   Phiton (roll/particle)     Chiton (wave/oscillation)
///   ─────────────────────      ─────────────────────────
///   phase angle θ              frequency ν = c/λ
///   wavelength λ               period T = 1/ν
///   band_n (quantum number)    angular frequency ω = 2πν
///   discrete color             continuous oscillation
///   spin ↻ (clockwise)         spin ↺ (counter-clockwise)
/// ```
///
/// The opposite spins of phiton and chiton create the standing wave
/// patterns that make linguistic word mapping and intelligence plausible:
/// words occupy stable positions in the interference space created by
/// the two counter-rotating aspects of light.

use super::types::LightQuantum;
use crate::units::{ALPHA, PHI, TWO_PI};
use serde::{Deserialize, Serialize};

/// Chiton — the wave aspect of light with opposite spin to Phiton.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Chiton {
    /// Angular frequency ω = 2πν (rad/s, scaled).
    pub angular_frequency: f64,
    /// Period T = 1/ν (scaled).
    pub period: f64,
    /// Spin direction: false = ↺ (counter-clockwise, wave), true = ↻ (clockwise, particle).
    pub spin_clockwise: bool,
    /// Reference wavelength (nm).
    pub wavelength_nm: f64,
}

impl Chiton {
    /// Creates a chiton (wave) from a phiton (particle/quantum).
    ///
    /// The wave has opposite spin to the particle, and its frequency
    /// is derived from the quantum's wavelength via ν = c/λ.
    pub fn from_quantum(quantum: &LightQuantum) -> Self {
        let freq_thz = super::Phiton::frequency_thz(quantum.wavelength_nm);
        let angular_freq = TWO_PI * freq_thz;
        let period = if freq_thz > 0.0 { 1.0 / freq_thz } else { 0.0 };

        Self {
            angular_frequency: angular_freq,
            period,
            spin_clockwise: false,
            wavelength_nm: quantum.wavelength_nm,
        }
    }

    /// Creates a chiton from a phase angle and band level.
    pub fn from_phase(phase: f64, amplitude: f64, band_n: u32) -> Self {
        let quantum = LightQuantum::from_phase(phase, amplitude, band_n);
        Self::from_quantum(&quantum)
    }

    /// Computes the standing wave amplitude at a given position.
    ///
    /// Standing waves emerge from the interference of the phiton (roll)
    /// and chiton (wave) with opposite spins. The amplitude is:
    ///   A(x) = 2·A₀·cos(kx)·cos(ωt)
    /// where k = 2π/λ is the wave number.
    pub fn standing_wave_amplitude(&self, position: f64, time: f64, amplitude: f64) -> f64 {
        let wavelength_m = self.wavelength_nm * 1e-9;
        let k = TWO_PI / wavelength_m;
        2.0 * amplitude * (k * position).cos() * (self.angular_frequency * time).cos()
    }

    /// Computes the resonance between two chitons (waves).
    ///
    /// Two waves resonate when their frequencies are close and their
    /// spins are opposite. The golden ratio φ weights the distance.
    pub fn resonance(&self, other: &Self) -> f64 {
        let freq_delta = (self.angular_frequency - other.angular_frequency).abs();
        let spin_match = if self.spin_clockwise != other.spin_clockwise { 1.0 } else { 0.5 };
        let wavelength_factor = 1.0 / (1.0 + PHI * (self.wavelength_nm - other.wavelength_nm).abs() / 100.0);
        spin_match * wavelength_factor / (1.0 + freq_delta * ALPHA)
    }

    /// Returns the wave number k = 2π/λ.
    pub fn wave_number(&self) -> f64 {
        let wavelength_m = self.wavelength_nm * 1e-9;
        TWO_PI / wavelength_m
    }

    /// Returns the complementary phiton (particle) from this chiton (wave).
    ///
    /// This completes the 2-in-1 cycle: phiton → chiton → phiton.
    pub fn to_quantum(&self, amplitude: f64, band_n: u32) -> LightQuantum {
        let phase = super::Phiton::wavelength_to_phase(self.wavelength_nm);
        LightQuantum::from_phase(phase, amplitude, band_n)
    }
}
