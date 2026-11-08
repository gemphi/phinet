use super::path::{data_rows, read_file};
use std::sync::OnceLock;

/// The spelling of a determiner kind (`definite`, `indefinite`, `near`,
/// `far`) for a number, from `determiner.tsv`. A missing row reads as `None`.
pub fn determiner_spelling(kind: &str, plural: bool) -> Option<String> {
    static CELL: OnceLock<Vec<Vec<String>>> = OnceLock::new();
    let number = if plural { "plural" } else { "singular" };
    let rows = CELL.get_or_init(|| data_rows(&read_file("determiner.tsv")));
    rows.iter().find(|r| r.len() >= 3 && r[0] == kind && r[1] == number).map(|r| r[2].clone())
}
