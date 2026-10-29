//! Optimiser steps on plain slices. `tensor::Adam` steps tape `Var`s; this is
//! the same rule for a parameter vector held as numbers, as phient's tuner
//! wrote it inline (the ruling of 2026-10-28), kept to its arithmetic.

/// One Adam step with bias correction at step `t` (from 1): the moments
/// `m`, `v` move toward the gradient `g`, and `w` moves against it.
#[allow(clippy::too_many_arguments)]
pub fn adam_step(w: &mut [f64], g: &[f64], m: &mut [f64], v: &mut [f64], t: i32, lr: f64, b1: f64, b2: f64, eps: f64) {
    for f in 0..w.len() {
        m[f] = b1 * m[f] + (1.0 - b1) * g[f];
        v[f] = b2 * v[f] + (1.0 - b2) * g[f] * g[f];
        let mh = m[f] / (1.0 - b1.powi(t));
        let vh = v[f] / (1.0 - b2.powi(t));
        w[f] -= lr * mh / (vh.sqrt() + eps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adam_walks_down_a_bowl() {
        let (mut w, mut m, mut v) = ([3.0, -2.0], [0.0; 2], [0.0; 2]);
        for t in 1..=500 {
            let g = [2.0 * w[0], 2.0 * w[1]];
            adam_step(&mut w, &g, &mut m, &mut v, t, 0.05, 0.9, 0.999, 1e-8);
        }
        assert!(w[0].abs() < 0.05 && w[1].abs() < 0.05, "{w:?}");
    }
}
