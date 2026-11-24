//! Learned phases for the twins (docs/phisor/kinds.md, "Learned phases").
//! The keyboard's `twins` turns the closing's arrow onto the opening's, so
//! the two ends can only reinforce. Here each kind's closing arrow is turned
//! by a phase of its own before the ends are added (phical's `intensity`),
//! so the arrows can cancel too. Only the relative phase matters, so the
//! opening's phases stay at 0.
//!
//! A phase is one of sixteen, a sixteenth of a turn apart (phical's q64
//! symbols 0, 4 … 60). The learner is coordinate ascent: one kind at a time,
//! every phase is tried, and one is kept only when Liskov's staged `better`
//! says the training sentences read better, with more of them peaking at
//! their own both-ends kind. No label is read: the target is the kind the
//! clustering already gave each training sentence. Counting and indexing
//! only; the arrows and their sums are phical's.

use super::{assignment_purity, greatest, ModeInput};
use crate::guide::Function;
use phical::c64;
use phical::circle::{amplitude, intensity};
use phical::liskov::staged::{better, Staged};
use phical::q64::lift;

/// A training sentence read from both ends, and the kind the clustering gave it.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub open: Vec<f64>,
    pub close: Vec<f64>,
    pub kind: usize,
}

/// The sixteen phases a kind may take: q64 symbols a sixteenth of a turn apart.
pub const TURN: [u8; 16] = [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60];
/// The most passes over the kinds.
pub const MAX_PASSES: usize = 16;

/// What the learner kept: each kind's phase as a q64 symbol, the training
/// sentences read as their own kind before the first pass and after each,
/// and how many kinds each pass turned.
#[derive(Clone, Debug, PartialEq)]
pub struct Phases {
    pub symbol: Vec<u8>,
    pub hits: Vec<u64>,
    pub changed: Vec<usize>,
}

impl Phases {
    /// Each kind's phase as an angle.
    pub fn angles(&self) -> Vec<f64> {
        self.symbol.iter().map(|&s| lift(s)).collect()
    }

    /// How many kinds were turned off phase 0, and how many of those sit a
    /// quarter turn or more away (symbols 16 to 48), where the two ends
    /// start to cancel.
    pub fn turned(&self) -> (usize, usize) {
        let off = self.symbol.iter().filter(|&&s| s != 0).count();
        let far = self.symbol.iter().filter(|&&s| (16..=48).contains(&s)).count();
        (off, far)
    }
}

/// The peak of an intensity with entry `k` read as `x`: the first of the
/// largest, as `greatest` reads it.
fn peak_with(v: &[f64], k: usize, x: f64) -> usize {
    let mut best = 0;
    for j in 0..v.len() {
        let at = |i: usize| match i == k {
            true => x,
            false => v[i],
        };
        match at(j).total_cmp(&at(best)) {
            std::cmp::Ordering::Greater => best = j,
            _ => {}
        }
    }
    best
}

/// One kind's intensity for a row with the closing's arrow at `theta`.
fn at(open: c64, close: f64, theta: f64) -> f64 {
    intensity(&[open], &amplitude(&[close], &[theta]))[0]
}

