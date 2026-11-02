//! The line-count bands (R11.6) and the one-word path rule, held as data so
//! the naming rule is enforced rather than remembered.
//!
//! `band` sorts a line count into the three bands R11.6 declares: ok below
//! 201, a warning at 201 to 250 naming the path and count without failing,
//! a failure above 250 naming the path and count. `covered_files` is the
//! set the bands run over — every `Cargo.toml` and every file under each of
//! the ten crates' `src`, each submodule's working tree read as the files
//! on disk it is, vendored, generated and `_temp/` paths excluded (R11.5).
//! `one_word_paths` holds every file and directory under the ten crates'
//! `src` and `tests` trees to the one-word rule — no segment carries an
//! underscore, a hyphen or an upper-case letter — naming the offending path
//! and segment. Like the rest of this target it reads the tree as files.

use std::path::{Path, PathBuf};

/// R11.6's two edges: the warning band opens at 201 lines and the failure
/// band opens above 250.
pub const WARN_FROM: usize = 201;
pub const FAIL_ABOVE: usize = 250;

/// Which band a file's line count falls in. A warning carries the count so
/// the message can name it; a failure likewise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Band {
    Ok,
    Warn { lines: usize },
    Fail { lines: usize },
}

impl Band {
    /// Does this band fail the run? Only the failure band does; the warning
    /// band names the file and lets it pass (R11.6).
    pub fn fails(&self) -> bool {
        matches!(self, Band::Fail { .. })
    }

    /// The message a band carries about a path; `Ok` says nothing.
    pub fn message(&self, path: &Path) -> Option<String> {
        let (at, note) = match self {
            Band::Ok => return None,
            Band::Warn { lines } => (lines, "in the 201 to 250 warning band"),
            Band::Fail { lines } => (lines, "past the cap"),
        };
        Some(format!("{} holds {at} lines, {note} ({FAIL_ABOVE} is the cap, R11.6)", path.display()))
    }
}

/// Sort a line count into its band: ok below 201, warn at 201 to 250, fail
/// above 250.
pub fn band(lines: usize) -> Band {
    if lines > FAIL_ABOVE {
        Band::Fail { lines }
    } else if lines >= WARN_FROM {
        Band::Warn { lines }
    } else {
        Band::Ok
    }
}

/// A segment the one-word rule forbids: an underscore, a hyphen or an
/// upper-case letter in the stem, the extension set aside so `tests.rs`
/// reads as `tests`. Digits are a word's own, so `phi4` and `p64` pass.
pub fn offending_segment(segment: &str) -> bool {
    let name = segment.rsplit_once('.').map_or(segment, |(stem, _)| stem);
    name.chars().any(|c| c == '_' || c == '-' || c.is_ascii_uppercase())
}

/// A path outside the rules' reach: a cargo or git generated tree, a
/// vendored tree, a test fixture's crate, and anything under `_temp/`. These
/// are not the project's to name, so both rules pass over them (R11.5).
pub fn is_excluded(path: &Path) -> bool {
    path.components().any(|c| {
        let seg = c.as_os_str().to_string_lossy();
        matches!(seg.as_ref(), "_temp" | "target" | "vendor" | ".git" | "fixtures")
    })
}

/// The one-word rule sets aside everything `is_excluded` does, and also
/// `phiano/data/`, where the generated phi4 corpus lives under names the
/// rule does not own.
pub fn is_excluded_for_naming(path: &Path) -> bool {
    is_excluded(path) || path.to_string_lossy().replace('\\', "/").contains("/phiano/data/")
}

/// The ten crates' roots under `crates/`.
fn crate_roots() -> Vec<PathBuf> {
    crate::CRATES.iter().map(|c| crate::workspace_root().join("crates").join(c)).collect()
}

/// Every file under `dir`, recursively, skipping excluded trees. A
/// submodule's working tree is the files on disk under its root, so the walk
/// reads each submodule's tree with no special case.
fn walk(dir: &Path, excluded: &dyn Fn(&Path) -> bool, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if excluded(&path) {
            continue;
        }
        if path.is_dir() {
            walk(&path, excluded, out);
        } else {
            out.push(path);
        }
    }
}

/// Every `Cargo.toml` and every file under each of the ten crates' `src`
/// directories, vendored, generated and `_temp/` paths excluded (R11.5).
pub fn covered_files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for root in crate_roots() {
        let manifest = root.join("Cargo.toml");
        if manifest.is_file() {
            out.push(manifest);
        }
        walk(&root.join("src"), &is_excluded, &mut out);
    }
    out.sort();
    out
}

/// Is this path one the bands cover? Every `Cargo.toml` and every
/// unexcluded file under a crate's `src`; nothing else.
pub fn is_covered(path: &Path) -> bool {
    if is_excluded(path) {
        return false;
    }
    let text = path.to_string_lossy().replace('\\', "/");
    let under_src = crate::CRATES.iter().any(|c| text.contains(&format!("/crates/{c}/src/")));
    under_src || text.ends_with("/Cargo.toml") || text == "Cargo.toml"
}

/// Every segment under the ten crates' `src` and `tests` trees that breaks
/// the one-word rule, each named with its path. Vendored, generated,
/// `_temp/` and `phiano/data/` paths are set aside (R11.6).
pub fn one_word_paths() -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for root in crate_roots() {
        for tree in ["src", "tests"] {
            let mut files = Vec::new();
            walk(&root.join(tree), &is_excluded_for_naming, &mut files);
            for file in files {
                for seg in file.strip_prefix(&root).unwrap_or(&file).components() {
                    let seg = seg.as_os_str().to_string_lossy().to_string();
                    if offending_segment(&seg) {
                        out.push((file.clone(), seg));
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

// Property 59 and the edge tests sit in `lines/tests.rs`, a single-word file
// under a folder named for its module, so each file stays under the cap and
// every path segment stays one word.
#[cfg(test)]
mod tests;
