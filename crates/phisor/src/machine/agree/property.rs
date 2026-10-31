use phical::keyboard::{table, FrozenOperands, Type, COUNT};

use super::agree;
use crate::machine::{Bar, Judged, Sense};

/// Property 16: The machine's result and the Rust reader's result are
/// byte-identical. The frozen file is traversed whole, not sampled.
///
/// **Validates: Requirements 7.6, 7.9**
#[test]
fn property_16_the_machines_result_and_the_rust_readers_result_are_byte_identical() {
    let frozen = FrozenOperands::load();
    assert!(
        Type::ALL.into_iter().all(|ty| frozen.of(ty).len() >= 8),
        "the frozen set must retain at least eight operands of every type"
    );
    assert_eq!(frozen.count(), 72, "the checked-in frozen input changed");

    let expected_cases: usize = table()
        .iter()
        .map(|operator| {
            operator
                .operands
                .iter()
                .map(|&ty| frozen.of(ty).len())
                .max()
                .unwrap_or(0)
        })
        .sum();
    let report = agree(&frozen);

    assert_eq!(
        report.cases, expected_cases,
        "the whole frozen input was not traversed"
    );
    assert_eq!(report.agreeing, COUNT, "mismatches: {:#?}", report.misses);
    assert!(report.misses.is_empty(), "mismatches: {:#?}", report.misses);
    assert_eq!(
        report.agreeing_bar,
        Bar {
            value: 64,
            sense: Sense::AtLeast
        }
    );
    assert_eq!(report.agreeing_verdict, Judged::Pass);
}
