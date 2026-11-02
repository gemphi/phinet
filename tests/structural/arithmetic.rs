//! The arithmetic home (R11.7) and the moved-calculation inventory (R11.8),
//! held as data so "where calculations live" is enforced rather than
//! remembered.
//!
//! Two rules sit here, and they pull in opposite directions so that between
//! them every piece of arithmetic has exactly one home. R11.7 forbids a
//! covered source file *outside* phical and phistd from adding arithmetic
//! over one of the eight phisor types — a measure that needs a sum computes
//! it in phical or phistd and calls across, it does not grow a local copy.
//! R11.8 guards the other door: a calculation *moved into* phical or phistd
//! is a real edit to the two crates every other crate leans on, so it is
//! refused until an inventory row names where it came from, where it landed,
//! and the `_temp/` copy that holds the moved text for review.
//!
//! `arithmetic_home_violations` reads each covered file under a crate that
//! is neither phical nor phistd and reports a line that applies an
//! arithmetic operator directly to a value of one of the eight types, naming
//! the path and the crate. The detection is the one `phisor`'s
//! `no_arithmetic_leaves_this_group` uses, lifted here and pinned with a
//! fixture corpus of offending and innocent source text so the reader can
//! see exactly what it catches and what it lets pass.
//!
//! `missing_inventory_rows` holds the inventory to R11.8: every moved
//! calculation this feature made must have a row, and the two changes to
//! `phical/src/classes/halve.rs` (the six-settings struct of task 4.1 and
//! the `liskov::staged` keep decision of task 4.2) each have theirs below.
//! A move whose row is absent, or a row missing any of its three paths, is
//! reported by name.
//!
//! Like the rest of this target the file is the whole input: it reads the
//! ten crates' source text and the inventory's own rows and links no crate.

use std::path::{Path, PathBuf};

use crate::lines::{covered_files, is_excluded};

/// The eight phisor types R11.7 forbids arithmetic over outside phical and
/// phistd. The ninth keyboard type, `Word`, is a handle and carries no
/// arithmetic, so it is not on the list — the same eight
/// `phisor::measure::seeds`'s `no_arithmetic_leaves_this_group` names.
pub const TYPES: &[&str] = &[
    "Phasor", "Class", "Level", "Variation", "Kind", "Act", "Frame", "Row",
];

/// The arithmetic operators the rule watches for: a value of one of the
/// eight types with one of these applied directly to it reads as arithmetic
/// over that type. Division and subtraction join addition and scaling so a
/// local average or difference is caught as readily as a local sum.
pub const OPERATORS: &[char] = &['+', '-', '*', '/'];

/// The two crates arithmetic is allowed to live in (R11.7): phical for the
/// algebra, phistd for the generic numerics. Every other crate calls across
/// to one of these rather than computing over a phisor type itself.
pub const HOMES: &[&str] = &["phical", "phistd"];

/// One moved-calculation inventory row (R11.8): a calculation that landed in
/// phical or phistd, with where it came from, where it landed, and the
/// `_temp/` copy holding the moved text. A move with no row, or a row with
/// an empty field or a destination outside the two homes, is a breach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Moved {
    /// A short name for the move, so a failure can say which one.
    pub what: &'static str,
    /// Where the calculation came from, relative to the workspace root.
    pub origin: &'static str,
    /// Where it landed under phical or phistd, relative to the root.
    pub destination: &'static str,
    /// The `_temp/` path holding the moved copy, relative to the root.
    pub temp: &'static str,
}

/// The inventory: every calculation this feature moved into phical or
/// phistd. The two changes to `phical/src/classes/halve.rs` are the only
/// edits the design asks for inside `phical/src`, and each is a move with
/// its own row — the six-settings struct of task 4.1 and the
/// `liskov::staged` keep decision of task 4.2. The file was split into
/// `halve/mod.rs` as it grew past the cap, so each row names that path as
/// the destination and the pre-split `halve.rs` text as the origin's copy.
pub const INVENTORY: &[Moved] = &[
    Moved {
        what: "the exchange routine's six settings as a struct (task 4.1, R9.1)",
        origin: "crates/phical/src/classes/halve.rs",
        destination: "crates/phical/src/classes/halve/mod.rs",
        temp: "_temp/halve/settings.rs",
    },
    Moved {
        what: "the keep decision routed through liskov::staged (task 4.2, R9.3)",
        origin: "crates/phical/src/classes/halve.rs",
        destination: "crates/phical/src/classes/halve/mod.rs",
        temp: "_temp/halve/staged.rs",
    },
];

