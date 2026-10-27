//! The sixty-four as three bases. A codon is `k` in base four over the
//! bases `ACGT`: `k = 16a + 4b + c`, so `AAA` is 0 and `TTT` is 63 — the
//! same six bits a hexagram reads as lines ([`crate::hexagram`]). Two
//! readings of the sixty-four use it: phiano's helix, where codon `k` is a
//! word's relation channel `k`, and phient's strand, where codon `k` is
//! level `k` of a text, from the word (`AAA`) to the whole story (`TTT`).

/// The four bases, in the order that makes `codon(k)` the base-four digits of `k`.
pub const BASES: [char; 4] = ['A', 'C', 'G', 'T'];
/// Sixty-four.
pub const CODONS: usize = 64;

/// `k` as three bases.
pub fn codon(k: usize) -> String {
    let k = k % CODONS;
    [BASES[k / 16], BASES[(k / 4) % 4], BASES[k % 4]].iter().collect()
}

/// The `k` a codon names, if it is one.
pub fn codon_index(s: &str) -> Option<usize> {
    let mut k = 0usize;
    for ch in s.chars() {
        k = k * 4 + BASES.iter().position(|b| *b == ch.to_ascii_uppercase())?;
    }
    (s.len() == 3).then_some(k)
}

/// The hexagram of `k`: its six bits as lines, bottom first.
pub fn hexagram_glyph(k: usize) -> String {
    // King Wen order, via the one table in `hexagram` (docs/159): codon 0 /
    // id 0 is ䷁ the Receptive, codon 63 / id 63 is ䷀ the Creative.
    crate::hexagram::glyph((k % 64) as u8).to_string()
}

/// B-DNA: codons per turn of the helix.
pub const PITCH_PER_TURN: f64 = 10.5;
/// B-DNA: rise per codon along the axis, in nanometres.
pub const RISE: f64 = 0.34;
/// Radius of the helix, in nanometres.
const RADIUS: f64 = 1.0;

/// Where codon `k` sits on the helix: angle `2πk / 10.5`, rise `0.34 k`.
pub fn helix_position(k: usize) -> [f64; 3] {
    let theta = 2.0 * std::f64::consts::PI * k as f64 / PITCH_PER_TURN;
    [RADIUS * theta.cos(), RADIUS * theta.sin(), RISE * k as f64]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codons_are_base_four_and_round_trip() {
        assert_eq!(codon(0), "AAA");
        assert_eq!(codon(63), "TTT");
        assert_eq!(codon(27), "CGT");
        for k in 0..CODONS {
            assert_eq!(codon_index(&codon(k)), Some(k));
        }
        assert_eq!(codon_index("AAAA"), None);
        assert_eq!(codon_index("AXA"), None);
        assert_eq!(hexagram_glyph(0), "䷁");
        assert_eq!(hexagram_glyph(63), "䷀");
    }

    #[test]
    fn the_helix_turns_every_ten_and_a_half() {
        let a = helix_position(0);
        let b = helix_position(21);
        assert!((a[0] - b[0]).abs() < 1e-9 && (a[1] - b[1]).abs() < 1e-9);
        assert!((b[2] - 21.0 * RISE).abs() < 1e-12);
    }
}
