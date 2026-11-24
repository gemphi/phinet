//! The Twins readings built from the guide: what [`super::measure_mode`]
//! reads (R9.6, R9.9). The three readings share one space of kinds: the
//! both-ends templates' kinds, read inside (`kind::Reading::Inside`), so a
//! sentence's both-ends reading is its own kind. A single end reads a
//! held-out sentence as the distribution of those kinds over the training
//! sentences that open (or close) the same way; the ends told fewer than
//! `TOLD` times share `<unk>`'s row. Each distribution is made an amplitude
//! at a seeded phase (`phical::circle::amplitude`), and `combine_amplitudes`
//! moves the phases. Counting and indexing only: the shares are phistd's,
//! the amplitudes phical's.

use super::phase::Row;
use super::{greatest, ModeInput, TestReading};
use crate::code::Coder;
use crate::guide::{Part, Split};
use crate::kind::{shape, Kinds, Reading, Twin, TOLD};
use phical::circle::amplitude;
use phical::classes::Grow;
use phistd::stats::share;
use phiton::p64::variants::Level;
use std::collections::HashMap;

const UNKNOWN: &str = "<unk>";

/// One end's counts: for each of its templates, how many training
/// sentences fell in each kind.
struct End {
    row: HashMap<String, Vec<f64>>,
}

impl End {
    fn count(template: &[String], kinds: &Kinds, train: &[usize]) -> End {
        let mut told: HashMap<&str, usize> = HashMap::new();
        for &i in train {
            *told.entry(template[i].as_str()).or_default() += 1;
        }
        let mut row: HashMap<String, Vec<f64>> = HashMap::new();
        for &i in train {
            let name = match told[template[i].as_str()] >= TOLD {
                true => template[i].clone(),
                false => UNKNOWN.to_string(),
            };
            row.entry(name).or_insert_with(|| vec![0.0; kinds.count()])[kinds.of(i)] += 1.0;
        }
        End { row }
    }

    /// A sentence read from this end: its template's row, or `<unk>`'s; no
    /// row at all reads as no evidence.
    fn read(&self, template: &str, kinds: usize) -> Vec<f64> {
        match self.row.get(template).or(self.row.get(UNKNOWN)) {
            Some(r) => share(r),
            None => share(&vec![0.0; kinds]),
        }
    }
}

/// Seeded phases, one per kind: the arguments of phistd's seeded unit phasors.
fn phases(kinds: usize, seed: u64) -> Vec<f64> {
    phistd::unit_phase(kinds, seed).iter().map(|z| z.arg()).collect()
}

/// The held-out readings at one Mode: the kinds learned from the training
/// sentences written by `coder` at `level`, the single ends' distributions
/// over them, and their amplitudes at phases drawn from `seed` (the opening's
/// from `seed`, the closing's from `seed ^ 1`).
pub fn mode_input(split: &Split, coder: &Coder, level: Level, grow: Grow, seed: u64) -> ModeInput {
    readings(split, coder, level, grow, seed).0
}

/// The held-out readings, as [`mode_input`], and the training sentences
/// read the same way with the kind the clustering gave each: what the
/// phases are learned on (`phase::learn`), no label read.
pub fn readings(split: &Split, coder: &Coder, level: Level, grow: Grow, seed: u64) -> (ModeInput, Vec<Row>) {
    let kinds = Kinds::learn(split, coder, level, Twin::Both, Reading::Inside, grow);
    let k = kinds.count();
    let written: Vec<Vec<u16>> = split.sentences.iter().map(|s| coder.write(&s.words, level)).collect();
    let open: Vec<String> = written.iter().map(|c| shape(c, Twin::Open)).collect();
    let close: Vec<String> = written.iter().map(|c| shape(c, Twin::Close)).collect();
    let train = split.of(Part::Train);
    let (opening, closing) = (End::count(&open, &kinds, &train), End::count(&close, &kinds, &train));
    let (open_phase, close_phase) = (phases(k, seed), phases(k, seed ^ 1));
    let sentences = split
        .of(Part::Test)
        .into_iter()
        .map(|i| {
            let (p_open, p_close) = (opening.read(&open[i], k), closing.read(&close[i], k));
            TestReading {
                split_index: i,
                label: split.sentences[i].function,
                opening_kind: greatest(&p_open),
                closing_kind: greatest(&p_close),
                both_kind: kinds.of(i),
                opening_amplitudes: amplitude(&p_open, &open_phase),
                closing_amplitudes: amplitude(&p_close, &close_phase),
                opening_probabilities: p_open,
                closing_probabilities: p_close,
            }
        })
        .collect();
    let rows = train.iter().map(|&i| Row { open: opening.read(&open[i], k), close: closing.read(&close[i], k), kind: kinds.of(i) }).collect();
    (ModeInput { level, sentences }, rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guide::{Function, Mark, Sentence};

    #[test]
    fn every_held_out_sentence_is_read_from_both_ends_in_one_space_of_kinds() {
        let said = ["were you waiting in the rain", "i was waiting there", "did she see the film", "she did not see it"];
        let sentences: Vec<Sentence> = (0..80)
            .map(|i| Sentence { words: said[i % 4].split(' ').map(String::from).collect(), function: Function::Explain, mark: Mark::Stop, unit: i / 4, run: 0 })
            .collect();
        let split = Split::new(sentences);
        let input = mode_input(&split, &Coder::Given, Level::P16, Grow::Paid, 7);
        assert_eq!(input.sentences.len(), split.of(Part::Test).len());
        for r in &input.sentences {
            let k = r.opening_probabilities.len();
            assert_eq!((r.closing_probabilities.len(), r.opening_amplitudes.len()), (k, k));
            assert!((r.opening_probabilities.iter().sum::<f64>() - 1.0).abs() < 1e-12);
            assert!(r.opening_kind < k && r.closing_kind < k && r.both_kind < k);
            // The amplitude's square gives the share back.
            for (a, p) in r.closing_amplitudes.iter().zip(&r.closing_probabilities) {
                assert!((a.norm_sqr() - p).abs() < 1e-12);
            }
        }
        // The same seed reads the same phases.
        assert_eq!(mode_input(&split, &Coder::Given, Level::P16, Grow::Paid, 7), input);
    }
}
