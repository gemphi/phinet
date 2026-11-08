//! Statistics shared by the context experiments (moved from `phiano::frames`, 14.4).
//!
//! Extracted from `bin/context.rs` when a second experiment needed them.
//! `predictive_gain` in particular is load-bearing and subtle — a held-out split
//! keyed on content rather than position, Laplace smoothing so an unseen
//! (class, mode) costs a finite amount — and two copies of it would drift.

use phistd::constant::PHASE_CHANNELS;
use phistd::{fnv1a, mix64};
use std::collections::HashMap;

/// Classes with fewer members than this cannot support a mean, and centering
/// them is subtracting a pair from itself.
pub const MIN_CLASS: usize = 8;

/// Assigns each pair a class, keeping the class-size distribution of `classes`
/// but destroying which pair belongs to which.
pub fn shuffle_classes(classes: &[u16], seed: u64) -> Vec<u16> {
    let mut out = classes.to_vec();
    let mut r = seed | 1;
    for i in (1..out.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        out.swap(i, (r % (i as u64 + 1)) as usize);
    }
    out
}

/// Merges classes too small to have a usable mean into one bucket.
pub fn collapse_small(classes: &[u16]) -> (Vec<u16>, usize) {
    let mut counts: HashMap<u16, usize> = HashMap::new();
    for c in classes {
        *counts.entry(*c).or_insert(0) += 1;
    }
    let out: Vec<u16> = classes
        .iter()
        .map(|c| match counts[c] >= MIN_CLASS {
            true => *c,
            false => u16::MAX,
        })
        .collect();
    let used = out.iter().collect::<std::collections::HashSet<_>>().len();
    (out, used)
}

/// Held-out log-loss of predicting a pair's harmonic mode from its context class,
/// and the gain over the context-free marginal.
///
/// Fitted on half the pairs and scored on the other half, split by content hash
/// rather than position — a per-head extractor emits pairs in an order that
/// index parity would stratify. Laplace smoothing, so an unseen (class, mode)
/// costs a finite amount rather than infinity.
///
/// Returns `(log_loss, marginal_log_loss - log_loss)`: positive gain means the
/// context helped.
pub fn predictive_gain(classes: &[u16], modes: &[usize]) -> (f64, f64) {
    let n = classes.len().min(modes.len());
    let side = |i: usize| -> bool {
        mix64(fnv1a(&format!("{}:{}", classes[i], i))) >> 63 == 0
    };

    const ALPHA: f64 = 0.5;
    let mut joint: HashMap<(u16, usize), f64> = HashMap::new();
    let mut per_class: HashMap<u16, f64> = HashMap::new();
    let mut marginal = vec![0.0f64; PHASE_CHANNELS];
    let mut fit_n = 0.0f64;

    for i in 0..n {
        if !side(i) {
            continue;
        }
        *joint.entry((classes[i], modes[i])).or_insert(0.0) += 1.0;
        *per_class.entry(classes[i]).or_insert(0.0) += 1.0;
        marginal[modes[i].min(PHASE_CHANNELS - 1)] += 1.0;
        fit_n += 1.0;
    }
    if fit_n < 1.0 {
        return (0.0, 0.0);
    }

    let m = PHASE_CHANNELS as f64;
    let (mut ll, mut ll_marg, mut held) = (0.0f64, 0.0f64, 0.0f64);
    for i in 0..n {
        if side(i) {
            continue;
        }
        let mode = modes[i].min(PHASE_CHANNELS - 1);
        let c = classes[i];
        let num = joint.get(&(c, mode)).copied().unwrap_or(0.0) + ALPHA;
        let den = per_class.get(&c).copied().unwrap_or(0.0) + ALPHA * m;
        ll -= (num / den).ln();
        ll_marg -= ((marginal[mode] + ALPHA) / (fit_n + ALPHA * m)).ln();
        held += 1.0;
    }
    match held < 1.0 {
        true => (0.0, 0.0),
        false => (ll / held, (ll_marg - ll) / held),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffling_keeps_the_class_sizes() {
        let classes: Vec<u16> = (0..40).map(|i| (i % 5) as u16).collect();
        let mut a = shuffle_classes(&classes, 7);
        let mut b = classes.clone();
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn small_classes_collapse_into_one_bucket() {
        let mut classes = vec![0u16; MIN_CLASS];
        classes.push(1);
        let (out, used) = collapse_small(&classes);
        assert_eq!(out.last(), Some(&u16::MAX));
        assert_eq!(used, 2);
    }

    #[test]
    fn a_context_free_label_gains_nothing() {
        let classes = vec![0u16; 200];
        let modes: Vec<usize> = (0..200).map(|i| i % 4).collect();
        let (_, gain) = predictive_gain(&classes, &modes);
        assert!(gain.abs() < 0.1);
    }
}