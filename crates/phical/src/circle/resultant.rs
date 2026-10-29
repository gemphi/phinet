//! The resultant of angles on the circle — Σ w·(cos θ, sin θ) — and the
//! readings phient took off it: its direction three ways, a direction only
//! where it has a length, the unit vector, the dispersion.
//!
//! A running sum rather than a function of a slice, because phient sums in
//! every loop shape: per channel inside a token loop, carried forward one
//! position at a time, per head or per class in a map. The caller adds in
//! its own order, so the bits stay the caller's.

use std::f64::consts::TAU;

/// A resultant this short points nowhere: phient's floor for "the vectors
/// cancelled", the same 1e-12 at every site.
pub const VANISH: f64 = 1e-12;

/// Σ w·cos θ and Σ w·sin θ, added one angle at a time.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Resultant {
    /// Σ w·cos θ.
    pub x: f64,
    /// Σ w·sin θ.
    pub y: f64,
}

impl Resultant {
    /// The empty sum.
    pub const fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Adds `weight`·(cos θ, sin θ).
    #[inline]
    pub fn add(&mut self, weight: f64, angle: f64) {
        self.x += weight * angle.cos();
        self.y += weight * angle.sin();
    }

    /// Adds the unit vector (cos θ, sin θ), with no multiply.
    #[inline]
    pub fn add_unit(&mut self, angle: f64) {
        self.x += angle.cos();
        self.y += angle.sin();
    }

    /// The direction, `atan2(y, x)`, in [−π, π].
    #[inline]
    pub fn direction(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// The direction reduced by `rem_euclid(τ)`, in [0, 2π).
    #[inline]
    pub fn direction_euclid(&self) -> f64 {
        self.y.atan2(self.x).rem_euclid(TAU)
    }

    /// The direction with τ added when it is negative, in [0, 2π).
    #[inline]
    pub fn direction_lifted(&self) -> f64 {
        let d = self.y.atan2(self.x);
        match d < 0.0 {
            true => d + TAU,
            false => d,
        }
    }

    /// The direction, or `None` when the length (`hypot`) is not above
    /// [`VANISH`]: a sum that cancelled has no direction.
    #[inline]
    pub fn direction_or_none(&self) -> Option<f64> {
        match self.x.hypot(self.y) > VANISH {
            true => Some(self.y.atan2(self.x)),
            false => None,
        }
    }

    /// Whether both sums are under [`VANISH`] in magnitude — a box test on
    /// each component, not the length.
    #[inline]
    pub fn vanishes(&self) -> bool {
        self.x.abs() < VANISH && self.y.abs() < VANISH
    }

    /// The sum scaled to unit length by `sqrt(x·x + y·y)` (not `hypot`, which
    /// can differ in the last bit); (0, 0) when that length is not positive.
    #[inline]
    pub fn unit(&self) -> (f64, f64) {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        match norm > 0.0 {
            true => (self.x / norm, self.y / norm),
            false => (0.0, 0.0),
        }
    }

    /// Circular dispersion of the `n` unit vectors summed: 1 − |R| / n, by
    /// `hypot`; 1 when there are none.
    #[inline]
    pub fn dispersion(&self, n: usize) -> f64 {
        match n {
            0 => 1.0,
            _ => 1.0 - self.x.hypot(self.y) / n as f64,
        }
    }
}

/// The mean direction of unweighted angles, reduced into [0, 2π).
///
/// Averaging angles numerically is wrong on a circle — 0.1 and 2π − 0.1 are
/// neighbours — and averaging their unit vectors is not.
pub fn mean(angles: &[f64]) -> f64 {
    let mut r = Resultant::new();
    for a in angles {
        r.add_unit(*a);
    }
    r.direction_euclid()
}
