//! Unit tests for the Relate family: the table is the declared sixteen rows
//! at indices 48 to 63 in order, each lifted Operator agrees with the
//! `phical::relate` function it wraps over the one told triple its operands
//! name, and the four new ones read that lift as the design declares.

use super::*;
use crate::relate;

use super::super::super::arena::{Arena, Handle};
use super::super::super::operator::{apply, Family};

/// A Row operand naming row `i`.
fn r(i: u32) -> Operand {
    Operand::Row(Handle::new(Type::Row, i))
}

/// A Kind operand naming kind `k`.
fn k(k: u16) -> Operand {
    Operand::Kind(k)
}

fn re_of(o: Operand) -> f64 {
    match o {
        Operand::Phasor(z) => z.re,
        other => panic!("expected a phasor result, got {other:?}"),
    }
}

#[test]
fn the_table_is_sixteen_rows_at_forty_eight_to_sixty_three() {
    let ops = table();
    assert_eq!(ops.len(), 16);
    for (at, op) in ops.iter().enumerate() {
        assert_eq!(op.index as usize, 48 + at);
        assert_eq!(op.family, Family::Relate);
        assert_eq!(op.operands.len(), op.arity as usize);
        assert_eq!(op.result, Type::Phasor);
    }
    let names: Vec<&str> = ops.iter().map(|o| o.name).collect();
    assert_eq!(
        names,
        [
            "find_laws", "find_inverses", "implied", "score", "prior", "two_hop", "chance",
            "score_laws", "laws_epoch", "inherit_epoch", "converse", "inherit", "law_holds",
            "symmetric", "confident", "bar"
        ]
    );
}

/// `chance` over the operator agrees with the bare `relate::chance`.
#[test]
fn chance_agrees_with_relate_chance() {
    let mut arena = Arena::new(1);
    let op = &table()[6];
    let got = re_of(apply(op, &[Operand::Class(5)], &mut arena).unwrap());
    assert!((got - relate::chance(5)).abs() < 1e-12);
}

/// `bar` is the gap a perfect top rank leaves over `chance`.
#[test]
fn bar_is_one_minus_chance() {
    let mut arena = Arena::new(1);
    let op = table().into_iter().find(|o| o.name == "bar").unwrap();
    let got = re_of(apply(&op, &[Operand::Class(5)], &mut arena).unwrap());
    assert!((got - (1.0 - relate::chance(5))).abs() < 1e-12);
}

/// `find_laws` over one told `w k v` returns its single law's confidence —
/// one, since the one triple agrees with itself.
#[test]
fn find_laws_reads_the_single_law_confidence() {
    let mut arena = Arena::new(1);
    let op = &table()[0];
    let got = re_of(apply(op, &[r(0), k(3), r(1)], &mut arena).unwrap());
    let laws = relate::find_laws(&[relate::Triple { about: 0, kind: 3, other: 1 }], 1);
    assert_eq!(got, laws.first().map_or(0.0, |l| l.confidence()));
}

/// An ill-typed application is refused before the family computes: a Class
/// where a Row is declared returns an OperandType fault and no value.
#[test]
fn an_ill_typed_triple_is_refused() {
    let mut arena = Arena::new(1);
    let op = &table()[0];
    let got = apply(op, &[Operand::Class(0), k(3), r(1)], &mut arena);
    assert!(got.is_err());
}

