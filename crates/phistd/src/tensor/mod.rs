//! A complex tensor engine with reverse-mode autograd and a Fourier transform.
//!
//! The minimum machinery that lets something be *learned* across the samples of
//! a function on a circle: tensors of complex numbers, a tape that records what
//! was done to them, gradients flowing back through it, and the Fourier
//! transform as a differentiable operation so that a learned filter can live in
//! the spectral domain.
//!
//! # Shape
//!
//! Every tensor is `[batch, width, space]`. `space` is a circle sampled at a
//! power-of-two number of points — a function's domain. `width` is the number
//! of such functions stacked as features. `batch` is how many are in flight.
//!
//! # Gradients of a real loss with respect to complex values
//!
//! The convention throughout is `g = ∂L/∂Re(z) + i·∂L/∂Im(z)`, the direction of
//! steepest ascent in the plane. Under it: for `y = x·w`, `g_x = g_y·conj(w)`;
//! for a unitary transform `y = U x`, `g_x = U⁻¹ g_y`; and a real-valued
//! non-linearity applied to `Re` and `Im` separately back-propagates through
//! each part on its own. No Wirtinger calculus is needed because every
//! parameter update is a real-vector update on `(Re, Im)`.
//!
//! The FFT needs nothing outside this crate: a radix-2 Cooley–Tukey on a power-of-two length,
//! scaled by `1/√N` in both directions so that it is unitary and its adjoint is
//! its inverse.
mod adam;
mod backward;
mod fft;
mod ops;
mod squash;
mod tape;
#[cfg(test)]
mod tests;

pub use adam::{Adam, AdamState, Schedule};
pub use fft::{fft2_inplace, fft_inplace};
pub use ops::MixtureTerms;
pub use tape::{Tape, Var};

use crate::c64;
use std::f64::consts::PI;

/// A dense complex tensor of shape `[batch, width, space]`.
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    pub shape: [usize; 3],
    pub data: Vec<c64>,
}

impl Tensor {
    pub fn zeros(shape: [usize; 3]) -> Self {
        Self { shape, data: vec![c64::new(0.0, 0.0); shape[0] * shape[1] * shape[2]] }
    }

    pub fn from_vec(shape: [usize; 3], data: Vec<c64>) -> Self {
        assert_eq!(data.len(), shape[0] * shape[1] * shape[2], "shape/data mismatch");
        Self { shape, data }
    }

    /// Deterministic complex-Gaussian initialisation, scaled by `scale`.
    pub fn randn(shape: [usize; 3], scale: f64, seed: u64) -> Self {
        let n = shape[0] * shape[1] * shape[2];
        let mut r = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let mut next = || {
            r ^= r << 13;
            r ^= r >> 7;
            r ^= r << 17;
            (r >> 11) as f64 / (1u64 << 53) as f64
        };
        let data = (0..n)
            .map(|_| {
                // Box–Muller.
                let u1 = next().max(1e-12);
                let u2 = next();
                let m = (-2.0 * u1.ln()).sqrt();
                c64::new(m * (2.0 * PI * u2).cos(), m * (2.0 * PI * u2).sin()) * scale
            })
            .collect();
        Self { shape, data }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    fn idx(&self, b: usize, w: usize, s: usize) -> usize {
        (b * self.shape[1] + w) * self.shape[2] + s
    }

    #[inline]
    pub fn get(&self, b: usize, w: usize, s: usize) -> c64 {
        self.data[self.idx(b, w, s)]
    }

    #[inline]
    pub fn set(&mut self, b: usize, w: usize, s: usize, v: c64) {
        let i = self.idx(b, w, s);
        self.data[i] = v;
    }

    /// Squared Frobenius norm.
    pub fn norm_sq(&self) -> f64 {
        self.data.iter().map(|z| z.norm_sqr()).sum()
    }
}
