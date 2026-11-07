//! Realising infinitive verb groups.

use super::*;

#[test]
fn a_control_infinitive_is_realised_with_to_and_its_complements() {
    // The reader builds promise[come]; the realiser must say the "to".
    let act = read("i promise to come.").expect("an infinitive is covered");
    assert_eq!(realise(&act), "i promise to come.");
    // The inner clause keeps its own complements.
    let act = read("i want to go to london.").expect("an infinitive with a phrase is covered");
    assert_eq!(realise(&act), "i want to go to london.");
}

#[test]
fn nested_infinitives_compose() {
    let act = read("i promise to try to pay the money.").expect("nested infinitives are covered");
    assert_eq!(realise(&act), "i promise to try to pay the money.");
}

#[test]
fn a_finite_clause_that_shares_the_subject_is_not_said_as_an_infinitive() {
    let act = read("i promise i will not tell anyone.").expect("a finite promise is covered");
    assert_eq!(realise(&act), "i promise i will not tell anyone.");
}
