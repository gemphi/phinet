//! Property 51: a run repeats from its recorded seeds.

use super::*;
use crate::guide::{Function, Mark, Sentence, Split};
use phistd::gen::{for_all, Gen, Seeded, CASES};
use phiton::p64::variants::Level;

/// A small fixed corpus: four shapes told over several units, every word
/// told more than twice so the Learned arm's code has something to grow and
/// the River has held-out pairs to score.
fn corpus() -> Vec<Sentence> {
    let said: [&[&str]; 4] = [
        &["the", "cat", "sat", "on", "the", "mat"],
        &["were", "you", "waiting", "in", "the", "rain"],
        &["yes", "i", "was", "waiting", "in", "the", "rain"],
        &["the", "dog", "ran", "over", "the", "hill"],
    ];
    (0..8)
        .flat_map(|unit| {
            said.into_iter().map(move |words| Sentence {
                words: words.iter().map(|w| w.to_string()).collect(),
                function: Function::Explain,
                mark: Mark::Stop,
                unit,
                run: 0,
            })
        })
        .collect()
}

/// The three Modes, picked by a drawn case so every Mode is exercised.
const MODES: [Level; 3] = [Level::P16, Level::P32, Level::P64];

#[derive(Debug)]
struct RunCase {
    seeds: Seeds,
    level: Level,
}

impl Gen for RunCase {
    fn draw(g: &mut Seeded) -> Self {
        RunCase {
            seeds: Seeds {
                move_order: g.u64(),
                shuffles: g.u64(),
                phases: g.u64(),
            },
            level: *g.pick(&MODES),
        }
    }
}

#[test]
fn property_51_a_run_repeats_from_its_recorded_seeds() {
    // **Property 51: A run repeats from its recorded seeds**
    // **Validates: Requirements 9.15**
    //
    // A run built from one record over one split reports the same numbers
    // every time, and a second run built from the same record and split
    // agrees number for number at the reported precision.
    let split = Split::new(corpus());
    let run = for_all::<RunCase>(0x51, CASES, |case| {
        let record = Record::of(case.seeds, case.level, &split);
        let first = Run::new(record, &split).unwrap();
        let again = Run::new(record, &split).unwrap();
        // The record round-trips to the seeds it was built from.
        if record.seeds() != case.seeds {
            return false;
        }
        // A repeat agrees exactly, and reading the same run twice agrees.
        first.numbers() == again.numbers() && first.numbers() == first.numbers()
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
