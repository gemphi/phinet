//! # phistd — the numbers: a Fourier tensor engine for neural operators on the circle
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and
//! composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical
//! stores no point: a word is 32–64 relations, each a complex phasor on a
//! closed circle with a count; meaning is a side effect of pairs of words
//! moving under a condition.
//!
//! phistd is the engine of the family (`phinet::std` from the one entry,
//! docs/160) — what torch is to neuraloperator: a
//! complex tensor type, a reverse-mode tape with a differentiable unitary FFT,
//! Adam, holographic bind/unbind, the sixty-four gates, and the phasor
//! ranges. It knows no words and holds no operator: the Fourier neural
//! operator and joint training are phical's, the sphere's harmonics phifno's,
//! the language model phient's. Small, dependency-light, and complete.
//!
//! A *function* here is a `Vec<Complex64>` — samples of a complex function on
//! a circle at a power-of-two number of points. Everything is generic over that
//! length: the same operator applies at 32, 64 or 128 samples.
//!
//! ```
//! use phistd::{cosine, fft_inplace, unit_random, Tape, Tensor};
//!
//! // A unitary FFT: energy is kept, and the tape's gradient is checked.
//! let f = unit_random(64, 7);
//! let mut g = f.clone();
//! fft_inplace(&mut g, false);
//! let energy: f64 = g.iter().map(|z| z.norm_sqr()).sum();
//! assert!((energy - 1.0).abs() < 1e-9);
//! assert!((cosine(&f, &f) - 1.0).abs() < 1e-12);
//! let mut tape = Tape::new();
//! let x = tape.constant(Tensor::from_vec([1, 1, 64], f.clone()));
//! let y = tape.fft(x);
//! assert_eq!(tape.value(y).shape, [1, 1, 64]);
//! ```
//!
//! | module | what |
//! |---|---|
//! | [`tensor`] | `Tensor` `[batch, width, space]`, `Tape` (add, mul, linear, mode_mix, fft/ifft, fft2/ifft2, gelu, gather, unit, cosine, hinge, cross_entropy, cosine_table, mixture_nll, mobius), `Adam` (AdamW, `Schedule`). Every gradient is finite-difference checked; `cosine_table` runs on all cores (rayon). |
//! | [`codon`] | the sixty-four as three bases: `codon`, `codon_index`, `hexagram_glyph`. |
//! | [`hrr`] | `bind`, `unbind`, `compose`, `role`. |
//! | [`hexagram`] | the sixty-four operations: I Ching hexagrams as spectral gates (`apply`, `gate`, `changing_lines`), the hexagram a function is (`hexagram_of`, `lines`), `novelty`. |
//! | [`phasor`] | the phasor ranges: `Resolution` P16–P128 × `Quantum` Q16–Q64 (`Range::ALL`), `resample`, `quantise`, `sector_of`; `Fno::at_resolution` moves an operator between them. |
//!
//! Gradients of a real loss with respect to complex values use the convention
//! `g = ∂L/∂Re + i·∂L/∂Im`; see [`tensor`].

pub mod codon;
pub mod hexagram;
pub mod hrr;
#[path = "phasor/mod.rs"]
pub mod phasor;
#[path = "tensor/mod.rs"]
pub mod tensor;

pub use num_complex::Complex64 as c64;

pub use phasor::{Quantum, Range, Resolution};
pub use tensor::{fft2_inplace, fft_inplace, Adam, AdamState, MixtureTerms, Schedule, Tape, Tensor, Var};

/// A complex function on the circle, sampled.
pub type Function = Vec<c64>;

/// FNV-1a over the bytes of a spelling: the hash a word's seed comes from.
pub fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 14695981039346656037;
    for byte in text.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

/// splitmix64's finaliser: avalanches every input bit into every output bit.
#[inline]
pub fn mix64(h: u64) -> u64 {
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The golden angle, 2π/φ²: successive channels seeded a golden angle apart
/// never cluster, however correlated their hash bits.
pub const GOLDEN_ANGLE: f64 = 2.399_963_229_728_653;

/// A word's base phase from its spelling: the word hash mixed, read as a
/// fraction of the circle.
pub fn seed_phase(word: &str) -> f64 {
    let h = mix64(fnv1a(word));
    (h >> 11) as f64 / (1u64 << 53) as f64 * std::f64::consts::TAU
}

/// THE seed (docs/159): where a fresh word's `channels` angles start, from its
/// spelling alone. Every phasor in the workspace — phiecs's, phiano's packed
/// one, the relational one — is born here, so the same word has the same
/// angles everywhere. Each channel gets its own hash stream (the word hash
/// re-mixed with the channel index), spread by the golden angle.
pub fn seed_angles(word: &str, channels: usize) -> Vec<f64> {
    let h = mix64(fnv1a(word));
    (0..channels)
        .map(|k| {
            let hk = mix64(h ^ (k as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let frac = (hk >> 11) as f64 / (1u64 << 53) as f64;
            (frac * std::f64::consts::TAU + k as f64 * GOLDEN_ANGLE).rem_euclid(std::f64::consts::TAU)
        })
        .collect()
}

/// Normalises a function to unit energy in place. Returns false if it was zero.
pub fn normalise(f: &mut [c64]) -> bool {
    let e: f64 = f.iter().map(|z| z.norm_sqr()).sum();
    if e < 1e-24 {
        return false;
    }
    let s = 1.0 / e.sqrt();
    for v in f.iter_mut() {
        *v *= s;
    }
    true
}

/// A unit-energy function with random phases and magnitudes, from a seed.
pub fn unit_random(space: usize, seed: u64) -> Function {
    let t = Tensor::randn([1, 1, space], 1.0, mix64(seed ^ 0xF00D));
    let mut f = t.data;
    normalise(&mut f);
    f
}

/// A unit-magnitude-per-sample function with random phases, from a seed —
/// the shape a role has.
pub fn unit_phase(space: usize, seed: u64) -> Function {
    let h = mix64(seed ^ 0x5EED);
    (0..space)
        .map(|k| {
            let hk = mix64(h ^ (k as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let frac = (hk >> 11) as f64 / (1u64 << 53) as f64;
            c64::from_polar(1.0, frac * std::f64::consts::TAU)
        })
        .collect()
}

/// Stacks functions into a `[b,1,space]` tensor.
pub fn stack(fs: &[Function], space: usize) -> Tensor {
    let mut data = Vec::with_capacity(fs.len() * space);
    for f in fs {
        assert_eq!(f.len(), space, "function length {} != space {}", f.len(), space);
        data.extend_from_slice(f);
    }
    Tensor::from_vec([fs.len(), 1, space], data)
}

/// The inverse of [`stack`] for a `[b,1,space]` tensor.
pub fn unstack(t: &Tensor) -> Vec<Function> {
    assert_eq!(t.shape[1], 1);
    t.data.chunks(t.shape[2]).map(|c| c.to_vec()).collect()
}

/// Real cosine between two functions.
pub fn cosine(a: &[c64], b: &[c64]) -> f64 {
    let (mut dot, mut na, mut nb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        dot += (x * y.conj()).re;
        na += x.norm_sqr();
        nb += y.norm_sqr();
    }
    dot / (na.sqrt() * nb.sqrt()).max(1e-12)
}