/// Is `path` under a crate that is neither phical nor phistd? The home
/// crates may hold any arithmetic; every other crate's covered files are
/// read for a breach of R11.7.
pub fn is_outside_home(path: &Path) -> bool {
    let text = path.to_string_lossy().replace('\\', "/");
    let under_home = HOMES.iter().any(|h| {
        let seg = format!("crates/{h}/");
        text.contains(&format!("/{seg}")) || text.starts_with(&seg)
    });
    !under_home
}

/// Does this code line apply an arithmetic operator directly to a value of
/// one of the eight types? A type name followed, after optional spaces, by
/// one of the four operators — `Phasor +`, `Kind*`, `Row / ` — reads as
/// arithmetic over that type. The scan skips comment and doc lines, which
/// name the types without computing on them, exactly as the phisor group's
/// own guard does.
pub fn adds_arithmetic(code: &str) -> Option<&'static str> {
    let trimmed = code.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with('*') {
        return None;
    }
    let bytes = code.as_bytes();
    for &ty in TYPES {
        let mut from = 0;
        while let Some(at) = code[from..].find(ty) {
            let start = from + at;
            let after = start + ty.len();
            // The type name must be a standalone token, not a slice of a
            // larger word: `Active` must not read as `Act`, nor `Classes` as
            // `Class`. A letter, digit or underscore on either side rules the
            // match out.
            let before_ok = start == 0 || !is_word_byte(bytes[start - 1]);
            let after_ok = after >= bytes.len() || !is_word_byte(bytes[after]);
            if before_ok && after_ok {
                let rest = code[after..].trim_start();
                if let Some(op) = rest.chars().next() {
                    if OPERATORS.contains(&op) {
                        return Some(ty);
                    }
                }
            }
            from = after;
        }
    }
    None
}

/// Is this byte part of an identifier — a letter, a digit or an underscore?
/// Used to hold a type match to a whole token.
fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Every covered source file outside phical and phistd that adds arithmetic
/// over one of the eight types, each named with the type it computes on and
/// the crate it sits in (R11.7). The crate is the segment after `crates/`,
/// so the message can name it as the requirement asks.
pub fn arithmetic_home_violations() -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for path in covered_files() {
        if is_excluded(&path) || !is_outside_home(&path) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines() {
            if let Some(ty) = adds_arithmetic(line) {
                let crate_name = crate_of(&path);
                out.push((path.clone(), format!("{crate_name}: arithmetic over a {ty}")));
                break;
            }
        }
    }
    out.sort();
    out
}

/// The crate a covered path sits in: the one segment after `crates/`, or
/// `the workspace root` for a top-level `Cargo.toml` that belongs to no
/// crate.
pub fn crate_of(path: &Path) -> String {
    let text = path.to_string_lossy().replace('\\', "/");
    for c in crate::CRATES {
        if text.contains(&format!("/crates/{c}/")) {
            return (*c).to_string();
        }
    }
    "the workspace root".to_string()
}

/// Every inventory breach (R11.8): a row missing one of its three paths, or
/// naming a destination outside phical or phistd, is reported by name. A
/// move with no row at all is caught by the test that pins the two halve
/// rows against the inventory — the data here is the record a missing move
/// must be added to.
pub fn missing_inventory_rows() -> Vec<String> {
    let mut out = Vec::new();
    for row in INVENTORY {
        if row.origin.is_empty() {
            out.push(format!("{}: the origin path is empty", row.what));
        }
        if row.destination.is_empty() {
            out.push(format!("{}: the destination path is empty", row.what));
        } else if is_outside_home(Path::new(row.destination)) {
            out.push(format!(
                "{}: the destination `{}` is outside phical and phistd",
                row.what, row.destination
            ));
        }
        if row.temp.is_empty() {
            out.push(format!("{}: the `_temp/` copy path is empty", row.what));
        } else if !row.temp.replace('\\', "/").contains("_temp/") {
            out.push(format!(
                "{}: the copy path `{}` is not under `_temp/`",
                row.what, row.temp
            ));
        }
    }
    out
}

// Property-style fixtures and the live-tree pass sit in `arithmetic/tests.rs`,
// a single-word file under a folder named for its module, so the file stays
// under the cap and every path segment stays one word.
#[cfg(test)]
mod tests;
