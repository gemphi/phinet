//! Properties 23 and 24: the Variation's three classes and the River's
//! run-bounded adjacency.

use crate::ladder::{river, variation, Variation};
use phistd::gen::{for_all, Gen, Seeded, CASES};
use phiton::p64::variants::Level;

/// A sentence of 3 to 50 classes at one named Mode. Three-word sentences
/// receive three quarters of the draws, because their second and last
/// positions are adjacent and expose accidental second-from-last readings.
#[derive(Debug)]
struct Classes {
    level: Level,
    written: Vec<u16>,
}

impl Gen for Classes {
    fn draw(g: &mut Seeded) -> Self {
        let level = *g.pick(&[Level::P16, Level::P32, Level::P64]);
        let len = match g.below(4) {
            0..=2 => 3,
            _ => 4 + g.below(47) as usize,
        };
        let written = (0..len)
            .map(|_| g.below(level.count() as u64) as u16)
            .collect();
        Classes { level, written }
    }
}

#[test]
fn property_23_a_variation_is_the_sentences_first_second_and_last_class() {
    // Property 23: A Variation is the sentence's first, second and last class.
    // **Validates: Requirements 5.4**
    let run = for_all::<Classes>(0x23, CASES, |drawn| {
        variation(&drawn.written, drawn.level)
            == Some(Variation {
                level: drawn.level,
                classes: [
                    drawn.written[0],
                    drawn.written[1],
                    drawn.written[drawn.written.len() - 1],
                ],
            })
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

/// One to fifty runs of zero to forty sentences. Empty and one-sentence
/// runs receive half the draws, so boundaries that contribute no pair are
/// exercised heavily. Sentence ids are globally unique, making any
/// cross-boundary pair observable.
#[derive(Debug)]
struct Runs(Vec<Vec<usize>>);

impl Gen for Runs {
    fn draw(g: &mut Seeded) -> Self {
        let count = 1 + g.below(50) as usize;
        let mut next = 0usize;
        let runs = (0..count)
            .map(|_| {
                let len = match g.below(4) {
                    0 => 0,
                    1 => 1,
                    _ => g.below(41) as usize,
                };
                let run: Vec<usize> = (next..next + len).collect();
                next += len;
                run
            })
            .collect();
        Runs(runs)
    }
}

#[test]
fn property_24_the_river_pairs_only_adjacent_sentences_and_never_across_a_run() {
    // Property 24: The River pairs only adjacent sentences, and never across a run.
    // **Validates: Requirements 5.5**
    let run = for_all::<Runs>(0x24, CASES, |drawn| {
        let expected: Vec<(usize, usize)> = drawn
            .0
            .iter()
            .flat_map(|run| run.windows(2).map(|pair| (pair[0], pair[1])))
            .collect();
        let boundaries: Vec<(usize, usize)> = drawn
            .0
            .windows(2)
            .filter_map(|runs| Some((*runs[0].last()?, *runs[1].first()?)))
            .collect();
        let got = river(&drawn.0);
        got == expected && boundaries.iter().all(|pair| !got.contains(pair))
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
