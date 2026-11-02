//! The frozen labelled set loads, meets every R8.7 well-formedness rule,
//! and refuses to load once it has been changed. These are unit tests over
//! the one frozen input; the per-class score over it is the integration
//! test `tests/labelled.rs`, which R8.8 and R8.9 own.

use super::*;
use phiecs::quest::Standing;

#[test]
fn the_frozen_set_is_well_formed_at_the_pinned_fingerprint() {
    let set = read().unwrap_or_else(|f| panic!("{f}"));
    // Size in range, at least 40 false, and the fingerprint the set was
    // frozen with.
    assert!((MIN..=MAX).contains(&set.claims.len()), "{} claims", set.claims.len());
    let false_count = set.claims.iter().filter(|(_, _, l)| is_false(l)).count();
    assert!(false_count >= MIN_FALSE, "{false_count} false claims");
    assert_eq!(set.fingerprint, FINGERPRINT);
    assert!(set.unchanged());
    // Every false claim named a transcript; the sources are carried.
    assert!(!set.source_transcripts.is_empty());
}

#[test]
fn exactly_one_of_the_four_labels_sits_on_every_claim() {
    let set = read().unwrap();
    // label_of returns one of the four for every label that loaded, and the
    // four are mutually exclusive, so a claim carries exactly one class.
    for (_, _, label) in &set.claims {
        let classes = [
            matches!(label, Standing::Degenerate { .. }),
            matches!(label, Standing::Contradicting { .. }),
            matches!(label, Standing::Unsupported),
            matches!(label, Standing::Consistent),
        ];
        assert_eq!(classes.iter().filter(|c| **c).count(), 1, "{label:?}");
    }
}

#[test]
fn a_changed_claim_fails_the_fingerprint() {
    let text = include_str!("../../../tests/claims.tsv");
    // One cell changed inside a claim: the size and false count still hold,
    // the fingerprint does not, so the modification fails the run.
    let changed = text.replacen("cat\tcolour\tgreen", "cat\tcolour\tgrey", 1);
    assert_ne!(changed, text);
    match of(&changed).err() {
        Some(Flaw::Fingerprint { read, declared }) => {
            assert_eq!(declared, FINGERPRINT);
            assert_ne!(read, FINGERPRINT);
        }
        other => panic!("a changed claim loaded: {other:?}"),
    }
}

#[test]
fn a_dropped_or_added_claim_moves_the_fingerprint() {
    let text = include_str!("../../../tests/claims.tsv");
    let last = text.trim_end().lines().last().unwrap();
    // One row dropped: 103 is still in range and still has 40 false, so the
    // fingerprint is what catches it.
    let dropped = text.replacen(&format!("\n{last}"), "", 1);
    assert!(matches!(of(&dropped), Err(Flaw::Fingerprint { .. })));
    // One row added: the same way round.
    let added = format!("{text}{last}\n");
    assert!(matches!(of(&added), Err(Flaw::Fingerprint { .. })));
}

#[test]
fn a_size_out_of_range_and_too_few_false_each_fail_by_name() {
    // A one-claim file reads its grammar, then fails the size.
    let one = "# head\nrobin\tis-a\tbird\t0.9\tconsistent\t-\n";
    assert_eq!(flaw_of(one), Flaw::Size { read: 1 });
    // 100 consistent claims: in range, but no false claim at all.
    let mut many = String::from("# head\n");
    for i in 0..100 {
        many.push_str(&format!("w{i}\tis-a\tthing\t0.9\tconsistent\t-\n"));
    }
    assert_eq!(flaw_of(&many), Flaw::TooFewFalse { read: 0 });
}

/// The flaw a text fails with, for a test that only needs the flaw: avoids
/// comparing a whole `Result` whose `Ok` holds a set with no `PartialEq`.
fn flaw_of(text: &str) -> Flaw {
    of(text).err().expect("the text was expected to fail the load")
}

#[test]
fn an_unreadable_row_names_the_row_the_column_and_the_cell() {
    let row = |cells: &str| of(&format!("# head\n{cells}\n"));
    let flaw = |cells: &str| match row(cells).err() {
        Some(Flaw::Row { id, column, cell, why }) => (id, column, cell, why),
        other => panic!("{cells:?} read as {other:?}"),
    };
    assert_eq!(
        flaw("cat\tcolour\tgreen\tnope\tcontradicting\tsrc").1,
        "weight"
    );
    assert_eq!(
        flaw("cat\tcolour\tgreen\t0.5\tdubious\tsrc"),
        (1, "label", "dubious".to_string(), format!("is not one of {}", labels()))
    );
    // A false claim with no transcript, and a consistent claim with one.
    assert_eq!(flaw("cat\tcolour\tgreen\t0.5\tcontradicting\t-").1, "source");
    assert_eq!(flaw("robin\tis-a\tbird\t0.9\tconsistent\tsrc").1, "source");
    assert_eq!(flaw("cat\tcolour\tgreen\t0.5").1, "row");
}
