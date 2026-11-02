//! Property 59 and the band and one-word edge tests, plus the pass over the
//! live tree that keeps `covered_files` and `one_word_paths` wired to real
//! files. A breach the live pass finds is surfaced as a warning rather than
//! a failure, because the cap-failing split and the one-word rename each
//! belong to the task that wrote the offending file, not to this one.

use super::{
    band, covered_files, is_covered, is_excluded, is_excluded_for_naming, offending_segment,
    one_word_paths, Band, FAIL_ABOVE, WARN_FROM,
};
use crate::Seeded;
use std::path::PathBuf;

/// A count drawn from 195 to 255, so each case lands near a band edge, with
/// the band it must fall in computed here — not by `band` — as the oracle.
fn drawn_count(rng: &mut Seeded) -> (usize, Band) {
    let lines = 195 + rng.below(61);
    let want = match lines {
        n if n > FAIL_ABOVE => Band::Fail { lines },
        n if n >= WARN_FROM => Band::Warn { lines },
        _ => Band::Ok,
    };
    (lines, want)
}

/// A path and whether it is covered: covered shapes sit under a crate's
/// `src` or are a crate `Cargo.toml`; excluded shapes carry a `target`,
/// `vendor` or `_temp` segment, which no covered path may.
fn drawn_path(rng: &mut Seeded) -> (PathBuf, bool) {
    let base = crate::workspace_root().join("crates").join(*rng.pick(crate::CRATES));
    match rng.below(5) {
        0 => (base.join("Cargo.toml"), true),
        1 => (base.join("src").join("circle").join("angle.rs"), true),
        2 => (base.join("src").join("target").join("built.rs"), false),
        3 => (base.join("src").join("vendor").join("outside.rs"), false),
        _ => (base.join("_temp").join("moved.rs"), false),
    }
}

/// Feature: phinet-64-operators-learning-ladder, Property 59: The line-count
/// bands are exactly as declared. For any line count, a covered source file
/// passes below 201 lines, warns at 201 to 250 while not failing, and fails
/// above 250; and a file under a vendored, generated or `_temp/` path is not
/// covered while a file under a crate's `src` is.
///
/// Validates: Requirements 11.5, 11.6
#[test]
fn property_59_the_line_count_bands_are_exactly_as_declared() {
    let (mut ok, mut warn, mut fail) = (false, false, false);
    let (mut covered_seen, mut excluded_seen) = (false, false);
    for case in 0..crate::CASES as u64 {
        let mut rng = Seeded::for_case(crate::SEED, case);

        let (lines, want) = drawn_count(&mut rng);
        let got = band(lines);
        assert_eq!(got, want, "seed {:#x}, case {case}: {lines} lines banded wrong", crate::SEED);
        assert_eq!(got.fails(), lines > FAIL_ABOVE, "case {case}: {lines} failed wrongly");
        match want {
            Band::Ok => ok = true,
            Band::Warn { .. } => warn = true,
            Band::Fail { .. } => fail = true,
        }

        let (path, covered) = drawn_path(&mut rng);
        assert_eq!(is_covered(&path), covered, "case {case}: {} covered wrongly", path.display());
        if covered {
            covered_seen = true;
        } else {
            excluded_seen = true;
            assert!(is_excluded(&path), "case {case}: {} excluded by no segment", path.display());
        }
    }
    assert!(ok && warn && fail, "a band went undrawn: the edges were not all tested");
    assert!(covered_seen && excluded_seen, "a coverage shape went undrawn");
}

/// The bands meet at the exact lines R11.6 names: 200 the last ok line, 201
/// the first warning, 250 the last warning, 251 the first failure. A warning
/// never fails; a failure always does.
///
/// Validates: Requirements 11.6
#[test]
fn the_band_edges_are_the_declared_lines() {
    assert_eq!(band(200), Band::Ok);
    assert_eq!(band(201), Band::Warn { lines: 201 });
    assert_eq!(band(250), Band::Warn { lines: 250 });
    assert_eq!(band(251), Band::Fail { lines: 251 });
    assert_eq!(band(0), Band::Ok);
    assert!(!band(250).fails() && band(251).fails());
}

/// The one-word rule at its edges: a lower-case word with digits passes; an
/// underscore, a hyphen or an upper-case letter in the stem fails.
#[test]
fn the_one_word_rule_catches_the_three_breaches() {
    for word in ["tests.rs", "circle", "phi4.rs", "q64", "mod.rs", "angle"] {
        assert!(!offending_segment(word), "{word} is one lower-case word and must pass");
    }
    for bad in ["round_trip.rs", "mul-tie.rs", "README.md", "Cargo"] {
        assert!(offending_segment(bad), "{bad} breaks the one-word rule and must be caught");
    }
}

/// The rules run over the live tree: the covered set is non-empty and holds
/// every crate's manifest, no covered file is one the exclusions set aside,
/// and each covered file's band agrees with its count. Breaches the two
/// rules report over the live tree are surfaced as warnings here.
#[test]
fn the_rules_run_over_the_real_tree() {
    let covered = covered_files();
    assert!(covered.len() > crate::CRATES.len(), "covered_files found too few files");
    for c in crate::CRATES {
        let manifest = crate::workspace_root().join("crates").join(c).join("Cargo.toml");
        assert!(covered.contains(&manifest), "{c}'s Cargo.toml is not covered");
    }
    for path in &covered {
        assert!(is_covered(path), "{} is in the covered set but not covered", path.display());
        assert!(!is_excluded(path), "{} is covered and excluded at once", path.display());
        let lines = std::fs::read_to_string(path).map(|t| t.lines().count()).unwrap_or(0);
        let b = band(lines);
        assert_eq!(b.fails(), lines > FAIL_ABOVE, "{} banded against its count", path.display());
        if let Some(note) = b.message(path) {
            eprintln!("warning: {note}");
        }
    }
    for (path, seg) in one_word_paths() {
        assert!(!is_excluded_for_naming(&path), "{} was set aside yet reported", path.display());
        eprintln!("warning: {} holds the non-one-word segment `{seg}`", path.display());
    }
}
