//! The per-class scoring, pinned by example: the eight numbers over a small
//! labelled set whose counts can be read by eye, the undefined class, the
//! two Bars of R8.9, the unread claim, and the fingerprint.
//!
//! No property is named for this scoring in the design, and the design says
//! why: the eight numbers are measured findings on the frozen set, and the
//! set's well-formedness is asserted where it is loaded. What is pinned here
//! is the arithmetic's shape — one-versus-rest, three places, undefined
//! rather than zero — on counts small enough to check by hand.

use super::*;
use crate::truth::Declared;
use phiecs::quest::{Degeneracy, Row};

fn claim(kind: &str, other: &str) -> Claim {
    Claim { kind: kind.to_string(), other: other.to_string(), weight: 0.9 }
}

fn labelled(about: &str, kind: &str, other: &str, label: Standing) -> (String, Claim, Standing) {
    (about.to_string(), claim(kind, other), label)
}

fn degenerate() -> Standing {
    Standing::Degenerate { test: Degeneracy::SelfEdge }
}

fn contradicting() -> Standing {
    Standing::Contradicting { rows: Vec::new() }
}

/// A world holding `sky colour blue`, so a second colour for the sky
/// contradicts under a functional `colour`.
fn world() -> World {
    World::from_rows(&[Row::new("sky", "colour", "blue")])
}

fn detector() -> Detector {
    Detector::new(Declared::none().functional(&["colour"]))
}

/// Five claims whose readings are known:
/// - `cat is-a cat` and `(empty) is-a mammal` read degenerate, labelled so;
/// - `sky colour green` reads contradicting, labelled so;
/// - `cat is-a mammal` reads unsupported, labelled so;
/// - `dog is-a mammal` reads unsupported but is labelled degenerate — a miss
///   for degenerate and a false positive for unsupported.
///
/// So degenerate holds 3 labelled with 2 found and 2 predicted; contradicting
/// 1, 1 and 1; unsupported 1, 1 and 2; consistent none at all.
fn set() -> LabelledSet {
    LabelledSet::new(
        vec![
            labelled("cat", "is-a", "cat", degenerate()),
            labelled("", "is-a", "mammal", degenerate()),
            labelled("sky", "colour", "green", contradicting()),
            labelled("cat", "is-a", "mammal", Standing::Unsupported),
            labelled("dog", "is-a", "mammal", degenerate()),
        ],
        vec![PathBuf::from("phinet/docs/transcripts/001_answers.md")],
    )
}

#[test]
fn the_four_classes_are_scored_one_versus_rest_to_three_places() {
    let scores = detector().judge_set(&set(), &world());
    // Degenerate: 2 of 3 found, and both predictions were right.
    assert_eq!(scores[0], ClassScore { labelled: 3, precision: Some(1.0), recall: Some(0.667) });
    // Contradicting: the one labelled claim, found.
    assert_eq!(scores[1], ClassScore { labelled: 1, precision: Some(1.0), recall: Some(1.0) });
    // Unsupported: found, but one claim labelled degenerate was read this
    // way too, so one of its two predictions was wrong.
    assert_eq!(scores[2], ClassScore { labelled: 1, precision: Some(0.5), recall: Some(1.0) });
    // The eight numbers, and the two undefined ones of the class nothing was
    // labelled.
    assert_eq!(scores[3], ClassScore { labelled: 0, precision: None, recall: None });
}

#[test]
fn a_class_holding_no_labelled_claim_is_undefined_rather_than_zero() {
    // Consistent and contradicting hold nothing here; both report None
    // rather than 0.0, which would read as a detector that found none of
    // them (R8.8).
    let set = LabelledSet::new(vec![labelled("cat", "is-a", "cat", degenerate())], Vec::new());
    let scores = detector().judge_set(&set, &world());
    assert_eq!(scores[0], ClassScore { labelled: 1, precision: Some(1.0), recall: Some(1.0) });
    for i in [1, 2, 3] {
        assert_eq!(scores[i].labelled, 0, "{} held a label", CLASS[i]);
        assert_eq!((scores[i].precision, scores[i].recall), (None, None), "{}", CLASS[i]);
    }
}

#[test]
fn the_scoring_reads_every_claim_and_leaves_one_reading_each() {
    let set = set();
    let mut d = detector();
    d.judge_set(&set, &world());
    assert_eq!(d.seen.len(), set.claims.len());
    // The labels are not shown to the classifier: the claim labelled
    // degenerate that is not degenerate was still read as unsupported.
    let read = d.seen.last().expect("the last claim was read");
    assert_eq!((read.about.as_str(), &read.standing), ("dog", &Standing::Unsupported));
}

