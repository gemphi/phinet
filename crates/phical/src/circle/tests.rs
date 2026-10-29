use super::*;
use crate::c64;
use std::f64::consts::{PI, TAU};

#[test]
fn delta_takes_the_short_way_round() {
    assert!((delta(0.1, 6.2) - (6.2 - 0.1 - TAU)).abs() < 1e-15);
    assert!((delta(6.2, 0.1) - (0.1 - 6.2 + TAU)).abs() < 1e-15);
    assert_eq!(delta(1.0, 2.0), 1.0);
    assert_eq!(delta(2.0, 1.0), -1.0);
}

#[test]
fn distance_is_symmetric_and_at_most_a_half_turn() {
    for (a, b) in [(0.1, 6.2), (3.0, 0.0), (1.0, 4.5), (0.0, PI)] {
        assert_eq!(distance(a, b), distance(b, a));
        assert!(distance(a, b) <= PI);
        assert!((distance(a, b) - delta(a, b).abs()).abs() < 1e-12);
    }
}

#[test]
fn step_toward_crosses_the_seam_the_short_way() {
    // From 6.2 to 0.1 is +0.183 the short way, not −6.1.
    assert!((delta_euclid(6.2, 0.1) - delta(6.2, 0.1)).abs() < 1e-12);
    let t = step_toward(6.2, 0.1, 0.5);
    assert!((0.0..TAU).contains(&t));
    assert!(distance(t, 6.2 + 0.5 * delta(6.2, 0.1)) < 1e-12, "got {t}");
    assert_eq!(analogy(1.0, 2.0, 3.0), 4.0);
}

#[test]
fn resultant_reads_one_direction_three_ways() {
    let mut r = Resultant::new();
    r.add(2.0, -0.4);
    r.add_unit(-0.6);
    let d = r.direction();
    assert!(d < 0.0 && d > -PI);
    assert!((r.direction_euclid() - (d + TAU)).abs() < 1e-12);
    assert!((r.direction_lifted() - (d + TAU)).abs() < 1e-12);
    assert!((r.direction_or_none().unwrap() - d).abs() < 1e-15);

    // Two opposite unit vectors cancel: no direction, full dispersion.
    let mut c = Resultant::new();
    c.add_unit(0.0);
    c.add_unit(PI);
    assert!(c.direction_or_none().is_none());
    assert!(c.vanishes());
    assert!((c.dispersion(2) - 1.0).abs() < 1e-12);
    assert_eq!(Resultant::new().dispersion(0), 1.0);
    assert_eq!(Resultant::new().unit(), (0.0, 0.0));
    let (ux, uy) = r.unit();
    assert!((ux.hypot(uy) - 1.0).abs() < 1e-12);
}

#[test]
fn mean_wraps_across_zero() {
    // 0.1 and 2π − 0.1 are neighbours; their mean is ~0, not ~π.
    let m = mean(&[0.1, TAU - 0.1]);
    assert!(!(0.05..=TAU - 0.05).contains(&m), "got {m}");
}

#[test]
fn agreement_finds_the_same_step() {
    let a = [0.5, 1.0, 2.0];
    let b = [0.5 + PI, 1.0 + PI, 2.0 + PI];
    assert!((agreement(&a, &a) - 1.0).abs() < 1e-12);
    assert!((agreement(&a, &b) + 1.0).abs() < 1e-12);
    assert_eq!(nearest(&a, &[b.to_vec(), a.to_vec()]).map(|(i, _)| i), Some(1));
    assert_eq!(nearest::<Vec<f64>>(&a, &[]), None);
    assert!((mean_nearest(&[a.to_vec()], &[b.to_vec(), a.to_vec()]) - 1.0).abs() < 1e-12);
    assert!((shape_similarity(&[0.1, -0.2], &[-0.2, 0.1]) - 1.0).abs() < 1e-12);
    assert_eq!(shape_similarity(&[], &[0.1]), 0.0);
}

#[test]
fn kmeans_separates_two_planted_steps() {
    // Five pairs a little around one step, five around its opposite.
    let mut points: Vec<Vec<f64>> = Vec::new();
    for i in 0..10 {
        let jitter = 0.01 * i as f64;
        let base = match i % 2 {
            0 => 0.3,
            _ => 0.3 + PI,
        };
        points.push(vec![base + jitter, base - jitter]);
    }
    let found = kmeans(&points, 2, 10, 2);
    assert_eq!(found.len(), 2);
    for c in &found {
        assert_eq!(c.members.len(), 5);
        let parity = c.members[0] % 2;
        assert!(c.members.iter().all(|i| i % 2 == parity));
        assert!(c.coherence > 0.99, "coherence {}", c.coherence);
        assert!((mean_agreement(&[c.centroid.clone()], &c.centroid) - 1.0).abs() < 1e-12);
    }
    assert!(kmeans(&points, 11, 10, 2).is_empty());
    assert!(kmeans(&points, 0, 10, 2).is_empty());
    let one = centroid(&points[..1], 2);
    assert!(one.iter().zip(&points[0]).all(|(a, b)| (a - b).abs() < 1e-12));
}

#[test]
fn centring_takes_the_mean_out() {
    assert_eq!(centred(1.0, 1.0), 0.0);
    assert_eq!(reduce(-0.5), TAU - 0.5);
    let d = double_centred(1.0, 0.25, 0.5, 2.0);
    assert!((d - 2.25).abs() < 1e-12);
    assert!((0.0..TAU).contains(&double_centred(0.0, 3.0, 3.0, 0.0)));
}

#[test]
fn the_field_votes_without_the_word() {
    let state = [c64::new(2.0, 0.0), c64::new(0.0, 4.0)];
    let own = [c64::new(1.0, 0.0), c64::new(0.0, 0.0)];
    let mut total = [c64::new(0.0, 0.0); 2];
    add_scaled(&mut total, &state, 0.5);
    assert_eq!(total, [c64::new(1.0, 0.0), c64::new(0.0, 2.0)]);

    // A sentence that is only the word says nothing about it.
    let loo = leave_one_out(&state, &own, 0.5);
    assert!(is_silent(&loo[..1]));
    assert!(!is_silent(&loo));
    let mut unit = [c64::new(0.0, 0.0); 2];
    add_directions(&mut unit, &loo);
    assert_eq!(unit, [c64::new(0.0, 0.0), c64::new(0.0, 1.0)]);

    // Four contexts: the floor is C/2, unless the concentration is higher.
    assert_eq!(rayleigh_floor(0.05, 1.3, 4.0), 0.65);
    assert_eq!(rayleigh_floor(0.9, 1.3, 4.0), 0.9);
    assert!(scattered(c64::new(1.0, 0.0), 4.0, 0.65));
    assert!(!scattered(c64::new(4.0, 0.0), 4.0, 0.65));

    // Contexts that are the corpus average leave nothing to move toward.
    let avg = c64::new(0.0, 2.0);
    assert_eq!(vote_target(avg, avg, c64::new(0.0, 0.0), 1.0, 1.0, true, 0.1), None);
    let up = vote_target(avg, avg, c64::new(0.0, 0.0), 1.0, 1.0, false, 0.1).unwrap();
    assert!((up - PI / 2.0).abs() < 1e-12);
}
