//! The learned code: every word told at least twice in training gets a code
//! from the exchange routine over its sentences' word pairs. The code grows
//! as deep as the data pays for (`Grow::Paid`) or to a fixed depth; a mode
//! cuts it to 4, 5 or 6 bits. A word told once, or never, is `<unk>` and
//! takes `<unk>`'s code, which is learned too, so a held-out word nobody was
//! told is still read. A sentence's edges are words of their own (`<s>`,
//! `</s>`).
//!
//! With the opening sense (`learn_opening`), a word that opens at least two
//! training sentences is a second unit there, `^do` beside `do`: the channel
//! after `<s>`. The routine then codes a sentence-opening `do` by what follows
//! it there (`do you …`, a question's inversion) apart from `do` elsewhere
//! (`i don't do …`), where one class per word had to hold both.

use crate::unit::Units;
use phical::classes::{halve, Grow, Halving};
use std::collections::HashMap;

/// The bits a mode cuts the code to at most: 6, for 64 classes.
pub const BITS: u32 = 6;
/// The mark of a word's opening sense: `^do` is `do` opening a sentence.
pub const OPENING: &str = "^";

const START: &str = "<s>";
const END: &str = "</s>";
const UNKNOWN: &str = "<unk>";

/// Each known word's code, and `<unk>`'s.
pub struct Code {
    codes: HashMap<String, u16>,
    /// How often each word was told in training.
    told: HashMap<String, usize>,
    unknown: u16,
    /// Whether the words that open sentences have senses of their own there.
    pub opening: bool,
    /// The routine's record: its depth, its classes, the likelihood gained pass by pass.
    pub halving: Halving,
}

impl Code {
    /// Learn the code from the training sentences.
    pub fn learn(sentences: &[&[String]], grow: Grow) -> Code {
        Self::learn_with(sentences, grow, false)
    }

    /// Learn the code with the opening sense: a word that opens at least two
    /// training sentences is coded there as a unit of its own (`^do`).
    pub fn learn_opening(sentences: &[&[String]], grow: Grow) -> Code {
        Self::learn_with(sentences, grow, true)
    }

    fn learn_with(sentences: &[&[String]], grow: Grow, opening: bool) -> Code {
        let mut told: HashMap<String, usize> = HashMap::new();
        for s in sentences {
            for (i, w) in s.iter().enumerate() {
                *told.entry(w.clone()).or_default() += 1;
                match (opening, i) {
                    (true, 0) => *told.entry(format!("{OPENING}{w}")).or_default() += 1,
                    _ => {}
                }
            }
        }
        let mut units = Units::new();
        for edge in [START, END, UNKNOWN] {
            units.id(edge);
        }
        let mut sequences = Vec::with_capacity(sentences.len());
        for s in sentences {
            let sensed = s.first().map(|w| format!("{OPENING}{w}")).filter(|n| opening && told.get(n).is_some_and(|&t| t >= 2));
            // An opening sense already says where it stands, so no `<s>` goes
            // before it: its only neighbour is what follows, and the routine
            // codes it by that (`^do you`, `^use the`), not by the start every
            // opener shares.
            let mut seq = match sensed {
                Some(_) => Vec::new(),
                None => vec![units.id(START)],
            };
            for (i, w) in s.iter().enumerate() {
                let name = match (i, &sensed, told.get(w).is_some_and(|&n| n >= 2)) {
                    (0, Some(n), _) => n.as_str(),
                    (_, _, true) => w.as_str(),
                    (_, _, false) => UNKNOWN,
                };
                seq.push(units.id(name));
            }
            seq.push(units.id(END));
            sequences.push(seq);
        }
        let halving = halve(units.len(), &sequences, grow);
        let codes: HashMap<String, u16> = units.names().iter().enumerate().map(|(i, n)| (n.clone(), halving.codes[i])).collect();
        let unknown = codes[UNKNOWN];
        Code { codes, told, unknown, opening, halving }
    }

    /// A sentence written as classes cut to `bits` bits, each word as itself.
    /// A first word with an opening sense is read on two channels: first in
    /// its opening sense (where it stands: `^do` apart from `^use`), then as
    /// itself (what it is: `use` a verb, `in` a preposition, `if` a
    /// conjunction), so the opening template holds both. Without opening
    /// senses this is each word's class.
    pub fn write(&self, words: &[String], bits: u32) -> Vec<u16> {
        let sense = words.first().and_then(|w| self.codes.get(&format!("{OPENING}{w}")));
        let opening = sense.map(|&c| self.cut(c, bits));
        opening.into_iter().chain(words.iter().map(|w| self.class(w, bits))).collect()
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
