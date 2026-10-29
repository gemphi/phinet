//! Offsets with a mean direction taken out, reduced into [0, 2π).
//!
//! A pair's offset can carry its head rather than the step: pairs cluster
//! because they share a word, not a relation. Taking out the head's mean
//! direction (and the filler's, with the grand mean put back) removes those
//! main effects, as double-centering does a two-way table; taking out a
//! context class's mean leaves what the pair does that its context does not.
//! The means themselves are [`Resultant`](super::Resultant) directions,
//! summed by the caller.

use std::f64::consts::TAU;

/// An angle reduced into [0, 2π) by `rem_euclid` (the uncentred offset).
#[inline]
pub fn reduce(angle: f64) -> f64 {
    angle.rem_euclid(TAU)
}

/// An offset with one mean direction taken out: `(offset − mean) rem τ`.
#[inline]
pub fn centred(offset: f64, mean: f64) -> f64 {
    (offset - mean).rem_euclid(TAU)
}

/// Double-centering: the head's and the filler's mean directions taken out
/// and the grand mean put back, `(offset − head − filler + grand) rem τ`.
#[inline]
pub fn double_centred(offset: f64, head: f64, filler: f64, grand: f64) -> f64 {
    (offset - head - filler + grand).rem_euclid(TAU)
}
