//! What moved between a frozen index table and the live one.
//!
//! Requirement 6 criterion 3 names four ways the table can move — an
//! Operator added, removed, renamed or re-typed — and requires the check to
//! fail until the record is re-recorded. A bare "the files differ" would
//! satisfy the failing but not the naming, so [`between`] reports which of
//! the four happened, at which index, and for a re-typing which field and
//! both of its values.
//!
//! The comparison is position by position, which is sound because both
//! sides are index-ordered bijections: [`table`](super::super::table)
//! asserts that over the live keyboard and [`parse`](super::parse) asserts
//! it over the frozen record.

use super::Row;

/// One difference between the frozen table and the live one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Drift {
    /// The live keyboard holds an Operator at an index the record does not.
    Added { index: u8, name: String },
    /// The record holds an Operator at an index the live keyboard does not.
    Removed { index: u8, name: String },
    /// Same index, different name.
    Renamed { index: u8, frozen: String, live: String },
    /// Same index and name, a different declaration: the family, the arity,
    /// an operand type, the result type, the idempotence, the inverse case or
    /// the tolerance.
    Retyped { index: u8, name: String, field: &'static str, frozen: String, live: String },
}

impl std::fmt::Display for Drift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Drift::Added { index, name } => {
                write!(f, "added: index {index} {name:?} is not in the frozen table")
            }
            Drift::Removed { index, name } => {
                write!(f, "removed: index {index} {name:?} is in the frozen table only")
            }
            Drift::Renamed { index, frozen, live } => {
                write!(f, "renamed: index {index} was {frozen:?}, is now {live:?}")
            }
            Drift::Retyped { index, name, field, frozen, live } => write!(
                f,
                "re-typed: index {index} {name:?} {field} was {frozen:?}, is now {live:?}"
            ),
        }
    }
}

/// Every difference between the frozen rows and the live rows, in index
/// order. An empty result is the record matching the keyboard, which is the
/// only passing outcome.
pub fn between(frozen: &[Row], live: &[Row]) -> Vec<Drift> {
    let mut drifts = Vec::new();
    let reach = frozen.len().max(live.len());
    for at in 0..reach {
        match (frozen.get(at), live.get(at)) {
            (Some(f), Some(l)) => compare(f, l, &mut drifts),
            (None, Some(l)) => {
                drifts.push(Drift::Added { index: l.index, name: l.name.clone() });
            }
            (Some(f), None) => {
                drifts.push(Drift::Removed { index: f.index, name: f.name.clone() });
            }
            (None, None) => unreachable!("at is below the longer of the two lengths"),
        }
    }
    drifts
}

/// Two rows at the same index. A rename is reported alone: when the name has
/// moved, the rest of the row describes a different Operator and reporting
/// its fields as re-typings would bury the rename in noise.
fn compare(frozen: &Row, live: &Row, drifts: &mut Vec<Drift>) {
    if frozen.name != live.name {
        drifts.push(Drift::Renamed {
            index: live.index,
            frozen: frozen.name.clone(),
            live: live.name.clone(),
        });
        return;
    }
    let fields: [(&'static str, String, String); 6] = [
        ("family", frozen.family.clone(), live.family.clone()),
        ("arity", frozen.arity.to_string(), live.arity.to_string()),
        ("operands", frozen.operands.join(","), live.operands.join(",")),
        ("result", frozen.result.clone(), live.result.clone()),
        ("idempotent", frozen.idempotent.to_string(), live.idempotent.to_string()),
        ("inverse", frozen.inverse.clone(), live.inverse.clone()),
    ];
    for (field, was, now) in fields {
        if was != now {
            drifts.push(Drift::Retyped {
                index: live.index,
                name: live.name.clone(),
                field,
                frozen: was,
                live: now,
            });
        }
    }
    let was = tolerance(frozen);
    let now = tolerance(live);
    if was != now {
        drifts.push(Drift::Retyped {
            index: live.index,
            name: live.name.clone(),
            field: "tolerance",
            frozen: was,
            live: now,
        });
    }
}

/// A row's tolerance as the record wrote it, an absent one included, so the
/// two sides compare as text and never as a re-parsed float.
fn tolerance(row: &Row) -> String {
    row.tolerance.clone().unwrap_or_else(|| super::NONE.to_string())
}
