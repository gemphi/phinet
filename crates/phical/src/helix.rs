//! The helix's code: what the 64-fold correspondence says about one channel
//! — its codon (three bases of four), its hexagram, the sphere harmonic's
//! degree and order (`phifno::sphere`), and its place on the B-DNA helix
//! (`phistd::codon`). It was phiano's `helix`; the facet written as a
//! directory tree of strands is `phiecs::phasor::facet::helix`.

use phistd::codon::{codon, helix_position, hexagram_glyph, CODONS};

/// One channel's full code: everything the 64-fold correspondence says about it.
#[derive(Clone, Debug, PartialEq)]
pub struct Codon {
    pub k: usize,
    pub codon: String,
    pub hexagram: String,
    pub degree: usize,
    pub order: isize,
    pub position: [f64; 3],
}

impl Codon {
    pub fn of(k: usize) -> Self {
        Self {
            k,
            codon: codon(k),
            hexagram: hexagram_glyph(k),
            degree: phifno::sphere::degree_of(k),
            order: phifno::sphere::order_of(k),
            position: helix_position(k),
        }
    }

    /// The whole table.
    pub fn table() -> Vec<Codon> {
        (0..CODONS).map(Codon::of).collect()
    }
}
