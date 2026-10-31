//! The guard itself: the index-and-string comparisons of R10.4, the
//! legitimate-duplicate exemption of R10.5, the cross-cut role check of
//! R10.7, and the per-phase label reads of R10.8/R10.9.

use super::breach::{first, Breach, Counts};
use super::{normalise, role_of, CutId, Phase, Role};
use std::collections::BTreeMap;

/// The guard over the cuts in play: each is a cut's sentences in index order.
pub struct HoldoutGuard {
    cuts: Vec<(CutId, Vec<String>)>,
    /// Counted per phase; the build phase's count must be zero (R10.9).
    label_reads: BTreeMap<(Phase, String), usize>,
}

impl HoldoutGuard {
    pub fn new(cuts: Vec<(CutId, Vec<String>)>) -> Self {
        HoldoutGuard { cuts, label_reads: BTreeMap::new() }
    }

    /// Compares the training input against the test set, the dev set and
    /// The_Key, both by Split_Rule index and by normalised string (R10.4).
    ///
    /// A training string held out under some cut is a breach unless it is a
    /// legitimate duplicate — train under every cut in play (R10.5). The
    /// breach is reported as an [`Breach::IndexOverlap`] when a held-out
    /// *index* carries the string, and as a [`Breach::StringOverlap`]
    /// otherwise. A Key match is never a duplicate (R10.2).
    pub fn check(&mut self, training: &[String], key: &[String]) -> Result<Counts, Breach> {
        self.roles_agree()?;

        let train_norm: Vec<String> = training.iter().map(|s| normalise(s)).collect();
        let key_norm: Vec<String> = key.iter().map(|s| normalise(s)).collect();

        // By string against The_Key: a match is a breach, never a duplicate.
        let key_cases: Vec<(usize, String)> = train_norm
            .iter()
            .enumerate()
            .filter(|(_, t)| key_norm.contains(t))
            .map(|(i, t)| (i, t.clone()))
            .collect();
        if !key_cases.is_empty() {
            return Err(Breach::KeyMatch { total: key_cases.len(), first: first(&key_cases) });
        }

        let mut counts = Counts::default();
        let mut index_breach: Vec<(usize, String)> = Vec::new();
        let mut string_breach: Vec<(usize, String)> = Vec::new();
        for (i, t) in train_norm.iter().enumerate() {
            let (test, dev, by_index) = self.held_roles(t);
            if !test && !dev {
                continue;
            }
            counts.test_string_overlaps += test as usize;
            counts.dev_string_overlaps += dev as usize;
            counts.index_overlaps += by_index as usize;
            // A legitimate duplicate: train under every cut in play (R10.5).
            if self.train_under_every_cut(t) {
                counts.legitimate_duplicates += 1;
            } else if by_index {
                index_breach.push((i, t.clone()));
            } else {
                string_breach.push((i, t.clone()));
            }
        }
        if !index_breach.is_empty() {
            return Err(Breach::IndexOverlap { total: index_breach.len(), first: first(&index_breach) });
        }
        if !string_breach.is_empty() {
            return Err(Breach::StringOverlap { total: string_breach.len(), first: first(&string_breach) });
        }
        Ok(counts)
    }

    /// Whether a normalised string lands in the test set, the dev set, and
    /// whether any such hold is at a held-out *index* (R10.1, R10.4).
    fn held_roles(&self, norm: &str) -> (bool, bool, bool) {
        let (mut test, mut dev, mut by_index) = (false, false, false);
        for (_, sentences) in &self.cuts {
            for (i, s) in sentences.iter().enumerate() {
                if normalise(s) != norm {
                    continue;
                }
                match role_of(i) {
                    Role::Test => (test, by_index) = (true, true),
                    Role::Dev => (dev, by_index) = (true, true),
                    Role::Train => {}
                }
            }
        }
        (test, dev, by_index)
    }

    /// Whether a normalised sentence carries a train index under every cut in
    /// play (R10.5): in every cut the string has at least one train-index
    /// occurrence, so the training copy legitimately came from a train slot
    /// everywhere. A cut that holds the string only at held-out indices means
    /// the only source for it was a held-out sentence — not a duplicate but a
    /// breach.
    fn train_under_every_cut(&self, norm: &str) -> bool {
        self.cuts.iter().all(|(_, sentences)| {
            sentences
                .iter()
                .enumerate()
                .filter(|(_, s)| normalise(s) == norm)
                .any(|(i, _)| role_of(i) == Role::Train)
        })
    }

    /// The role a cut resolves a normalised string to: Train if the string
    /// sits at any train index in that cut (a legitimate-duplicate home,
    /// R10.5), otherwise the held-out role it carries. Two positions of one
    /// string inside one cut are therefore not a disagreement with itself.
    fn cut_role(sentences: &[String], norm: &str) -> Option<Role> {
        let mut found: Option<Role> = None;
        for (i, s) in sentences.iter().enumerate() {
            if normalise(s) != norm {
                continue;
            }
            match role_of(i) {
                Role::Train => return Some(Role::Train),
                r => found = Some(r),
            }
        }
        found
    }

    /// Every cut must resolve the same normalised sentence to the same role
    /// (R10.7); a disagreement across cuts stops the run naming each cut's
    /// count and the count of sentences whose roles differ. The comparison is
    /// across cuts, so a string's two slots inside one cut — the shape of a
    /// legitimate duplicate (R10.5) — are resolved by [`Self::cut_role`]
    /// before any cut is compared to another.
    fn roles_agree(&self) -> Result<(), Breach> {
        let Some(((_, base), rest)) = self.cuts.split_first() else { return Ok(()) };
        let mut differing = 0usize;
        for (_, sentences) in rest {
            let mut seen: BTreeMap<String, ()> = BTreeMap::new();
            for s in sentences {
                let norm = normalise(s);
                if seen.insert(norm.clone(), ()).is_some() {
                    continue;
                }
                let here = Self::cut_role(sentences, &norm);
                let there = Self::cut_role(base, &norm);
                if let (Some(a), Some(b)) = (here, there) {
                    differing += (a != b) as usize;
                }
            }
        }
        match differing {
            0 => Ok(()),
            _ => {
                let per_cut = self.cuts.iter().map(|(c, s)| (*c, s.len())).collect();
                Err(Breach::RoleDisagreement { per_cut, differing })
            }
        }
    }

    /// A label may be read only while a number is scored; every read is
    /// counted against the phase it occurred in (R10.8). A read in the build
    /// phase is a breach that discards the training input (R10.9).
    pub fn read_label(&mut self, phase: Phase, label: &str) -> Result<(), Breach> {
        *self.label_reads.entry((phase, label.to_string())).or_insert(0) += 1;
        match phase {
            Phase::ScoreNumber => Ok(()),
            Phase::BuildTrainingInput => {
                let reads = self
                    .label_reads
                    .iter()
                    .filter(|((p, _), _)| *p == Phase::BuildTrainingInput)
                    .map(|((_, l), n)| (l.clone(), *n))
                    .collect();
                Err(Breach::LabelLeak { reads })
            }
        }
    }

    /// The per-phase label-read counts for the run record (R10.8).
    pub fn per_phase_reads(&self) -> Vec<(Phase, String, usize)> {
        self.label_reads.iter().map(|((p, l), n)| (*p, l.clone(), *n)).collect()
    }
}
