//! The eight trigrams (Bagua / 八卦): a hexagram is two of them, lower
//! lines (bits 0–2) and upper (bits 3–5).

use serde::{Deserialize, Serialize};

/// The 8 fundamental Trigrams (Bagua / 八卦).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Trigram {
    /// 乾 (Qián) — Unity / Creative (111)
    Unity,
    /// 兌 (Duì) — Lake / Joyous (110)
    Lake,
    /// 離 (Lí) — Fire / Clinging / Clarity (101)
    Fire,
    /// 震 (Zhèn) — Thunder / Arousing (100)
    Thunder,
    /// 巽 (Xùn) — Wind / Gentle (011)
    Wind,
    /// 坎 (Kǎn) — Water / Abyssal (010)
    Water,
    /// 艮 (Gèn) — Mountain / Keeping Still (001)
    Mountain,
    /// 坤 (Kūn) — Earth / Receptive (000)
    Earth,
}

impl Trigram {
    /// Returns the 3-bit binary representation (0..7).
    pub fn bits(self) -> u8 {
        match self {
            Self::Earth => 0b000,
            Self::Mountain => 0b001,
            Self::Water => 0b010,
            Self::Wind => 0b011,
            Self::Thunder => 0b100,
            Self::Fire => 0b101,
            Self::Lake => 0b110,
            Self::Unity => 0b111,
        }
    }

    /// Creates a trigram from 3-bit binary value (0..7).
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b111 {
            0b000 => Self::Earth,
            0b001 => Self::Mountain,
            0b010 => Self::Water,
            0b011 => Self::Wind,
            0b100 => Self::Thunder,
            0b101 => Self::Fire,
            0b110 => Self::Lake,
            _ => Self::Unity,
        }
    }

    /// Returns the Unicode symbol for the trigram.
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Unity => "☰",
            Self::Lake => "☱",
            Self::Fire => "☲",
            Self::Thunder => "☳",
            Self::Wind => "☴",
            Self::Water => "☵",
            Self::Mountain => "☶",
            Self::Earth => "☷",
        }
    }

    /// Returns the pinyin name of the trigram.
    pub fn name_pinyin(self) -> &'static str {
        match self {
            Self::Unity => "Qián",
            Self::Lake => "Duì",
            Self::Fire => "Lí",
            Self::Thunder => "Zhèn",
            Self::Wind => "Xùn",
            Self::Water => "Kǎn",
            Self::Mountain => "Gèn",
            Self::Earth => "Kūn",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigram_bits_and_symbols_round_trip() {
        for bits in 0..8u8 {
            assert_eq!(Trigram::from_bits(bits).bits(), bits);
        }
        assert_eq!((Trigram::Unity.bits(), Trigram::Unity.symbol()), (0b111, "☰"));
        assert_eq!((Trigram::Earth.bits(), Trigram::Earth.symbol()), (0b000, "☷"));
    }
}
