//! The frozen index table, beside the file it reads.
//!
//! `keyboard/index.golden` is a photograph of the 64 Operator declarations,
//! generated from `phical::keyboard::index::render()` and committed. This
//! test renders the live keyboard again and compares. When they differ it
//! names what moved — an Operator added, removed, renamed or re-typed — and
//! says what the only legitimate way forward is: a re-recording carrying the
//! previous table, the new table and the stated reason (R6.3, R11.10).
//!
//! The record is not a second copy of the declarations. It cannot be edited
//! into agreement with a change nobody meant to make, because the only way
//! to write it is to render the live table, which is the thing under test.
//!
//! Validates: Requirements 6.2, 6.3, 11.10

use phical::keyboard::index::{self, drift, Rerecord, Verdict};
use phical::keyboard::COUNT;

/// The frozen record, compiled in: deleting the file is a build failure
/// rather than a skipped test.
const FROZEN: &str = include_str!("keyboard/index.golden");

/// The record with carriage returns dropped, so a checkout with either line
/// ending compares the same.
fn frozen() -> String {
    FROZEN.replace("\r\n", "\n")
}

#[test]
fn the_frozen_index_table_matches_the_keyboard() {
    let live = index::render();
    if live == frozen() {
        return;
    }

    let was = index::parse(&frozen()).expect("the frozen index table reads");
    let now = index::parse(&live).expect("the rendered index table reads");
    let drifts = drift::between(&was, &now);

    let mut report = format!(
        "the index table has moved from its frozen record: {} rows frozen, {} rows live\n",
        was.len(),
        now.len()
    );
    for one in &drifts {
        report.push_str(&format!("  {one}\n"));
    }
    if drifts.is_empty() {
        report.push_str("  the rows agree, so the difference is in the record's head text\n");
    }
    report.push_str(
        "\nto re-record, write phical/tests/keyboard/index.golden from\n\
         `phical::keyboard::index::render()` and record the change through\n\
         `index::Rerecord`, carrying the previous table, the new table and the\n\
         reason; any one of the three absent is unjudged and fails (R11.10)\n",
    );
    panic!("{report}");
}

#[test]
fn the_frozen_index_table_pins_all_sixty_four_rows_by_index_and_name() {
    let rows = index::parse(&frozen()).expect("the frozen index table reads");
    assert_eq!(rows.len(), COUNT, "the record pins exactly {COUNT} Operators");

    let mut names: Vec<&str> = rows.iter().map(|row| row.name.as_str()).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), COUNT, "the record's names are unique");

    for (at, row) in rows.iter().enumerate() {
        assert_eq!(row.index as usize, at, "{:?} is out of index order", row.name);
        assert_eq!(row.operands.len(), row.arity as usize, "{:?} arity", row.name);
    }
}

/// The re-recording path judged from the outside, on this record: moving the
/// file with the previous table, the new table and a reason is recorded;
/// leaving out any one of the three is unjudged and fails.
#[test]
fn re_recording_this_record_needs_all_three_of_previous_new_and_reason() {
    let whole = Rerecord::new(&frozen(), &index::render(), "the reason the table moved");
    assert_eq!(whole.judge(), Verdict::Recorded);
    assert!(!whole.judge().fails());

    let no_reason = Rerecord { reason: None, ..whole.clone() };
    assert!(no_reason.judge().fails(), "a re-recording with no stated reason fails");
    assert!(no_reason.page().contains("verdict: unjudged"));

    let no_previous = Rerecord { previous: None, ..whole };
    assert!(no_previous.judge().fails(), "a re-recording with no previous table fails");
}
