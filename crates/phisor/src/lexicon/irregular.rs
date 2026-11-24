//! `lexicon::irregular` - the irregular verbs the reader and realiser name,
//! read from `irregular_verb.tsv`.
//!
//! One file, one source: `lemma<TAB>past<TAB>participle<TAB>ing`, a `-` where
//! the form is not irregular (the regular rule applies). The Guide's own
//! Appendix 1.4 (see [`super::prime_irregular`]) is the richer source and is
//! asked first when primed; this table is the fallback when it is not, and it
//! carries `be`, which the Guide table is parsed without.
use super::list::{data_rows, lex_dir, Words};
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

// The Guide's own table of irregular verbs (its Appendix 1.4), read out of
// its text: the richer source, asked first once primed (`super::prime_irregular`).

/// An irregular verb's forms, as the Guide's Appendix 1.4 lists them. A form
/// may have alternatives ("got/gotten", "shown/showed"); the first is the
/// one the realiser writes, and every one is recognised on reading.
#[derive(Clone, Debug, PartialEq)]
pub struct IrregularForms {
    pub past: Vec<String>,
    pub participle: Vec<String>,
}

/// The irregular verbs, with the reverse maps the reader needs.
#[derive(Clone, Debug, Default)]
pub struct Irregulars {
    by_lemma: HashMap<String, IrregularForms>,
    past_to_lemma: HashMap<String, String>,
    participle_to_lemma: HashMap<String, String>,
}

impl Irregulars {
    /// How many irregular verbs were read.
    pub fn len(&self) -> usize {
        self.by_lemma.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_lemma.is_empty()
    }

    pub fn forms(&self, lemma: &str) -> Option<&IrregularForms> {
        self.by_lemma.get(lemma)
    }

    pub fn lemma_of_past(&self, word: &str) -> Option<&str> {
        self.past_to_lemma.get(word).map(String::as_str)
    }

    pub fn lemma_of_participle(&self, word: &str) -> Option<&str> {
        self.participle_to_lemma.get(word).map(String::as_str)
    }
}

/// Read the Guide's table of irregular verbs (Appendix 1.4) out of its text.
/// The table is two blocks of `infinitive  past simple  past participle` side
/// by side, so a line carries one or two verbs; a pronunciation note in
/// brackets or starred is dropped, and the table ends at the `*` footnote.
/// `be` is skipped: the reader and realiser have `be` forms of their own.
/// Pure, so it can be tested on a snippet without priming anything.
pub fn parse_irregulars(text: &str) -> Irregulars {
    let mut out = Irregulars::default();
    let mut lines = text.lines();
    // The table sits after its own heading; find the column header after it.
    let mut in_table = false;
    let mut seen_heading = false;
    let mut rows: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();
    for line in lines.by_ref() {
        let trimmed = line.trim();
        if !seen_heading {
            seen_heading = trimmed.contains("List of irregular verbs");
            continue;
        }
        if !in_table {
            in_table = trimmed.starts_with("infinitive") && trimmed.contains("past participle");
            continue;
        }
        if trimmed.starts_with('*') {
            break;
        }
        let tokens: Vec<&str> = trimmed
            .split_whitespace()
            .filter(|t| !t.starts_with('[') && !t.contains('*'))
            .collect();
        if tokens.is_empty() || tokens.len() % 3 != 0 || tokens.len() > 6 {
            continue;
        }
        for chunk in tokens.chunks(3) {
            let split = |s: &str| -> Vec<String> {
                s.split('/').map(|a| a.trim().to_lowercase()).filter(|a| !a.is_empty()).collect()
            };
            rows.push((chunk[0].to_lowercase(), split(chunk[1]), split(chunk[2])));
        }
    }
    rows.retain(|(lemma, _, _)| lemma != super::list::word_of("copula_lemma"));
    let lemmas: std::collections::HashSet<String> = rows.iter().map(|(l, _, _)| l.clone()).collect();
    for (lemma, past, participle) in rows {
        for form in &past {
            // A form that is itself another verb's base ("lay", past of "lie")
            // reads as that base, not as the past.
            if !lemmas.contains(form) || *form == lemma {
                out.past_to_lemma.entry(form.clone()).or_insert_with(|| lemma.clone());
            }
        }
        for form in &participle {
            if !lemmas.contains(form) || *form == lemma {
                out.participle_to_lemma.entry(form.clone()).or_insert_with(|| lemma.clone());
            }
        }
        out.by_lemma.insert(lemma, IrregularForms { past, participle });
    }
    out
}

