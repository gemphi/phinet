//! The abstract stage of the learned layer (design section 5, stage 1):
//! each word maps to a class and each sentence to a variation, its first two
//! classes plus its last. It reuses the class code (`code::Code`, cut at the
//! P16, P32 or P64 levels of `phiton::p64`) and the variation shape
//! (`kind::shape` with `Twin::Both`); nothing is duplicated here.
//!
//! - The class count is read from the data: the exchange routine grows the
//!   code as far as it pays (`Grow::Paid`). 64 is an assumption to test, so
//!   [`Report`] says what the data paid for and whether that reaches 64.
//! - It learns from the train part of the split only. [`Abstraction::train`]
//!   exposes the indices it used, so a test can show they miss the held out.
//! - It is off by default: [`Config::default`] has `enabled: false` and
//!   [`Abstraction::learn`] returns `None` until the flag is turned on.
use crate::code::Code;
use crate::guide::{Part, Split};
use crate::kind::{shape, Twin};
use phical::classes::Grow;
/// How the abstract stage runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// The flag. Off by default: the Reader runs alone.
    pub enabled: bool,
    /// How deep the code grows; `Paid` lets the data say.
    pub grow: Grow,
}
impl Default for Config {
    fn default() -> Self {
        Config { enabled: false, grow: Grow::Paid }
    }
}
/// What the data paid for, against the 64-class assumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Report {
    /// Training sentences the code was learned from.
    pub sentences: usize,
    /// Words that have a code of their own.
    pub words: usize,
    /// Classes the data paid for.
    pub classes: usize,
    /// Bits of depth the data paid for.
    pub depth: u32,
    /// The assumption under test.
    pub assumed: usize,
    /// Whether the data paid for at least the assumed count.
    pub reaches_assumed: bool,
}
/// The learned word classes and the train indices they were learned from.
pub struct Abstraction {
    code: Code,
    train: Vec<usize>,
}
/// The count the design assumes and tests.
pub const ASSUMED_CLASSES: usize = 64;
impl Abstraction {
    /// Learn from the split's train part, or `None` when the flag is off.
    pub fn learn(split: &Split, config: Config) -> Option<Abstraction> {
        config.enabled.then(|| {
            let train = split.of(Part::Train);
            let told: Vec<&[String]> = train.iter().map(|&i| split.sentences[i].words.as_slice()).collect();
            Abstraction { code: Code::learn(&told, config.grow), train }
        })
    }
    /// The indices the code was trained on: the train part, nothing else.
    pub fn train(&self) -> &[usize] {
        &self.train
    }
    /// A word's class, at the depth the data paid for.
    pub fn class(&self, word: &str) -> u16 {
        self.code.class(word, self.code.halving.depth)
    }
    /// A sentence's classes.
    pub fn classes(&self, words: &[String]) -> Vec<u16> {
        words.iter().map(|w| self.class(w)).collect()
    }
    /// A sentence's variation: its first two classes plus its last.
    pub fn variation(&self, words: &[String]) -> String {
        shape(&self.classes(words), Twin::Both)
    }
    /// What the data paid for.
    pub fn report(&self) -> Report {
        let classes = self.code.halving.classes;
        Report {
            sentences: self.train.len(),
            words: self.code.len(),
            classes,
            depth: self.code.halving.depth,
            assumed: ASSUMED_CLASSES,
            reaches_assumed: classes >= ASSUMED_CLASSES,
        }
    }
}
#[cfg(test)]
mod tests;