/// Learn the phases of `kinds` kinds on the training rows.
pub fn learn(rows: &[Row], kinds: usize) -> Phases {
    let open: Vec<Vec<c64>> = rows.iter().map(|r| amplitude(&r.open, &[])).collect();
    let mut seen: Vec<Vec<f64>> = rows.iter().zip(&open).map(|(r, a)| intensity(a, &amplitude(&r.close, &[]))).collect();
    // The rows a kind's phase can move: both ends give it mass.
    let mut moves: Vec<Vec<usize>> = vec![Vec::new(); kinds];
    for (j, r) in rows.iter().enumerate() {
        for k in 0..kinds {
            match r.open.get(k).is_some_and(|&p| p > 0.0) && r.close.get(k).is_some_and(|&p| p > 0.0) {
                true => moves[k].push(j),
                false => {}
            }
        }
    }
    let mut hit: Vec<bool> = rows.iter().zip(&seen).map(|(r, v)| greatest(v) == r.kind).collect();
    let mut hits = hit.iter().filter(|&&h| h).count() as u64;
    let mut symbol = vec![0u8; kinds];
    let mut out = Phases { symbol: Vec::new(), hits: vec![hits], changed: Vec::new() };
    for _ in 0..MAX_PASSES {
        let mut changed = 0;
        for k in (0..kinds).filter(|&k| !moves[k].is_empty()) {
            let mut best = (symbol[k], Staged { stage: hits, measure: 0.0 });
            for s in TURN.into_iter().filter(|&s| s != symbol[k]) {
                let mut stage = hits;
                for &j in &moves[k] {
                    let now = peak_with(&seen[j], k, at(open[j][k], rows[j].close[k], lift(s))) == rows[j].kind;
                    match (hit[j], now) {
                        (true, false) => stage -= 1,
                        (false, true) => stage += 1,
                        _ => {}
                    }
                }
                let q = Staged { stage, measure: 0.0 };
                match better(best.1, q) {
                    true => best = (s, q),
                    false => {}
                }
            }
            match best.0 != symbol[k] {
                true => {
                    symbol[k] = best.0;
                    for &j in &moves[k] {
                        seen[j][k] = at(open[j][k], rows[j].close[k], lift(symbol[k]));
                        hit[j] = greatest(&seen[j]) == rows[j].kind;
                    }
                    hits = best.1.stage;
                    changed += 1;
                }
                false => {}
            }
        }
        out.hits.push(hits);
        out.changed.push(changed);
        match changed {
            0 => break,
            _ => {}
        }
    }
    out.symbol = symbol;
    out
}

/// The held-out readings combined at the learned phases: purity of the
/// guide's labels in the kinds read at each sentence's peak, to one decimal.
pub fn purity_at(input: &ModeInput, phases: &Phases) -> f64 {
    let angle = phases.angles();
    let kinds: Vec<usize> = input
        .sentences
        .iter()
        .map(|r| greatest(&intensity(&amplitude(&r.opening_probabilities, &[]), &amplitude(&r.closing_probabilities, &angle))))
        .collect();
    let labels: Vec<Function> = input.sentences.iter().map(|r| r.label).collect();
    assignment_purity(&kinds, &labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_phase_is_turned_only_when_the_training_sentences_read_better() {
        // Both ends give kind 0 more mass than kind 1, but every sentence's own
        // kind is 1. In phase, kind 0 peaks (2.4 against 1.6). A quarter turn
        // or more on kind 0 cancels enough of it that kind 1 peaks, so kind
        // 0's phase is turned and every sentence reads as its own kind.
        let row = Row { open: vec![0.6, 0.4], close: vec![0.6, 0.4], kind: 1 };
        let rows = vec![row.clone(); 10];
        let phases = learn(&rows, 2);
        assert_eq!((phases.hits[0], *phases.hits.last().unwrap()), (0, 10));
        // The first phase that reads them all is a quarter turn (symbol 16);
        // a later phase that reads no more is not better, so it is not kept.
        assert_eq!(phases.symbol, [16, 0]);
        assert_eq!(phases.turned(), (1, 1));
        // Already read right, nothing is turned.
        let settled = learn(&[Row { open: vec![0.1, 0.9], close: vec![0.2, 0.8], kind: 1 }], 2);
        assert_eq!((settled.symbol.clone(), settled.changed.clone()), (vec![0, 0], vec![0]));
        assert_eq!(settled.turned(), (0, 0));
    }

    #[test]
    fn the_peak_with_one_entry_changed_reads_as_greatest_does() {
        let v = [0.2, 0.5, 0.5];
        assert_eq!(peak_with(&v, 0, 0.1), greatest(&[0.1, 0.5, 0.5]));
        assert_eq!(peak_with(&v, 2, 0.9), 2);
        assert_eq!(peak_with(&v, 1, 0.0), greatest(&[0.2, 0.0, 0.5]));
    }
}
