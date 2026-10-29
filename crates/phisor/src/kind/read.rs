//! Reading the kinds against what the guide carries. Nothing here was
//! told: the kinds were learned with no label, and the labels are counted
//! against them only on the sentences of one part (the held-out ones in a
//! report). Counting; purity, mutual information and their nulls are
//! phistd's.

use super::Kinds;
use crate::guide::{Function, Mark, Part, Sentence, Split};

/// Kinds × labels on one part, for both of the guide's label sets.
pub struct Tables {
    /// Rows are the kinds, columns explain, instruct, answer.
    pub function: Vec<Vec<f64>>,
    /// Rows are the kinds, columns stop, question, exclamation, line; the
    /// units' sentences only, since the Key carries no marks (every answer
    /// would read as `line`).
    pub mark: Vec<Vec<f64>>,
}

impl Tables {
    pub fn read(kinds: &Kinds, split: &Split, part: Part) -> Tables {
        let function = table(kinds, split, part, Function::ALL.len(), |s| Function::ALL.iter().position(|&f| f == s.function));
        let mark = table(kinds, split, part, Mark::ALL.len(), mark_of);
        Tables { function, mark }
    }
}

/// A sentence's mark as a column, for the units' sentences only.
pub fn mark_of(s: &Sentence) -> Option<usize> {
    match s.function {
        Function::Answer => None,
        _ => Mark::ALL.iter().position(|&m| m == s.mark),
    }
}

/// A kinds × labels count over the sentences of `part` that have a label.
pub fn table(kinds: &Kinds, split: &Split, part: Part, labels: usize, label: impl Fn(&Sentence) -> Option<usize>) -> Vec<Vec<f64>> {
    let mut t = vec![vec![0.0; labels]; kinds.count()];
    for i in split.of(part) {
        match label(&split.sentences[i]) {
            Some(j) => t[kinds.of(i)][j] += 1.0,
            None => {}
        }
    }
    t
}

/// The river on one part: for every sentence of `part` that has one, the
/// kind of the sentence before it and its own.
pub fn river(kinds: &Kinds, split: &Split, part: Part) -> Vec<(usize, usize)> {
    split.of(part).into_iter().filter_map(|i| split.before(i).map(|p| (kinds.of(p), kinds.of(i)))).collect()
}

/// A list of pairs as a `k × k` count.
pub fn pairs_table(pairs: &[(usize, usize)], k: usize) -> Vec<Vec<f64>> {
    let mut t = vec![vec![0.0; k]; k];
    for &(a, b) in pairs {
        t[a][b] += 1.0;
    }
    t
}
