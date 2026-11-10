use super::*;
use crate::ladder::tests::corpus;

#[test]
fn a_variation_reads_first_second_and_last_at_the_named_mode() {
    assert_eq!(
        variation(&[9, 2, 14, 5, 1, 11], Level::P32),
        Some(Variation {
            level: Level::P32,
            classes: [9, 2, 11],
        })
    );
    // In a three-word sentence the second class is also the last class.
    assert_eq!(
        variation(&[7, 4, 6], Level::P16),
        Some(Variation {
            level: Level::P16,
            classes: [7, 4, 6],
        })
    );
    assert_eq!(variation(&[7, 4], Level::P64), None);
}

#[test]
fn the_river_pairs_only_immediate_predecessors_inside_each_run() {
    let runs = vec![vec![4, 5, 6], vec![], vec![10], vec![20, 21]];
    assert_eq!(river(&runs), [(4, 5), (5, 6), (20, 21)]);
    assert!(!river(&runs).contains(&(6, 10)));
    assert!(!river(&runs).contains(&(10, 20)));
}

#[test]
fn unshaped_sentences_break_pairs_but_do_not_hide_shaped_variations() {
    let written = vec![vec![1, 2, 3], vec![8, 9], vec![4, 5, 6]];
    let settings = Settings {
        max_passes: 1,
        ..Settings::default()
    };
    let report = kinds(
        Arm::HashNull,
        Level::P16,
        &written,
        &[vec![0, 1, 2]],
        &settings,
    );
    assert!(report.pairs.is_empty(), "an unshaped sentence was bridged");
    assert_eq!(report.settings, settings);
    assert_eq!(report.of(1), None);
    let mut reported: Vec<[u16; 3]> = report
        .kinds
        .iter()
        .flat_map(|kind| kind.variations.iter().map(|v| v.classes))
        .collect();
    reported.sort_unstable();
    assert_eq!(reported, [[1, 2, 3], [4, 5, 6]]);
}

#[test]
fn every_occupied_kind_is_reported_even_when_no_labels_are_supplied() {
    // The Kind height accepts no labels. Thus every group here is an
    // unmatched-label Kind, and every one still carries its Variations.
    let written = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let report = kinds(
        Arm::Given,
        Level::P64,
        &written,
        &[vec![0], vec![1], vec![2]],
        &Settings::default(),
    );
    assert_eq!(report.occupied, report.kinds.len());
    assert!(report.kinds.iter().all(|kind| !kind.variations.is_empty()));
    assert_eq!(
        report.kinds.iter().map(|kind| kind.variations.len()).sum::<usize>(),
        3
    );
}

#[test]
fn hash_given_and_learned_arms_each_report_kinds_and_variations() {
    let split = crate::guide::Split::new(corpus());
    let ladder = Ladder::new(&split, crate::ladder::Seeds::DECLARED);
    let settings = Settings {
        max_passes: 1,
        ..Settings::default()
    };
    let reports = ladder.all_kinds(Level::P16, &settings);
    assert_eq!(reports.each_ref().map(|report| report.arm), Arm::ALL);
    for report in reports {
        assert_eq!(report.level, Level::P16);
        assert_eq!(report.settings, settings);
        assert_eq!(report.occupied, report.kinds.len());
        assert!(report.occupied > 0, "{} reported no Kind", report.arm.name());
        assert!(report.kinds.iter().all(|kind| !kind.variations.is_empty()));
        assert!(report
            .halving
            .passes
            .iter()
            .all(|passes| passes.len() <= settings.max_passes));
    }
}
