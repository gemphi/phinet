//! Kind rows: empty and unheld kinds, the set algebra, and Property 19.

use super::*;

#[test]
fn a_kind_holding_no_rows_answers_an_empty_row_sequence() {
    let mut h = host();
    assert_eq!(h.kinds().rows("empty"), Ok(&[][..]));
    assert_eq!(
        h.call("phisor-rows", vec![Value::Str("empty".into())]),
        Ok(Value::List(Vec::new()))
    );
}

#[test]
fn a_kind_the_set_does_not_hold_errors_and_names_it() {
    let mut h = host();
    let refused = h
        .call("phisor-rows", vec![Value::Str("regal".into())])
        .unwrap_err();
    assert!(refused.contains("regal"), "{refused}");
    assert_eq!(h.kinds().rows("regal").unwrap_err(), refused);
    // The set holds the three Kinds told and no fourth.
    assert_eq!(h.kinds().names(), HELD.to_vec());
    assert_eq!(h.kinds().len(), 3);
    assert!(!h.kinds().is_empty());
}

#[test]
fn the_set_algebra_reads_phisor_rows_with_no_conversion() {
    let mut m = checked(host()).expect("the five phisor- names are free");
    // `king is-a man` and `queen is-a woman` are held, so one referent of
    // `man` answers under the Kind `is-a`.
    assert_eq!(
        m.run("(count (referents (phisor-rows \"is-a\") man))"),
        Ok(Value::Num(1.0))
    );
    assert_eq!(
        m.run("(count (phisor-rows \"empty\"))"),
        Ok(Value::Num(0.0))
    );
}

// ----- Property 19: a named Kind's rows come back, an unnamed one errors -----

/// A drawn Kind name against the fixture set: one the set holds, one
/// differing only in case, one with a trailing dash, and one free.
#[derive(Debug)]
struct Name(String);

impl Gen for Name {
    fn draw(g: &mut Seeded) -> Self {
        let held = *g.pick(&HELD);
        Name(match g.below(4) {
            0 => held.to_string(),
            1 => held.to_uppercase(),
            2 => format!("{held}-"),
            _ => format!("kind{}", g.below(50)),
        })
    }
}

#[test]
fn property_19_a_named_kinds_rows_come_back_and_an_unnamed_one_errors() {
    let run = for_all::<Name>(0x19, CASES, |drawn| {
        let mut h = host();
        let held = HELD.contains(&drawn.0.as_str());
        let direct = h.kinds().rows(&drawn.0).map(|rows| rows.len());
        match (
            direct,
            h.call("phisor-rows", vec![Value::Str(drawn.0.clone())]),
        ) {
            // A Kind the set holds: the same count through both doors, and
            // empty for the Kind told with no rows.
            (Ok(count), Ok(Value::List(items))) => {
                held && items.len() == count && (drawn.0 != "empty" || items.is_empty())
            }
            // A Kind it does not hold: one error, naming the Kind.
            (Err(direct), Err(through)) => !held && direct == through && direct.contains(&drawn.0),
            _ => false,
        }
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
