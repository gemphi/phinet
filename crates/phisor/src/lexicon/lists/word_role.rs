use super::words::load_pairs;
use std::sync::OnceLock;

/// The `(word, role)` rows of `word_role.tsv`, read once.
fn rows() -> &'static [(String, String)] {
    static CELL: OnceLock<Vec<(String, String)>> = OnceLock::new();
    CELL.get_or_init(|| load_pairs("word_role.tsv"))
}

/// Whether `word` plays `role` in `word_role.tsv`.
pub fn is_role(word: &str, role: &str) -> bool {
    rows().iter().any(|(w, r)| w == word && r == role)
}

/// The first word of `role` in `word_role.tsv`, or empty when the file has none.
pub fn word_of(role: &str) -> &'static str {
    rows().iter().find(|(_, r)| r == role).map_or("", |(w, _)| w.as_str())
}
