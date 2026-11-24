//! Amplitudes from a distribution (Feynman, QED, ch. 1: the chance of an
//! event is the square of the length of its arrow). Each share becomes an
//! arrow of length √share at a phase, so |a|² gives the share back and the
//! phase is free to make two arrows reinforce or cancel when they are added
//! (`intensity`).

use crate::c64;

/// One amplitude per share: magnitude √share, argument the phase. A missing
/// phase is 0; a negative share is read as 0.
pub fn amplitude(shares: &[f64], phases: &[f64]) -> Vec<c64> {
    shares.iter().enumerate().map(|(i, &p)| c64::from_polar(p.max(0.0).sqrt(), phases.get(i).copied().unwrap_or(0.0))).collect()
}

/// Two sets of arrows added arrow by arrow, and each sum squared: the
/// intensity the chance is read from, before it is normalised. No phase is
/// moved, unlike the keyboard's `twins`, so two arrows half a turn apart
/// cancel. A missing arrow is 0.
pub fn intensity(a: &[c64], b: &[c64]) -> Vec<f64> {
    let zero = c64::new(0.0, 0.0);
    (0..a.len().max(b.len())).map(|i| (a.get(i).copied().unwrap_or(zero) + b.get(i).copied().unwrap_or(zero)).norm_sqr()).collect()
}
