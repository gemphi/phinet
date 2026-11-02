//! The measurement of R8.8 and R8.9 over the frozen labelled set: the
//! detector reads every claim against a World the set's facts were held in,
//! then the eight numbers — precision and recall one-versus-rest for each of
//! the four classes — are recorded as findings, and the two Bars R8.9
//! declares are judged: degenerate-class recall against 0.95 and
//! contradicting-class recall against 0.80.
//!
//! This is an integration test rather than a unit test because it reads the
//! frozen file under `tests/` the same way a run does, through
//! `phisor::truth::set::read`, and runs the detector over it exactly once:
//! the freeze is what makes the numbers a measurement rather than a fit.

use phiecs::quest::{Row, Standing};
use phiecs::world::World;
use phisor::truth::score::{judged, ClassScore, CLASS, RECALL_BAR};
use phisor::truth::set;
use phisor::truth::{Declared, Detector};

/// The World the frozen set's false claims were read against: one held row
/// per functional kind and about, so a second, different other for that kind
/// reads as contradicting rather than unsupported. Nothing is held for the
/// words the consistent claims are about, so each of those reads unsupported
/// — the classifier core returns no Consistent (R8.1), and that is reported
/// as a finding, not judged.
fn held_world() -> World {
    let rows = [
        Row::new("cat", "colour", "black"),
        Row::new("dog", "colour", "brown"),
        Row::new("paris", "colour", "grey"),
        Row::new("sky", "colour", "blue"),
        Row::new("iron", "colour", "grey"),
        Row::new("whale", "colour", "grey"),
        Row::new("cat", "capital-of", "void"),
        Row::new("dog", "capital-of", "void"),
        Row::new("paris", "capital-of", "france"),
        Row::new("iron", "melts-at", "fifteen-hundred"),
        Row::new("whale", "bigger-than", "shark"),
        Row::new("whale", "is-a", "mammal"),
    ];
    World::from_rows(&rows)
}

/// The Laws the quest declared its kinds under: the functional kinds a
/// second other breaks, matching the held rows above.
fn declared() -> Declared {
    Declared::none().functional(&["colour", "capital-of", "melts-at", "bigger-than", "is-a"])
}

#[test]
fn the_frozen_set_scores_and_the_two_bars_are_judged() {
    let set = set::read().unwrap_or_else(|f| panic!("{f}"));
    // The set is read once and left unmodified: the measurement is on the
    // frozen bytes.
    assert!(set.unchanged());

    let mut detector = Detector::new(declared());
    let scores = detector.judge_set(&set, &held_world());

    // The eight numbers, recorded as findings — precision and recall for
    // each class beside its labelled count.
    for (i, s) in scores.iter().enumerate() {
        println!(
            "{:>13}: labelled {:>3}  precision {}  recall {}",
            CLASS[i],
            s.labelled,
            show(s.precision),
            show(s.recall)
        );
    }

    // Every class holds at least one labelled claim, so no number is
    // undefined here.
    for (i, s) in scores.iter().enumerate() {
        assert!(s.labelled > 0, "{} holds no labelled claim", CLASS[i]);
        assert!(s.precision.is_some() && s.recall.is_some(), "{} is undefined", CLASS[i]);
    }

    // The two Bars R8.9 declares.
    let verdicts = judged(&scores);
    let degenerate = verdicts[0].expect("degenerate recall is judged against 0.95");
    let contradicting = verdicts[1].expect("contradicting recall is judged against 0.80");
    assert!(degenerate, "degenerate recall {} below the 0.95 Bar", show(scores[0].recall));
    assert!(contradicting, "contradicting recall {} below the 0.80 Bar", show(scores[1].recall));

    // The other two classes carry no Bar, so neither is judged.
    assert_eq!(verdicts[2], None, "unsupported carries no Bar");
    assert_eq!(verdicts[3], None, "consistent carries no Bar");
    assert_eq!(RECALL_BAR, [Some(0.95), Some(0.80), None, None]);
}

#[test]
fn every_degenerate_and_contradicting_claim_is_read_as_its_class() {
    // The recall Bars pass because the held World makes the contradicting
    // claims break a Law and the degenerate ones self-edge or empty-field,
    // so each false claim of those two classes is read as its own class.
    let set = set::read().unwrap();
    let mut detector = Detector::new(declared());
    let scores = detector.judge_set(&set, &held_world());
    // Degenerate is read with no world dependence, so its recall is whole.
    assert_eq!(scores[0].recall, Some(1.0), "degenerate recall is not whole");
    // Contradicting reaches its Bar: at least four in five are read as such.
    assert!(scores[1].recall.unwrap() >= 0.80, "contradicting recall {:?}", scores[1].recall);
}

/// A score to three places or `undefined`, as R8.8 reports it.
fn show(x: Option<f64>) -> String {
    match x {
        Some(v) => format!("{v:.3}"),
        None => "undefined".to_string(),
    }
}

/// Guard: `ClassScore` is the shape the report reads, four of them.
#[allow(dead_code)]
fn _shape(_: [ClassScore; 4], _: Standing) {}
