use super::path::{data_lines, data_rows, read_file};
use std::collections::HashSet;

/// An ordered word list with constant-time membership.
#[derive(Debug, Default)]
pub struct Words {
    items: Vec<String>,
    set: HashSet<String>,
}

impl Words {
    pub fn new(items: Vec<String>) -> Words {
        let set = items.iter().cloned().collect();
        Words { items, set }
    }
    /// Whether the list holds the word exactly.
    pub fn contains(&self, word: impl AsRef<str>) -> bool {
        self.set.contains(word.as_ref())
    }
    /// The entries in file order.
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.items.iter()
    }
    pub fn as_slice(&self) -> &[String] {
        &self.items
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Read a plain list: one entry per data line.
pub fn load_plain(file: &str) -> Words {
    Words::new(data_lines(&read_file(file)))
}

/// Read a tagged list: the first field of each row of `file`, for the rows
/// whose second field (a comma-separated tag set) holds `tag`.
pub fn load_tagged(file: &str, tag: &str) -> Words {
    let rows = data_rows(&read_file(file));
    let pick = |r: &Vec<String>| r.get(1).is_some_and(|t| t.split(',').any(|x| x.trim() == tag));
    Words::new(rows.iter().filter(|r| pick(r)).map(|r| r[0].clone()).collect())
}

/// Read a phrase list: one phrase per data line, its words space-separated.
pub fn load_phrases(file: &str) -> Vec<Vec<String>> {
    data_lines(&read_file(file)).iter().map(|l| l.split_whitespace().map(str::to_string).collect()).collect()
}

/// Read a two-column file as ordered `(first, second)` pairs.
pub fn load_pairs(file: &str) -> Vec<(String, String)> {
    data_rows(&read_file(file)).into_iter().filter(|r| r.len() >= 2).map(|r| (r[0].clone(), r[1].clone())).collect()
}
