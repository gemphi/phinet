//! The four new Relate Operators, indices 60 to 63.
//!
//! None reads a trained [`Joint`](crate::Joint) and none invents arithmetic:
//! each is built on one of the twelve lifted in [`super`]. `law_holds` and
//! `confident` are `find_laws` read for presence and for confidence;
//! `symmetric` is `find_inverses` read for a self-inverse; `bar` is the gap a
//! perfect top rank leaves over `chance`.
//!
//! The rows, in index order: 60 law_holds, 61 symmetric, 62 confident, 63
//! bar.

use crate::relate::{self, Triple};

use super::super::super::arena::{Arena, Fault};
use super::super::super::operator::Operator;
use super::super::super::types::{Operand, Type};
use super::{count, kind, op, scalar, triple, truth, MIN_SUPPORT, TRIPLE};

/// The four new Relate Operators in index order, 60 to 63.
pub(super) fn table() -> Vec<Operator> {
    vec![
        op(60, "law_holds", &[Type::Kind, Type::Kind], law_holds),
        op(61, "symmetric", &[Type::Kind], symmetric),
        op(62, "confident", TRIPLE, confident),
        op(63, "bar", &[Type::Class], bar),
    ]
}

/// New: does a law `k₁ ⇒ k₂` hold at all over the one told triple? Built on
/// `find_laws` — a law holds when the function returns one for the pair.
fn law_holds(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let t = Triple { about: 0, kind: kind(&a[0]), other: 1 };
    let laws = relate::find_laws(&[t], MIN_SUPPORT);
    Ok(truth(laws.iter().any(|l| l.k2 == kind(&a[1]))))
}

/// New: is the kind its own inverse (`unlike`, both ways)? Built on
/// `find_inverses` — a kind is symmetric when it inverts to itself.
fn symmetric(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let k = kind(&a[0]);
    let inv = relate::find_inverses(&[Triple { about: 0, kind: k, other: 1 }], MIN_SUPPORT);
    Ok(truth(inv.iter().any(|i| i.k == i.k_inv)))
}

/// New: is the triple's law confident — its confidence at least a half? Built
/// on `find_laws`' confidence.
fn confident(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let laws = relate::find_laws(&[triple(a)], MIN_SUPPORT);
    Ok(truth(laws.first().is_some_and(|l| l.confidence() >= 0.5)))
}

/// New: the gap a reading has to beat — a perfect top rank (1.0) less the
/// random `chance` over `n`. Built on `chance`.
fn bar(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(1.0 - relate::chance(count(&a[0]))))
}
