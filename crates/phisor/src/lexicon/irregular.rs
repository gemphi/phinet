//! `lexicon::irregular` - the irregular verbs the reader and realiser name,
//! read from `irregular_verb.tsv`.
//!
//! One file, one source: `lemma<TAB>past<TAB>participle<TAB>ing`, a `-` where
//! the form is not irregular (the regular rule applies). The Guide's own
//! Appendix 1.4 (see [`super::prime_irregular`]) is the richer source and is
//! asked first when primed; this table is the fallback when it is not, and it
//! carries `be`, which the Guide table is parsed without.
use super::lists::{data_rows, lex_dir, Words};
use std::collections::HashMap;
use std::sync::OnceLock;

/// The forms of one irregular verb; `None` where the regular rule applies.
#[derive(Debug, Default, Clone)]
pub struct Forms {
    pub past: Option<String>,
    pub participle: Option<String>,
    pub ing: Option<String>,
}

/// The parsed table with the reverse (past -> lemma) map.
#[derive(Debug, Default)]
pub struct Table {
    lemmas: Words,
    by_lemma: HashMap<String, Forms>,
    past_to_lemma: HashMap<String, String>,
}

impl Table {
    /// Parse the file's text.
    pub fn parse(tsv: &str) -> Table {
        let form = |r: &[String], i: usize| r.get(i).filter(|f| !f.is_empty() && *f != "-").cloned();
        let mut by_lemma = HashMap::new();
        let mut past_to_lemma = HashMap::new();
        let mut order = Vec::new();
        for r in data_rows(tsv).into_iter().filter(|r| !r[0].is_empty()) {
            let forms = Forms { past: form(&r, 1), participle: form(&r, 2), ing: form(&r, 3) };
            if let Some(p) = &forms.past {
                past_to_lemma.entry(p.clone()).or_insert_with(|| r[0].clone());
            }
            if forms.participle.is_some() {
                order.push(r[0].clone());
            }
            by_lemma.insert(r[0].clone(), forms);
        }
        Table { lemmas: Words::new(order), by_lemma, past_to_lemma }
    }
}

fn table() -> &'static Table {
    static CELL: OnceLock<Table> = OnceLock::new();
    CELL.get_or_init(|| Table::parse(&std::fs::read_to_string(lex_dir().join("irregular_verb.tsv")).unwrap_or_default()))
}

/// The irregular lemmas the reader inverts a participle against: those the
/// file gives an explicit participle.
pub fn lemmas() -> &'static Words {
    &table().lemmas
}

/// The simple past of an irregular verb named in the file.
pub fn past(lemma: &str) -> Option<String> {
    table().by_lemma.get(lemma)?.past.clone()
}

/// The past participle of an irregular verb named in the file.
pub fn participle(lemma: &str) -> Option<String> {
    table().by_lemma.get(lemma)?.participle.clone()
}

/// The irregular `-ing` form of a verb named in the file (`be` -> `being`).
pub fn ing(lemma: &str) -> Option<String> {
    table().by_lemma.get(lemma)?.ing.clone()
}

/// The verb a form is the simple past of, per the file ("went" -> "go").
pub fn lemma_of_past(word: &str) -> Option<String> {
    table().past_to_lemma.get(word).cloned()
}
