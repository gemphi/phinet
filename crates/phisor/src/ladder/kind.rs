//! The Variation and Kind heights (R5.4, R5.5, R5.17).
//!
//! A [`Variation`] is exactly three Mode classes: the first, second and
//! last class of a shaped sentence. The River is supplied as separate
//! training runs. [`river`] takes adjacent pairs inside each run, so neither
//! a held-out sentence nor a run boundary can create a pair. Those pairs are
//! handed to the same `phical::classes` exchange routine used at the word
//! height, with the caller's [`Settings`] copied unchanged.
//!
//! Kind reports are built from variations alone. Labels are neither read nor
//! accepted here, and every occupied Kind is retained with its variations.
//! A later scorer may therefore find no function or terminator label for a
//! Kind without causing that Kind to disappear.

use super::{Arm, Ladder, SHAPED};
use phical::classes::{halve_with, Grow, Halving, Settings};
use phiton::p64::variants::Level;
use std::collections::{BTreeMap, HashMap};

#[cfg(test)]
mod tests;

/// A shaped sentence at one named Mode: first, second and last class, in
/// that order (R5.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variation {
    pub level: Level,
    pub classes: [u16; 3],
}

/// One occupied Kind and every training Variation assigned to it. There is
/// deliberately no label field: labels do not select report rows (R5.17).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KindGroup {
    pub kind: usize,
    pub variations: Vec<Variation>,
}

/// The Kind height for one arm and Mode.
#[derive(Clone, Debug)]
pub struct KindReport {
    pub arm: Arm,
    pub level: Level,
    /// The word-height settings, copied without alteration for the same
    /// exchange routine at this height.
    pub settings: Settings,
    /// Only immediate-predecessor pairs consumed by the exchange routine.
    pub pairs: Vec<(usize, usize)>,
    pub halving: Halving,
    pub occupied: usize,
    pub kinds: Vec<KindGroup>,
    /// A sentence has a Kind only when it is shaped and its Variation was
    /// present in training. No held-out Variation is added to the learner.
    sentence_kinds: Vec<Option<usize>>,
}

/// Read a sentence already written at `level` as first, second and last
/// class, in that order. Fewer than three classes is unshaped (R5.4, R5.15).
pub fn variation(written: &[u16], level: Level) -> Option<Variation> {
    if written.len() < SHAPED {
        return None;
    }
    Some(Variation {
        level,
        classes: [written[0], written[1], written[written.len() - 1]],
    })
}

/// Every adjacent pair inside each run, and no pair between runs (R5.5).
pub fn river(runs: &[Vec<usize>]) -> Vec<(usize, usize)> {
    runs
        .iter()
        .flat_map(|run| run.windows(2).map(|pair| (pair[0], pair[1])))
        .collect()
}

/// Group the training Variations by the River with the same exchange
/// settings used at the word height. `written` may include held-out
/// sentences, but only indices named by `runs` become learner units.
pub fn kinds(
    arm: Arm,
    level: Level,
    written: &[Vec<u16>],
    runs: &[Vec<usize>],
    settings: &Settings,
) -> KindReport {
    let variations: Vec<Option<Variation>> = written
        .iter()
        .map(|sentence| variation(sentence, level))
        .collect();

    // Intern every shaped training Variation in first-seen River order.
    // Even a singleton with no pair remains a reported Variation (R5.17).
    let mut units = HashMap::<Variation, usize>::new();
    let mut names = Vec::<Variation>::new();
    for &sentence in runs.iter().flatten() {
        let Some(Some(shape)) = variations.get(sentence) else {
            continue;
        };
        if !units.contains_key(shape) {
            let unit = names.len();
            units.insert(*shape, unit);
            names.push(*shape);
        }
    }

    let unit_of: Vec<Option<usize>> = variations
        .iter()
        .map(|shape| shape.and_then(|shape| units.get(&shape).copied()))
        .collect();
    let pairs: Vec<(usize, usize)> = river(runs)
        .into_iter()
        .filter(|&(before, after)| {
            unit_of.get(before).and_then(|unit| *unit).is_some()
                && unit_of.get(after).and_then(|unit| *unit).is_some()
        })
        .collect();
    // One two-unit sequence per sentence pair makes the consumed multiset
    // explicit and cannot bridge across an unshaped sentence.
    let sequences: Vec<Vec<usize>> = pairs
        .iter()
        .map(|&(before, after)| vec![unit_of[before].unwrap(), unit_of[after].unwrap()])
        .collect();
    let halving = halve_with(names.len(), &sequences, Grow::Paid, settings);

    let mut codes: Vec<u16> = halving.codes.clone();
    codes.sort_unstable();
    codes.dedup();
    let dense: BTreeMap<u16, usize> = codes
        .into_iter()
        .enumerate()
        .map(|(kind, code)| (code, kind))
        .collect();
    let unit_kinds: Vec<usize> = halving
        .codes
        .iter()
        .map(|code| dense[code])
        .collect();
    let mut groups: Vec<KindGroup> = (0..dense.len())
        .map(|kind| KindGroup {
            kind,
            variations: Vec::new(),
        })
        .collect();
    for (unit, shape) in names.into_iter().enumerate() {
        groups[unit_kinds[unit]].variations.push(shape);
    }
    let sentence_kinds = unit_of
        .into_iter()
        .map(|unit| unit.map(|unit| unit_kinds[unit]))
        .collect();
    let occupied = groups.len();

    KindReport {
        arm,
        level,
        settings: *settings,
        pairs,
        halving,
        occupied,
        kinds: groups,
        sentence_kinds,
    }
}

impl KindReport {
    /// The Kind assigned to sentence `sentence`, when that sentence is
    /// shaped and its Variation was present in training.
    pub fn of(&self, sentence: usize) -> Option<usize> {
        self.sentence_kinds.get(sentence).copied().flatten()
    }

    /// Every Variation reported for an occupied Kind. No label lookup can
    /// remove the Kind because this report contains no label filter.
    pub fn variations(&self, kind: usize) -> &[Variation] {
        self.kinds
            .get(kind)
            .map_or(&[], |group| group.variations.as_slice())
    }
}

impl Ladder<'_> {
    /// Read a written sentence as a Variation at its named Mode.
    pub fn variation(&self, written: &[u16], level: Level) -> Option<Variation> {
        variation(written, level)
    }

    /// Run one arm's Kind height with the supplied word-height settings
    /// unchanged.
    pub fn kinds(&self, arm: Arm, level: Level, settings: &Settings) -> KindReport {
        let coder = self.coder(arm, Grow::Paid);
        let written: Vec<Vec<u16>> = self
            .split()
            .sentences
            .iter()
            .map(|sentence| coder.write(&sentence.words, level))
            .collect();
        kinds(
            arm,
            level,
            &written,
            &self.split().training_runs(),
            settings,
        )
    }

    /// Report the Kind height for HashNull, Given and Learned, in that order.
    pub fn all_kinds(&self, level: Level, settings: &Settings) -> [KindReport; 3] {
        Arm::ALL.map(|arm| self.kinds(arm, level, settings))
    }
}
