//! `set` — the frozen labelled claim set loader (R8.7).
//!
//! [`score::LabelledSet`] is the set in memory, fingerprinted as it is built.
//! This module is the one path that reads it from the frozen file, asserting
//! every well-formedness rule R8.7 names before the detector is allowed to
//! run on it:
//!
//! - the size is 100 to 1,000 claims;
//! - at least 40 are labelled false — one of the three falsehood classes
//!   (degenerate, contradicting, unsupported), not consistent;
//! - each claim carries exactly one of the four class labels;
//! - the fingerprint reads the pinned [`FINGERPRINT`], so editing a row,
//!   adding one or dropping one fails the load rather than quietly moving
//!   the yardstick.
//!
//! The frozen file is `phisor/tests/claims.tsv`: six tab-separated columns,
//! described in full in its own header. It sits under `tests/` because it is
//! a frozen input rather than source, and is read here with `include_str!`
//! so the fingerprint is over the bytes the build saw. The fingerprint is
//! `phistd::mix64` folded over the canonical claim lines of the parsed set
//! ([`LabelledSet::fingerprint_of`]), so a line ending a checkout chose
//! cannot move the number.

use super::score::LabelledSet;
use phiecs::quest::Standing;

mod read;
#[cfg(test)]
mod tests;

pub use read::{Flaw, Labelled};

/// Where the frozen file lives, as a failure must name it.
pub const PATH: &str = "phinet/crates/phisor/tests/claims.tsv";

/// The least and most claims R8.7 admits.
pub const MIN: usize = 100;
pub const MAX: usize = 1_000;

/// The least claims R8.7 requires to be labelled false.
pub const MIN_FALSE: usize = 40;

/// The pinned fingerprint of the frozen set, from [`LabelledSet::fingerprint_of`].
pub const FINGERPRINT: u64 = 0x6146_367e_bc57_22f7;

/// The frozen file's bytes, as the build saw them.
const TEXT: &str = include_str!("../../../tests/claims.tsv");

/// Read the frozen set, asserting the size, the false count, the one label
/// per claim and the pinned fingerprint. Every false claim's source
/// transcript is carried on the returned set.
pub fn read() -> Result<LabelledSet, Flaw> {
    of(TEXT)
}

/// The same loader over given text, so a test can show what a changed row,
/// a dropped row or an out-of-range size does without editing the frozen
/// file.
pub fn of(text: &str) -> Result<LabelledSet, Flaw> {
    let rows = read::all(text)?;
    let size = rows.len();
    if !(MIN..=MAX).contains(&size) {
        return Err(Flaw::Size { read: size });
    }
    let false_count = rows.iter().filter(|r| is_false(&r.label)).count();
    if false_count < MIN_FALSE {
        return Err(Flaw::TooFewFalse { read: false_count });
    }
    let sources = rows
        .iter()
        .filter_map(|r| r.source.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(std::path::PathBuf::from)
        .collect();
    let claims = rows.into_iter().map(|r| (r.about, r.claim, r.label)).collect();
    let set = LabelledSet::new(claims, sources);
    match set.fingerprint {
        FINGERPRINT => Ok(set),
        read => Err(Flaw::Fingerprint { read, declared: FINGERPRINT }),
    }
}

/// Whether a label is one of the three falsehood classes R8.7 counts as
/// false: degenerate, contradicting or unsupported, but not consistent.
pub fn is_false(label: &Standing) -> bool {
    !matches!(label, Standing::Consistent)
}

/// The label a word names, for the loader and for a failure; `None` for a
/// word that is not one of the four class labels.
pub(crate) fn label_of(word: &str) -> Option<Standing> {
    match word {
        "degenerate" => Some(Standing::Degenerate { test: phiecs::quest::Degeneracy::EmptyField }),
        "contradicting" => Some(Standing::Contradicting { rows: Vec::new() }),
        "unsupported" => Some(Standing::Unsupported),
        "consistent" => Some(Standing::Consistent),
        _ => None,
    }
}

/// The four class names a label column admits, for a failure that lists them.
pub(crate) fn labels() -> String {
    ["degenerate", "contradicting", "unsupported", "consistent"].join(" ")
}
