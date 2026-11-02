//! The learned-count independence of the Operator table (R6.14): no index,
//! name, type, arity, family or width in the frozen keyboard changes with any
//! learned count phisor grows.
//!
//! phisor grows uncapped counts - the templates, the word classes, the kinds -
//! as far as the data pays for them. The 64 Operators are a keyboard, not one
//! of those grown things: their table is designed and closed, and R6.14 holds
//! that the two never touch. The live half of that claim - that the table is
//! byte-identical across runs whose grown counts differ - runs the Ladder, so
//! it is phisor's own Property 61 (`ladder` cannot be linked here). What lives
//! here is the structural half: the frozen index table carries no learned
//! count at all, and its fingerprint is pinned, so a run that somehow wrote a
//! grown number into the table would move the fingerprint and fail.
//!
//! The target links no crate, so the table is read from its golden record at
//! `crates/phical/tests/keyboard/index.golden`, the same record task 21.5's
//! keyboard check reads. The fold is phistd's `mix64`, written out as `frozen`
//! writes it, since the target links neither phical nor phistd.
//!
//! Validates: Requirements 6.14

/// The frozen index table, relative to the workspace root.
const INDEX: &str = "crates/phical/tests/keyboard/index.golden";

/// The pinned fingerprint of the table's operative rows. Recorded from the
/// fold below over the record as it stands; a learned count written into any
/// row would move it. Re-record only when the designed table itself changes
/// (task 21.5's path), never for a learned count - that is the point.
const PINNED: u64 = 0x2ef2_a85a_c47b_8e03;

/// The nine column headers the table declares. None of them is a learned
/// count: there is no templates column, no word-class column, no kind column.
/// R6.14 is first of all this absence, checked by name.
const COLUMNS: [&str; 9] = [
    "index", "name", "family", "arity", "operands", "result", "idempotent",
    "inverse", "tolerance",
];

/// Names a learned count must never appear as a column of the Operator table.
const LEARNED: [&str; 4] = ["templates", "word_classes", "kinds", "variations"];

/// phistd's `mix64`: splitmix64's finaliser with the golden-ratio add.
fn mix64(h: u64) -> u64 {
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The record's data rows (the uncommented, non-blank lines), each trimmed of
/// a trailing carriage return so a CRLF checkout folds the same.
fn rows() -> Vec<String> {
    let path = crate::workspace_root().join(INDEX);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is unreadable: {e}", path.display()));
    text.lines()
        .map(|l| l.trim_end_matches('\r').to_string())
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .collect()
}

/// The fingerprint of the table's rows: `mix64` folded over every byte of
/// every data row in order. Comment lines are excluded, so a reworded header
/// does not move it, but any operative cell - index, name, family, type - does.
fn fingerprint(rows: &[String]) -> u64 {
    let mut hash = 0u64;
    for row in rows {
        for byte in row.bytes() {
            hash = mix64(hash ^ byte as u64);
        }
        hash = mix64(hash ^ 0x0A);
    }
    hash
}

/// The header line's columns, read from the one comment line that lists them.
fn header_columns() -> Vec<String> {
    let path = crate::workspace_root().join(INDEX);
    let text = std::fs::read_to_string(&path).unwrap();
    let line = text
        .lines()
        .map(|l| l.trim_start_matches('#').trim())
        .find(|l| l.starts_with("index") && l.contains("family"))
        .unwrap_or_else(|| panic!("{INDEX} has no column header line"));
    line.split('\t').map(|c| c.trim().to_string()).collect()
}

/// The table declares exactly the nine designed columns and no learned count.
/// This is R6.14 read as absence: a templates or word-class column could only
/// have come from a grown number leaking into the designed table.
///
/// Validates: Requirements 6.14
#[test]
fn the_table_carries_no_learned_count_column() {
    let columns = header_columns();
    assert_eq!(columns, COLUMNS, "{INDEX} declares columns the designed table does not");
    for learned in LEARNED {
        assert!(
            !columns.iter().any(|c| c == learned),
            "{INDEX} carries a learned-count column `{learned}` (R6.14)"
        );
    }
}

/// The table's fingerprint is the pinned one. Because the rows hold no learned
/// count, this value is a pure function of the designed table, so a run that
/// grew a different number of templates or kinds reads the same fingerprint -
/// which is independence (R6.14). A drift means the designed table changed,
/// not a count, and is re-recorded through task 21.5's path.
///
/// Validates: Requirements 6.14
#[test]
fn the_table_fingerprint_is_pinned_and_countless() {
    let found = fingerprint(&rows());
    assert_eq!(
        found, PINNED,
        "{INDEX} fingerprints {found:#018x}; the pin is {PINNED:#018x}. The Operator table \
         is independent of every learned count (R6.14), so this moves only when the designed \
         table itself changes - re-record through the keyboard index path, never for a count"
    );
}

/// The fingerprint moves when an operative cell moves, so the pin is not
/// vacuous: a renamed Operator, a retyped operand or a moved family each fail.
/// A learned count has no cell to change, which is why none can reach this.
///
/// Validates: Requirements 6.14
#[test]
fn a_changed_operative_cell_moves_the_fingerprint() {
    let rows = rows();
    let base = fingerprint(&rows);
    assert!(!rows.is_empty(), "{INDEX} holds no rows to perturb");
    // Rename the first Operator.
    let mut renamed = rows.clone();
    renamed[0] = renamed[0].replacen("identity", "renamed", 1);
    assert_ne!(fingerprint(&renamed), base, "a renamed Operator kept the fingerprint");
    // Change an operand type somewhere in the table.
    let mut retyped = rows.clone();
    if let Some(row) = retyped.iter_mut().find(|r| r.contains("phasor")) {
        *row = row.replacen("phasor", "class", 1);
    }
    assert_ne!(fingerprint(&retyped), base, "a retyped operand kept the fingerprint");
}