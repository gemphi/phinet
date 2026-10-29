//! Two or three angles at a time: the shortest signed turn from one angle
//! to another, the distance between two, the turn taken through
//! `rem_euclid` and a step along it, and the analogy c + b − a.
//!
//! Not `phistd::wrap`: that reduces through `rem_euclid`, and for a negative
//! difference `(d + τ) − τ` is not bit-identical to `d`, so swapping it in
//! would move every recorded number that reads these angles. [`delta`] and
//! [`delta_euclid`] are two more forms of the same angle, kept apart for the
//! same reason.

use std::f64::consts::{PI, TAU};

/// The shortest signed angle from `from` to `to`, in [−π, π].
#[inline]
pub fn delta(from: f64, to: f64) -> f64 {
    let mut d = to - from;
    while d > PI {
        d -= TAU;
    }
    while d < -PI {
        d += TAU;
    }
    d
}

/// The distance between two angles on the circle, in [0, π] for angles
/// less than a full turn apart.
#[inline]
pub fn distance(a: f64, b: f64) -> f64 {
    let d = (a - b).abs();
    match d > PI {
        true => TAU - d,
        false => d,
    }
}

/// The signed turn from `from` to `to` through `rem_euclid`:
/// `(to − from + π) rem τ − π`, in [−π, π). The same angle as [`delta`],
/// not the same bits.
#[inline]
pub fn delta_euclid(from: f64, to: f64) -> f64 {
    (to - from + PI).rem_euclid(TAU) - PI
}

/// A step of `step` (a fraction of the turn) from `theta` toward `target`
/// along [`delta_euclid`], reduced into [0, 2π).
#[inline]
pub fn step_toward(theta: f64, target: f64, step: f64) -> f64 {
    (theta + step * delta_euclid(theta, target)).rem_euclid(TAU)
}

/// The analogy a : b :: c : ?, on one channel: `c + b − a`, unreduced.
#[inline]
pub fn analogy(a: f64, b: f64, c: f64) -> f64 {
    c + b - a
}
