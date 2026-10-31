//! What the guard reports: the four counts of R10.4 (plus the
//! legitimate-duplicate count of R10.5) when a run is clean, and the breach
//! shapes a run stops on (R10.6, R10.7, R10.9).

use super::CutId;

/// The four counts of R10.4, plus the legitimate-duplicate count of R10.5.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub index_overlaps: usize,
    pub test_string_overlaps: usize,
    pub dev_string_overlaps: usize,
    pub key_matches: usize,
    /// A normalised duplicate carrying a train index under every cut in play:
    /// excluded from the scored set, counted, and not fatal (R10.5).
    pub legitimate_duplicates: usize,
}

/// Every breach stops the run before any weight is written, naming at most
/// the first 20 cases with their Split_Rule indices and the total count
/// (R10.6, R10.7, R10.9).
#[derive(Clone, Debug, PartialEq)]
pub enum Breach {
    IndexOverlap { total: usize, first: Vec<(usize, String)> },
    KeyMatch { total: usize, first: Vec<(usize, String)> },
    StringOverlap { total: usize, first: Vec<(usize, String)> },
    /// Two cuts assign different roles to the same normalised sentence
    /// (R10.7).
    RoleDisagreement { per_cut: Vec<(CutId, usize)>, differing: usize },
    /// A label was read while a training input was built (R10.9).
    LabelLeak { reads: Vec<(String, usize)> },
}

/// At most the first [`FIRST`] cases, as `(index, normalised)` pairs (R10.6).
pub(super) const FIRST: usize = 20;

pub(super) fn first(cases: &[(usize, String)]) -> Vec<(usize, String)> {
    cases.iter().take(FIRST).cloned().collect()
}
