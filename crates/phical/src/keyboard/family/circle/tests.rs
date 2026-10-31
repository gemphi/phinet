//! Unit tests for the Circle family: the table is the declared fourteen
//! rows at indices 0 to 13 with the identity first, and each lifted Operator
//! agrees with the `phical::circle` (or `liskov`) function it wraps.

use super::*;
use crate::c64;
use crate::circle as c;

use super::super::super::arena::Arena;
use super::super::super::operator::{apply, Family};

/// A unit phasor at `theta`, the shape the lifts read their angle off.
fn ph(theta: f64) -> Operand {
    Operand::Phasor(c64::from_polar(1.0, theta))
}

fn arg_of(o: Operand) -> f64 {
    match o {
        Operand::Phasor(z) => z.arg(),
        _ => panic!("expected a phasor result"),
    }
}

fn re_of(o: Operand) -> f64 {
    match o {
        Operand::Phasor(z) => z.re,
        _ => panic!("expected a phasor result"),
    }
}

#[test]
fn the_circle_table_is_fourteen_rows_at_indices_zero_to_thirteen() {
    let t = table();
    assert_eq!(t.len(), 14, "Circle declares fourteen Operators");
    for (at, row) in t.iter().enumerate() {
        assert_eq!(row.index as usize, at, "{:?} is out of index order", row.name);
        assert_eq!(row.family, Family::Circle, "{:?} is not in Circle", row.name);
        assert_eq!(row.operands.len(), row.arity as usize, "{:?} arity", row.name);
    }
    assert_eq!(t[0].name, "identity", "the identity leads the family");
    assert_eq!(t[0].index, 0);
    assert_eq!(t[13].name, "twins", "twins closes the family at index 13");
}

#[test]
fn an_angle_lift_agrees_with_the_circle_function() {
    let mut arena = Arena::new(1);
    let t = table();
    let delta = t.iter().find(|o| o.name == "delta").unwrap();
    let got = apply(delta, &[ph(0.3), ph(1.1)], &mut arena).unwrap();
    // delta returns an angle, repacked as a unit phasor at that angle.
    let want = c::delta(0.3, 1.1);
    assert!((arg_of(got) - want).abs() < 1e-12, "delta {} vs {want}", arg_of(got));
}

#[test]
fn a_scalar_lift_carries_the_value_on_the_real_axis() {
    let mut arena = Arena::new(1);
    let t = table();
    let distance = t.iter().find(|o| o.name == "distance").unwrap();
    let got = apply(distance, &[ph(0.2), ph(2.0)], &mut arena).unwrap();
    assert!((re_of(got) - c::distance(0.2, 2.0)).abs() < 1e-12);
}

#[test]
fn identity_preserves_its_operand() {
    let mut arena = Arena::new(1);
    let id = identity();
    let w = Operand::handle(Type::Word, 7);
    assert_eq!(apply(&id, &[w], &mut arena).unwrap(), w);
}

#[test]
fn twins_is_a_non_negative_squared_amplitude() {
    let mut arena = Arena::new(1);
    let t = table();
    let twins = t.iter().find(|o| o.name == "twins").unwrap();
    let got = apply(twins, &[ph(0.0), ph(0.5)], &mut arena).unwrap();
    assert!(re_of(got) >= 0.0, "a squared magnitude is non-negative");
}
