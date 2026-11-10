//! Property 5: the agreement data holds the forms English spells, checked
//! against a fixed table (the Realiser now reads this data, so it can no
//! longer serve as the oracle).
use crate::ir::Tense;
use crate::lexicon::agreement::{aux_form, Number, Person};
use crate::lexicon::auxiliary::Family;
const PERSONS: [Person; 3] = [Person::First, Person::Second, Person::Third];
const NUMBERS: [Number; 2] = [Number::Singular, Number::Plural];
const TENSES: [Tense; 2] = [Tense::Present, Tense::Past];
fn be(t: Tense, p: Person, n: Number) -> &'static str {
    match (t, p, n) {
        (Tense::Present, Person::First, Number::Singular) => "am",
        (Tense::Present, Person::Third, Number::Singular) => "is",
        (Tense::Present, _, _) => "are",
        (Tense::Past, _, Number::Singular) if p != Person::Second => "was",
        (Tense::Past, _, _) => "were",
    }
}
fn have(t: Tense, p: Person, n: Number) -> &'static str {
    match (t, p, n) {
        (Tense::Past, _, _) => "had",
        (_, Person::Third, Number::Singular) => "has",
        _ => "have",
    }
}
fn does(t: Tense, p: Person, n: Number) -> &'static str {
    match (t, p, n) {
        (Tense::Past, _, _) => "did",
        (_, Person::Third, Number::Singular) => "does",
        _ => "do",
    }
}
#[test]
fn agreement_data_equals_the_fixed_table() {
    for t in TENSES {
        for p in PERSONS {
            for n in NUMBERS {
                assert_eq!(aux_form(Family::Be, t, p, n), Some(be(t, p, n)), "be {t:?} {p:?} {n:?}");
                assert_eq!(aux_form(Family::Have, t, p, n), Some(have(t, p, n)), "have {t:?} {p:?} {n:?}");
                assert_eq!(aux_form(Family::Do, t, p, n), Some(does(t, p, n)), "do {t:?} {p:?} {n:?}");
            }
        }
    }
}
