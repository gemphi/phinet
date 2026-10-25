//! The sixty-four operations: the I Ching hexagrams as spectral gates on
//! the circle, and the hexagram a function *is*.
//!
//! A hexagram is six lines, each yin or yang: a 6-bit number. A function on
//! a power-of-two circle has a spectrum whose modes fall into six octaves —
//! `{1}, {2,3}, {4..7}, {8..15}, {16..31}, {32..}` — so a hexagram names a
//! *gate*: octave `j` passes when line `j` is yang. Line 0 is the bottom
//! line and the lowest octave; line 5 the top line and the highest. Mode 0
//! (the mean) always passes. Sixty-four gates, one per hexagram, all
//! unitary-FFT exact; ䷀ (111111, "the Creative") is the identity and
//! ䷁ (000000, "the Receptive") keeps only the mean. `changing_lines` walks
//! from one gate to another by flipping lines — how the oracle moves.
//!
//! The other direction: a function *has* a hexagram — its centroid phase
//! quantised into 64 sectors (the Q64 juncture). So a sentence is a walk of
//! hexagrams, and an operator's prediction has one; whether the predicted
//! hexagram matches the next word's is a measurable claim (chance 1/64).

use crate::tensor::fft_inplace;
use crate::{c64, Function};

/// Number of hexagrams / lines.
pub const HEXAGRAMS: usize = 64;

/// The King Wen sequence: binary id (bottom line = bit 0) → traditional
/// number 1..=64. Id 0 (all yin, ䷁ the Receptive) is 2; id 63 (all yang,
/// ䷀ the Creative) is 1. Lives here, beside the gate, because it is the
/// hexagrams' semantics (docs/159); phiano re-exports it.
pub const KING_WEN_MAP: [u8; 64] = [
    2, 23, 8, 20, 16, 35, 45, 12, 15, 52, 39, 53, 62, 56, 31, 33,
    7, 4, 29, 59, 40, 64, 47, 6, 46, 18, 48, 57, 32, 50, 28, 44,
    24, 27, 3, 42, 51, 21, 17, 25, 36, 22, 63, 37, 55, 30, 49, 13,
    19, 41, 60, 61, 54, 38, 58, 10, 11, 26, 5, 9, 34, 14, 43, 1,
];

/// A hexagram's King Wen number, 1..=64.
pub fn king_wen(id: u8) -> u8 {
    KING_WEN_MAP[id as usize % HEXAGRAMS]
}

/// The Unicode glyph of a hexagram by binary id. The block U+4DC0..U+4DFF
/// runs in King Wen order, so the id is mapped through `KING_WEN_MAP` first —
/// id 0 is ䷁, id 63 is ䷀.
pub fn glyph(id: u8) -> char {
    char::from_u32(0x4DC0 + (king_wen(id) as u32 - 1)).unwrap_or('?')
}
pub const LINES: usize = 6;

/// Octave `j` of a spectrum of `space` samples: the positive frequencies
/// `[2^j, 2^(j+1))`, with the top octave taking everything up to Nyquist.
/// Negative frequencies mirror their positive partner.
pub fn octave(mode: usize, space: usize) -> Option<usize> {
    if mode == 0 {
        return None;
    }
    let half = space / 2;
    let m = if mode <= half { mode } else { space - mode };
    if m == 0 {
        return None;
    }
    let j = (usize::BITS - 1 - m.leading_zeros()) as usize; // floor(log2 m)
    Some(j.min(LINES - 1))
}

/// The gate mask of hexagram `id`: which modes pass.
pub fn gate(id: u8, space: usize) -> Vec<bool> {
    (0..space)
        .map(|m| match octave(m, space) {
            None => true,
            Some(j) => (id >> j) & 1 == 1,
        })
        .collect()
}

