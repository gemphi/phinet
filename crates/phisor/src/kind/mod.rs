//! `kind` — the kinds of intention and their variations, found, not named
//! (docs/phisor/kind.md). A sentence written in a mode has a template: how
//! it opens and closes, or the longest opening the guide shows twice. That
//! template is its **variation**. The one routine groups the templates into
//! **kinds**, read one of two ways (`learn`): by the river, which template
//! follows which, or from inside, which class pairs their sentences hold.
//! How many kinds there are is the data's when the routine grows as far as
//! it pays. No label is told; the guide's marks and functions are read
//! against the kinds afterwards (`read`).
//!
//! | file | what |
//! |---|---|
//! | `shape` | `Twin`: a variation read from the opening, the closing, both, or grown |
//! | `learn` | `Reading`: the river's sequences, or the inside's template × class-pair pairs |
//! | `read` | the held-out sentences' kinds; the tables the labels and the river are read from |
//! | `resemblance` | Wittgenstein's family resemblance, counted within a kind |

mod learn;
mod read;
mod resemblance;
mod shape;
#[cfg(test)]
mod tests;

pub use learn::{features, Reading};
pub use read::{mark_of, pairs_table, river, table, Tables};
pub use resemblance::{resemblance, Resemblance};
pub use shape::{grown, grown_length, opening, openings, shape, Twin};

use crate::code::Coder;
use crate::guide::Split;
use crate::units::Units;
use learn::{END, START};
use phical::classes::{halve, Grow, Halving};
use phiton::p64::variants::Level;
use std::collections::HashMap;

/// A variation told this many times in training has a code of its own;
/// rarer ones share `<unk>`'s. A grown template is an opening told this often.
pub const TOLD: usize = 2;

const UNKNOWN: &str = "<unk>";

/// The kinds learned for one arm, mode, template and reading.
pub struct Kinds {
    pub level: Level,
    pub twin: Twin,
    pub reading: Reading,
    /// Every sentence's variation, in the guide's order, as text.
    pub variation: Vec<String>,
    /// The edges and `<unk>` (0, 1, 2), the templates, then any features.
    pub units: Units,
    pub halving: Halving,
    /// Each unit's kind, as a dense id.
    kind: Vec<usize>,
    /// Where the templates end among the units.
    end: usize,
    /// Every sentence in the mode, for backing a grown template off.
    written: Vec<Vec<u16>>,
    /// Each opening of a grown template told, and the most told template beneath it.
    below: HashMap<Vec<u16>, (usize, usize)>,
}

impl Kinds {
    /// Learn the kinds from the training runs of `split`, every sentence
    /// written by `coder` at `level`, its template read as `twin`, the
    /// templates grouped by `reading`.
    pub fn learn(split: &Split, coder: &Coder, level: Level, twin: Twin, reading: Reading, grow: Grow) -> Kinds {
        let written: Vec<Vec<u16>> = split.sentences.iter().map(|s| coder.write(&s.words, level)).collect();
        let runs = split.training_runs();
        let told_openings = openings(runs.iter().flatten().map(|&i| written[i].as_slice()));
        let length: Vec<usize> = written.iter().map(|c| grown_length(c, &told_openings, TOLD)).collect();
        let variation: Vec<String> = written
            .iter()
            .zip(&length)
            .map(|(c, &n)| match twin {
                Twin::Grown => opening(c, n),
                _ => shape(c, twin),
            })
            .collect();
        let mut told: HashMap<&str, usize> = HashMap::new();
        for &i in runs.iter().flatten() {
            *told.entry(variation[i].as_str()).or_default() += 1;
        }
        let mut units = Units::new();
        for edge in [START, END, UNKNOWN] {
            units.id(edge);
        }
        // The templates first, so they are units 3 .. end.
        let unknown = units.id(UNKNOWN);
        let mut unit = vec![unknown; written.len()];
        for &i in runs.iter().flatten() {
            let v = variation[i].as_str();
            unit[i] = match told.get(v).is_some_and(|&n| n >= TOLD) {
                true => units.id(v),
                false => unknown,
            };
        }
        let end = units.len();
        let sequences = match reading {
            Reading::River => learn::river(&runs, &unit, &mut units),
            Reading::Inside => learn::inside(&runs, &unit, &written, &mut units),
        };
        let halving = halve(units.len(), &sequences, grow);
        let mut codes: Vec<u16> = halving.codes.clone();
        codes.sort_unstable();
        codes.dedup();
        let kind = halving.codes.iter().map(|c| codes.binary_search(c).unwrap_or(0)).collect();
        // The trie of the grown templates told: every opening of each
        // template leads to the most told template beneath it.
        let mut below: HashMap<Vec<u16>, (usize, usize)> = HashMap::new();
        for &i in runs.iter().flatten() {
            match (twin, unit[i] != unknown, told.get(variation[i].as_str())) {
                (Twin::Grown, true, Some(&n)) => {
                    for p in 1..=length[i] {
                        let e = below.entry(written[i][..p].to_vec()).or_insert((unit[i], n));
                        match n > e.1 {
                            true => *e = (unit[i], n),
                            false => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Kinds { level, twin, reading, variation, units, halving, kind, end, written, below }
    }

    /// Sentence `i`'s unit: its variation's. A grown template not told twice
    /// as a template backs off along the trie to the most told template that
    /// shares its longest opening, as a back-off model does. `<unk>`'s when
    /// none shares even the first class.
    fn unit(&self, i: usize) -> usize {
        let unknown = self.units.get(UNKNOWN).unwrap_or(0);
        match (self.units.get(&self.variation[i]).filter(|&u| u < self.end), self.twin) {
            (Some(u), _) => u,
            (None, Twin::Grown) => {
                let c = &self.written[i];
                (1..=c.len()).rev().find_map(|n| self.below.get(&c[..n]).map(|&(u, _)| u)).unwrap_or(unknown)
            }
            (None, _) => unknown,
        }
    }

    /// How many kinds there are (the edges', `<unk>`'s and the features' included).
    pub fn count(&self) -> usize {
        self.kind.iter().max().map_or(0, |&k| k + 1)
    }

    /// Sentence `i`'s kind: its unit's (see `unit`).
    pub fn of(&self, i: usize) -> usize {
        self.kind[self.unit(i)]
    }

    /// The templates of one kind.
    pub fn variations(&self, kind: usize) -> Vec<&str> {
        (3..self.end).filter(|&u| self.kind[u] == kind).map(|u| self.units.names()[u].as_str()).collect()
    }

    /// How many templates have a code of their own.
    pub fn templates(&self) -> usize {
        self.end - 3
    }

    /// The templates' names and kinds, in unit order.
    pub fn template_kinds(&self) -> (&[String], &[usize]) {
        (&self.units.names()[3..self.end], &self.kind[3..self.end])
    }

    /// How many kinds hold at least one template.
    pub fn occupied(&self) -> usize {
        let mut seen = vec![false; self.count()];
        for &k in &self.kind[3..self.end] {
            seen[k] = true;
        }
        seen.iter().filter(|&&s| s).count()
    }
}
