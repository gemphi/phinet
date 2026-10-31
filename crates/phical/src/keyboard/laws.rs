//! The declared law tables, read *out of* the Operator declarations rather
//! than kept beside them. Every fact these functions return is already a
//! field of a row in [`table`](super::table): an Operator's `idempotent`
//! flag, its `inverse` case, its arity and its operand and result types. So
//! the laws are checkable against the keyboard rather than a second copy of
//! the keyboard that could drift from it — change a family file and these
//! tables change with it, no edit here (Requirement 6 criteria 10, 11, 12).
//!
//! Three tables, one per law:
//!
//! - [`idempotent_ops`] — the indices whose row declares `idempotent: true`.
//!   Property 11 runs each twice and checks `f(f(x)) == f(x)` under
//!   [`equal`](super::equal).
//! - [`inverse_pairs`] — each Operator declaring an inverse, paired with the
//!   Operator that undoes it, and the [`Inverse`] case (exact, or approximate
//!   with its tolerance). A self-inverse (the identity, `sibling`) pairs with
//!   itself; the one cross pair is q64's `quantise`↔`lift`. Property 12
//!   round-trips each and checks the exact ones to the bit and the
//!   approximate one within its declared tolerance.
//! - [`composition_chains`] — every ordered triple `(f, g, h)` of arity-1
//!   Operators whose types line up for `f ∘ g ∘ h`, enumerated from the type
//!   table by matching each result type to the next operand type. Property 13
//!   checks `(f ∘ g) ∘ h` equals `f ∘ (g ∘ h)`.

use super::operator::{table, Inverse, Operator};
use super::types::Type;

/// The indices of every Operator whose row declares `idempotent: true`,
/// read straight from [`table`]. Today that is the identity (index 0) and
/// `reduce` (index 1), both in the Circle family — but the list is whatever
/// the declarations say, so a family file flipping a flag moves this with no
/// edit here.
pub fn idempotent_ops() -> Vec<u8> {
    table().iter().filter(|op| op.idempotent).map(|op| op.index).collect()
}

/// Each Operator declaring an inverse, paired with the Operator that undoes
/// it and the [`Inverse`] case it declares. The pairing is derived, not
/// listed:
///
/// - an **arity-1 Operator whose result type equals its operand type** is its
///   own inverse (applying it, then it again, returns the start) — the
///   identity (`word → word`) and `sibling` (`class → class`), both
///   [`Inverse::Exact`];
/// - otherwise its inverse is the Operator in the **same family** whose
///   operand and result types are the mirror of its own and whose inverse
///   case matches — q64's `quantise` (`phasor → class`) and `lift`
///   (`class → phasor`), the one [`Inverse::Approximate`] pair.
///
/// Every Operator declaring an inverse appears once as the first element, so
/// the cross pair appears twice — `(quantise, lift)` and `(lift, quantise)` —
/// which is what lets Property 12 round-trip from either side.
pub fn inverse_pairs() -> Vec<(u8, u8, Inverse)> {
    let ops = table();
    let mut pairs = Vec::new();
    for op in &ops {
        if matches!(op.inverse, Inverse::None) {
            continue;
        }
        if let Some(partner) = partner_of(op, &ops) {
            pairs.push((op.index, partner, op.inverse));
        }
    }
    pairs
}

/// The index of the Operator that undoes `op`, or `None` when the keyboard
/// declares an inverse case but holds no partner (which no current row does —
/// the guard keeps a future declaration honest rather than silently wrong).
fn partner_of(op: &Operator, ops: &[Operator]) -> Option<u8> {
    // A self-inverse: arity 1, result type the operand type. Flipped twice is
    // the start, so the partner is the Operator itself.
    if op.arity == 1 && op.result == op.operands[0] {
        return Some(op.index);
    }
    // A cross inverse: the Operator in the same family whose operand and
    // result mirror this one's, with a matching inverse case. Its own index
    // is skipped so a self-inverse never matches here.
    let mirror = ops.iter().find(|other| {
        other.index != op.index
            && other.family == op.family
            && other.arity == 1
            && op.arity == 1
            && other.operands[0] == op.result
            && other.result == op.operands[0]
            && same_case(other.inverse, op.inverse)
    });
    mirror.map(|m| m.index)
}

/// Two inverse cases are the same case: both exact, both none, or both
/// approximate with the same declared tolerance (the one cross pair's two
/// rows declare the identical tolerance, so they match).
fn same_case(a: Inverse, b: Inverse) -> bool {
    match (a, b) {
        (Inverse::Exact, Inverse::Exact) => true,
        (Inverse::None, Inverse::None) => true,
        (Inverse::Approximate { tolerance: x }, Inverse::Approximate { tolerance: y }) => x == y,
        _ => false,
    }
}

/// Every ordered triple of arity-1 Operators whose types line up for the
/// composition `f ∘ g ∘ h`: applied to a value, `h` runs first, then `g` on
/// its result, then `f` on that. So the chain is well typed exactly when
/// `h`'s result type is `g`'s operand type and `g`'s result type is `f`'s
/// operand type — the two joins enumerated here from the type table, each
/// Operator's `result` matched against the next one's single operand.
///
/// The triple is returned as `(f, g, h)` — outermost first — which is the
/// order Property 13 applies them in and the order it brackets them two ways.
///
/// Only arity-1 Operators take part: a composition joins one Operator's
/// single result to the next's single operand, which an Operator of two or
/// more operands has no single slot for.
pub fn composition_chains() -> Vec<(u8, u8, u8)> {
    let ops = table();
    let unary: Vec<&Operator> = ops.iter().filter(|op| op.arity == 1).collect();
    let mut chains = Vec::new();
    for f in &unary {
        for g in &unary {
            // g's result feeds f's operand.
            if g.result != f.operands[0] {
                continue;
            }
            for h in &unary {
                // h's result feeds g's operand.
                if h.result == g.operands[0] {
                    chains.push((f.index, g.index, h.index));
                }
            }
        }
    }
    chains
}

/// The operand type and result type of the Operator at `index`, for a test
/// that needs a chain's endpoints without re-deriving them.
pub fn endpoints(index: u8) -> (Type, Type) {
    let op = &table()[index as usize];
    (op.operands[0], op.result)
}

#[cfg(test)]
mod tests;
