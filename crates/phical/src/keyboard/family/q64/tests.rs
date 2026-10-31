//! Unit tests for the Q64 family: the table is the declared eight rows at
//! indices 14 to 21, each lifted Operator agrees with the `phical::q64`
//! function it wraps, and the one approximate inverse's declared tolerance is
//! both within Requirement 6 criterion 17's cap and actually met over a full
//! sweep of the circle.

use super::*;
use crate::c64;
use crate::q64 as base;
use crate::TAU;

use super::super::super::arena::Arena;
use super::super::super::operator::{apply, Family};

/// A unit phasor at `theta`, the shape `quantise` reads its angle off.
fn ph(theta: f64) -> Operand {
    Operand::Phasor(c64::from_polar(1.0, theta))
}

fn class_of(o: Operand) -> u16 {
    match o {
        Operand::Class(c) => c,
        other => panic!("expected a class result, got {other:?}"),
    }
}

fn re_of(o: Operand) -> f64 {
    match o {
        Operand::Phasor(z) => z.re,
        other => panic!("expected a phasor result, got {other:?}"),
    }
}

fn row(name: &str) -> Operator {
    table().into_iter().find(|o| o.name == name).expect("the row is declared")
}

#[test]
fn the_q64_table_is_eight_rows_at_indices_fourteen_to_twenty_one() {
    let t = table();
    assert_eq!(t.len(), 8, "Q64 declares eight Operators");
    for (at, r) in t.iter().enumerate() {
        assert_eq!(r.index as usize, 14 + at, "{:?} is out of index order", r.name);
        assert_eq!(r.family, Family::Q64, "{:?} is not in Q64", r.name);
        assert_eq!(r.operands.len(), r.arity as usize, "{:?} arity", r.name);
        assert!((1..=3).contains(&r.arity), "{:?} arity is 1, 2 or 3", r.name);
    }
    assert_eq!(t[0].name, "quantise", "quantise opens the family at index 14");
    assert_eq!(t[7].name, "sector_nearest", "sector_nearest closes it at index 21");
}

#[test]
fn quantise_and_lift_agree_with_the_q64_functions() {
    let mut arena = Arena::new(1);
    let (quantise, lift) = (row("quantise"), row("lift"));
    for k in 0..200 {
        let theta = TAU * k as f64 / 200.0;
        let got = class_of(apply(&quantise, &[ph(theta)], &mut arena).unwrap());
        // `arg()` wraps to [−π, π]; `quantise` is defined on either reading.
        assert_eq!(got as u8, base::quantise(c64::from_polar(1.0, theta).arg()));

        let back = apply(&lift, &[Operand::Class(got)], &mut arena).unwrap();
        match back {
            Operand::Phasor(z) => {
                // `lift` returns a phase in [0, τ) and `arg()` reads it back
                // in [−π, π], so the two are compared wrapped.
                let off = crate::wrap(z.arg() - base::lift(got as u8)).abs();
                assert!(off < 1e-12, "lift {got} is off by {off}");
                assert!((z.norm() - 1.0).abs() < 1e-12, "a lifted sector is a unit phasor");
            }
            other => panic!("expected a phasor, got {other:?}"),
        }
    }
}

#[test]
fn a_class_operand_is_a_sector_read_modulo_the_sixty_four() {
    let mut arena = Arena::new(1);
    let lift = row("lift");
    // 64 + 3 and 3 name the same sector, so every u16 a class may hold is a
    // legal sector and no drawn operand can fault or panic.
    let a = apply(&lift, &[Operand::Class(3)], &mut arena).unwrap();
    let b = apply(&lift, &[Operand::Class(3 + CHANNELS as u16)], &mut arena).unwrap();
    assert_eq!(a, b);
    assert!(apply(&lift, &[Operand::Class(u16::MAX)], &mut arena).is_ok());
}

