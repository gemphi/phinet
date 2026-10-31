//! The per-class scoring: how well the detector classified, one class at a
//! time, on the frozen labelled set (R8.7, R8.8, R8.9).
//!
//! Four classes are scored — degenerate, contradicting, unsupported,
//! consistent — in the precedence order of R8.1 after the readability test,
//! and each is scored one-versus-rest: for one class, a claim labelled that
//! class and read as that class is a hit, a claim read as that class but
//! labelled another is a false positive, and a claim labelled that class but
//! read as another is a miss. That is eight numbers, reported to three
//! decimal places beside the count of labelled claims in the class.
//!
//! A class holding no labelled claim reports `None` for both numbers. Zero
//! would be a finding — "the detector found none of them" — where the truth
//! is that there were none to find, so the undefined case is carried as
//! undefined rather than rounded into a score.
//!
//! The arithmetic is [`phistd::stats::precision`] and
//! [`phistd::stats::recall`], which already read a hit count against a
//! predicted and an expected count; phisor adds none of its own (R9.13).
//! Only the rounding to the reported three places is local, and that is
//! reporting precision rather than a statistic.
//!
//! The predictions are the [`Reading`]s the detector has already made: the
//! frozen set's false claims are verbatim Oracle answers (R8.7), and the
//! detector read them when they arrived, against the World as it then stood.
//! [`Detector::judge_set`] is the replay path — it reads every claim in the
//! set against a given World and then scores — so a caller cannot score
//! claims the detector never read without seeing every class come back
//! unread.

use super::{Detector, Reading};
use phiecs::quest::{Claim, Standing};
use phiecs::world::World;
use phistd::stats::{precision, recall};
use phistd::{fnv1a, mix64};
use std::path::PathBuf;

/// The four classes R8.7 labels claims with, named in the order
/// [`Detector::score`] reports them.
pub const CLASS: [&str; 4] = ["degenerate", "contradicting", "unsupported", "consistent"];

/// The Bars R8.9 declares, one per class in [`CLASS`]'s order: degenerate
/// recall against 0.95 and contradicting recall against 0.80. No Bar is
/// declared for the other two classes, so neither is judged here — the
/// Recorder reports their four numbers as findings.
pub const RECALL_BAR: [Option<f64>; 4] = [Some(0.95), Some(0.80), None, None];

/// The frozen labelled set of R8.7: 100 to 1,000 claims, at least 40 of them
/// false, every false one taken verbatim from a recorded Oracle answer, with
/// the transcripts it came from named and a fingerprint over the whole set so
/// a modification fails the run.
///
/// The size, the false count and the one-label-per-claim rule are asserted
/// where the set is loaded, against the file; what the type holds is the
/// fingerprint, so a set already in memory can be shown unmodified.
pub struct LabelledSet {
    /// Each claim as `(about, claim, label)`, the label being the one class
    /// of the four that the claim was labelled with by hand.
    pub claims: Vec<(String, Claim, Standing)>,
    pub fingerprint: u64,
    pub source_transcripts: Vec<PathBuf>,
}

/// One class's score: how many claims were labelled it, and its precision
/// and recall one-versus-rest at three decimal places. `None` where the
/// class holds no labelled claim and the number is undefined (R8.8).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClassScore {
    pub labelled: usize,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
}

impl LabelledSet {
    /// A set fingerprinted as it is built, from the claims and the
    /// transcripts they were taken from.
    pub fn new(claims: Vec<(String, Claim, Standing)>, source_transcripts: Vec<PathBuf>) -> Self {
        let fingerprint = Self::fingerprint_of(&claims);
        Self { claims, fingerprint, source_transcripts }
    }

    /// The fingerprint of a labelled set: phistd's `fnv1a` over each claim's
    /// canonical line, mixed in order, so a changed claim, a changed label,
    /// a claim added or removed, or the same claims in another order all
    /// give another number.
    pub fn fingerprint_of(claims: &[(String, Claim, Standing)]) -> u64 {
        claims.iter().fold(0xcbf2_9ce4_8422_2325, |held, (about, claim, label)| {
            let kind = &claim.kind;
            let line = format!("{about}|{kind}|{}|{}|{}", claim.other, claim.weight, name(label));
            mix64(held ^ fnv1a(&line))
        })
    }

