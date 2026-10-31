//! Properties 45 and 47: unchanged settings and the first stopping pass.

use super::*;
use phistd::gen::{for_all, Gen, Seeded, CASES};

#[derive(Debug)]
struct LearningCases(Vec<(Settings, Settings, usize)>);

impl Gen for LearningCases {
    fn draw(g: &mut Seeded) -> Self {
        let word = Settings {
            target_classes: g.below(256) as usize,
            max_passes: 1 + g.below(200) as usize,
            stop_fraction: (1 + g.below(100)) as f64 / 100_000.0,
            min_unit_frequency: g.below(32) as usize,
            split_cost_penalty: g.below(1_000) as f64 / 100.0,
            move_order_seed: g.u64(),
        };
        let offset = g.below(6) as usize;
        let mut cases = Vec::with_capacity(7);
        for drift_count in 0..=6 {
            let mut sentence = word;
            for step in 0..drift_count {
                perturb(&mut sentence, (offset + step) % 6);
            }
            cases.push((word, sentence, g.below(4) as usize));
        }
        LearningCases(cases)
    }
}

fn perturb(settings: &mut Settings, field: usize) {
    match field {
        0 => settings.target_classes += 1,
        1 => settings.max_passes += 1,
        2 => settings.stop_fraction += 0.000_001,
        3 => settings.min_unit_frequency += 1,
        4 => settings.split_cost_penalty += 0.125,
        5 => settings.move_order_seed = settings.move_order_seed.wrapping_add(1),
        _ => unreachable!(),
    }
}

fn direct_differences(word: &Settings, sentence: &Settings) -> Vec<&'static str> {
    let changed = [
        word.target_classes != sentence.target_classes,
        word.max_passes != sentence.max_passes,
        word.stop_fraction != sentence.stop_fraction,
        word.min_unit_frequency != sentence.min_unit_frequency,
        word.split_cost_penalty != sentence.split_cost_penalty,
        word.move_order_seed != sentence.move_order_seed,
    ];
    Settings::NAMES
        .into_iter()
        .zip(changed)
        .filter_map(|(name, changed)| changed.then_some(name))
        .collect()
}

#[test]
fn property_45_the_second_height_learns_with_no_new_setting_or_every_drifting_setting_is_named() {
    // **Property 45: The second height learns with no new setting, or every drifting setting is named**
    // **Validates: Requirements 9.1, 9.2**
    let run = for_all::<LearningCases>(0x45, CASES, |drawn| {
        drawn.0.iter().all(|(word, sentence, kept)| {
            let expected = direct_differences(word, sentence);
            if Settings::differing(word, sentence) != expected {
                return false;
            }
            let beside = Settings::beside(word, sentence);
            if beside.map(|entry| entry.0) != Settings::NAMES {
                return false;
            }
            match learned_how_to_learn(word, sentence, *kept) {
                Judged::Pass => expected.is_empty() && *kept >= 1,
                Judged::Miss { differing } => {
                    !(expected.is_empty() && *kept >= 1) && differing == expected
                }
            }
        })
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

#[derive(Debug)]
struct GainCases(Vec<Vec<f64>>);

impl Gen for GainCases {
    fn draw(g: &mut Seeded) -> Self {
        let first = 1.0 + g.below(10_000) as f64;
        let monotone = (0..120).map(|i| first * 0.9_f64.powi(i)).collect();

        let mut oscillating: Vec<f64> = (0..120)
            .map(|i| {
                if i % 2 == 0 {
                    first * 0.5
                } else {
                    first * 0.01
                }
            })
            .collect();
        oscillating[0] = first;
        oscillating[2 + g.below(88) as usize] = first * 0.000_5;

        let mut immediate = vec![first, first * 0.000_5];
        immediate.resize(120, first);
        let equal = vec![first; 120];
        let mut over_hundred = vec![first; 111];
        over_hundred[110] = 0.0;
        let zero_start = vec![0.0; 120];

        GainCases(vec![
            monotone,
            oscillating,
            immediate,
            equal,
            over_hundred,
            zero_start,
        ])
    }
}

fn direct_stop(gains: &[f64]) -> (usize, Stopped) {
    let threshold = gains[0] * 0.001;
    if let Some(offset) = gains.iter().take(100).position(|gain| *gain < threshold) {
        let index = offset + 1;
        (index, Stopped::GainBelowThousandth { at: index })
    } else {
        (gains.len().min(100), Stopped::PassLimit)
    }
}

#[test]
fn property_47_the_difference_engine_stops_at_the_first_qualifying_pass() {
    // **Property 47: The difference engine stops at the first qualifying pass**
    // **Validates: Requirements 9.4, 9.5**
    let run = for_all::<GainCases>(0x47, CASES, |drawn| {
        drawn
            .0
            .iter()
            .all(|gains| stop_at(gains) == direct_stop(gains))
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
