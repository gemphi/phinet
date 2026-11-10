//! The hard 300-line cap on Rust source: no `.rs` file under any crate's
//! `src` or `examples` may hold more than 300 lines. Unlike the R11.6 bands
//! in `lines`, which only warn over the live tree, this rule fails the run
//! and names every offending path with its count.
//!
//! The walk skips `target/` and every other tree `is_excluded` sets aside,
//! and anything under a `.sync/` directory: those hold backup copies, such
//! as `phiano/.sync/backup/facet.pre-intern.rs`, that are not compiled.
use super::is_excluded;
use std::path::{Path, PathBuf};

/// The most lines a Rust source file may hold.
pub const CAP: usize = 300;

/// The trees under each crate the cap covers.
const TREES: [&str; 2] = ["src", "examples"];

/// A path the cap passes over: an `is_excluded` tree or a `.sync/` backup.
pub fn is_skipped(path: &Path) -> bool {
    is_excluded(path) || path.components().any(|c| c.as_os_str() == ".sync")
}

/// Every `.rs` file under `dir`, recursively, skipped trees left out.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if is_skipped(&path) {
            continue;
        }
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every `.rs` file under `crates/*/src` and `crates/*/examples`, sorted.
/// Every directory under `crates/` counts, not only the ten named crates, so
/// a new crate is capped from its first file.
pub fn capped_files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let crates = crate::workspace_root().join("crates");
    let Ok(entries) = std::fs::read_dir(&crates) else { return out };
    for entry in entries.flatten() {
        for tree in TREES {
            walk(&entry.path().join(tree), &mut out);
        }
    }
    out.sort();
    out
}

/// Every capped file over the cap, with its line count.
pub fn over_cap() -> Vec<(PathBuf, usize)> {
    capped_files()
        .into_iter()
        .filter_map(|path| {
            let text = std::fs::read_to_string(&path).ok()?;
            let lines = text.lines().count();
            (lines > CAP).then_some((path, lines))
        })
        .collect()
}

#[test]
fn no_rust_source_file_holds_more_than_300_lines() {
    let files = capped_files();
    assert!(files.len() > crate::CRATES.len(), "capped_files found too few files: {}", files.len());
    let over = over_cap();
    let root = crate::workspace_root();
    let report: Vec<String> = over
        .iter()
        .map(|(path, lines)| format!("  {}: {lines} lines", path.strip_prefix(&root).unwrap_or(path).display()))
        .collect();
    assert!(
        over.is_empty(),
        "{} Rust source file(s) exceed the {CAP}-line cap; split each into files named for what they hold:\n{}",
        over.len(),
        report.join("\n")
    );
}

#[test]
fn the_cap_skips_target_and_sync_backups_and_keeps_source() {
    let base = crate::workspace_root().join("crates").join("phiano");
    assert!(is_skipped(&base.join("target").join("debug").join("built.rs")));
    assert!(is_skipped(&base.join(".sync").join("backup").join("facet.pre-intern.rs")));
    assert!(!is_skipped(&base.join("src").join("lib.rs")));
    assert!(!is_skipped(&base.join("examples").join("riley").join("main.rs")));
}
