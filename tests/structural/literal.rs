//! The language-literal scan (Requirement 8), run as a report.
//!
//! It reads `phisor/src` outside `#[cfg(test)]` code and test files and
//! finds hard-coded language data of three kinds:
//!
//! - `array`: a `const` or `static` whose type holds `&str` in a slice or
//!   array and whose body holds a word;
//! - `match`: a `match` arm or `matches!` over a literal English word;
//! - `eq`: a `==` or `!=` against a literal English word.
//!
//! The scan prints every hit as `file:line kind text`, then the count per
//! file and the total, and does not fail on them: task 11.3 turns the report
//! into an assertion once the cleanup (tasks 10.2 to 10.9) has taken the
//! count to zero. `ALLOWED` holds the non-language identifiers the array
//! rule passes over, each with its reason (R8.4).
//!
//! Run it with `cargo test --test structural literal -- --nocapture`.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Const and static names that hold no language, each with its reason.
pub const ALLOWED: &[(&str, &str)] = &[
    ("FEATURES", "names of feature slots, not words of a language"),
    ("KINDS", "names of Rust-side enum kinds used as report labels"),
    ("MODES", "names of run modes, not words of a language"),
    ("CLASS", "names of function classes, an internal label set"),
    ("TERMINATORS", "sentence-final punctuation marks, not words"),
    ("OCODE_NAMES", "mnemonics of the machine's operation codes"),
];

/// One finding: where, which rule, and the source line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    pub file: String,
    pub line: usize,
    pub kind: &'static str,
    pub text: String,
}

/// Is this file a test file? Test trees and `tests.rs`/`*_test.rs` files are
/// passed over, like `#[cfg(test)]` code.
fn is_test_file(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    name == "tests.rs"
        || name.ends_with("_test.rs")
        || name.ends_with("_tests.rs")
        || path.components().any(|c| c.as_os_str() == "tests")
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") && !is_test_file(&path) {
            out.push(path);
        }
    }
}

/// Every non-test `.rs` file under `phisor/src`, in path order.
pub fn scanned_files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(&crate::workspace_root().join("crates").join("phisor").join("src"), &mut out);
    out.sort();
    out
}

/// The lines of a source with `#[cfg(test)]` items and comments left out,
/// each kept with its 1-based line number.
fn live_lines(source: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let (mut skipping, mut depth, mut opened) = (false, 0i32, false);
    for (i, line) in source.lines().enumerate() {
        let t = line.trim();
        if skipping {
            depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
            opened |= line.contains('{');
            if (opened && depth <= 0) || (!opened && t.ends_with(';')) {
                skipping = false;
                (depth, opened) = (0, false);
            }
            continue;
        }
        if t.starts_with("#[cfg(test)]") {
            skipping = true;
            continue;
        }
        if t.starts_with("//") || t.is_empty() {
            continue;
        }
        out.push((i + 1, line));
    }
    out
}

/// The string literals on a line (no escapes handled beyond `\"`).
fn literals(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let (mut cur, mut inside, mut esc) = (String::new(), false, false);
    for ch in line.chars() {
        match (inside, esc, ch) {
            (true, true, _) => esc = false,
            (true, false, '\\') => esc = true,
            (true, false, '"') => {
                out.push(std::mem::take(&mut cur));
                inside = false;
            }
            (true, false, c) => cur.push(c),
            (false, _, '"') => inside = true,
            _ => {}
        }
    }
    out
}

/// A literal English word: two or more letters, lower case, an apostrophe
/// or hyphen allowed inside. Format strings, paths and symbols are not.
fn is_word(s: &str) -> bool {
    s.len() >= 2
        && s.chars().all(|c| c.is_ascii_lowercase() || c == '\'' || c == '-')
        && s.chars().next().is_some_and(|c| c.is_ascii_lowercase())
}

fn has_word(line: &str) -> bool {
    literals(line).iter().any(|s| is_word(s))
}

