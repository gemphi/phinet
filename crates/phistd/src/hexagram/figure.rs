//! The hexagram as a figure: its 6-bit id, its King Wen number and its two
//! trigrams, over the functions of the module (moved from phiano's phinum,
//! where it had grown its own copies of them).

use super::{changing_lines, glyph, king_wen, names, phase_of, Trigram, HEXAGRAMS};
use serde::{Deserialize, Serialize};
use std::f64::consts::TAU;

/// One of the sixty-four.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hexagram {
    /// Binary id 0..63, the bottom line bit 0.
    pub id: u8,
    /// The King Wen number 1..64.
    pub number: u8,
    /// The upper trigram (bits 3–5), the outer aspect.
    pub upper: Trigram,
    /// The lower trigram (bits 0–2), the inner aspect.
    pub lower: Trigram,
}

impl Hexagram {
    pub fn from_id(id: u8) -> Self {
        let id = id % HEXAGRAMS as u8;
        Self { id, number: king_wen(id), upper: Trigram::from_bits(id >> 3), lower: Trigram::from_bits(id) }
    }

    /// The hexagram whose sector of the circle holds `phase`.
    pub fn from_phase(phase: f64) -> Self {
        let t = phase.rem_euclid(TAU) / TAU;
        Self::from_id(((t * HEXAGRAMS as f64).floor() as usize % HEXAGRAMS) as u8)
    }

    /// The sector's central phase: `from_phase(h.phase_angle()) == h`.
    pub fn phase_angle(self) -> f64 {
        phase_of(self.id)
    }

    /// Turned by `delta` round the circle.
    pub fn spin(self, delta: f64) -> Self {
        Self::from_phase(self.phase_angle() + delta)
    }

    /// The oracle's changing lines: the bits of `mask` flipped.
    pub fn changing_lines(self, mask: u8) -> Self {
        Self::from_id(changing_lines(self.id, mask))
    }

    pub fn unicode_char(self) -> char {
        glyph(self.id)
    }

    pub fn archetype_name(self) -> &'static str {
        names::archetype_name(self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_figure_agrees_with_the_functions() {
        for id in 0..64u8 {
            let h = Hexagram::from_id(id);
            assert_eq!((h.number, h.unicode_char()), (king_wen(id), glyph(id)));
            assert_eq!(h.phase_angle(), phase_of(id));
            assert_eq!(Hexagram::from_phase(h.phase_angle()), h, "the round trip");
            assert_eq!(h.lower.bits() | h.upper.bits() << 3, id);
            assert!(!h.archetype_name().is_empty());
        }
        let receptive = Hexagram::from_id(0);
        assert_eq!((receptive.lower, receptive.upper), (Trigram::Earth, Trigram::Earth));
        assert_eq!(receptive.spin(std::f64::consts::PI).id, 32);
        assert_eq!(receptive.changing_lines(0b000001).id, 1);
    }
}
