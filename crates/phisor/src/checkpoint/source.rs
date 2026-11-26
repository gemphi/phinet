//! The queue of text to learn from. `source.tsv` is kept by hand: one
//! source to a line, `name <tab> path`, a path relative to the workspace
//! root or absolute. `taken.tsv` is the run's: one line for each source a
//! night took, `stamp <tab> name <tab> sentences`. A night learns from every
//! source taken so far and takes the next ones not yet taken.

use std::path::{Path, PathBuf};

/// One text in the queue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub name: String,
    pub path: PathBuf,
}

/// The queue as `dir/source.tsv` lists it, in order.
pub fn queue(dir: &Path, root: &Path) -> Vec<Source> {
    phistd::lexfile::read(dir.join("source.tsv"))
        .iter()
        .filter_map(|l| l.split_once('\t'))
        .map(|(name, path)| Source { name: name.trim().to_string(), path: root.join(path.trim()) })
        .collect()
}

/// The names of the sources taken so far, in the order they were taken.
pub fn taken(dir: &Path) -> Vec<String> {
    phistd::lexfile::read(dir.join("taken.tsv")).iter().filter_map(|l| l.split('\t').nth(1)).map(str::to_string).collect()
}

/// The sources already taken, then the next `n` not yet taken: what a
/// night learns from.
pub fn tonight(dir: &Path, root: &Path, n: usize) -> (Vec<Source>, Vec<Source>) {
    let names = taken(dir);
    let (old, new): (Vec<Source>, Vec<Source>) = queue(dir, root).into_iter().partition(|s| names.contains(&s.name));
    (old, new.into_iter().take(n).collect())
}

/// Record that tonight took `source`, with how many sentences it gave.
pub fn take(dir: &Path, stamp: &str, source: &Source, sentences: usize) -> std::io::Result<bool> {
    phistd::lexfile::append(dir.join("taken.tsv"), &format!("{stamp}\t{}\t{sentences}", source.name))
}
