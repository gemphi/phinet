//! The golden test and the re-recording checks over the two numbers pinned
//! on The_Guide.

use super::{
    drift, fingerprint, measured_loss, measured_scores, task_4_2_rerecording, Drift, Part,
    Rerecord, Verdict, GOLDEN_FINGERPRINT, GOLDEN_LOSS, GOLDEN_SCORES,
};

/// The golden test proper: the measured loss and the score fingerprint are
/// the pinned ones, so the numbers on The_Guide have not moved (R11.9). A
/// drift fails here, names both numbers side by side, and is a Miss.
#[test]
fn the_numbers_on_the_guide_do_not_move() {
    let drift = drift(measured_loss(), &measured_scores());
    assert_eq!(drift, Drift::Held, "{}", drift.report());
    assert!(!drift.is_miss());
}

/// The pinned fingerprint is the fold over the pinned scores. If this fails
/// the pin and the scores it is a pin of have drifted apart — read the
/// measured value from the message and re-pin `GOLDEN_FINGERPRINT`.
#[test]
fn the_pinned_fingerprint_is_the_fold_over_the_pinned_scores() {
    let folded = fingerprint(GOLDEN_SCORES);
    assert_eq!(
        folded, GOLDEN_FINGERPRINT,
        "the score fingerprint folds to {folded:#018x}; the pin is {GOLDEN_FINGERPRINT:#018x}",
    );
}

/// The pin is worth having only if it moves when a score does. Each score
/// nudged by one tenth — past the one-decimal rounding — gives a fingerprint
/// that is not the pinned one, and a score dropped does too.
#[test]
fn a_moved_score_fails_the_fingerprint() {
    for at in 0..GOLDEN_SCORES.len() {
        let mut moved = GOLDEN_SCORES.to_vec();
        moved[at] += 0.1;
        assert_ne!(
            fingerprint(&moved),
            GOLDEN_FINGERPRINT,
            "score {at} moved by a tenth kept the fingerprint",
        );
    }
    let dropped = &GOLDEN_SCORES[..GOLDEN_SCORES.len() - 1];
    assert_ne!(fingerprint(dropped), GOLDEN_FINGERPRINT, "a dropped score kept the fingerprint");
    let mut reordered = GOLDEN_SCORES.to_vec();
    reordered.reverse();
    assert_ne!(fingerprint(&reordered), GOLDEN_FINGERPRINT, "reordered scores kept the fingerprint");
}

/// A loss drift past the epsilon is a Miss that names the pinned value beside
/// the measured one (R11.9); a wobble within the epsilon is the same reading.
#[test]
fn a_moved_loss_is_a_miss_that_names_both_numbers() {
    let held = drift(GOLDEN_LOSS + 0.05, GOLDEN_SCORES);
    assert_eq!(held, Drift::Held, "a loss within a tenth is not a drift");

    let moved = drift(GOLDEN_LOSS + 5.0, GOLDEN_SCORES);
    assert_eq!(moved, Drift::Loss { pinned: GOLDEN_LOSS, measured: GOLDEN_LOSS + 5.0 });
    assert!(moved.is_miss());
    let report = moved.report();
    assert!(report.contains(&format!("{GOLDEN_LOSS}")), "the report names the pinned loss");
    assert!(report.contains(&format!("{}", GOLDEN_LOSS + 5.0)), "and the measured loss");
}

/// A changed score with the loss held is a score drift, not a loss drift: the
/// failure names the fingerprint that moved rather than the loss that did not.
#[test]
fn a_moved_score_with_a_held_loss_is_a_score_drift() {
    let mut moved = GOLDEN_SCORES.to_vec();
    moved[0] += 1.0;
    match drift(GOLDEN_LOSS, &moved) {
        Drift::Score { pinned, measured } => {
            assert_eq!(pinned, GOLDEN_FINGERPRINT);
            assert_ne!(measured, GOLDEN_FINGERPRINT);
        }
        other => panic!("a moved score read as {other:?}"),
    }
}

/// The re-recording path (R11.10): the previous values, the new values and
/// the reason all present is `Recorded` and does not fail; any one absent is
/// `Unjudged`, names what is missing on its page, and fails.
#[test]
fn a_re_recording_needs_all_three_of_previous_new_and_reason() {
    let whole = Rerecord::new("loss 101.2; scores …", "loss 99.8; scores …", "a better move rule");
    assert_eq!(whole.judge(), Verdict::Recorded);
    assert!(!whole.judge().fails());
    assert!(whole.page().contains("verdict: recorded"));

    for absent in Part::ALL {
        let mut partial = whole.clone();
        match absent {
            Part::Previous => partial.previous = None,
            Part::New => partial.new = None,
            Part::Reason => partial.reason = None,
        }
        let verdict = partial.judge();
        assert!(verdict.fails(), "a re-recording missing the {} fails", absent.name());
        assert_eq!(verdict, Verdict::Unjudged { absent: vec![absent] });
        assert!(partial.page().contains("verdict: unjudged"));
        assert!(partial.page().contains(absent.name()));
    }
}

/// A blank part is an absent part: a reason of spaces or an empty previous
/// value is unstated however it is spelled, so it is `Unjudged` too.
#[test]
fn a_blank_part_is_an_absent_part() {
    let blank_reason = Rerecord::new("was", "now", "   ");
    assert!(blank_reason.judge().fails(), "a blank reason is unstated");

    let empty_previous = Rerecord::new("", "now", "a reason");
    assert!(empty_previous.judge().fails(), "an empty previous value is absent");
}

/// Task 4.2's deliberate change to what is learned is accounted for through
/// the re-recording path: it carries its before, its after and its why, so it
/// is `Recorded` rather than a silent drift that fails the golden test
/// (R11.10). This is the one change the plan says must not be a silent drift.
#[test]
fn task_4_2s_change_is_recorded_not_a_silent_drift() {
    let rerecording = task_4_2_rerecording();
    assert_eq!(rerecording.judge(), Verdict::Recorded);
    assert!(!rerecording.judge().fails());
    let page = rerecording.page();
    assert!(page.contains("gain > 0.0"), "the previous rule is named");
    assert!(page.contains("staged"), "the new rule is named");
    assert!(page.contains("4.2"), "the reason names the task");
}
