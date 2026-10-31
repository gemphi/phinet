use super::*;
use crate::guide::holdout::{HoldoutGuard, Phase};
use crate::guide::{Function, Mark, Sentence, Split};
use crate::ladder::{kinds, Arm, CodeReport, DepthReport, Height, Ladder, Seeds};
use phical::classes::{Halving, Settings};
use phiton::p64::variants::Level;

fn sentence(function: Function, mark: Mark, unit: usize, run: u8) -> Sentence {
    Sentence {
        words: ["the", "same", "shape", "again"]
            .into_iter()
            .map(String::from)
            .collect(),
        function,
        mark,
        unit,
        run,
    }
}

/// Three held-out unit sentences carry one of each terminator. Their
/// functions are explain, instruct, explain; four Key answers follow.
fn labelled_split() -> Split {
    let mut sentences: Vec<Sentence> = (0..=20)
        .map(|i| {
            let function = if i == 10 {
                Function::Instruct
            } else {
                Function::Explain
            };
            let mark = match i {
                10 => Mark::Question,
                20 => Mark::Exclamation,
                _ => Mark::Stop,
            };
            sentence(function, mark, 0, 0)
        })
        .collect();
    sentences.extend((0..4).map(|_| sentence(Function::Answer, Mark::Line, 1, 1)));
    Split::new(sentences)
}

fn one_kind(split: &Split, arm: Arm) -> crate::ladder::KindReport {
    let written = vec![vec![1, 2, 3]; split.sentences.len()];
    kinds(
        arm,
        Level::P16,
        &written,
        &split.training_runs(),
        &Settings {
            max_passes: 1,
            ..Settings::default()
        },
    )
}

#[test]
fn function_labels_are_read_only_by_the_scorer_and_rounded_to_one_decimal() {
    let split = labelled_split();
    let report = one_kind(&split, Arm::Learned);
    let mut guard = HoldoutGuard::new(vec![]);
    assert!(guard.per_phase_reads().is_empty());

    let scored = function_purity(&report, &split, &mut guard).unwrap();
    assert_eq!(scored.table, [vec![2.0, 1.0, 4.0]]);
    assert_eq!((scored.scored, scored.unassigned), (7, 0));
    assert_eq!(scored.purity.value, 57.1);
    assert_eq!(scored.purity.bar.value, FUNCTION_BAR);
    assert_eq!(scored.purity.verdict, Judged::Pass);
    assert!(guard
        .per_phase_reads()
        .iter()
        .all(|(phase, _, _)| *phase == Phase::ScoreNumber));
    assert_eq!(
        guard.per_phase_reads(),
        vec![
            (Phase::ScoreNumber, "answer".into(), 4),
            (Phase::ScoreNumber, "explain".into(), 2),
            (Phase::ScoreNumber, "instruct".into(), 1),
        ]
    );
}

#[test]
fn equal_kind_gap_and_terminators_record_their_declared_readings() {
    let split = labelled_split();
    let learned = one_kind(&split, Arm::Learned);
    let hash = one_kind(&split, Arm::HashNull);
    let mut guard = HoldoutGuard::new(vec![]);

    let gap = function_gap(&learned, &hash, &split, &mut guard).unwrap();
    assert_eq!((gap.kinds, gap.scored), (1, 7));
    assert_eq!((gap.learned, gap.hash_null), (57.1, 57.1));
    assert_eq!(
        gap.difference.verdict,
        Judged::Miss {
            value: 0.0,
            bar: 5.0,
            shortfall: 5.0,
        }
    );

    let terminators = terminator_purity(&learned, &split, &mut guard).unwrap();
    assert_eq!(terminators.table, [vec![1.0, 1.0, 1.0]]);
    assert_eq!((terminators.scored, terminators.unassigned), (3, 0));
    assert_eq!(terminators.majority_baseline, 33.3);
    assert_eq!(terminators.purity.value, 33.3);
    assert_eq!(terminators.purity.verdict, Judged::Pass);
    assert_eq!(
        terminators.question_f1,
        Some(BestKindF1 {
            kind: 0,
            value: 0.5,
        })
    );
    // The words were identical for '.', '?' and '!': the terminator label
    // was not present in the Variation that formed the Kind.
    assert_eq!(learned.occupied, 1);
}

#[test]
fn river_uses_the_same_held_out_pairs_and_exactly_one_thousand_shuffles() {
    let split = labelled_split();
    let learned = one_kind(&split, Arm::Learned);
    let hash = one_kind(&split, Arm::HashNull);
    let report = river_z(&learned, &hash, &split, 0x5151).unwrap();
    assert_eq!(report.sentence_pairs, [(9, 10), (19, 20), (21, 22), (22, 23), (23, 24)]);
    assert_eq!(report.shuffles, SHUFFLES);
    assert_eq!(report.learned.null_draws, 1_000);
    assert_eq!(report.hash_null.null_draws, 1_000);
    assert_eq!(report.against_three.verdict, Judged::Miss { value: 0.0, bar: 3.0, shortfall: 3.0 });
    assert_eq!(report.against_hash.verdict, Judged::Pass);
}

#[test]
fn all_modes_and_hash_null_use_the_same_count_agreement_rule() {
    let split = labelled_split();
    let ladder = Ladder::new(&split, Seeds::DECLARED);
    let reports = ladder.all_counts(&Settings { max_passes: 1, ..Settings::default() });
    assert_eq!(reports.each_ref().map(|report| report.arm), Arm::ALL);
    for report in reports {
        assert_eq!(report.modes.each_ref().map(|mode| mode.level), crate::code::LEVELS);
        assert_eq!(report.agree, counts_agree(report.occupied));
        assert!(report.modes.iter().all(|mode| mode.variations_per_kind.len() == mode.occupied));
    }
}

#[test]
fn grown_counts_have_no_sixty_four_item_ceiling() {
    let written: Vec<Vec<u16>> = (0..100).map(|i| vec![i, i + 1, i + 2]).collect();
    let runs: Vec<Vec<usize>> = (0..written.len()).map(|i| vec![i]).collect();
    let kinds = kinds(Arm::Learned, Level::P64, &written, &runs, &Settings { max_passes: 1, ..Settings::default() });
    let code = CodeReport {
        settings: Settings::default(),
        depths: vec![DepthReport { depth: 7, classes: 137, splits_paid_for: 0 }],
        stopped_at: 7,
        halving: Halving { classes: 137, ..Halving::default() },
    };
    assert_eq!(grown(&kinds, &code), GrownCounts { templates: 100, word_classes: 137 });
}

#[test]
fn a_miss_is_retained_without_stopping_later_heights() {
    let miss = Measured::at_least("first", 40.0, 56.7, Unit::Percentage);
    let pass = Measured::at_least("later", 100.0, 100.0, Unit::Percentage);
    let mut log = MeasureLog::default();
    log.record(Height::Kind, miss);
    log.record(Height::Act, pass);
    assert_eq!(log.entries.len(), 2);
    assert!(matches!(log.entries[0].1.verdict, Judged::Miss { .. }));
    assert_eq!(log.entries[1].1.verdict, Judged::Pass);
}
