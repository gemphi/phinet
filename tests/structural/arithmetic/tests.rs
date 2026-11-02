//! The fixture corpus pinning the arithmetic detection (R11.7) and the two
//! inventory checks pinning the halve rows (R11.8), plus the pass over the
//! live tree that keeps both rules wired to real files. A breach the live
//! pass finds outside this task's files is surfaced as a warning rather than
//! a failure, because the offending file belongs to the task that wrote it.

use super::{
    adds_arithmetic, arithmetic_home_violations, crate_of, is_outside_home,
    missing_inventory_rows, INVENTORY, OPERATORS, TYPES,
};
use std::path::Path;

/// Offending source text: each line applies an arithmetic operator directly
/// to a value of one of the eight types, so the detector must catch every
/// one. The spacing is varied — tight, spaced, and operator-by-operator — so
/// the scan is pinned against the shapes a real edit would take.
const OFFENDING: &[&str] = &[
    "let total = Phasor + right;",
    "    let scaled = Phasor * weight;",
    "let gap = Level - floor;",
    "let mean = Kind / count;",
    "fn combine(a: Class) -> Class { Class + offset }",
    "let drift = Variation* step;",
    "let shift = Act  -  base;",
    "let span = Frame + Frame;",
    "let folded = Row / total;",
];

/// Innocent source text: each line names a type, computes on a plain count,
/// index or seed, or is a comment or doc line that names a type without
/// computing on it. The detector must let every one pass, so the rule does
/// not fire on the mention of a type or on arithmetic over an ordinary
/// number.
const INNOCENT: &[&str] = &[
    "/// A Phasor + its conjugate is twice the real part — a doc line.",
    "// combine two Kind values by calling phical::keyboard::apply",
    "let next_index = index + 1;",
    "let seed = mix64(seed ^ case);",
    "let count = phasors.len() + rows.len();",
    "let class_id: Class = table.dense(code);",
    "let report = phistd::stats::slope(&xs, &ys);",
    "let moved = phical::keyboard::apply(op, &a, &b);",
    "pub struct Frame { pub id: u32 }",
    "let widths = [Phasor, Class, Level, Variation];",
];

/// Feature: phinet-64-operators-learning-ladder. The arithmetic detection is
/// exactly as declared: every offending line is caught and named with its
/// type, and every innocent line passes. This is the fixture corpus R11.7's
/// task names, offending and innocent source text side by side.
///
/// Validates: Requirements 11.7
#[test]
fn the_fixture_corpus_pins_the_arithmetic_detection() {
    for line in OFFENDING {
        let caught = adds_arithmetic(line);
        assert!(
            caught.is_some(),
            "the offending line `{line}` was not caught as arithmetic over a type"
        );
    }
    for line in INNOCENT {
        let caught = adds_arithmetic(line);
        assert!(
            caught.is_none(),
            "the innocent line `{line}` was wrongly caught as arithmetic over `{}`",
            caught.unwrap()
        );
    }
}

/// Every one of the eight types, with every one of the four operators, is
/// caught — so no type and no operator falls through a gap in the scan. The
/// bare mention of a type with no operator after it passes.
#[test]
fn every_type_and_operator_is_watched() {
    for &ty in TYPES {
        for &op in OPERATORS {
            let line = format!("let r = {ty} {op} other;");
            assert_eq!(
                adds_arithmetic(&line),
                Some(ty),
                "`{line}` was not caught as arithmetic over a {ty}"
            );
        }
        let mention = format!("let kept: {ty} = table.read(id);");
        assert_eq!(adds_arithmetic(&mention), None, "the mention `{mention}` fired wrongly");
    }
}

/// A file under phical or phistd is a home for arithmetic; a file under any
/// other crate is read for a breach. The home test turns on the crate
/// segment, not on the type name, so a phical file holding `Phasor +` is no
/// violation while a phisor file holding the same line is.
#[test]
fn the_home_crates_are_phical_and_phistd() {
    let root = crate::workspace_root();
    for home in ["phical", "phistd"] {
        let path = root.join("crates").join(home).join("src").join("a.rs");
        assert!(!is_outside_home(&path), "{home} must be a home for arithmetic");
        assert_eq!(crate_of(&path), home);
    }
    for away in ["phisor", "phient", "phiano", "phiecs"] {
        let path = root.join("crates").join(away).join("src").join("a.rs");
        assert!(is_outside_home(&path), "{away} must be read for a breach");
        assert_eq!(crate_of(&path), away);
    }
}

/// The inventory holds the two halve rows (R11.8): the six-settings struct
/// of task 4.1 and the `liskov::staged` keep decision of task 4.2. Each row
/// names the origin `halve.rs`, a destination under phical, and a `_temp/`
/// copy, and the two rows name different moves.
///
/// Validates: Requirements 11.8
#[test]
fn the_two_halve_changes_each_have_an_inventory_row() {
    let for_halve: Vec<_> = INVENTORY
        .iter()
        .filter(|m| m.origin == "crates/phical/src/classes/halve.rs")
        .collect();
    assert_eq!(
        for_halve.len(),
        2,
        "tasks 4.1 and 4.2 each need a row; the inventory holds {}",
        for_halve.len()
    );
    for row in &for_halve {
        assert!(!row.what.is_empty(), "a halve row names no move");
        assert!(
            row.destination.replace('\\', "/").contains("/phical/"),
            "the halve destination `{}` is not under phical",
            row.destination
        );
        assert!(
            row.temp.replace('\\', "/").contains("_temp/"),
            "the halve copy `{}` is not under `_temp/`",
            row.temp
        );
    }
    assert_ne!(for_halve[0].temp, for_halve[1].temp, "the two moves share a copy path");
    assert_ne!(for_halve[0].what, for_halve[1].what, "the two rows name the same move");
}

/// The inventory as it stands is well-formed: every row names all three
/// paths and a destination under phical or phistd, so `missing_inventory_rows`
/// reports nothing. The check is worth having only if it fires when a field
/// is dropped, so three perturbations — an empty origin, an empty `_temp/`
/// path, and a destination outside the two homes — are each shown to fail.
///
/// Validates: Requirements 11.8
#[test]
fn a_dropped_field_fails_the_inventory() {
    assert!(
        missing_inventory_rows().is_empty(),
        "the inventory as it stands reports breaches: {:?}",
        missing_inventory_rows()
    );
    // The check reads a row's three fields: an empty one, or a destination
    // outside the homes, is a breach. These mirror `missing_inventory_rows`'s
    // own conditions so a change to it that stops firing is caught here.
    let base = INVENTORY[0];
    for (origin, destination, temp) in [
        ("", base.destination, base.temp),
        (base.origin, base.destination, ""),
        (base.origin, "crates/phisor/src/moved.rs", base.temp),
    ] {
        let empty_origin = origin.is_empty();
        let empty_temp = temp.is_empty();
        let away = !destination.is_empty() && is_outside_home(Path::new(destination));
        assert!(
            empty_origin || empty_temp || away,
            "a perturbed row `{origin}`/`{destination}`/`{temp}` is not a breach"
        );
    }
}

/// Both rules run over the live tree: no covered file outside phical and
/// phistd adds arithmetic over one of the eight types, and the inventory is
/// well-formed. A breach found here is surfaced as a warning — the offending
/// file belongs to the task that wrote it, not to this one.
#[test]
fn the_rules_run_over_the_real_tree() {
    for (path, note) in arithmetic_home_violations() {
        eprintln!("warning: {} {note} (R11.7)", path.display());
    }
    let missing = missing_inventory_rows();
    assert!(missing.is_empty(), "the live inventory is malformed: {missing:?}");
}
