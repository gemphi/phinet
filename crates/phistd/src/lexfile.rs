//! A list of words in a file, one per line: what a crate reads instead of
//! holding the words in its code, and what a run appends to when it has
//! learned one (the ruling of 2026-11-07: no data in code — every level of
//! what is known lives in the file system, and the code keeps only the
//! functions that find the abstractions).
//!
//! The format is the plainest there is, so a person can edit it and a diff
//! can read it: one entry per line; a line that is empty or starts with `#`
//! is a comment. An entry is the whole line, spaces and all — "at the
//! moment" is one entry, and "no, " with its trailing space is a different
//! entry from "no,", because a prefix test needs that difference.

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

/// The entries of the file, in order, comments and blanks dropped. A
/// missing file is an empty list, not an error: a crate's rule holds with
/// nothing known, and the first `append` creates the file.
pub fn read(path: impl AsRef<Path>) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(text) => parse(&text),
        Err(_) => Vec::new(),
    }
}

/// The entries of a text in the file's format.
pub fn parse(text: &str) -> Vec<String> {
    text.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).map(str::to_string).collect()
}

/// Adds `entry` to the file unless it is already there. Returns whether
/// it was added. Creates the file, and its folder, when missing.
pub fn append(path: impl AsRef<Path>, entry: &str) -> io::Result<bool> {
    let path = path.as_ref();
    if entry.trim().is_empty() || entry.starts_with('#') || entry.contains('\n') {
        return Ok(false);
    }
    let known: HashSet<String> = read(path).into_iter().collect();
    if known.contains(entry) {
        return Ok(false);
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut text = fs::read_to_string(path).unwrap_or_default();
    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(entry);
    text.push('\n');
    fs::write(path, text)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_list_is_read_appended_and_not_doubled() {
        let path = std::env::temp_dir().join(format!("phistd-lexfile-{}/words.txt", std::process::id()));
        let _ = fs::remove_file(&path);
        assert!(read(&path).is_empty(), "a missing file is an empty list");
        assert_eq!(parse("# the pronouns\ni\n\nno, \nat the moment\r\n"), ["i", "no, ", "at the moment"]);
        assert!(append(&path, "i").unwrap());
        assert!(append(&path, "no, ").unwrap());
        assert!(!append(&path, "i").unwrap(), "already there");
        assert!(!append(&path, "  ").unwrap());
        assert_eq!(read(&path), ["i", "no, "]);
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(path.parent().unwrap());
    }
}