    /// Whether the set still prints to the fingerprint it was frozen with.
    pub fn unchanged(&self) -> bool {
        Self::fingerprint_of(&self.claims) == self.fingerprint
    }
}

impl Detector {
    /// Precision and recall one-versus-rest for each of the four classes, to
    /// three decimal places, beside that class's labelled count (R8.8).
    ///
    /// The prediction for a claim is the detector's own latest reading of it.
    /// A claim the detector has not read is counted against its label's
    /// recall — it was there to be found and was not — and toward no class's
    /// precision, since nothing was predicted for it.
    ///
    /// A claim labelled unreadable, which R8.7's four labels do not admit,
    /// belongs to no scored class: it adds to no class's labelled count, and
    /// it tells against the precision of whichever class the detector read
    /// it as.
    pub fn score(&self, set: &LabelledSet) -> [ClassScore; 4] {
        let mut labelled = [0usize; 4];
        let mut predicted = [0usize; 4];
        let mut hit = [0.0f64; 4];
        for (about, claim, label) in &set.claims {
            let told = class(label);
            let read = self.reading(about, claim).and_then(|r| class(&r.standing));
            if let Some(i) = told {
                labelled[i] += 1;
            }
            if let Some(j) = read {
                predicted[j] += 1;
            }
            if let (Some(i), Some(j)) = (told, read) {
                if i == j {
                    hit[i] += 1.0;
                }
            }
        }
        let mut scores = [ClassScore { labelled: 0, precision: None, recall: None }; 4];
        for (i, score) in scores.iter_mut().enumerate() {
            // Both numbers are undefined together: a class nothing was
            // labelled has nothing to be precise or complete about.
            let defined = labelled[i] > 0;
            *score = ClassScore {
                labelled: labelled[i],
                precision: defined.then(|| place(precision(hit[i], predicted[i]))),
                recall: defined.then(|| place(recall(hit[i], labelled[i]))),
            };
        }
        scores
    }

    /// Reads every claim in the set against `world`, then scores: the whole
    /// measurement of R8.8 in one call, with no way to score a set the
    /// detector has not read.
    ///
    /// The set itself is never written to — the claims are read in the order
    /// frozen and the labels are not shown to the classifier.
    pub fn judge_set(&mut self, set: &LabelledSet, world: &World) -> [ClassScore; 4] {
        for (about, claim, _) in &set.claims {
            self.classify(about, claim, world);
        }
        self.score(set)
    }

    /// The detector's latest reading of this claim, matched on the whole
    /// claim including its weight, so an off-scale copy of a claim is not
    /// taken for the readable one.
    fn reading(&self, about: &str, claim: &Claim) -> Option<&Reading> {
        self.seen.iter().rev().find(|r| {
            r.about == about
                && r.claim.kind == claim.kind
                && r.claim.other == claim.other
                && r.claim.weight.to_bits() == claim.weight.to_bits()
        })
    }
}

/// Each class's recall judged against the Bar R8.9 declares for it: `true`
/// where the recall reaches the Bar, `false` where it misses, and `None`
/// where no Bar is declared for the class or the class holds no labelled
/// claim — an undefined recall is not a miss.
pub fn judged(scores: &[ClassScore; 4]) -> [Option<bool>; 4] {
    let mut out = [None; 4];
    for (i, verdict) in out.iter_mut().enumerate() {
        *verdict = match (RECALL_BAR[i], scores[i].recall) {
            (Some(bar), Some(reached)) => Some(reached >= bar),
            _ => None,
        };
    }
    out
}

/// What a standing is called, for a report and for a fingerprint;
/// `"unreadable"` for the standing that is not one of the four classes.
pub fn name(standing: &Standing) -> &'static str {
    match class(standing) {
        Some(i) => CLASS[i],
        None => "unreadable",
    }
}

/// Which of the four scored classes a standing is, by position in [`CLASS`];
/// `None` for unreadable, which R8.12 settles before any class test and
/// which is therefore no class.
fn class(standing: &Standing) -> Option<usize> {
    match standing {
        Standing::Degenerate { .. } => Some(0),
        Standing::Contradicting { .. } => Some(1),
        Standing::Unsupported => Some(2),
        Standing::Consistent => Some(3),
        Standing::Unreadable { .. } => None,
    }
}

/// `x` at the three decimal places R8.8 reports.
fn place(x: f64) -> f64 {
    (x * 1_000.0).round() / 1_000.0
}

#[cfg(test)]
mod tests;
