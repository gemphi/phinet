//! Property 52: the exact Mode-count agreement boundary.

use crate::ladder::counts_agree;
use phistd::gen::{for_all, Gen, Seeded, CASES};

#[derive(Debug)]
struct Triples(Vec<[usize; 3]>);

impl Gen for Triples {
    fn draw(g: &mut Seeded) -> Self {
        let scale = 1 + g.below(10_000) as usize;
        let arbitrary = [
            g.below(20_001) as usize,
            g.below(20_001) as usize,
            g.below(20_001) as usize,
        ];
        Triples(vec![
            [0, 0, 0],
            [5 * scale, 5 * scale, 5 * scale],
            // Mean 5s: the low and high values lie exactly at 80% and 120%.
            [4 * scale, 5 * scale, 6 * scale],
            // One unit inside and outside the lower boundary.
            [4 * scale + 1, 5 * scale - 1, 6 * scale],
            [4 * scale - 1, 5 * scale + 1, 6 * scale],
            // One unit inside and outside the upper boundary.
            [4 * scale, 5 * scale + 1, 6 * scale - 1],
            [4 * scale, 5 * scale - 1, 6 * scale + 1],
            arbitrary,
        ])
    }
}

fn direct_predicate(counts: [usize; 3]) -> bool {
    let mean = counts.iter().sum::<usize>() as f64 / 3.0;
    if mean == 0.0 {
        // Zero mean means [0, 0, 0]; all three counts are exactly at it.
        return true;
    }
    counts
        .into_iter()
        .all(|count| (count as f64 - mean).abs() <= mean * 0.2 + 1e-12)
}

#[test]
fn property_52_the_counts_agree_exactly_when_each_lies_within_20_percent_of_their_mean() {
    // **Property 52: The counts agree exactly when each lies within 20% of their mean**
    // **Validates: Requirements 5.10**
    let run = for_all::<Triples>(0x52, CASES, |drawn| {
        drawn
            .0
            .iter()
            .copied()
            .all(|triple| counts_agree(triple) == direct_predicate(triple))
            && counts_agree([0, 0, 0])
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