/// Applies hexagram `id` to `f`: FFT, gate, inverse FFT. Exact and linear.
pub fn apply(f: &[c64], id: u8) -> Function {
    let n = f.len();
    let mut g = f.to_vec();
    fft_inplace(&mut g, false);
    for (z, keep) in g.iter_mut().zip(gate(id, n)) {
        if !keep {
            *z = c64::new(0.0, 0.0);
        }
    }
    fft_inplace(&mut g, true);
    g
}

/// Flips the lines set in `mask` (6 bits): the oracle's changing lines.
pub fn changing_lines(id: u8, mask: u8) -> u8 {
    (id ^ (mask & 0b11_1111)) & 0b11_1111
}

/// The hexagram a function *is*: its centroid phase in 64 sectors, with the
/// half-step so the round trip through `phase_of` is exact.
pub fn hexagram_of(f: &[c64]) -> u8 {
    let c: c64 = f.iter().sum();
    let phase = c.arg().rem_euclid(std::f64::consts::TAU);
    ((phase / std::f64::consts::TAU) * HEXAGRAMS as f64).floor() as usize as u8 % HEXAGRAMS as u8
}

/// The central phase of hexagram `id`.
pub fn phase_of(id: u8) -> f64 {
    (id as f64 + 0.5) / HEXAGRAMS as f64 * std::f64::consts::TAU
}

/// The novelty of applying `id` to `f`: `1 − cos(f, H_id f)`, in [0, 2].
/// Zero for the Creative, largest when the gate removes what `f` is made of.
pub fn novelty(f: &[c64], id: u8) -> f64 {
    1.0 - crate::cosine(f, &apply(f, id))
}

/// Energy of `f` in each of the six octaves, normalised to sum to one: the
/// hexagram *reading* of a function — which lines are strong.
pub fn lines(f: &[c64]) -> [f64; LINES] {
    let n = f.len();
    let mut g = f.to_vec();
    fft_inplace(&mut g, false);
    let mut e = [0.0f64; LINES];
    for (m, z) in g.iter().enumerate() {
        if let Some(j) = octave(m, n) {
            e[j] += z.norm_sqr();
        }
    }
    let s: f64 = e.iter().sum::<f64>().max(1e-300);
    for v in e.iter_mut() {
        *v /= s;
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit_random;

    #[test]
    fn creative_is_identity_and_receptive_is_the_mean() {
        let f = unit_random(64, 3);
        let g = apply(&f, 0b11_1111);
        assert!(f.iter().zip(&g).all(|(a, b)| (a - b).norm() < 1e-12));
        let h = apply(&f, 0);
        let mean: c64 = f.iter().sum::<c64>() / 64.0;
        assert!(h.iter().all(|z| (z - mean).norm() < 1e-12));
    }

    #[test]
    fn gates_partition_the_spectrum() {
        // Every mode but 0 belongs to exactly one octave, and the six
        // single-line hexagrams together pass every mode.
        let n = 64;
        let mut seen = vec![0usize; n];
        for j in 0..LINES {
            for (m, keep) in gate(1 << j, n).iter().enumerate() {
                if *keep && m != 0 {
                    seen[m] += 1;
                }
            }
        }
        assert!(seen[1..].iter().all(|&c| c == 1), "{seen:?}");
        assert_eq!(octave(1, n), Some(0));
        assert_eq!(octave(3, n), Some(1));
        assert_eq!(octave(32, n), Some(5));
        assert_eq!(octave(63, n), Some(0), "mode 63 is −1");
    }

    #[test]
    fn hexagram_round_trip_and_novelty() {
        for id in 0..64u8 {
            let f: Function = (0..64).map(|_| c64::from_polar(1.0, phase_of(id))).collect();
            assert_eq!(hexagram_of(&f), id);
        }
        let f = unit_random(64, 9);
        assert!(novelty(&f, 0b11_1111).abs() < 1e-12);
        assert!(novelty(&f, 0) > 0.5, "dropping five octaves is novel");
        assert_eq!(changing_lines(0b10_1010, 0b00_0011), 0b10_1001);
        let e = lines(&f);
        assert!((e.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    }
}
