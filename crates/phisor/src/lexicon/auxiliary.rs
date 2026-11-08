//! `lexicon::auxiliary` - the finite auxiliaries and modals, read from
//! `auxiliary.tsv`: each word's tense, its family (`be`, `have`, `do`, or one
//! of the modals) and whether it can open a question. Nothing is written here
//! but the labels the file may use (the `Tense` and `Modal` names).
use super::lists::{data_rows, lex_dir};
use crate::ir::{Modal, Tense};
use std::collections::HashMap;
use std::sync::OnceLock;

/// The family an auxiliary belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Be,
    Have,
    Do,
    Modal(Modal),
}

/// One auxiliary form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    pub tense: Tense,
    pub family: Family,
    /// Whether the word can open a question (do-support or a modal).
    pub operator: bool,
}

/// The parsed file, keyed by word.
pub fn parse(tsv: &str) -> HashMap<String, Entry> {
    let mut out = HashMap::new();
    for r in data_rows(tsv) {
        let (Some(tense), Some(family)) = (r.get(1).and_then(|t| parse_tense(t)), r.get(2).and_then(|f| parse_family(f))) else {
            continue;
        };
        let operator = r.get(3).is_some_and(|t| t.split(',').any(|x| x.trim() == "operator"));
        out.entry(r[0].clone()).or_insert(Entry { tense, family, operator });
    }
    out
}

pub(super) fn parse_tense(label: &str) -> Option<Tense> {
    match label {
        "present" => Some(Tense::Present),
        "past" => Some(Tense::Past),
        _ => None,
    }
}

pub(super) fn parse_family(label: &str) -> Option<Family> {
    Some(match label {
        "be" => Family::Be,
        "have" => Family::Have,
        "do" => Family::Do,
        "can" => Family::Modal(Modal::Can),
        "will" => Family::Modal(Modal::Will),
        "may" => Family::Modal(Modal::May),
        "shall" => Family::Modal(Modal::Shall),
        "must" => Family::Modal(Modal::Must),
        _ => return None,
    })
}

/// The auxiliary a word is, or `None` for any other word.
pub fn entry(word: &str) -> Option<Entry> {
    static CELL: OnceLock<HashMap<String, Entry>> = OnceLock::new();
    let table = CELL.get_or_init(|| parse(&std::fs::read_to_string(lex_dir().join("auxiliary.tsv")).unwrap_or_default()));
    table.get(word).copied()
}

/// Whether the word is an auxiliary of this family.
pub fn is_family(word: &str, family: Family) -> bool {
    entry(word).is_some_and(|e| e.family == family)
}
