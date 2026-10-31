use super::*;

fn settings() -> Settings {
    Settings {
        target_classes: 64,
        max_passes: 100,
        stop_fraction: 0.001,
        min_unit_frequency: 3,
        split_cost_penalty: 0.25,
        move_order_seed: 42,
    }
}

#[test]
fn names_and_beside_report_all_six_settings_in_order() {
    assert_eq!(
        Settings::NAMES,
        [
            "target_classes",
            "max_passes",
            "stop_fraction",
            "min_unit_frequency",
            "split_cost_penalty",
            "move_order_seed",
        ]
    );
    let word = settings();
    let mut sentence = word;
    sentence.target_classes = 32;
    sentence.max_passes = 99;
    sentence.stop_fraction = 0.002;
    sentence.min_unit_frequency = 4;
    sentence.split_cost_penalty = 0.5;
    sentence.move_order_seed = 43;
    assert_eq!(
        Settings::beside(&word, &sentence),
        [
            ("target_classes", 64.0, 32.0),
            ("max_passes", 100.0, 99.0),
            ("stop_fraction", 0.001, 0.002),
            ("min_unit_frequency", 3.0, 4.0),
            ("split_cost_penalty", 0.25, 0.5),
            ("move_order_seed", 42.0, 43.0),
        ]
    );
    assert_eq!(Settings::differing(&word, &sentence), Settings::NAMES);
}

#[test]
fn learning_requires_matching_settings_and_a_kept_move() {
    let word = settings();
    assert_eq!(learned_how_to_learn(&word, &word, 1), Judged::Pass);
    assert_eq!(
        learned_how_to_learn(&word, &word, 0),
        Judged::Miss { differing: vec![] }
    );
    let mut sentence = word;
    sentence.max_passes += 1;
    sentence.move_order_seed += 1;
    assert_eq!(
        learned_how_to_learn(&word, &sentence, 4),
        Judged::Miss {
            differing: vec!["max_passes", "move_order_seed"]
        }
    );
}

#[test]
fn pass_records_its_index_counts_and_gain() {
    let pass = Pass {
        index: 7,
        evaluated: 20,
        kept: 3,
        rejected: 17,
        gain: 0.125,
    };
    assert_eq!(
        (
            pass.index,
            pass.evaluated,
            pass.kept,
            pass.rejected,
            pass.gain
        ),
        (7, 20, 3, 17, 0.125)
    );
}

#[test]
fn the_gain_boundary_is_strict_and_the_hundredth_pass_is_the_limit() {
    let mut gains = vec![10.0, 0.01];
    gains.resize(PASS_LIMIT + 1, 10.0);
    assert_eq!(stop_at(&gains), (PASS_LIMIT, Stopped::PassLimit));

    gains[1] = 0.009;
    assert_eq!(stop_at(&gains), (2, Stopped::GainBelowThousandth { at: 2 }));

    let zeros = vec![0.0; PASS_LIMIT + 1];
    assert_eq!(stop_at(&zeros), (PASS_LIMIT, Stopped::PassLimit));
}
