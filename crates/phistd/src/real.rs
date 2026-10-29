//! Real-valued vectors and the real part of complex dot products: what
//! phient computed inline (the ruling of 2026-10-28), written exactly as it
//! was. The complex `normalise` and `cosine` of the crate root work on whole
//! functions; these are the plain-number forms beside them.

use crate::c64;

/// Normalises an f32 vector to unit L2 length in place; a zero vector stays.
pub fn normalise_f32(v: &mut [f32]) {
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

/// The dot product of two f32 vectors, each product widened to f64 before
/// it is summed (the cosine of two unit vectors).
pub fn dot_f32(a: &[f32], b: &[f32]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x * y) as f64).sum()
}

/// The L2 norm of a complex vector: `√Σ|z|²`.
pub fn norm(y: &[c64]) -> f64 {
    y.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
}

/// `Re(Σ y_k · conj f_k)` against `f` stored interleaved as (re, im) pairs,
/// over as many channels as both have.
pub fn re_dot_interleaved(y: &[c64], flat: &[f64]) -> f64 {
    let mut dot = 0.0;
    for (yk, f) in y.iter().zip(flat.chunks_exact(2)) {
        dot += yk.re * f[0] + yk.im * f[1];
    }
    dot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_forms() {
        let mut v = [3.0f32, 4.0];
        normalise_f32(&mut v);
        assert!((dot_f32(&v, &v) - 1.0).abs() < 1e-6);
        let y = [c64::new(1.0, 2.0), c64::new(0.0, -1.0)];
        assert!((norm(&y) - 6f64.sqrt()).abs() < 1e-12);
        assert_eq!(re_dot_interleaved(&y, &[1.0, 1.0, 2.0, 2.0]), 1.0 + 2.0 - 2.0);
    }
}
