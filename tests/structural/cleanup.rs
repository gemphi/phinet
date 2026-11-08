//! The Cleanup_Map covers every `phiano` module (R10.7), and records the
//! naming rule it applies (R10.8).
//!
//! The map's section 1 is a table whose first column holds each module name
//! in backticks. A top-level entry of `crates/phiano/src` is a folder, or a
//! `.rs` file whose name without the extension is the module. `uncovered`
//! returns the entries the table lacks; the test asserts there are none, so a
//! new module cannot land without a row. Like the rest of this target it
//! reads files only.
use std::collections::BTreeSet;
use std::path::PathBuf;

/// The map, from the workspace root.
fn map_path() -> PathBuf {
    crate::workspace_root().join("docs").join("phiano_cleanup_map.md")
}

/// The module names in section 1's first column: the backticked name that
/// opens each table row between the `## 1.` and `## 2.` headings.
pub fn mapped(text: &str) -> BTreeSet<String> {
    let mut in_section = false;
    let mut names = BTreeSet::new();
    for line in text.lines() {
        if line.starts_with("## ") {
            in_section = line.starts_with("## 1.");
            continue;
        }
        if !in_section {
            continue;
        }
        let Some(rest) = line.strip_prefix("| `") else { continue };
        if let Some((name, _)) = rest.split_once('`') {
            names.insert(name.to_string());
        }
    }
    names
}

/// The top-level entries of `phiano/src`: folders by name, `.rs` files
/// without the extension. Other files are not modules.
pub fn entries() -> BTreeSet<String> {
    let src = crate::workspace_root().join("crates").join("phiano").join("src");
    let mut names = BTreeSet::new();
    for entry in std::fs::read_dir(&src).expect("phiano/src is readable").flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            names.insert(name);
        } else if let Some(stem) = name.strip_suffix(".rs") {
            names.insert(stem.to_string());
        }
    }
    names
}

/// The entries of `entries` that `mapped` does not name.
pub fn uncovered(entries: &BTreeSet<String>, mapped: &BTreeSet<String>) -> Vec<String> {
    entries.difference(mapped).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_map() -> String {
        std::fs::read_to_string(map_path()).expect("the Cleanup_Map is readable")
    }

    fn set(xs: &[&str]) -> BTreeSet<String> {
        xs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn every_phiano_src_entry_has_a_row_in_the_map() {
        let missing = uncovered(&entries(), &mapped(&read_map()));
        assert!(
            missing.is_empty(),
            "phinet/docs/phiano_cleanup_map.md section 1 has no row for these \
             top-level entries of crates/phiano/src: {missing:?}"
        );
    }

    #[test]
    fn a_missing_row_is_reported() {
        let text = "## 1. Module table\n| Module | x |\n|---|---|\n| `arena` | a |\n\
                    | `bin` | b |\n## 2. Next\n| `model` | c |\n";
        let found = mapped(text);
        assert_eq!(found, set(&["arena", "bin"]));
        assert_eq!(uncovered(&set(&["arena", "bin", "model"]), &found), vec!["model"]);
    }

    #[test]
    fn the_map_records_the_naming_rule() {
        let text = read_map();
        for needle in ["`.kiro/steering` holds no `phinet` naming or line-cap rule", "singular names", "`act.rs` split by role"] {
            assert!(text.contains(needle), "the map must say: {needle}");
        }
    }
}
