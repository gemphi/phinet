//! `lexicon::agreement` - which auxiliary form agrees with a subject, read
//! from the person and number columns of `auxiliary.tsv`. Nothing is written
//! here but the labels the file may use (`1`/`2`/`3`, `sg`/`pl`, `*`, `-`).
use super::auxiliary::{parse_family, parse_tense, Family};
use super::list::{data_rows, lex_dir};
use crate::ir::Tense;
use std::sync::OnceLock;

/// The grammatical person of a subject.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Person {
    First,
    Second,
    Third,
}

/// The grammatical number of a subject.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Number {
    Singular,
    Plural,
}

/// One row: a form and the subjects it agrees with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Form {
    pub word: String,
    pub tense: Tense,
    pub family: Family,
    persons: Vec<Person>,
    numbers: Vec<Number>,
}

fn persons(label: &str) -> Vec<Person> {
    let all = [Person::First, Person::Second, Person::Third];
    match label {
        "*" => all.to_vec(),
        _ => label
            .split(',')
            .filter_map(|p| match p.trim() {
                "1" => Some(Person::First),
                "2" => Some(Person::Second),
                "3" => Some(Person::Third),
                _ => None,
            })
            .collect(),
    }
}

fn numbers(label: &str) -> Vec<Number> {
    match label {
        "*" => vec![Number::Singular, Number::Plural],
        _ => label
            .split(',')
            .filter_map(|n| match n.trim() {
                "sg" => Some(Number::Singular),
                "pl" => Some(Number::Plural),
                _ => None,
            })
            .collect(),
    }
}

/// The parsed file: every row that names a tense, a family, a person set and
/// a number set, in file order.
pub fn parse(tsv: &str) -> Vec<Form> {
    data_rows(tsv)
        .into_iter()
        .filter_map(|r| {
            let (tense, family) = (parse_tense(r.get(1)?)?, parse_family(r.get(2)?)?);
            Some(Form {
                word: r[0].clone(),
                tense,
                family,
                persons: persons(r.get(4)?),
                numbers: numbers(r.get(5)?),
            })
        })
        .collect()
}

/// The form of an auxiliary family that agrees with a subject, or `None` when
/// the file is missing or has no such row.
pub fn aux_form(family: Family, tense: Tense, person: Person, number: Number) -> Option<&'static str> {
    static CELL: OnceLock<Vec<Form>> = OnceLock::new();
    let rows = CELL.get_or_init(|| parse(&std::fs::read_to_string(lex_dir().join("auxiliary.tsv")).unwrap_or_default()));
    rows.iter()
        .find(|f| f.family == family && f.tense == tense && f.persons.contains(&person) && f.numbers.contains(&number))
        .map(|f| f.word.as_str())
}