#[test]
fn a_claim_the_detector_never_read_misses_its_class_and_accuses_none() {
    // Scored without reading: nothing was predicted, so the labelled class
    // recalls nothing and no other class's precision is touched.
    let scores = detector().score(&set());
    assert_eq!(scores[0], ClassScore { labelled: 3, precision: Some(0.0), recall: Some(0.0) });
    assert_eq!(scores[1], ClassScore { labelled: 1, precision: Some(0.0), recall: Some(0.0) });
    assert_eq!(scores[3], ClassScore { labelled: 0, precision: None, recall: None });
}

#[test]
fn an_off_scale_copy_of_a_claim_is_not_taken_for_the_readable_one() {
    let readable = labelled("cat", "is-a", "mammal", Standing::Unsupported);
    let off_scale = (
        "cat".to_string(),
        Claim { kind: "is-a".into(), other: "mammal".into(), weight: 2.0 },
        Standing::Unsupported,
    );
    let mut d = detector();
    // Only the readable one is read; the unreadable twin matches no reading.
    d.classify(&readable.0, &readable.1, &world());
    let scores = d.score(&LabelledSet::new(vec![readable, off_scale], Vec::new()));
    // Two labelled unsupported, one found.
    assert_eq!(scores[2], ClassScore { labelled: 2, precision: Some(1.0), recall: Some(0.5) });
}

#[test]
fn a_claim_labelled_unreadable_belongs_to_no_class_and_costs_a_precision() {
    let unreadable = Standing::Unreadable { reason: "weight 2 outside 0..=1".to_string() };
    let set = LabelledSet::new(
        vec![
            labelled("cat", "is-a", "mammal", Standing::Unsupported),
            labelled("dog", "is-a", "mammal", unreadable),
        ],
        Vec::new(),
    );
    let scores = detector().judge_set(&set, &world());
    // One labelled unsupported; two read that way, so precision is halved
    // and the unreadable label counts in no class's labelled total.
    assert_eq!(scores[2], ClassScore { labelled: 1, precision: Some(0.5), recall: Some(1.0) });
    assert_eq!(scores.iter().map(|s| s.labelled).sum::<usize>(), 1);
}

#[test]
fn the_bars_are_degenerate_recall_at_0_95_and_contradicting_recall_at_0_80() {
    assert_eq!(RECALL_BAR, [Some(0.95), Some(0.80), None, None]);
    assert_eq!(CLASS, ["degenerate", "contradicting", "unsupported", "consistent"]);
    let scores = detector().judge_set(&set(), &world());
    // Degenerate recall is 0.667, a miss of the 0.95 Bar; contradicting
    // recall is 1.0, which reaches 0.80. The two unbarred classes are not
    // judged, and neither is a class with no labelled claim.
    assert_eq!(judged(&scores), [Some(false), Some(true), None, None]);
}

#[test]
fn a_recall_exactly_at_its_bar_reaches_it() {
    let at = ClassScore { labelled: 20, precision: Some(1.0), recall: Some(0.95) };
    let under = ClassScore { labelled: 20, precision: Some(1.0), recall: Some(0.949) };
    let none = ClassScore { labelled: 0, precision: None, recall: None };
    assert_eq!(judged(&[at, at, at, at]), [Some(true), Some(true), None, None]);
    assert_eq!(judged(&[under, under, none, none])[0], Some(false));
    // 0.949 reaches the contradicting Bar of 0.80 even as it misses 0.95.
    assert_eq!(judged(&[under, under, none, none])[1], Some(true));
    assert_eq!(judged(&[none, none, none, none]), [None; 4]);
}

#[test]
fn a_standing_is_named_for_the_report_and_unreadable_is_no_class() {
    assert_eq!(name(&degenerate()), "degenerate");
    assert_eq!(name(&contradicting()), "contradicting");
    assert_eq!(name(&Standing::Unsupported), "unsupported");
    assert_eq!(name(&Standing::Consistent), "consistent");
    assert_eq!(name(&Standing::Unreadable { reason: "w".into() }), "unreadable");
}

#[test]
fn a_modified_set_no_longer_prints_to_its_frozen_fingerprint() {
    let frozen = set();
    assert!(frozen.unchanged());
    // A changed label, a changed claim, a claim removed, and the same claims
    // reordered each give another fingerprint.
    let mut relabelled = set();
    relabelled.claims[0].2 = Standing::Consistent;
    let mut rekinded = set();
    rekinded.claims[0].1.kind = "was-a".to_string();
    let mut shorter = set();
    shorter.claims.pop();
    let mut reordered = set();
    reordered.claims.reverse();
    for modified in [relabelled, rekinded, shorter, reordered] {
        assert!(!modified.unchanged(), "a modification kept the fingerprint");
        assert_ne!(LabelledSet::fingerprint_of(&modified.claims), frozen.fingerprint);
    }
    // And the transcripts the false claims came from are kept beside them.
    assert_eq!(frozen.source_transcripts.len(), 1);
}
