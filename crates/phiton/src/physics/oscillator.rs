//! The oscillator: a word as a living oscillator on a sphere (it was
//! phiano's `oscillator`). Each word spins at its own frequency, set by its
//! band and the fine-structure constant; the sphere's surface is a colour
//! spectrum — hue from longitude, brightness from latitude. It stands beside
//! [`super::standing`], Bentov's standing-wave modes: the same circle, read
//! as a rotation in time. What reads a facet's words onto it is
//! `phiecs::phasor::oscillator`.

use crate::units::ALPHA;
use phistd::c64;
use std::f64::consts::PI;

/// Frequency scaling factor for oscillator rotation speed.
pub const OSCILLATOR_FREQ_SCALE: f64 = 100.0;

/// Frequency tolerance for synchronization.
pub const OSCILLATOR_FREQ_TOLERANCE: f64 = 5.0;

/// Latitude scaling - maps amplitude to latitude on the sphere.
pub const OSCILLATOR_LAT_SCALE: f64 = 4.0;

/// Number of latitude bands shown in sphere projection.
pub const OSCILLATOR_LATITUDE_BANDS: usize = 8;

/// Warmup steps for oscillator training (gradual LR ramp-up).
pub const OSCILLATOR_WARMUP_STEPS: usize = 4;

/// Weight decay for oscillator training (amplitude regularization).
pub const OSCILLATOR_WEIGHT_DECAY: f64 = 0.001;

/// Convergence delta for oscillator training (stop if improvement < this).
pub const OSCILLATOR_CONVERGENCE_DELTA: f64 = 0.0005;

#[derive(Debug, Clone, Copy)]
pub struct Oscillator {
    /// Longitude on sphere [0, 2π) - maps to hue.
    pub longitude: f64,
    /// Latitude on sphere [-π/2, +π/2] - maps to brightness.
    pub latitude: f64,
    /// Angular frequency - rotation speed.
    pub frequency: f64,
    /// Amplitude - intensity / vividness.
    pub amplitude: f64,
}

impl Oscillator {
    /// Creates an oscillator from a facet phasor.
    pub fn from_phasor(phase: f64, amplitude: f64, band_n: u32) -> Self {
        let longitude = phase.rem_euclid(2.0 * PI);
        let frequency = (1.0 + (band_n.max(1) as f64).ln()) * ALPHA * OSCILLATOR_FREQ_SCALE;
        let latitude = ((amplitude - 1.0) * PI / OSCILLATOR_LAT_SCALE)
            .clamp(-PI / 2.0, PI / 2.0);

        Self { longitude, latitude, frequency, amplitude }
    }

    /// Returns the visible longitude at time t.
    pub fn visible_longitude(&self, t: f64) -> f64 {
        (self.longitude + self.frequency * t).rem_euclid(2.0 * PI)
    }

    /// Returns the hue [0, 360) at time t.
    pub fn hue(&self, t: f64) -> f64 {
        self.visible_longitude(t) * 180.0 / PI
    }

    /// Returns the color name at time t.
    pub fn color(&self, t: f64) -> String {
        let phase = self.visible_longitude(t);
        let n = crate::SpectralBand::BANDS.len();
        let idx = ((phase / (2.0 * PI)) * n as f64).floor() as usize;
        crate::SpectralBand::at_index(idx).name.to_string()
    }

    /// Computes the spherical visibility weight from a viewing angle.
    ///
    /// w = cos(θ)·cos(θ_v)·cos(Δφ) + sin(θ)·sin(θ_v)
    /// Range: [-1, 1]. 1.0 = directly facing you, -1.0 = behind.
    pub fn visibility(&self, view_lat: f64, view_lon: f64, t: f64) -> f64 {
        let vis_lon = self.visible_longitude(t);
        let d_lon = vis_lon - view_lon;
        self.latitude.cos() * view_lat.cos() * d_lon.cos()
            + self.latitude.sin() * view_lat.sin()
    }

    /// Computes the synchronization strength with another oscillator.
    ///
    /// sync = r · exp(-|ω₁ - ω₂| / Ω) · lat_factor
    pub fn synchronization(&self, other: &Oscillator) -> f64 {
        let z1 = c64::from_polar(1.0, self.longitude);
        let z2 = c64::from_polar(1.0, other.longitude);
        let r = (z1 + z2).norm() / 2.0;

        let freq_diff = (self.frequency - other.frequency).abs();
        let freq_factor = (-freq_diff / OSCILLATOR_FREQ_TOLERANCE).exp();

        let lat_factor = 1.0 - (self.latitude - other.latitude).abs() / PI;

        r * freq_factor * lat_factor
    }
}
