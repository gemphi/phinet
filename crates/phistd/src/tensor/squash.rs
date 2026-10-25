//! The phase-preserving activation (docs/154): `z ↦ z · tanh|z| / |z|`.
//! It shrinks a sample's modulus and keeps its phase, so it commutes with
//! a global phase, `σ(e^{iφ}z) = e^{iφ}σ(z)`, and with conjugation,
//! `σ(z̄) = conj σ(z)` — the GELU on the real and imaginary parts does
//! neither. An operator built from it, complex-linear maps and no biases
//! reads a phase-shifted wave as the same wave shifted.

use super::tape::Op;
use super::{Tape, Tensor, Var};
use crate::c64;

/// `s(r) = tanh(r) / r`, with `s(0) = 1`.
fn shrink(r: f64) -> f64 {
    if r < 1e-6 {
        1.0 - r * r / 3.0
    } else {
        r.tanh() / r
    }
}

/// `s'(r) / r = (r·sech²r − tanh r) / r³`, with its limit −2/3 at 0.
fn shrink_slope(r: f64) -> f64 {
    if r < 1e-3 {
        -2.0 / 3.0 + 0.8 * r * r
    } else {
        let t = r.tanh();
        (r * (1.0 - t * t) - t) / (r * r * r)
    }
}

impl Tape {
    /// `z ↦ z · tanh|z| / |z|` on every sample.
    pub fn squash(&mut self, x: Var) -> Var {
        let xv = &self.nodes[x.0].value;
        let data = xv.data.iter().map(|z| z * shrink(z.norm())).collect();
        let t = Tensor::from_vec(xv.shape, data);
        let rg = self.any_grad(&[x]);
        self.push(t, Op::Squash(x), rg)
    }

    /// The gradient through `squash`, the real 2×2 Jacobian of
    /// `(x, y) ↦ (x·s, y·s)` applied to each sample's `(∂/∂re, ∂/∂im)`.
    pub(super) fn squash_grad(&self, x: Var, g: &Tensor) -> Tensor {
        let xv = &self.nodes[x.0].value;
        let data = g
            .data
            .iter()
            .zip(&xv.data)
            .map(|(gy, z)| {
                let r = z.norm();
                let (s, k) = (shrink(r), shrink_slope(r));
                let dot = gy.re * z.re + gy.im * z.im;
                c64::new(gy.re * s + k * z.re * dot, gy.im * s + k * z.im * dot)
            })
            .collect();
        Tensor::from_vec(g.shape, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squash_keeps_phase_and_matches_its_numerical_gradient() {
        let mut t = Tape::new();
        let z = [c64::new(0.3, -1.2), c64::new(-2.0, 0.5), c64::new(1e-4, 2e-4)];
        let x = t.param(Tensor::from_vec([1, 1, 3], z.to_vec()));
        let y = t.squash(x);
        let out = t.value(y).clone();
        for (a, b) in z.iter().zip(&out.data) {
            assert!((a.arg() - b.arg()).abs() < 1e-12 && (b.norm() - a.norm().tanh()).abs() < 1e-12, "phase kept, modulus tanh|z|");
        }
        // Loss = Re Σ c·y for a fixed c; compare the tape's gradient with finite differences.
        let c = [c64::new(0.7, 0.2), c64::new(-0.4, 1.1), c64::new(0.5, -0.9)];
        let g = Tensor::from_vec([1, 1, 3], c.iter().map(|c| c.conj()).collect());
        let gx = t.squash_grad(x, &g);
        let loss = |z: &[c64]| z.iter().zip(&c).map(|(z, c)| (c * z * shrink(z.norm())).re).sum::<f64>();
        for i in 0..3 {
            for (d, part) in [(c64::new(1e-6, 0.0), gx.data[i].re), (c64::new(0.0, 1e-6), gx.data[i].im)] {
                let (mut p, mut m) = (z, z);
                p[i] += d;
                m[i] -= d;
                let num = (loss(&p) - loss(&m)) / 2e-6;
                assert!((num - part).abs() < 1e-6, "sample {i}: {num} vs {part}");
            }
        }
    }
}
