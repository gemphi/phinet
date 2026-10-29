//! The split: docs/203's rule on the units' sentences. One in ten is held
//! out (`i % 10 == 0`), and nothing is told it or tuned on it. One in twenty
//! is dev (`i % 20 == 5`); the rest is training. Every answer from the Key
//! is held out, because the Key is never told.

use super::{Function, Sentence};

/// Which part of the split a sentence is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Part {
    Train,
    Dev,
    Test,
}

/// The guide's sentences and the part each is in, in the guide's order.
pub struct Split {
    pub sentences: Vec<Sentence>,
    pub parts: Vec<Part>,
}

impl Split {
    /// docs/203's rule over the units' sentences, counted in order; the Key's
    /// answers held out.
    pub fn new(sentences: Vec<Sentence>) -> Self {
        let mut i = 0usize;
        let parts = sentences
            .iter()
            .map(|s| match s.function {
                Function::Answer => Part::Test,
                _ => {
                    let part = match (i % 10, i % 20) {
                        (0, _) => Part::Test,
                        (_, 5) => Part::Dev,
                        _ => Part::Train,
                    };
                    i += 1;
                    part
                }
            })
            .collect();
        Split { sentences, parts }
    }

    /// The indices of one part.
    pub fn of(&self, part: Part) -> Vec<usize> {
        (0..self.parts.len()).filter(|&i| self.parts[i] == part).collect()
    }

    /// The sentence before `i` in the same run and unit, if there is one.
    pub fn before(&self, i: usize) -> Option<usize> {
        let p = i.checked_sub(1)?;
        let (a, b) = (&self.sentences[p], &self.sentences[i]);
        match a.run == b.run && a.unit == b.unit {
            true => Some(p),
            false => None,
        }
    }

    /// The runs of consecutive training sentences: a held-out sentence, a
    /// new unit or the Key breaks a run, so no pair ever spans one.
    pub fn training_runs(&self) -> Vec<Vec<usize>> {
        let mut runs: Vec<Vec<usize>> = Vec::new();
        let mut current: Vec<usize> = Vec::new();
        for i in 0..self.parts.len() {
            let joined = self.before(i).is_some_and(|p| self.parts[p] == Part::Train);
            match (self.parts[i], joined) {
                (Part::Train, true) => current.push(i),
                (Part::Train, false) => {
                    runs.extend((!current.is_empty()).then(|| std::mem::take(&mut current)));
                    current.push(i);
                }
                _ => runs.extend((!current.is_empty()).then(|| std::mem::take(&mut current))),
            }
        }
        runs.extend((!current.is_empty()).then_some(current));
        runs
    }
}