/// The name of a `const`/`static` declaration whose type holds `&str` in a
/// slice or array, if this line opens one.
fn array_name(line: &str) -> Option<&str> {
    let t = line.trim().trim_start_matches("pub ").trim_start_matches("pub(crate) ");
    let rest = t.strip_prefix("const ").or_else(|| t.strip_prefix("static "))?;
    let (name, ty) = rest.split_once(':')?;
    let ty = ty.split('=').next()?;
    (ty.contains('[') && ty.contains("&str")).then_some(name.trim())
}

/// Scan one source text; `file` is the name hits carry.
pub fn scan_source(file: &str, source: &str) -> Vec<Hit> {
    let lines = live_lines(source);
    let mut hits = Vec::new();
    let mut hit = |n: usize, kind: &'static str, l: &str| {
        hits.push(Hit { file: file.into(), line: n, kind, text: l.trim().into() })
    };
    let mut i = 0;
    while i < lines.len() {
        let (n, line) = lines[i];
        let t = line.trim();
        if let Some(name) = array_name(line) {
            let allowed = ALLOWED.iter().any(|(a, _)| *a == name);
            // The body runs to the line that ends the statement.
            let mut j = i;
            while j < lines.len() && !lines[j].1.trim_end().ends_with(';') {
                j += 1;
            }
            let end = j.min(lines.len() - 1);
            if !allowed && lines[i..=end].iter().any(|(_, l)| has_word(l)) {
                hit(n, "array", line);
            }
            i = end + 1;
            continue;
        }
        let code = line.split(" // ").next().unwrap_or(line);
        let arm = t.starts_with('"') && code.contains("=>") && has_word(code.split("=>").next().unwrap_or(""));
        let mac = code.contains("matches!(") && has_word(code);
        if arm || mac {
            hit(n, "match", line);
        } else if (code.contains("== \"") || code.contains("!= \"")) && has_word(code) {
            hit(n, "eq", line);
        }
        i += 1;
    }
    hits
}

/// Every hit under `phisor/src`.
pub fn scan() -> Vec<Hit> {
    let root = crate::workspace_root();
    let mut all = Vec::new();
    for path in scanned_files() {
        let Ok(source) = std::fs::read_to_string(&path) else { continue };
        let name = path.strip_prefix(&root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
        all.extend(scan_source(&name, &source));
    }
    all
}

/// The count per file, in file order.
pub fn by_file(hits: &[Hit]) -> BTreeMap<&str, usize> {
    let mut m = BTreeMap::new();
    for h in hits {
        *m.entry(h.file.as_str()).or_insert(0) += 1;
    }
    m
}

#[test]
fn the_language_literal_scan_reports_every_hit() {
    let files = scanned_files();
    assert!(!files.is_empty(), "no source under crates/phisor/src was scanned");
    let hits = scan();
    for h in &hits {
        println!("{}:{} {} {}", h.file, h.line, h.kind, h.text);
    }
    for (file, n) in by_file(&hits) {
        println!("count {n:>4} {file}");
    }
    println!("language literal hits: {} in {} files scanned", hits.len(), files.len());
}

#[test]
fn the_scan_finds_each_kind_and_skips_test_code_and_the_allowlist() {
    let src = "const WORDS: &[&str] = &[\n    \"the\",\n];\n\
               const KINDS: &[&str] = &[\"alpha\"];\n\
               fn f(w: &str) -> bool {\n    match w {\n        \"was\" | \"were\" => true,\n        _ => w == \"wish\",\n    }\n}\n\
               #[cfg(test)]\nmod tests {\n    const T: &[&str] = &[\"skip\"];\n}\n";
    let kinds: Vec<_> = scan_source("x.rs", src).iter().map(|h| (h.line, h.kind)).collect();
    assert_eq!(kinds, vec![(1, "array"), (7, "match"), (8, "eq")]);
}
