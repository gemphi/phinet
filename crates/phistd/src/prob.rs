//! Probabilities from scores: the softmax and its normaliser, perplexity, and
//! a draw from a softmax at a temperature. phient wrote each inline, many
//! times (the ruling of 2026-10-28: the calculations live below phient);
//! here they are written exactly as the sites wrote them, so no recorded
//! number moves.

/// Perplexity from a summed log-probability over `n` positions:
/// `exp(−Σ ln p / n)`; infinite for no positions.
pub fn perplexity(log_sum: f64, n: usize) -> f64 {
    match n {
        0 => f64::INFINITY,
        _ => (-log_sum / n as f64).exp(),
    }
}

/// Perplexity from a summed negative log-likelihood over `n` positions:
/// `exp(nll / n)`, with `n` at least one.
pub fn perplexity_of_nll(nll: f64, n: usize) -> f64 {
    (nll / n.max(1) as f64).exp()
}

/// The softmax normaliser at sharpness `beta`, shifted by `top`:
/// `Σ e^{β(s − top)}`, summed in order.
pub fn softmax_z(scores: impl IntoIterator<Item = f64>, top: f64, beta: f64) -> f64 {
    scores.into_iter().map(|s| (beta * (s - top)).exp()).sum()
}

/// One score's softmax probability: `e^{β(s − top)} / z`.
#[inline]
pub fn softmax_p(score: f64, top: f64, beta: f64, z: f64) -> f64 {
    (beta * (score - top)).exp() / z
}

/// `ln softmax(scores)[i]`: the score less the log of the normaliser, both
/// shifted by the largest score.
pub fn log_softmax_at(scores: &[f64], i: usize) -> f64 {
    let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let z: f64 = scores.iter().map(|s| (s - max).exp()).sum();
    scores[i] - max - z.ln()
}

/// The gradient of `−ln softmax(scores)[truth]` with respect to each score:
/// the softmax less one at the truth.
pub fn softmax_minus_onehot(scores: &[f64], truth: usize) -> Vec<f64> {
    let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let ex: Vec<f64> = scores.iter().map(|s| (s - max).exp()).collect();
    let z: f64 = ex.iter().sum();
    ex.iter()
        .enumerate()
        .map(|(i, e)| {
            let prob = e / z;
            prob - match i == truth {
                true => 1.0,
                false => 0.0,
            }
        })
        .collect()
}

/// An index drawn from the softmax of `scores` at `temperature`. At zero
/// temperature or below, the best score (ties to the earlier). `draw` gives
/// a uniform in [0, 1) and is called only when a draw is made, so a
/// generator behind it advances exactly as before. A total that is not a
/// positive number reads as the first index.
pub fn sample_softmax(scores: &[f64], temperature: f64, draw: impl FnOnce() -> f64) -> Option<usize> {
    if scores.is_empty() {
        return None;
    }
    if temperature <= 0.0 {
        return scores
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i);
    }
    let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp: Vec<f64> = scores.iter().map(|s| ((s - max) / temperature).exp()).collect();
    let total: f64 = exp.iter().sum();
    if total.is_nan() || total <= 0.0 {
        return Some(0);
    }
    let mut r = draw() * total;
    for (i, w) in exp.iter().enumerate() {
        r -= w;
        if r <= 0.0 {
            return Some(i);
        }
    }
    Some(exp.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_softmax_sums_to_one() {
        let s = [1.0, 2.0, 3.0];
        let z = softmax_z(s, 3.0, 1.0);
        let total: f64 = s.iter().map(|x| softmax_p(*x, 3.0, 1.0, z)).sum();
        assert!((total - 1.0).abs() < 1e-12);
    }

    #[test]
    fn perplexity_of_a_fair_die_is_its_faces() {
        let log_sum = 4.0 * (1.0f64 / 6.0).ln();
        assert!((perplexity(log_sum, 4) - 6.0).abs() < 1e-9);
        assert!((perplexity_of_nll(-log_sum, 4) - 6.0).abs() < 1e-9);
        assert_eq!(perplexity(0.0, 0), f64::INFINITY);
    }

    #[test]
    fn a_cold_draw_is_the_best_and_a_warm_one_is_in_range() {
        assert_eq!(sample_softmax(&[0.1, 0.9, 0.5], 0.0, || panic!("no draw when cold")), Some(1));
        assert!(sample_softmax(&[0.1, 0.9, 0.5], 1.0, || 0.5).is_some_and(|i| i < 3));
        assert_eq!(sample_softmax(&[], 1.0, || 0.5), None);
    }
}
