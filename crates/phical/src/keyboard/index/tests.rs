//! The renderer, the reader and the two judgements over them: what a drift
//! between a frozen table and the live one is reported as, and what an
//! incomplete re-recording is judged.
//!
//! The frozen file itself is compared against [`render`] by
//! `phical/tests/keyboard.rs`, beside the file it reads. These tests are the
//! machinery that test rests on, so they plant each of the four drifts
//! deliberately rather than waiting for one to happen.

use super::record::{Part, Rerecord, Verdict};
use super::{drift, parse, render, Drift, Row, HEAD, NONE};
use crate::keyboard::operator::{table, Inverse, COUNT};

/// The live table read back through the renderer and the reader, which is
/// how both sides of every drift test below are built.
fn live() -> Vec<Row> {
    parse(&render()).expect("the rendered table reads back")
}

#[test]
fn the_rendered_table_reads_back_as_the_whole_keyboard() {
    let rows = live();
    assert_eq!(rows.len(), COUNT, "the rendered record holds all {COUNT} rows");
    let ops = table();
    for (row, op) in rows.iter().zip(&ops) {
        assert_eq!(row.index, op.index);
        assert_eq!(row.name, op.name);
        assert_eq!(row.family, op.family.name());
        assert_eq!(row.arity, op.arity);
        assert_eq!(row.result, op.result.name());
        assert_eq!(row.idempotent, op.idempotent);
        let declared: Vec<&str> = op.operands.iter().map(|ty| ty.name()).collect();
        assert_eq!(row.operands, declared);
    }
}

#[test]
fn the_rendered_table_writes_the_inverse_case_and_its_tolerance() {
    let rows = live();
    for (row, op) in rows.iter().zip(table()) {
        match op.inverse {
            Inverse::Exact => {
                assert_eq!(row.inverse, "exact");
                assert_eq!(row.tolerance, None, "an exact inverse declares no tolerance");
            }
            Inverse::None => {
                assert_eq!(row.inverse, "none");
                assert_eq!(row.tolerance, None);
            }
            Inverse::Approximate { tolerance } => {
                assert_eq!(row.inverse, "approximate");
                let written = row.tolerance.clone().expect("an approximate case writes one");
                let read: f64 = written.parse().expect("the tolerance reads back as a number");
                assert!((read - tolerance).abs() <= 1e-12, "{written} against {tolerance}");
                assert!(read <= 0.0099, "the written tolerance is within the declared cap");
            }
        }
    }
}

#[test]
fn a_record_with_no_rows_reads_as_no_rows_and_the_live_one_as_added() {
    let frozen = parse(HEAD).expect("the head alone reads");
    assert!(frozen.is_empty());
    let drifts = drift::between(&frozen, &live());
    assert_eq!(drifts.len(), COUNT, "every Operator is reported added");
    assert!(matches!(drifts[0], Drift::Added { index: 0, .. }));
}

#[test]
fn the_live_table_drifts_from_itself_in_no_way() {
    assert!(drift::between(&live(), &live()).is_empty());
}

#[test]
fn a_renamed_operator_is_reported_as_a_rename_and_nothing_else() {
    let frozen = live();
    let mut moved = frozen.clone();
    moved[7].name = "renamed".to_string();
    moved[7].result = "kind".to_string();
    let drifts = drift::between(&frozen, &moved);
    assert_eq!(drifts.len(), 1, "the rename is reported alone: {drifts:?}");
    assert!(
        matches!(&drifts[0], Drift::Renamed { index: 7, frozen: was, live: now }
            if was == &frozen[7].name && now == "renamed"),
        "{:?}",
        drifts[0]
    );
}

#[test]
fn a_removed_operator_is_reported_as_a_removal() {
    let frozen = live();
    let mut shorter = frozen.clone();
    let gone = shorter.pop().expect("the table is not empty");
    let drifts = drift::between(&frozen, &shorter);
    assert_eq!(drifts.len(), 1);
    assert!(
        matches!(&drifts[0], Drift::Removed { index, name }
            if *index == gone.index && name == &gone.name),
        "{:?}",
        drifts[0]
    );
}

#[test]
fn a_retyped_operator_names_every_field_that_moved() {
    let frozen = live();
    let mut moved = frozen.clone();
    moved[3].result = "kind".to_string();
    moved[3].idempotent = !moved[3].idempotent;
    moved[3].inverse = "exact".to_string();
    moved[3].tolerance = Some("0.000500000000".to_string());
    let drifts = drift::between(&frozen, &moved);
    let fields: Vec<&str> = drifts
        .iter()
        .map(|d| match d {
            Drift::Retyped { field, .. } => *field,
            other => panic!("expected a re-typing, got {other}"),
        })
        .collect();
    assert_eq!(fields, ["result", "idempotent", "inverse", "tolerance"]);
}

#[test]
fn a_record_out_of_index_order_is_a_flaw_not_a_drift() {
    let text = format!("{HEAD}1\thold\tcircle\t1\tword\tword\tyes\texact\t{NONE}\n");
    let flaw = parse(&text).expect_err("a row at the wrong position is refused");
    let first = HEAD.lines().count() + 1;
    assert_eq!(flaw.line, first, "the flaw names the line it sits on");
    assert!(flaw.to_string().contains("position 0"), "{flaw}");
}

#[test]
fn a_malformed_record_line_is_refused_by_field_count_and_by_value() {
    let short = format!("{HEAD}0\thold\n");
    assert!(parse(&short).unwrap_err().reason.contains("expected 9"));

    let bad_flag = format!("{HEAD}0\thold\tcircle\t1\tword\tword\tmaybe\texact\t{NONE}\n");
    assert!(parse(&bad_flag).unwrap_err().reason.contains("yes or no"));

    let bad_arity = format!("{HEAD}0\thold\tcircle\t2\tword\tword\tyes\texact\t{NONE}\n");
    assert!(parse(&bad_arity).unwrap_err().reason.contains("arity 2"));
}

#[test]
fn a_complete_rerecording_is_recorded_and_does_not_fail() {
    let record = Rerecord::new("the previous table", "the new table", "a family was re-typed");
    assert_eq!(record.judge(), Verdict::Recorded);
    assert!(!record.judge().fails());
    let page = record.page();
    assert!(page.contains("verdict: recorded"), "{page}");
    assert!(page.contains("a family was re-typed"));
}

#[test]
fn a_rerecording_missing_any_of_the_three_is_unjudged_and_fails() {
    let whole = Rerecord::new("was", "is", "why");
    for (part, strip) in [
        (Part::Previous, (|r: &mut Rerecord| r.previous = None) as fn(&mut Rerecord)),
        (Part::New, |r: &mut Rerecord| r.new = None),
        // A blank reason is an unstated reason.
        (Part::Reason, |r: &mut Rerecord| r.reason = Some("   ".to_string())),
    ] {
        let mut record = whole.clone();
        strip(&mut record);
        let verdict = record.judge();
        assert_eq!(verdict, Verdict::Unjudged { absent: vec![part] }, "{part:?}");
        assert!(verdict.fails(), "an unjudged re-recording fails the run");
        let page = record.page();
        assert!(page.contains("verdict: unjudged"), "{page}");
        assert!(page.contains(part.name()), "the page names the absent part: {page}");
        assert!(page.contains("(absent)"), "{page}");
    }

    let empty = Rerecord::default();
    assert_eq!(empty.judge(), Verdict::Unjudged { absent: Part::ALL.to_vec() });
}
