//! The frozen operand set's own tests: that the set is what R7.6 asks for —
//! at least 8 operands of each of the nine types, each type's declared
//! minimum, maximum and zero among them, at least 72 overall, and the phasor
//! bit shapes a float carried as bits either survives or does not — that
//! every entry round-trips through its declared layout, that the file is
//! exactly what the seeded run wrote, and that the fingerprint moves when a
//! byte does.

use super::{fingerprint, generate, unhex, Flaw, FrozenOperands, Reason, PER_TYPE, SEED, TEXT};
use crate::keyboard::{Operand, Type};

/// The floor R7.6 states: at least 8 of each of the nine.
const OVERALL: usize = 9 * PER_TYPE;

/// The set as the file holds it.
fn frozen() -> FrozenOperands {
    FrozenOperands::load()
}

/// Validates: Requirements 7.6
#[test]
fn the_set_holds_eight_distinct_operands_of_each_type_and_seventy_two_overall() {
    let set = frozen();
    assert_eq!(OVERALL, 72, "the stated floor of R7.6");
    assert!(
        set.count() >= OVERALL,
        "{} holds {} operands, under the floor of {OVERALL}",
        super::PATH,
        set.count()
    );
    for ty in Type::ALL {
        let mut bytes: Vec<Vec<u8>> = set.of(ty).iter().map(Operand::encode).collect();
        let held = bytes.len();
        bytes.sort_unstable();
        bytes.dedup();
        assert_eq!(bytes.len(), held, "{} holds the same operand twice", ty.name());
        assert!(held >= PER_TYPE, "{} holds {held} operands, under {PER_TYPE}", ty.name());
    }
}

/// Validates: Requirements 7.6
#[test]
fn every_type_s_declared_minimum_maximum_and_zero_are_in_the_set() {
    let set = frozen();
    for ty in Type::ALL {
        let held: Vec<Vec<u8>> = set.of(ty).iter().map(Operand::encode).collect();
        for (role, edge) in
            [("minimum", ty.minimum()), ("maximum", ty.maximum()), ("zero", ty.zero())]
        {
            assert!(
                held.contains(&edge.encode()),
                "{}'s declared {role} {:?} is absent from the frozen set",
                ty.name(),
                edge
            );
        }
        // Every operand of the type is of the type the column named it.
        assert!(set.of(ty).iter().all(|o| o.kind() == ty), "{} holds another type", ty.name());
    }
}

/// The four bit shapes R7.6 turns on, beside the `+0.0` the declared zero
/// already carries: the other signed zero, a subnormal, an infinity and a
/// NaN. A phasor is two components, so a shape counts wherever it appears.
///
/// Validates: Requirements 7.6
#[test]
fn the_phasor_entries_carry_both_signed_zeros_a_subnormal_an_infinity_and_a_nan() {
    let set = frozen();
    let component: Vec<f64> = set
        .of(Type::Phasor)
        .iter()
        .flat_map(|o| match o {
            Operand::Phasor(z) => [z.re, z.im],
            other => panic!("{other:?} is in the phasor rows"),
        })
        .collect();
    let any = |hit: fn(f64) -> bool| component.iter().copied().any(hit);

    assert!(any(|x| x.to_bits() == 0), "+0.0 is absent");
    assert!(any(|x| x.to_bits() == 0x8000_0000_0000_0000), "-0.0 is absent");
    assert!(any(|x| x != 0.0 && x.is_subnormal()), "a subnormal is absent");
    assert!(any(f64::is_infinite), "an infinity is absent");
    assert!(any(f64::is_nan), "a NaN bit pattern is absent");
}

/// Validates: Requirements 7.6, 7.7
#[test]
fn every_frozen_operand_round_trips_through_its_declared_layout() {
    for operand in frozen().all() {
        let ty = operand.kind();
        let bytes = operand.encode();
        assert_eq!(bytes.len(), ty.width(), "{:?} is not {} bytes", operand, ty.width());
        let read = Operand::decode(ty, &bytes).expect("the declared width");
        assert_eq!(read.encode(), bytes, "{operand:?} does not survive its own layout");
    }
}

/// The record is the seeded run it claims to be: `generate(SEED)` is the file
/// byte for byte. A failure here is one of two things, and the diff says
/// which — the file was edited by hand, or the generator moved under it.
///
/// Validates: Requirements 7.6
#[test]
fn the_file_is_exactly_what_the_seeded_run_wrote() {
    let written = generate(SEED);
    // Compared with carriage returns dropped, which the fingerprint drops
    // too: a CRLF checkout holds the same record.
    let same = |text: &str| text.replace('\r', "");
    assert_eq!(
        same(&written),
        same(TEXT),
        "{} is not the record generate({SEED:#018x}) writes",
        super::PATH
    );
}

/// Validates: Requirements 7.6
#[test]
fn the_header_states_the_numbers_it_was_written_under() {
    let comment: Vec<&str> = TEXT.lines().filter(|l| l.starts_with('#')).collect();
    let header = comment.join("\n");
    assert!(header.contains(&format!("{SEED:#018x}")), "the header omits the seed");
    assert!(header.contains(&format!("{PER_TYPE} distinct operands")), "the header omits 8");
    assert!(header.contains("allowlist.golden"), "the header omits where the pin lives");
}

/// A modification fails the run, which rests on the fingerprint moving when
/// any byte of the record does. One byte changed in each of three places —
/// the header, a type column and a byte column — is three fingerprints, none
/// of them the record's.
///
/// Validates: Requirements 7.6
#[test]
fn a_modified_record_has_a_different_fingerprint() {
    let pinned = frozen().fingerprint;
    let edits = [
        TEXT.replacen("# The frozen", "# the frozen", 1),
        TEXT.replacen("phasor\tminimum", "phasor\tMINIMUM", 1),
        format!("{TEXT}level\tdrawn\t07\n"),
    ];
    for edited in edits {
        assert_ne!(edited, TEXT, "the edit changed nothing, so it tests nothing");
        assert_ne!(fingerprint(&edited), pinned, "an edited record kept its fingerprint");
    }
    assert_eq!(fingerprint(TEXT), pinned, "the fingerprint is not a function of the text");
}

/// Validates: Requirements 7.6
#[test]
fn a_malformed_row_is_a_flaw_naming_its_line() {
    let flaw = |text: &str| FrozenOperands::parse(text).expect_err("malformed");
    assert_eq!(
        flaw("# a comment\nlevel\tzero\n"),
        Flaw { line: 2, reason: Reason::Count(2) }
    );
    assert_eq!(
        flaw("phase\tzero\t00\n"),
        Flaw { line: 1, reason: Reason::Type("phase".to_string()) }
    );
    assert_eq!(flaw("level\tzero\t0g\n"), Flaw { line: 1, reason: Reason::Hex });
    assert_eq!(flaw("level\tzero\t000\n"), Flaw { line: 1, reason: Reason::Hex });
    let wide = flaw("level\tzero\t0000");
    assert_eq!(wide.line, 1);
    assert!(matches!(wide.reason, Reason::Width(bad) if bad.given == 2 && bad.declared == 1));
}

/// The hex column is a two-way reading: every byte in 0..256 round-trips,
/// and a sign is not a hex digit.
#[test]
fn the_hex_column_reads_both_ways() {
    let every: Vec<u8> = (0..=255u8).collect();
    assert_eq!(unhex(&super::hex(&every)), Some(every));
    assert_eq!(unhex("+f"), None);
    assert_eq!(unhex(""), Some(Vec::new()));
}