#[test]
fn the_declared_tolerance_is_within_the_cap_and_the_round_trip_lands_inside_it() {
    assert!(
        tolerance() <= TOLERANCE_CAP,
        "declared tolerance {} exceeds the cap {TOLERANCE_CAP}",
        tolerance()
    );

    let mut arena = Arena::new(1);
    let (quantise, lift) = (row("quantise"), row("lift"));
    let mut worst = 0.0f64;
    for k in 0..10_000 {
        let theta = TAU * k as f64 / 10_000.0;
        let q = apply(&quantise, &[ph(theta)], &mut arena).unwrap();
        let back = apply(&lift, &[q], &mut arena).unwrap();
        match back {
            // The resonance distance the tolerance is declared under.
            Operand::Phasor(z) => worst = worst.max(1.0 - crate::wrap(z.arg() - theta).cos()),
            other => panic!("expected a phasor, got {other:?}"),
        }
    }
    assert!(worst <= tolerance() + 1e-12, "round trip {worst} > tolerance {}", tolerance());

    // The other direction is exact: a sector lifted and quantised again is
    // the sector it was, since a lifted phase lands in its own sector.
    for s in 0..CHANNELS as u16 {
        let back = apply(&lift, &[Operand::Class(s)], &mut arena).unwrap();
        assert_eq!(class_of(apply(&quantise, &[back], &mut arena).unwrap()), s);
    }
}

#[test]
fn a_word_quantises_to_three_sectors_at_the_symbol_level() {
    let mut arena = Arena::new(1);
    let quantise_word = row("quantise_word");
    let phases = [0.0, 1.0, 2.0];
    let got = apply(
        &quantise_word,
        &[ph(phases[0]), ph(phases[1]), ph(phases[2])],
        &mut arena,
    )
    .unwrap();
    let want = base::quantise_word(&phases);
    match got {
        Operand::Variation(classes, bits) => {
            assert_eq!(classes.to_vec(), want.iter().map(|s| *s as u16).collect::<Vec<_>>());
            assert_eq!(bits, SYMBOL_BITS, "a word of sectors carries level 6");
        }
        other => panic!("expected a variation, got {other:?}"),
    }
}

#[test]
fn hamming_counts_the_differing_channels_and_lift_word_resolves_them() {
    let mut arena = Arena::new(1);
    let (hamming, lift_word) = (row("hamming"), row("lift_word"));
    let a = Operand::Variation([0, 1, 2], SYMBOL_BITS);
    let b = Operand::Variation([0, 9, 2], SYMBOL_BITS);
    let got = apply(&hamming, &[a, b], &mut arena).unwrap();
    assert_eq!(re_of(got), base::hamming(&[0, 1, 2], &[0, 9, 2]) as f64);
    assert_eq!(re_of(apply(&hamming, &[a, a], &mut arena).unwrap()), 0.0);

    // Three equal sectors resolve to a resultant of magnitude 3.
    let flat = Operand::Variation([5, 5, 5], SYMBOL_BITS);
    match apply(&lift_word, &[flat], &mut arena).unwrap() {
        Operand::Phasor(z) => {
            assert!((z.norm() - 3.0).abs() < 1e-12, "three agreeing channels");
            assert!((z.arg() - base::lift(5)).abs() < 1e-12, "at the shared sector");
        }
        other => panic!("expected a phasor, got {other:?}"),
    }
}

#[test]
fn sector_distance_rotate_and_nearest() {
    let mut arena = Arena::new(1);
    let (distance, rotate, nearest) =
        (row("sector_distance"), row("sector_rotate"), row("sector_nearest"));

    let got = apply(&distance, &[Operand::Class(63), Operand::Class(0)], &mut arena).unwrap();
    assert_eq!(re_of(got), base::sector_distance(63, 0) as f64);

    // The rotation wraps the 64 sectors, so its result is a sector.
    let turned = apply(&rotate, &[Operand::Class(63), Operand::Class(2)], &mut arena).unwrap();
    assert_eq!(class_of(turned), 1);
    let noop = apply(&rotate, &[Operand::Class(7), Operand::Class(0)], &mut arena).unwrap();
    assert_eq!(class_of(noop), 7);

    // 2 is nearer to 0 than 40 is; the tie goes to the first candidate.
    let args = [Operand::Class(0), Operand::Class(2), Operand::Class(40)];
    assert_eq!(class_of(apply(&nearest, &args, &mut arena).unwrap()), 2);
    let tie = [Operand::Class(0), Operand::Class(5), Operand::Class(59)];
    assert_eq!(class_of(apply(&nearest, &tie, &mut arena).unwrap()), 5);
}
