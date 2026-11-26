//! A night's paths. Each file holds one learned tree, a unit to a line:
//! `name <tab> path <tab> number`, the path the unit's bits from the root
//! (`0110`, `-` for the root itself) and the number what the tree saves
//! beside it (how often a word was told, a template's kind). Nights are
//! folders under `state/`, named by their stamp; the newest is resumed.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

/// The word code's file in a night.
pub const CODE: &str = "code.tsv";

/// The kinds' file in a night, at a mode's bits.
pub fn kind_file(bits: u32) -> String {
    format!("kind-{}.tsv", 1u32 << bits)
}

/// Tonight's folder.
pub fn night(dir: &Path, stamp: &str) -> PathBuf {
    dir.join("state").join(stamp)
}

/// The newest saved night: the last folder under `state/` by name.
pub fn newest(dir: &Path) -> Option<PathBuf> {
    let mut nights: Vec<PathBuf> = std::fs::read_dir(dir.join("state")).ok()?.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_dir()).collect();
    nights.sort();
    nights.pop()
}

fn bits(path: &[bool]) -> String {
    match path.is_empty() {
        true => "-".to_string(),
        false => path
            .iter()
            .map(|&b| match b {
                true => '1',
                false => '0',
            })
            .collect(),
    }
}

/// Save a tree's rows.
pub fn write(file: &Path, rows: &[(&str, Vec<bool>, usize)]) -> io::Result<()> {
    let mut text = String::from("# name\tpath\tnumber\n");
    for (name, path, n) in rows {
        text.push_str(&format!("{name}\t{}\t{n}\n", bits(path)));
    }
    std::fs::create_dir_all(file.parent().unwrap_or(Path::new(".")))?;
    std::fs::write(file, text)
}

/// A saved tree's paths by name; empty when the file is missing.
pub fn read(file: &Path) -> HashMap<String, Vec<bool>> {
    phistd::lexfile::read(file)
        .iter()
        .filter_map(|l| {
            let mut f = l.split('\t');
            let (name, path) = (f.next()?, f.next()?);
            Some((name.to_string(), path.chars().filter(|c| matches!(c, '0' | '1')).map(|c| c == '1').collect()))
        })
        .collect()
}
