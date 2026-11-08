//! No persona code exists in two places (R11.3).
//!
//! Persona logic has one home, `crates/phiano/src/persona`. The three other
//! persona folders (`src/command/persona`, `src/bin/persona`,
//! `examples/persona`) hold only dispatch, entry glue and demo calls. Two
//! checks hold that, both reading source text only:
//!
//! * the named persona items (`sample`, `author_of`, `authors`,
//!   `three_personas`, `held_out`, the `LEXICON` and `STORE` constants) are
//!   defined in at most one file across `phiano/src`, `phiano/examples` and
//!   `phient/src`;
//! * no top-level item name is defined in two files across the four persona
//!   folders, `main` and `run` excepted: each binary has a `main` and each\n//!   subcommand module a `run` entry.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The persona items that must be defined once.
const NAMED_FNS: &[&str] = &["sample", "author_of", "authors", "three_personas", "held_out"];
const NAMED_CONSTS: &[&str] = &["LEXICON", "STORE"];

fn crate_dir(name: &str) -> PathBuf {
    crate::workspace_root().join("crates").join(name)
}

/// The three roots the named items are searched in.
fn scope() -> Vec<PathBuf> {
    vec![
        crate_dir("phiano").join("src"),
        crate_dir("phiano").join("examples"),
        crate_dir("phient").join("src"),
    ]
}

/// The four persona folders.
fn persona_dirs() -> Vec<PathBuf> {
    let p = crate_dir("phiano");
    vec![
        p.join("src").join("persona"),
        p.join("src").join("command").join("persona"),
        p.join("src").join("bin").join("persona"),
        p.join("examples").join("persona"),
    ]
}

/// Every `.rs` file under `dir`, sorted; empty if the folder is absent.
fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(read) = std::fs::read_dir(dir) else { return out };
    for entry in read.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(rs_files(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out.sort();
    out
}

/// Strip the visibility prefix of a definition line, if it has one.
fn without_vis(line: &str) -> &str {
    let rest = line.strip_prefix("pub").unwrap_or(line);
    let rest = match rest.strip_prefix('(') {
        Some(r) => r.split_once(')').map_or(r, |(_, after)| after),
        None => rest,
    };
    rest.trim_start()
}

/// The (kind, name) a line defines, when it begins a definition. A name
/// followed by anything other than the item's own syntax is not a match.
pub fn defined(line: &str) -> Option<(&'static str, String)> {
    let rest = without_vis(line);
    for (kind, kw) in [
        ("fn", "fn "),
        ("fn", "async fn "),
        ("const", "const "),
        ("const", "static "),
        ("type", "struct "),
        ("type", "enum "),
        ("type", "trait "),
        ("type", "type "),
    ] {
        if let Some(r) = rest.strip_prefix(kw) {
            let name: String = r.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            if !name.is_empty() {
                return Some((kind, name));
            }
        }
    }
    None
}

/// Definitions in `text` as (name, kind). `top_only` keeps column-0 lines;
/// otherwise indented lines count as well.
pub fn definitions(text: &str, top_only: bool) -> Vec<(String, &'static str)> {
    text.lines()
        .filter(|l| !top_only || !l.starts_with(char::is_whitespace))
        .filter(|l| !l.trim_start().starts_with("//"))
        .filter_map(|l| defined(l.trim_start()).map(|(k, n)| (n, k)))
        .collect()
}

/// Names defined in more than one file: name -> files.
pub fn duplicated(files: &[(PathBuf, String)], top_only: bool, keep: &dyn Fn(&str, &str) -> bool) -> BTreeMap<String, Vec<PathBuf>> {
    let mut seen: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
    for (path, text) in files {
        for (name, kind) in definitions(text, top_only) {
            if keep(&name, kind) {
                let at = seen.entry(name).or_default();
                if !at.contains(path) {
                    at.push(path.clone());
                }
            }
        }
    }
    seen.retain(|_, v| v.len() > 1);
    seen
}

fn read_all(dirs: &[PathBuf]) -> Vec<(PathBuf, String)> {
    let mut out: Vec<(PathBuf, String)> = Vec::new();
    for f in dirs.iter().flat_map(|d| rs_files(d)) {
        if out.iter().any(|(p, _)| *p == f) {
            continue;
        }
        let text = std::fs::read_to_string(&f).expect("source file is readable");
        out.push((f, text));
    }
    out
}

fn show(map: &BTreeMap<String, Vec<PathBuf>>) -> String {
    map.iter().map(|(n, fs)| format!("{n}: {fs:?}")).collect::<Vec<_>>().join("\n")
}

fn is_named(name: &str, kind: &str) -> bool {
    (kind == "fn" && NAMED_FNS.contains(&name)) || (kind == "const" && NAMED_CONSTS.contains(&name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_persona_items_are_defined_once() {
        let files = read_all(&scope());
        assert!(!files.is_empty(), "no source files found under the scope");
        let dup = duplicated(&files, false, &is_named);
        assert!(dup.is_empty(), "persona items defined in more than one file:\n{}", show(&dup));
    }

    #[test]
    fn persona_folders_share_no_item_name() {
        let files = read_all(&persona_dirs());
        assert!(!files.is_empty(), "no persona folder found");
        let dup = duplicated(&files, true, &|n, _| n != "main" && n != "run");
        assert!(dup.is_empty(), "item names defined in more than one persona file:\n{}", show(&dup));
    }

    #[test]
    fn a_second_definition_is_reported() {
        let a = (PathBuf::from("a.rs"), "pub fn sample(v: u8) {}\n".to_string());
        let b = (PathBuf::from("b.rs"), "fn sample() {}\n    pub const STORE: &str = \"x\";\n".to_string());
        let c = (PathBuf::from("c.rs"), "pub const STORE: &str = \"y\";\n".to_string());
        let dup = duplicated(&[a, b, c], false, &is_named);
        assert_eq!(dup.keys().map(String::as_str).collect::<Vec<_>>(), ["STORE", "sample"]);
    }

    #[test]
    fn a_single_definition_and_other_names_pass() {
        let a = (PathBuf::from("a.rs"), "pub fn sample() {}\nfn other() {}\n".to_string());
        let b = (PathBuf::from("b.rs"), "fn other() {}\n// fn sample() {}\n".to_string());
        assert!(duplicated(&[a, b], false, &is_named).is_empty());
    }
}
