//! Property 5: the agreement data gives the forms the Realiser spells.
use super::*;
use crate::ir::Tense;
use crate::lexicon::agreement::{aux_form, Number, Person};
use crate::lexicon::auxiliary::Family;

const PERSONS: [Person; 3] = [Person::First, Person::Second, Person::Third];
const NUMBERS: [Number; 2] = [Number::Singular, Number::Plural];
const TENSES: [Tense; 2] = [Tense::Present, Tense::Past];

#[test]
fn agreement_data_equals_the_realiser_forms() {
    for t in TENSES {
        for p in PERSONS {
            for n in NUMBERS {
                let sg = n == Number::Singular;
                let (third, first) = (sg && p == Person::Third, sg && p == Person::First);
                assert_eq!(aux_form(Family::Be, t, p, n), Some(be_form(t, third, first).as_str()), "be {t:?} {p:?} {n:?}");
                assert_eq!(aux_form(Family::Have, t, p, n), Some(have_form(t, third).as_str()), "have {t:?} {p:?} {n:?}");
                let d = match (t, third) {
                    (Tense::Past, _) => "did",
                    (_, true) => "does",
                    _ => "do",
                };
                assert_eq!(aux_form(Family::Do, t, p, n), Some(d), "do {t:?} {p:?} {n:?}");
            }
        }
    }
}
