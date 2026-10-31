//! Unit tests for the Classes family: the table is the declared twelve rows
//! at indices 28 to 39, each lifted Operator agrees with the
//! `phical::classes` function it wraps, the three split-table rows compose,
//! and the Mode rows are one code cut at 4, 5 and 6 bits.

use super::*;
use crate::classes as base;

use super::super::super::arena::Arena;
use super::super::super::operator::{apply, Family};

/// A count on a phasor's real axis, the shape the split-table rows read.
fn n(v: f64) -> Operand {
    Operand::Phasor(c64::new(v, 0.0))
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

/// One application of the named row, the Arena unused by this family.
fn run(name: &str, args: &[Operand]) -> Operand {
    let mut arena = Arena::new(1);
    apply(&row(name), args, &mut arena).expect("a well-typed application")
}

#[test]
fn the_classes_table_is_twelve_rows_at_indices_twenty_eight_to_thirty_nine() {
    let t = table();
    assert_eq!(t.len(), 12, "Classes declares twelve Operators");
    for (at, r) in t.iter().enumerate() {
        assert_eq!(r.index as usize, 28 + at, "{:?} is out of index order", r.name);
        assert_eq!(r.family, Family::Classes, "{:?} is not in Classes", r.name);
        assert_eq!(r.operands.len(), r.arity as usize, "{:?} arity", r.name);
        assert!((1..=3).contains(&r.arity), "{:?} arity is 1, 2 or 3", r.name);
        assert!(!r.idempotent, "{:?} declares no idempotence", r.name);
    }
    assert_eq!(t[0].name, "likelihood", "likelihood opens the family at index 28");
    assert_eq!(t[11].name, "size", "size closes it at index 39");
}

#[test]
fn likelihood_agrees_with_the_classes_function_over_the_split_table() {
    for (a, b) in [(6.0, 4.0), (1.0, 1.0), (10.0, 0.0), (0.0, 0.0)] {
        let (pairs, left, right) = (vec![a, 0.0, 0.0, b], vec![a, b], vec![a, b]);
        let want = base::likelihood(&pairs, &left, &right);
        assert_eq!(re_of(run("likelihood", &[n(a), n(b)])), want, "likelihood of {a}, {b}");
    }
}

#[test]
fn the_move_gains_what_apply_move_leaves_behind() {
    // The three split-table rows compose: the likelihood after the move is
    // the likelihood before it plus the move's gain, which is the identity
    // `classes`' own test holds `move_gain` and `apply_move` to.
    for (a, b, half) in [(6.0, 4.0, 0u16), (6.0, 4.0, 1), (12.0, 1.0, 1), (3.0, 3.0, 0)] {
        let before = re_of(run("likelihood", &[n(a), n(b)]));
        let gain = re_of(run("move_gain", &[n(a), n(b), Operand::Class(half)]));
        let after = re_of(run("apply_move", &[n(a), n(b), Operand::Class(half)]));
        assert!((after - (before + gain)).abs() < 1e-9, "{a}, {b}, half {half}: {after} vs {}", before + gain);
        // Merging the two halves can only lose likelihood.
        assert!(gain <= 0.0, "a merge does not gain: {gain}");
    }
}

#[test]
fn split_cost_agrees_with_the_classes_function() {
    for (classes, pairs) in [(1usize, 100.0), (4, 1e4), (0, 1e4), (53, 2.0)] {
        let got = re_of(run("split_cost", &[Operand::Class(classes as u16), n(pairs)]));
        assert_eq!(got, base::split_cost(classes, pairs), "split_cost {classes}, {pairs}");
    }
}

#[test]
fn halve_grows_the_code_to_the_level_it_is_given() {
    for (units, level) in [(8u16, 4u8), (20, 5), (31, 6), (0, 6), (3, 0)] {
        let got = class_of(run("halve", &[Operand::Class(units), Operand::Level(level)]));
        let told: Vec<usize> = (0..units as usize).collect();
        let want = base::halve(units as usize, &[told], Grow::Fixed(level as u32));
        assert_eq!(got as usize, want.classes, "halve over {units} units at level {level}");
        assert!(got <= units, "a code names no more classes than it has units");
    }
}

#[test]
fn the_modes_are_one_code_cut_at_four_five_and_six() {
    // Property 22's shift identity, here as the Operators: the Mode-16 class
    // is the Mode-32 class shifted right one bit, and the Mode-32 class is
    // the Mode-64 class shifted right one bit.
    for c in 0..64u16 {
        let at = |bits: u8| class_of(run("mode", &[Operand::Class(c), Operand::Level(bits)]));
        assert_eq!(at(6), c, "Mode 64 is the six-bit code itself");
        assert_eq!(at(5), at(6) >> 1, "Mode 32 is Mode 64 coarsened");
        assert_eq!(at(4), at(5) >> 1, "Mode 16 is Mode 32 coarsened");
        // `cut` is the same shift read off a code at a named depth.
        let cut = class_of(run("cut", &[Operand::Class(c), Operand::Level(6), Operand::Level(4)]));
        assert_eq!(cut, at(4), "cut at 6 bits down to 4 is the Mode-16 class");
    }

    // Each Mode's class count, and the containment that count bounds.
    for (bits, size) in [(4u8, 16u16), (5, 32), (6, 64)] {
        assert_eq!(class_of(run("size", &[Operand::Level(bits)])), size);
        for c in 0..64u16 {
            let coarse = class_of(run("mode", &[Operand::Class(c), Operand::Level(bits)]));
            assert!(coarse < size, "a Mode-{size} class is below {size}");
            let args = [Operand::Class(c), Operand::Class(coarse), Operand::Level(bits)];
            assert_eq!(re_of(run("inside", &args)), 1.0, "code {c} is inside its own Mode class");
            let other = [Operand::Class(c), Operand::Class(coarse + size), Operand::Level(bits)];
            assert_eq!(re_of(run("inside", &other)), 1.0, "a coarse class is read modulo its width");
            let away = [Operand::Class(c), Operand::Class((coarse + 1) % size), Operand::Level(bits)];
            assert_eq!(re_of(run("inside", &away)), 0.0, "code {c} is in one Mode class only");
        }
    }
}

#[test]
fn sibling_is_its_own_inverse_and_parent_steps_one_depth_back() {
    assert_eq!(row("sibling").inverse, Inverse::Exact, "sibling declares an exact inverse");
    for c in [0u16, 1, 2, 7, 4096, u16::MAX] {
        let once = class_of(run("sibling", &[Operand::Class(c)]));
        assert_ne!(once, c, "a sibling is the other half");
        assert_eq!(class_of(run("sibling", &[Operand::Class(once)])), c, "and its own inverse");
        assert_eq!(class_of(run("parent", &[Operand::Class(c)])), c >> 1);
        // The two halves of one split share a parent.
        assert_eq!(
            class_of(run("parent", &[Operand::Class(once)])),
            class_of(run("parent", &[Operand::Class(c)])),
            "siblings share a parent"
        );
    }
}

#[test]
fn pay_keeps_a_split_only_when_the_merge_loses_more_than_a_class_costs() {
    // The rule `classes::pay` settles a depth by: the merge's loss against
    // what one more class costs over the same pairs.
    for (a, b, leaves) in [(60.0, 40.0, 2u16), (60.0, 40.0, 40), (1.0, 1.0, 2), (0.0, 0.0, 2)] {
        let loss = -re_of(run("move_gain", &[n(a), n(b), Operand::Class(1)]));
        let cost = base::split_cost(leaves as usize - 1, a + b);
        let got = re_of(run("pay", &[n(a), n(b), Operand::Class(leaves)]));
        assert_eq!(got, f64::from(u8::from(loss > cost)), "pay {a}, {b} among {leaves} leaves");
    }
    // A split of two well-told halves pays; the same split among many
    // classes, each costing more to describe, does not.
    assert_eq!(re_of(run("pay", &[n(60.0), n(40.0), Operand::Class(2)])), 1.0);
    assert_eq!(re_of(run("pay", &[n(60.0), n(40.0), Operand::Class(40)])), 0.0);
}

#[test]
fn every_class_is_a_code_and_every_level_a_depth_it_can_stand_at() {
    // As with a Q64 sector, no drawn operand can fault or panic: a class is
    // read modulo its depth's width and a level modulo the deepest depth, so
    // the extremes of both are legal.
    let mut arena = Arena::new(1);
    for r in table() {
        let args: Vec<Operand> = r
            .operands
            .iter()
            .map(|&ty| match ty {
                Type::Class => Operand::Class(u16::MAX),
                Type::Level => Operand::Level(u8::MAX),
                _ => Operand::Phasor(c64::new(f64::MAX, 0.0)),
            })
            .collect();
        let got = apply(&r, &args, &mut arena).expect("the extremes are legal operands");
        assert_eq!(got.kind(), r.result, "{:?} returns its declared type", r.name);
    }
    assert_eq!(DEPTHS, 13, "a depth is read modulo one past MAX_DEPTH");
}
