#[allow(unused_imports)]
use super::*;

/// A counter of 1.8 million and a quantum number of 13 are the same point.
#[test]
fn counters_do_not_walk_the_wheel() {
    let a = ColorSpaceTimePoint::from_phase(1.0, 13);
    let b = ColorSpaceTimePoint::from_phase(1.0, 1_800_000);
    assert_eq!(a.band_n, b.band_n);
    assert!((a.effective_phase - b.effective_phase).abs() < 1e-12);
    assert!((Physics::oscillation_energy(1.0, 13) - Physics::oscillation_energy(1.0, 1_800_000)).abs() < 1e-40);
    assert!(Physics::color_space_time_distance(1.0, 13, 1.0, 1_800_000).abs() < 1e-9);
}
