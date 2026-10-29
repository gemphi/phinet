//! The learned code: every word told at least twice in training gets a code
//! from the exchange routine over its sentences' word pairs. The code grows
//! as deep as the data pays for (`Grow::Paid`) or to a fixed depth; a mode
//! cuts it to 4, 5 or 6 bits. A word told once, or never, is `<unk>` and
//! takes `<unk>`'s code, which is learned too, so a held-out word nobody was
//! told is still read. A sentence's edges are words of their own (`<s>`,
//! `</s>`).

use crate::units::Units;
use phical::classes::{halve, Grow, Halving};
use std::collections::HashMap;

/// The bits a mode cuts the code to at most: 6, for 64 classes.
pub const BITS: u32 = 6;

const START: &str = "<s>";
const END: &str = "</s>";
const UNKNOWN: &str = "<unk>";

/// Each known word's code, and `<unk>`'s.
pub struct Code {
    codes: HashMap<String, u16>,
    /// How often each word was told in training.
    told: HashMap<String, usize>,
    unknown: u16,
    /// The routine's record: its depth, its classes, the likelihood gained pass by pass.
    pub halving: Halving,
}

impl Code {
    /// Learn the code from the training sentences.
    pub fn learn(sentences: &[&[String]], grow: Grow) -> Code {
        let mut told: HashMap<&str, usize> = HashMap::new();
        for w in sentences.iter().flat_map(|s| s.iter()) {
            *told.entry(w.as_str()).or_default() += 1;
        }
        let mut units = Units::new();
        for edge in [START, END, UNKNOWN] {
            units.id(edge);
        }
        let mut sequences = Vec::with_capacity(sentences.len());
        for s in sentences {
            let mut seq = vec![units.id(START)];
            for w in s.iter() {
                let known = told.get(w.as_str()).is_some_and(|&n| n >= 2);
                seq.push(match known {
                    true => units.id(w),
                    false => units.id(UNKNOWN),
                });
            }
            seq.push(units.id(END));
            sequences.push(seq);
        }
        let halving = halve(units.len(), &sequences, grow);
        let codes: HashMap<String, u16> = units.names().iter().enumerate().map(|(i, n)| (n.clone(), halving.codes[i])).collect();
        let unknown = codes[UNKNOWN];
        let told = told.into_iter().map(|(w, n)| (w.to_string(), n)).collect();
        Code { codes, told, unknown, halving }
    }

    /// A word's whole code; `<unk>`'s when the word was not told twice.
    pub fn of(&self, word: &str) -> u16 {
        self.codes.get(word).copied().unwrap_or(self.unknown)
    }

    /// A word's class with the code cut to `bits` bits.
    pub fn class(&self, word: &str, bits: u32) -> u16 {
        self.cut(self.of(word), bits)
    }

    fn cut(&self, code: u16, bits: u32) -> u16 {
        match self.halving.depth > bits {
            true => code >> (self.halving.depth - bits),
            false => code,
        }
    }

    /// How many words have a code of their own.
    pub fn len(&self) -> usize {
        self.codes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    /// The words of one class at a cut, most told first: a class read by
    /// its members.
    pub fn members(&self, class: u16, bits: u32) -> Vec<&str> {
        let mut m: Vec<(&str, usize)> = self
            .codes
            .iter()
            .filter(|(_, &c)| self.cut(c, bits) == class)
            .map(|(w, _)| (w.as_str(), self.told.get(w).copied().unwrap_or(0)))
            .collect();
        m.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        m.into_iter().map(|(w, _)| w).collect()
    }
}
