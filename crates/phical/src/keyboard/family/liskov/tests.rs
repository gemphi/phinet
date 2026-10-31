//! Unit tests for the Liskov family: the table is the declared eight rows at
//! indices 40 to 47, each lifted Operator agrees with the `phical::liskov`
//! function it wraps, and the two new rows `stage` and `measure` read back the
//! fields the lifted staged pair compares.

use super::*;
use crate::c64;
use crate::liskov as base;

use super::super::super::arena::Arena;
use super::super::super::operator::{apply, Family};

/// A one-channel Function carried as a phasor operand: the phasor itself.
fn ph(re: f64, im: f64) -> Operand {
    Operand::Phasor(c64::new(re, im))
}

/// A Staged carried as a phasor: measure on the real axis, stage on the
/// imaginary axis.
fn staged_op(stage: u64, measure: f64) -> Operand {
    Operand::Phasor(c64::new(measure, stage as f64))
}

fn re_of(o: Operand) -> f64 {
    match o {
        Operand::Phasor(z) => z.re,
        other => panic!("expected a phasor result, got {other:?}"),
    }
}

fn class_of(o: Operand) -> u16 {
    match o {
        Operand::Class(c) => c,
        other => panic!("expected a class result, got {other:?}"),
    }
}

fn row(name: &str) -> Operator {
    table().into_iter().find(|o| o.name == name).expect("the row is declared")
}

#[test]
fn the_liskov_table_is_eight_rows_at_indices_forty_to_forty_seven() {
    let t = table();
    assert_eq!(t.len(), 8, "Liskov declares eight Operators");
    for (at, r) in t.iter().enumerate() {
        assert_eq!(r.index as usize, 40 + at, "{:?} is out of index order", r.name);
        assert_eq!(r.family, Family::Liskov, "{:?} is not in Liskov", r.name);
        assert_eq!(r.operands.len(), r.arity as usize, "{:?} arity", r.name);
        assert!((1..=3).contains(&r.arity), "{:?} arity is 1, 2 or 3", r.name);
    }
    assert_eq!(t[0].name, "better", "better opens the family at index 40");
    assert_eq!(t[7].name, "measure", "measure closes it at index 47");
}

#[test]
fn better_agrees_with_liskov_and_is_antisymmetric() {
    let mut arena = Arena::new(1);
    let better = row("better");
    let (p, a, b) = (ph(1.0, 0.0), ph(0.9, 0.1), ph(0.2, 0.8));
    let got = re_of(apply(&better, &[p, a, b], &mut arena).unwrap());
    let want = base::better(&[c64::new(1.0, 0.0)], &[c64::new(0.9, 0.1)], &[c64::new(0.2, 0.8)]);
    assert_eq!(got, want);
    // Antisymmetric: better(p, a, b) == -better(p, b, a).
    let flip = re_of(apply(&better, &[p, b, a], &mut arena).unwrap());
    assert!((got + flip).abs() < 1e-12, "better is antisymmetric");
}

#[test]
fn loss_is_zero_once_the_target_beats_the_decoy_by_the_margin() {
    let mut arena = Arena::new(1);
    let loss = row("loss");
    let (p, decoy) = (ph(1.0, 0.0), ph(-1.0, 0.0));
    // target == pred, decoy opposite: better is large, so the hinge is zero.
    let got = re_of(apply(&loss, &[p, p, decoy], &mut arena).unwrap());
    let want = base::loss(&[c64::new(1.0, 0.0)], &[c64::new(1.0, 0.0)], &[c64::new(-1.0, 0.0)], MARGIN);
    assert_eq!(got, want);
    assert_eq!(got, 0.0, "a clear win past the margin has zero loss");
}

#[test]
fn worse_reports_a_refutation_and_forcing_choice_picks_the_nearer() {
    let mut arena = Arena::new(1);
    let (worse, forcing) = (row("worse"), row("forcing_choice"));

    // worse agrees with the function over the one-continuation pool [a].
    let (p, mv, after) = (ph(1.0, 0.0), ph(0.9, 0.1), ph(0.5, 0.5));
    let got = re_of(apply(&worse, &[p, mv, after], &mut arena).unwrap());
    let f = |o: Operand| match o {
        Operand::Phasor(z) => vec![z],
        _ => unreachable!(),
    };
    let want = base::worse(&f(p), &f(mv), &f(after), &[f(mv)]);
    assert_eq!(got, want);

    // forcing_choice: the first candidate equals pred, so it wins (index 0).
    let near = p;
    let far = ph(-1.0, 0.0);
    assert_eq!(re_of(apply(&forcing, &[p, near, far], &mut arena).unwrap()), 0.0);
    // Swap them: the nearer is now the second candidate (index 1).
    assert_eq!(re_of(apply(&forcing, &[p, far, near], &mut arena).unwrap()), 1.0);
}

#[test]
fn staged_better_and_worse_read_the_lexicographic_order() {
    let mut arena = Arena::new(1);
    let (sb, sw) = (row("staged_better"), row("staged_worse"));

    // A higher stage is better whatever the measure does.
    assert_eq!(re_of(apply(&sb, &[staged_op(1, 0.0), staged_op(2, 99.0)], &mut arena).unwrap()), 1.0);
    // Same stage, smaller measure is better.
    assert_eq!(re_of(apply(&sb, &[staged_op(1, 5.0), staged_op(1, 4.0)], &mut arena).unwrap()), 1.0);
    // A lower stage is worse.
    assert_eq!(re_of(apply(&sw, &[staged_op(2, 0.0), staged_op(1, 0.0)], &mut arena).unwrap()), 1.0);
    // Neither better nor worse when the pair is equal.
    let eq = [staged_op(1, 4.0), staged_op(1, 4.0)];
    assert_eq!(re_of(apply(&sb, &eq, &mut arena).unwrap()), 0.0);
    assert_eq!(re_of(apply(&sw, &eq, &mut arena).unwrap()), 0.0);

    // Each is the other read backwards: better(p, q) == worse(q, p).
    let (p, q) = (staged_op(1, 1.0), staged_op(2, 3.0));
    let forward = re_of(apply(&sb, &[p, q], &mut arena).unwrap());
    let backward = re_of(apply(&sw, &[q, p], &mut arena).unwrap());
    assert_eq!(forward, backward);
}

#[test]
fn stage_and_measure_read_back_the_two_fields() {
    let mut arena = Arena::new(1);
    let (stage, measure) = (row("stage"), row("measure"));
    let s = staged_op(7, 3.25);
    assert_eq!(class_of(apply(&stage, &[s], &mut arena).unwrap()), 7);
    assert_eq!(re_of(apply(&measure, &[s], &mut arena).unwrap()), 3.25);

    // The fields agree with the Staged the staged comparison reads.
    let read = base::staged::Staged { stage: 7, measure: 3.25 };
    assert_eq!(class_of(apply(&stage, &[s], &mut arena).unwrap()) as u64, read.stage);
    assert_eq!(re_of(apply(&measure, &[s], &mut arena).unwrap()), read.measure);
}
