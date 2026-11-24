//! Identical blocks of code, held as a test rather than as a memory (the
//! ruling of 2026-11-24: duplication is a defect, and a part does one job
//! and nothing else).
//!
//! A scratchpad script found eleven duplications in one afternoon and was
//! lost with the scratchpad, so the scan lives here, beside the line bands
//! and the arithmetic home. It reads the tree as files, as the rest of this
//! target does, and names every run of [`BLOCK`] lines that stands in more
//! than one place: the lines trimmed, the blank ones and the comments gone,
//! so indentation and wording do not hide a copy.
//!
//! The count is pinned in `block.golden`. A new duplication raises it and
//! fails the run; a duplication removed lowers it and fails the run too,
//! so the file records what is left to do and cannot drift either way.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// How many lines make a block worth naming.
pub const BLOCK: usize = 6;

/// A block whose lines come to less than this is a run of punctuation - a
/// few `}`, a `match` arm, a `Some(x) => x` - and says nothing.
pub const SHORTEST: usize = 90;

/// Where the count is pinned, from the workspace root.
pub const GOLDEN: &str = "tests/structural/block.golden";

/// One place a block stands: the path as the workspace reads it, the line
/// its first line is on, and which of the file's code lines that is - the
/// line a reader goes to, and the step a slide is read by.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Place {
    pub path: String,
    pub line: usize,
    pub step: usize,
}

/// Where a group stands, by code line rather than source line: the same
/// group one step on is that group sliding, whatever comments lie between.
fn walk(where_: &[Place], on: usize) -> Vec<(String, usize)> {
    where_.iter().map(|p| (p.path.clone(), p.step + on)).collect()
}

/// The lines of one file as the scan reads them: trimmed, the blank ones
/// and the comments gone, each keeping the line it came from.
pub fn said(path: &Path) -> Vec<(usize, String)> {
    let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
    let kept = |(i, l): (usize, &str)| {
        let said = l.trim().to_string();
        match said.is_empty() || said.starts_with("//") || said.starts_with("#[") {
            true => None,
            false => Some((i + 1, said)),
        }
    };
    text.lines().enumerate().filter_map(kept).collect()
}

/// Every block that stands in more than one place, by the block's own
/// text. A run longer than [`BLOCK`] gives one group, not one for each
/// window of it: a group whose every place is the line after another
/// group's is that group sliding, and only the last of the slide is kept.
pub fn repeated(files: &[PathBuf]) -> BTreeMap<String, Vec<Place>> {
    let mut seen: BTreeMap<String, Vec<Place>> = BTreeMap::new();
    for path in files {
        let lines = said(path);
        let name = named_path(path);
        for (step, window) in lines.windows(BLOCK).enumerate() {
            let block: Vec<&str> = window.iter().map(|(_, l)| l.as_str()).collect();
            match block.iter().map(|l| l.len()).sum::<usize>() < SHORTEST {
                true => continue,
                false => {}
            }
            let place = Place { path: name.clone(), line: window[0].0, step };
            seen.entry(block.join("\n")).or_default().push(place);
        }
    }
    seen.retain(|_, where_| where_.len() > 1);
    let slid: std::collections::BTreeSet<Vec<(String, usize)>> =
        seen.values().map(|w| walk(w, 1)).collect();
    seen.retain(|_, where_| !slid.contains(&walk(where_, 0)));
    seen
}

/// A path as a message names it: from the crate down, so a group reads the
/// same on any machine.
pub fn named_path(path: &Path) -> String {
    let text = path.to_string_lossy().replace('\\', "/");
    match text.find("/crates/") {
        Some(at) => text[at + 1..].to_string(),
        None => text,
    }
}

/// The files the scan covers: every crate's own source, its tests aside -
/// a test may repeat its setup, and a fixture is not code anyone reads.
pub fn scanned() -> Vec<PathBuf> {
    let code = |p: &PathBuf| {
        let text = p.to_string_lossy().replace('\\', "/");
        p.extension().is_some_and(|x| x == "rs")
            && !text.contains("/tests/")
            && !text.ends_with("/tests.rs")
            && !text.contains("/fixture")
    };
    crate::lines::covered_files().into_iter().filter(code).collect()
}

/// The count pinned in `block.golden`, and the whole file for its note.
pub fn pinned() -> usize {
    let text = std::fs::read_to_string(GOLDEN).unwrap_or_default();
    let number = |l: &str| l.trim().parse::<usize>().ok();
    text.lines().find(|l| !l.starts_with('#')).and_then(number).unwrap_or(0)
}

/// The first few groups, largest first, as a message names them.
pub fn named(groups: &BTreeMap<String, Vec<Place>>, most: usize) -> String {
    let mut by_places: Vec<(&String, &Vec<Place>)> = groups.iter().collect();
    by_places.sort_by_key(|(block, where_)| (usize::MAX - where_.len(), block.len()));
    let one = |(block, where_): &(&String, &Vec<Place>)| {
        let places: Vec<String> = where_.iter().map(|p| format!("{}:{}", p.path, p.line)).collect();
        format!("  {}\n    | {}", places.join("  "), block.replace('\n', "\n    | "))
    };
    by_places.iter().take(most).map(one).collect::<Vec<String>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rule: a block of six lines or more stands in one place. What is
    /// left of the sweep of 2026-11-24 is pinned, so neither a new copy nor
    /// a copy removed passes unnoticed.
    #[test]
    fn a_block_of_code_stands_in_one_place() {
        let files = scanned();
        assert!(files.len() > 100, "the scan reads {} files", files.len());
        let groups = repeated(&files);
        let (found, pinned) = (groups.len(), pinned());
        assert_eq!(
            found, pinned,
            "{found} blocks of {BLOCK} lines stand in more than one place, {pinned} pinned in {GOLDEN}.\n\
             A copy made raises the count, a copy removed lowers it: read the groups, say the block \
             once, and write the new count in the golden file.\n{}",
            named(&groups, 6)
        );
    }

    /// What the scan reads of a file: the lines that are code, trimmed.
    #[test]
    fn the_scan_reads_the_code_and_not_the_comments() {
        let path = Path::new("tests/structural/block.rs");
        let lines = said(path);
        assert!(!lines.is_empty(), "this file reads as nothing");
        assert!(lines.iter().all(|(_, l)| !l.starts_with("//") && !l.is_empty()), "a comment got through");
        assert!(lines.iter().all(|(n, _)| *n > 0), "a line is numbered from one");
        assert_eq!(said(Path::new("tests/structural/there-is-no-such-file.rs")), Vec::new());
    }

    /// A block too short to mean anything is not a duplication: two files
    /// that both close a block with `}` have not said the same thing.
    #[test]
    fn a_run_of_punctuation_is_not_a_duplication() {
        let short: usize = ["}", "}", "}", "})", "};", "}"].iter().map(|l| l.len()).sum();
        assert!(short < SHORTEST, "six closing lines come to {short}");
    }
}
