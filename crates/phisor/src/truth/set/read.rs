//! The grammar of the frozen claim set: a row is six tab-separated cells —
//! about, kind, other, weight, label, source — one claim per row.
//!
//! The reader is forgiving of nothing: a row with the wrong column count, a
//! weight that will not parse, a label that is not one of the four, or a
//! false claim with no source transcript all fail the load naming the row,
//! the column and the cell, so a malformed frozen file is noticed rather
//! than half-read.

use super::{is_false, label_of, labels};
use phiecs::quest::{Claim, Standing};

/// One parsed row of the frozen file: a claim, its hand label, and the
/// transcript a false claim was taken from.
#[derive(Clone, Debug, PartialEq)]
pub struct Labelled {
    pub id: usize,
    pub about: String,
    pub claim: Claim,
    pub label: Standing,
    pub source: Option<String>,
}

/// Why the frozen set did not load. The size, the false count and the
/// fingerprint are the freeze itself; the row flaw names the row, the
/// column and the cell that would not read.
#[derive(Clone, Debug, PartialEq)]
pub enum Flaw {
    Size { read: usize },
    TooFewFalse { read: usize },
    Fingerprint { read: u64, declared: u64 },
    Row { id: usize, column: &'static str, cell: String, why: String },
}

/// Every claim row of the file, in file order, ids from 1. A blank line and
/// a line opening `#` are the header and the notes, not claims.
pub(super) fn all(text: &str) -> Result<Vec<Labelled>, Flaw> {
    let mut out = Vec::new();
    for line in text.lines().map(str::trim_end) {
        match line.is_empty() || line.starts_with('#') {
            true => continue,
            false => out.push(row(out.len() + 1, line)?),
        }
    }
    Ok(out)
}

fn row(id: usize, line: &str) -> Result<Labelled, Flaw> {
    let read: Vec<&str> = line.split('\t').collect();
    let column: [&str; 6] = read
        .try_into()
        .map_err(|r: Vec<&str>| why(id, "row", line, format!("holds {} columns, not 6", r.len())))?;
    let [about, kind, other, weight, label, source] = column;
    let weight: f64 = weight
        .trim()
        .parse()
        .map_err(|_| why(id, "weight", weight, "is not a number".to_string()))?;
    let label = label_of(label.trim())
        .ok_or_else(|| why(id, "label", label, format!("is not one of {}", labels())))?;
    let source = match source.trim() {
        "-" | "" => None,
        s => Some(s.to_string()),
    };
    // Exactly one label per claim is the column itself: one `label` cell, one
    // of the four. A false claim must name the transcript it was taken from
    // (R8.7); a consistent claim is written for the set and names none.
    if is_false(&label) && source.is_none() {
        return Err(why(id, "source", source.as_deref().unwrap_or("-"), "a false claim names no transcript".to_string()));
    }
    if !is_false(&label) && source.is_some() {
        return Err(why(id, "source", source.as_deref().unwrap_or("-"), "a consistent claim names a transcript".to_string()));
    }
    Ok(Labelled {
        id,
        about: about.trim().to_string(),
        claim: Claim { kind: kind.trim().to_string(), other: other.trim().to_string(), weight },
        label,
        source,
    })
}

fn why(id: usize, column: &'static str, cell: &str, why: String) -> Flaw {
    Flaw::Row { id, column, cell: cell.to_string(), why }
}

impl std::fmt::Display for Flaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = super::PATH;
        match self {
            Flaw::Size { read } => write!(
                f,
                "{path} holds {read} claims, not the {}..={} R8.7 admits",
                super::MIN,
                super::MAX
            ),
            Flaw::TooFewFalse { read } => {
                write!(f, "{path} labels {read} claims false, fewer than the {} R8.7 requires", super::MIN_FALSE)
            }
            Flaw::Fingerprint { read, declared } => {
                write!(f, "{path} fingerprints {read:#018x}, not the pinned {declared:#018x}")
            }
            Flaw::Row { id, column, cell, why } => {
                write!(f, "{path} row {id}, column {column}: {cell:?} {why}")
            }
        }
    }
}
