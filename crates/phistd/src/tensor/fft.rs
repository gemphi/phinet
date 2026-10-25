//! The unitary FFTs (one- and two-dimensional) and the GELU used on parts.

use super::Tensor;
use crate::c64;
use std::f64::consts::PI;

/// In-place radix-2 FFT of a power-of-two length, unitary scaling.
///
/// `inverse` conjugates the twiddles. Both directions divide by `√N`, so
/// `ifft(fft(x)) == x` and each is the adjoint of the other.
pub fn fft_inplace(x: &mut [c64], inverse: bool) {
    let n = x.len();
    assert!(n.is_power_of_two(), "fft length must be a power of two, got {n}");
    // Bit reversal.
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            x.swap(i, j);
        }
    }
    let sign = if inverse { 1.0 } else { -1.0 };
    let mut len = 2;
    while len <= n {
        let ang = sign * 2.0 * PI / len as f64;
        let wlen = c64::new(ang.cos(), ang.sin());
        let mut i = 0;
        while i < n {
            let mut w = c64::new(1.0, 0.0);
            for k in 0..len / 2 {
                let u = x[i + k];
                let v = x[i + k + len / 2] * w;
                x[i + k] = u + v;
                x[i + k + len / 2] = u - v;
                w *= wlen;
            }
            i += len;
        }
        len <<= 1;
    }
    let scale = 1.0 / (n as f64).sqrt();
    for v in x.iter_mut() {
        *v *= scale;
    }
}

/// Two-dimensional unitary FFT of every `[·,·,s]` row read as an `s1 × s2`
/// torus (`s1 * s2 == s`): rows of length `s2`, then columns of length `s1`.
/// Both directions unitary, so `ifft2(fft2(x)) == x` and the adjoint is the
/// inverse, exactly as in one dimension.
pub fn fft2_inplace(x: &mut [c64], s1: usize, s2: usize, inverse: bool) {
    assert_eq!(x.len(), s1 * s2, "fft2: torus shape mismatch");
    for row in x.chunks_mut(s2) {
        fft_inplace(row, inverse);
    }
    let mut col = vec![c64::new(0.0, 0.0); s1];
    for j in 0..s2 {
        for i in 0..s1 {
            col[i] = x[i * s2 + j];
        }
        fft_inplace(&mut col, inverse);
        for i in 0..s1 {
            x[i * s2 + j] = col[i];
        }
    }
}

pub(super) fn fft2_tensor(x: &Tensor, s1: usize, s2: usize, inverse: bool) -> Tensor {
    let mut out = x.clone();
    let s = x.shape[2];
    assert_eq!(s, s1 * s2, "fft2: sample axis is not the torus {s1}×{s2}");
    for chunk in out.data.chunks_mut(s) {
        fft2_inplace(chunk, s1, s2, inverse);
    }
    out
}

pub(super) fn fft_tensor(x: &Tensor, inverse: bool) -> Tensor {
    let mut out = x.clone();
    let s = x.shape[2];
    for chunk in out.data.chunks_mut(s) {
        fft_inplace(chunk, inverse);
    }
    out
}

// ── GELU ─────────────────────────────────────────────────────────────────────

#[inline]
pub(super) fn gelu(x: f64) -> f64 {
    0.5 * x * (1.0 + (0.797_884_560_8 * (x + 0.044_715 * x * x * x)).tanh())
}

#[inline]
pub(super) fn gelu_prime(x: f64) -> f64 {
    let k = 0.797_884_560_8;
    let u = k * (x + 0.044_715 * x * x * x);
    let t = u.tanh();
    let du = k * (1.0 + 3.0 * 0.044_715 * x * x);
    0.5 * (1.0 + t) + 0.5 * x * (1.0 - t * t) * du
}
