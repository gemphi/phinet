//! The Operator-count and index-table checks (R6.1, R6.3, R6.18): the
//! keyboard enumerates exactly 64 Operators, the six families partition
//! those 64, and the frozen index table is the pinned truth an add or a
//! remove must re-record before it passes.
//!
//! This lives in the Structure_Check rather than in phical because its whole
//! input is files: the frozen index record at
//! `phical/tests/keyboard/index.golden` and the operand set beside it. The
//! target links none of the ten crates, so it cannot ask `phical::keyboard`
//! how many Operators it enumerates; instead it reads the Operators from the
//! golden record, which `phical`'s own tests (3.11/3.12) hold to the live
//! enumeration. The record is therefore the pinned truth: a live
//! enumeration that added or removed an Operator without re-recording the
//! table fails here, naming the previous count, the new count and the reason
//! the re-record must carry (R11.10).
//!
//! The record's shape, read from its header: `#` comment lines, then one
//! tab-separated row per Operator, columns
//! `index name family arity operands result idempotent inverse tolerance`.
//! The rows run `0..64` in index order and the family is the third column.

/// The frozen index table, relative to the workspace root.
const INDEX: &str = "crates/phical/tests/keyboard/index.golden";

/// The pinned Operator count. The keyboard is the 64-Operator keyboard
/// (R6.1): this is the one number the whole feature is named for, so it is
/// written here and not derived, and a record of any other length fails
/// against it.
const PINNED_COUNT: usize = 64;

/// The six families and the per-family count the frozen table records
/// (R6.2), pinned here as the table's judged truth. The six sum to
/// [`PINNED_COUNT`]; `the_pinned_family_counts_sum_to_sixty_four` holds that
/// so the pin cannot drift out of step with the total it partitions.
const FAMILIES: &[(&str, usize)] = &[
    ("circle", 14),
    ("q64", 8),
    ("helix", 6),
    ("classes", 12),
    ("liskov", 8),
    ("relate", 16),
];

/// One Operator row, reduced to the two columns these checks read.
struct Row {
    index: usize,
    family: String,
}

/// The index table's rows, read from the golden record in index order. A
/// `#` comment line or a blank line is skipped; every other line is a row,
/// and a row short of its family column is a malformed record, which fails
/// loudly rather than counting as a family of its own.
fn rows() -> Vec<Row> {
    let path = crate::workspace_root().join(INDEX);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is unreadable: {e}", path.display()));
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        if line.trim_start().starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let mut cols = line.split('\t');
        let index = cols.next().and_then(|c| c.trim().parse::<usize>().ok()).unwrap_or_else(|| {
            panic!("{INDEX}: `{line}` has no integer index in its first column")
        });
        let family = cols.nth(1).unwrap_or_else(|| {
            panic!("{INDEX}: row {index} has no family column; the record is malformed")
        });
        out.push(Row { index, family: family.trim().to_string() });
    }
    out
}

/// The enumerated Operator count equals 64. The record is the pinned truth
/// for the live enumeration (phical's 3.11/3.12 tests hold the two equal),
/// so a count that is not 64 is an Operator added or removed without the
/// re-record R6.3 asks for. The failure names the previous count, the new
/// count and the re-record the table needs, because the fix is not a change
/// here but a change to the record and its golden history.
///
/// Validates: Requirements 6.1, 6.3
#[test]
fn the_keyboard_enumerates_exactly_sixty_four_operators() {
    let found = rows().len();
    assert_eq!(
        found,
        PINNED_COUNT,
        "{INDEX} enumerates {found} Operators; the keyboard is pinned at {PINNED_COUNT}. An \
         Operator was added or removed: re-record the index table through \
         `phical::keyboard::index::record` with the previous table ({PINNED_COUNT} Operators), \
         the new table ({found} Operators) and the stated reason, all three (R6.3, R11.10), \
         before this passes"
    );
}

/// The index column runs `0..count` with no gap and no repeat, so the 64
/// rows are 64 distinct Operators and not one Operator written twice. A
/// hole or a duplicate would let the count read 64 over a table that is not
/// the keyboard's.
///
/// Validates: Requirements 6.1
#[test]
fn the_index_column_is_dense_and_without_repeat() {
    let rows = rows();
    for (expected, row) in rows.iter().enumerate() {
        assert_eq!(
            row.index, expected,
            "{INDEX}: row {expected} carries index {}; the index column must run 0..{} in order",
            row.index,
            rows.len()
        );
    }
}

/// The pinned per-family counts sum to exactly 64. This holds the pin to
/// itself: the six numbers in [`FAMILIES`] are the table's judged partition
/// of the keyboard, so if they stop summing to the count the keyboard is
/// named for, one of the two pins was edited without the other.
///
/// Validates: Requirements 6.18
#[test]
fn the_pinned_family_counts_sum_to_sixty_four() {
    let sum: usize = FAMILIES.iter().map(|(_, n)| n).sum();
    assert_eq!(
        sum, PINNED_COUNT,
        "the pinned family counts sum to {sum}, not {PINNED_COUNT}: FAMILIES and PINNED_COUNT \
         in tests/structural/keyboard.rs have drifted apart"
    );
}

/// Each Operator sits in exactly one family, and the per-family counts in
/// the record are the pinned ones (R6.18). Every row's family is one of the
/// six — a seventh family name fails — and the record's tally of each
/// family equals [`FAMILIES`], so an Operator moved from one family to
/// another fails even when the total stays 64. The per-family counts
/// summing to the total is what makes "exactly one family" the same claim
/// as "64 Operators partitioned".
///
/// Validates: Requirements 6.18, 6.3
#[test]
fn the_six_families_partition_the_sixty_four_operators() {
    let rows = rows();
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for row in &rows {
        let known = FAMILIES.iter().any(|(f, _)| *f == row.family);
        assert!(
            known,
            "{INDEX}: Operator {} sits in family `{}`, which is not one of the six \
             ({}). A new family is a re-record, not a seventh name (R6.3)",
            row.index,
            row.family,
            FAMILIES.iter().map(|(f, _)| *f).collect::<Vec<_>>().join(", ")
        );
        *tally.entry(row.family.clone()).or_default() += 1;
    }
    for (family, pinned) in FAMILIES {
        let found = tally.get(*family).copied().unwrap_or(0);
        assert_eq!(
            found, *pinned,
            "{INDEX}: family `{family}` holds {found} Operators; the table pins {pinned}. An \
             Operator moved families or was added or removed: re-record the index table with \
             the previous counts, the new counts and the reason (R6.3, R11.10)"
        );
    }
    let total: usize = tally.values().sum();
    assert_eq!(
        total, PINNED_COUNT,
        "{INDEX}: the families together hold {total} Operators, not {PINNED_COUNT}"
    );
}
