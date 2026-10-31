use super::*;

fn amplitudes(probabilities: &[f64]) -> Vec<c64> {
    probabilities
        .iter()
        .map(|probability| c64::from_polar(probability.sqrt(), 0.0))
        .collect()
}

fn sentence(
    split_index: usize,
    label: Function,
    opening_kind: usize,
    closing_kind: usize,
    both_kind: usize,
    opening_probabilities: &[f64],
    closing_probabilities: &[f64],
) -> TestReading {
    TestReading {
        split_index,
        label,
        opening_kind,
        closing_kind,
        both_kind,
        opening_probabilities: opening_probabilities.to_vec(),
        closing_probabilities: closing_probabilities.to_vec(),
        opening_amplitudes: amplitudes(opening_probabilities),
        closing_amplitudes: amplitudes(closing_probabilities),
    }
}

fn missed(level: Level) -> ModeInput {
    let same = [0.8, 0.1, 0.1];
    ModeInput {
        level,
        sentences: vec![
            sentence(0, Function::Explain, 0, 0, 0, &same, &same),
            sentence(10, Function::Explain, 0, 1, 0, &same, &same),
            sentence(20, Function::Instruct, 1, 1, 0, &same, &same),
        ],
    }
}

#[test]
fn all_three_modes_report_rounded_purity_and_continue_after_misses() {
    let inputs = TWIN_MODES.map(missed);
    let reports = measure_modes(&inputs);
    assert_eq!(reports.each_ref().map(|report| report.level), TWIN_MODES);
    for report in reports {
        assert_eq!(report.scored, 3);
        assert_eq!(report.purity, TwinPurity { opening: 100.0, closing: 66.7, both: 66.7 });
        assert_eq!(report.both_ends.bar.value, 102.0);
        assert_eq!(
            report.both_ends.verdict,
            BarJudged::Miss { value: 66.7, bar: 102.0, shortfall: 35.3 }
        );
    }
}

#[test]
fn both_ends_passes_only_two_points_above_the_better_single_end() {
    let reading = TwinPurity { opening: 50.0, closing: 50.0, both: 52.0 };
    assert_eq!(judge_twins(reading), BarJudged::Pass);
    assert!(matches!(
        judge_twins(TwinPurity { both: 51.9, ..reading }),
        BarJudged::Miss { .. }
    ));
}

#[test]
fn disagreements_keep_the_total_but_list_only_the_first_hundred_indices() {
    let indices: Vec<usize> = (0..105).map(|i| i * 10).collect();
    let opening = vec![3; indices.len()];
    let closing = vec![9; indices.len()];
    let report = disagreements_at(&indices, &opening, &closing);
    assert_eq!(report.count, 105);
    assert_eq!(report.listed.len(), MAX_LISTED);
    assert_eq!(report.listed.first().unwrap().split_index, 0);
    assert_eq!(report.listed.last().unwrap().split_index, 990);
    assert!(report.listed.iter().all(|row| (row.opening_kind, row.closing_kind) == (3, 9)));
}

#[test]
fn probability_addition_and_the_circle_operator_each_form_a_distribution() {
    assert_eq!(combine_probabilities(&[0.25, 0.75], &[0.75, 0.25]), vec![0.5, 0.5]);

    let a = amplitudes(&[0.6, 0.34, 0.06]);
    let mut b = amplitudes(&[0.1, 0.34, 0.56]);
    b[1] *= c64::from_polar(1.0, 1.7);
    let combined = combine_amplitudes(&a, &b);
    assert!(combined.iter().all(|value| *value >= 0.0));
    assert!((combined.iter().sum::<f64>() - 1.0).abs() <= 1e-12);
    assert_eq!(greatest(&combined), 1, "balanced support wins after amplitudes are squared");
}

#[test]
fn amplitude_purity_is_judged_against_probability_purity_in_points() {
    let easy = [0.8, 0.1, 0.1];
    let crossed_a = [0.6, 0.34, 0.06];
    let crossed_b = [0.1, 0.34, 0.56];
    let input = ModeInput {
        level: Level::P16,
        sentences: vec![
            sentence(0, Function::Explain, 0, 0, 0, &easy, &easy),
            sentence(10, Function::Explain, 0, 0, 0, &easy, &easy),
            sentence(20, Function::Instruct, 0, 0, 0, &crossed_a, &crossed_b),
            sentence(30, Function::Instruct, 0, 0, 0, &crossed_a, &crossed_b),
        ],
    };
    let report = measure_mode(&input).combinations;
    assert_eq!(report.probability, 50.0);
    assert_eq!(report.amplitude.value, 100.0);
    assert_eq!(report.amplitude.bar.value, 50.0);
    assert_eq!(report.amplitude.verdict, BarJudged::Pass);
    assert_eq!(report.difference_points, 50.0);
}
