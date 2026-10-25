//! Holographic binding on the circle.
//!
//! A role is a unit function; binding a filler to it is pointwise
//! multiplication (phase addition on every sample), unbinding is
//! multiplication by the conjugate, and a composite is the normalised sum of
//! bound fillers. This is the Fourier-domain form of holographic reduced
//! representations (Plate, 1995): circular convolution in the sample domain is
//! pointwise multiplication here.
//!
//! Recovery degrades geometrically with nesting because every other filler's
//! crosstalk survives each unbind. The remedy is a **cleanup** after each
//! unbind, and `phical::Fno` trained on `(unbind → clean target)`
//! pairs is a learned one — see the `compose` module of a consumer for the
//! measurement.

use crate::{c64, Function};

/// A deterministic unit-magnitude role from a seed.
pub fn role(space: usize, seed: u64) -> Function {
    crate::unit_phase(space, seed)
}

pub fn bind(f: &[c64], r: &[c64]) -> Function {
    f.iter().zip(r).map(|(a, b)| a * b).collect()
}

pub fn unbind(f: &[c64], r: &[c64]) -> Function {
    f.iter().zip(r).map(|(a, b)| a * b.conj()).collect()
}

/// Superposes bound fillers into one unit-energy composite.
pub fn compose(parts: &[Function]) -> Function {
    let s = parts.first().map(|p| p.len()).unwrap_or(0);
    let mut out = vec![c64::new(0.0, 0.0); s];
    for p in parts {
        for (o, v) in out.iter_mut().zip(p) {
            *o += v;
        }
    }
    crate::normalise(&mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosine;
    use crate::unit_random;

    #[test]
    fn test_bind_unbind_roundtrip_and_single_level_recovery() {
        let a = unit_random(64, 1);
        let r = role(64, 1);
        let back = unbind(&bind(&a, &r), &r);
        for (x, y) in back.iter().zip(&a) {
            assert!((x - y).norm() < 1e-12);
        }
        let fs: Vec<Function> = (0..4).map(|i| unit_random(64, 10 + i)).collect();
        let parts: Vec<Function> = fs.iter().enumerate().map(|(d, f)| bind(f, &role(64, d as u64 + 1))).collect();
        let id = compose(&parts);
        for d in 0..4 {
            let q = unbind(&id, &role(64, d as u64 + 1));
            let own = cosine(&q, &fs[d]);
            for (j, other) in fs.iter().enumerate() {
                if j != d {
                    assert!(own > cosine(&q, other), "role {d}: own {own:.3} vs {j}");
                }
            }
        }
    }
}
