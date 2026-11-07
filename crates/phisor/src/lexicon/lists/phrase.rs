use super::path::{data_rows, read_file};
use super::words::Words;
use std::sync::OnceLock;

/// An Express frame said as a fixed phrase (`expressive_phrase.tsv`).
#[derive(Debug, Clone)]
pub struct ExpressivePhrase {
    /// The frame's predicate.
    pub pred: String,
    /// The words that open the sentence in place of subject and verb.
    pub opening: Vec<String>,
    /// Whether the frame's theme follows the opening.
    pub theme: bool,
    /// The role the phrase leaves unsaid and the pronoun that fills it, which
    /// the Reader restores.
    pub unsaid: Option<(String, String)>,
}

/// The fixed phrases Express frames are said as, in file order.
pub fn expressive_phrases() -> &'static Vec<ExpressivePhrase> {
    static CELL: OnceLock<Vec<ExpressivePhrase>> = OnceLock::new();
    CELL.get_or_init(|| {
        data_rows(&read_file("expressive_phrase.tsv"))
            .into_iter()
            .filter(|r| r.len() >= 4 && r[1] == "phrase")
            .map(|r| ExpressivePhrase {
                pred: r[0].clone(),
                opening: r[2].split_whitespace().map(str::to_string).collect(),
                theme: r[3] == "theme",
                unsaid: r.get(4).and_then(|u| u.split_once('=')).map(|(role, who)| (role.to_string(), who.to_string())),
            })
            .collect()
    })
}

/// The psychological-state predicates a first-person present statement is an
/// Express for (the `predicate` rows of `expressive_phrase.tsv`).
pub fn expressive_predicates() -> &'static Words {
    static CELL: OnceLock<Words> = OnceLock::new();
    CELL.get_or_init(|| {
        Words::new(
            data_rows(&read_file("expressive_phrase.tsv"))
                .into_iter()
                .filter(|r| r.len() >= 2 && r[1] == "predicate")
                .map(|r| r[0].clone())
                .collect(),
        )
    })
}

/// The words that stand for a wished-for clause's verb group (`wish_form.tsv`):
/// the row for `key` (`be`, or a modal's present form) and the polarity.
pub fn wish_form(key: &str, negative: bool) -> Option<&'static Vec<String>> {
    wish_forms().iter().find(|(k, n, _)| k == key && *n == negative).map(|(_, _, words)| words)
}

/// Every row of `wish_form.tsv` as `(key, negative, words)`, in file order, for
/// the Reader to match a wished-for verb group against.
pub fn wish_forms() -> &'static Vec<(String, bool, Vec<String>)> {
    static CELL: OnceLock<Vec<(String, bool, Vec<String>)>> = OnceLock::new();
    CELL.get_or_init(|| {
        data_rows(&read_file("wish_form.tsv"))
            .into_iter()
            .filter(|r| r.len() >= 3)
            .map(|r| (r[0].clone(), r[1] == "negative", r[2].split_whitespace().map(str::to_string).collect()))
            .collect()
    })
}

/// The words a fronted wh-hole is said as in a `Via(prep)` slot
/// (`wh_phrase.tsv`), when that differs from the bare wh-word: `how` in a
/// `for` slot is "how long".
pub fn wh_phrase(wh: &str, prep: &str) -> Option<&'static Vec<String>> {
    static CELL: OnceLock<Vec<(String, String, Vec<String>)>> = OnceLock::new();
    let rows = CELL.get_or_init(|| {
        data_rows(&read_file("wh_phrase.tsv"))
            .into_iter()
            .filter(|r| r.len() >= 3)
            .map(|r| (r[0].clone(), r[1].clone(), r[2].split_whitespace().map(str::to_string).collect()))
            .collect()
    });
    rows.iter().find(|(w, p, _)| w == wh && p == prep).map(|(_, _, words)| words)
}
