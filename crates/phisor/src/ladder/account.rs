//! The sentence accounting of R5.15. A sentence of fewer than three words
//! is **unshaped**: it is excluded from every Kind count and every purity
//! number, and it is counted rather than dropped. Every height above the
//! word height passes its sentences through [`Account::holds`], so the rule
//! is written once here and read, never re-decided.

use crate::guide::Sentence;

/// The fewest words a sentence must hold to be shaped: three, because a
/// Variation is its first, its second and its last class (R5.4, R5.15).
pub const SHAPED: usize = 3;

/// Whether a sentence is shaped: three words or more (R5.15).
pub fn shaped(sentence: &Sentence) -> bool {
    sentence.words.len() >= SHAPED
}

/// Which sentences a height scores and which it leaves out. A sentence of
/// fewer than three words is unshaped: it is excluded from every Kind count
/// and every purity number, and it is counted rather than dropped (R5.15).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Account {
    /// The indices every Kind count and every purity number reads, in the
    /// text's order.
    pub scored: Vec<usize>,
    /// The indices held fewer than [`SHAPED`] words, in the text's order:
    /// reported, never scored.
    pub unshaped: Vec<usize>,
}

impl Account {
    /// The accounting of a text: every sentence in exactly one of the two
    /// lists, so the two counts sum to the text's.
    pub fn of(text: &[Sentence]) -> Account {
        let mut account = Account::default();
        for (i, sentence) in text.iter().enumerate() {
            match shaped(sentence) {
                true => account.scored.push(i),
                false => account.unshaped.push(i),
            }
        }
        account
    }

    /// Whether sentence `i` is scored: the one gate every Kind count and
    /// every purity number passes its sentences through. Both lists are
    /// built in the text's order, so the search is a bisection.
    pub fn holds(&self, i: usize) -> bool {
        self.scored.binary_search(&i).is_ok()
    }

    /// The sentences accounted for: the scored and the unshaped together,
    /// which is the text's own count.
    pub fn total(&self) -> usize {
        self.scored.len() + self.unshaped.len()
    }
}
