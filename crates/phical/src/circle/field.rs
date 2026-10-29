//! The mean field of phient's top-down descent: every sentence's state,
//! length-normalised, votes for each of its words with that word taken out,
//! and a word moves toward where its contexts point — or, under contrast,
//! toward what its contexts have that the average context does not.
//!
//! Complex per channel (`c64`), as the descent sums them. A word's own
//! share and the corpus total are kept exactly so it can be removed from
//! the average: a word inside the leave-one-out state of every other word
//! would otherwise be pushed away from itself.

use super::resultant::VANISH;
use crate::c64;

/// Adds a sentence's state, scaled by `inv` (one over its length), into a
/// running total, channel by channel.
pub fn add_scaled(total: &mut [c64], state: &[c64], inv: f64) {
    for (k, z) in state.iter().enumerate() {
        total[k] += *z * inv;
    }
}

/// A word's leave-one-out context in one sentence: the state scaled by
/// `inv`, less the word's own share, channel by channel.
pub fn leave_one_out(state: &[c64], own: &[c64], inv: f64) -> Vec<c64> {
    state.iter().zip(own.iter()).map(|(s, o)| *s * inv - *o).collect()
}

/// Whether a context vanishes on every channel (no length above
/// [`VANISH`]): a sentence made only of the word says nothing about it.
pub fn is_silent(context: &[c64]) -> bool {
    !context.iter().any(|z| z.norm() > VANISH)
}

/// Adds each channel's unit direction `z / |z|` into `sum`, skipping the
/// channels whose length is not above [`VANISH`].
pub fn add_directions(sum: &mut [c64], context: &[c64]) {
    for (s, z) in sum.iter_mut().zip(context.iter()) {
        if z.norm() > VANISH {
            *s += *z / z.norm();
        }
    }
}

/// The agreement a word's `n` contexts must reach to move it: `c / √n`,
/// never below `concentration`.
///
/// `n` unit vectors drawn uniformly have expected resultant √π / (2√n) —
/// 0.44 at four, 0.09 at a hundred — so a fixed bar lets rare words move on
/// noise and holds frequent ones still. The floor is the chance level for
/// the word's own count, scaled by `c`.
pub fn rayleigh_floor(concentration: f64, c: f64, n: f64) -> f64 {
    concentration.max(c / n.max(1.0).sqrt())
}

/// Whether a word's `n` contexts disagree: the length of their summed unit
/// directions, per context, is under `floor`.
pub fn scattered(unit: c64, n: f64, floor: f64) -> bool {
    unit.norm() / n < floor
}

/// Where a word moves on one channel, or `None` when it holds still.
///
/// `context` is the sum of its `n` leave-one-out contexts, `total` the
/// corpus total over `sentences` and `own` the word's own share of it.
/// The target is the direction of the mean context `d`, or under `contrast`
/// of `d − m`, `m` the average context with the word taken out; a residual
/// under `contrast_min · |m|` is a rounding error and holds still, and so
/// does any target shorter than [`VANISH`].
pub fn vote_target(
    context: c64,
    total: c64,
    own: c64,
    n: f64,
    sentences: f64,
    contrast: bool,
    contrast_min: f64,
) -> Option<f64> {
    let d = context / n;
    let m = (total - own) / sentences;
    let t = match contrast {
        true => d - m,
        false => d,
    };
    if contrast && t.norm() < contrast_min * m.norm() {
        return None;
    }
    if t.norm() < VANISH {
        return None;
    }
    Some(t.arg())
}
