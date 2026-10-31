//! The three law properties, each run over the table [`super`] derives from
//! the Operator declarations and on the workspace's seeded driver
//! [`phistd::gen`] — no fact is restated here that a family file does not
//! already declare.
//!
//! - Property 11: every `idempotent` Operator applied twice equals it applied
//!   once, under [`equal`].
//! - Property 12: an [`Inverse::Exact`] pair round-trips to the bit, and the
//!   one [`Inverse::Approximate`] pair (q64's `quantise`↔`lift`) lands within
//!   its declared tolerance with phases swept over the full circle.
//! - Property 13: composing three arity-1 Operators is associative —
//!   `(f ∘ g) ∘ h` equals `f ∘ (g ∘ h)`.

use super::{composition_chains, idempotent_ops, inverse_pairs};
use crate::c64;
use crate::keyboard::operator::{apply, table, Inverse};
use crate::keyboard::types::draw::arbitrary;
use crate::keyboard::{equal, Arena, Operand, Type};
use crate::CHANNELS;
use phistd::gen::{for_all, Gen, Seeded, CASES};

/// A seed with no meaning of its own; every property here shares it.
const SEED: u64 = 0x5EED_1A05;

/// Apply one Operator by index, in a fresh Arena, to one operand.
fn run(index: u8, arg: Operand) -> Result<Operand, crate::keyboard::Fault> {
    let ops = table();
    let mut arena = Arena::new(SEED);
    apply(&ops[index as usize], &[arg], &mut arena)
}

// --- Property 11 -----------------------------------------------------------

/// One case of Property 11: an idempotent Operator's index and a start
/// operand of its single operand type. Drawn from [`idempotent_ops`], so the
/// case set is exactly the declared idempotent Operators and no others.
#[derive(Debug)]
struct Idempotent {
    index: u8,
    arg: Operand,
}

impl Gen for Idempotent {
    fn draw(g: &mut Seeded) -> Self {
        let ops = idempotent_ops();
        let index = *g.pick(&ops);
        let operand = table()[index as usize].operands[0];
        Idempotent { index, arg: arbitrary(g, operand) }
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 11: A declared
// idempotent Operator is idempotent — for every Operator whose row declares
// `idempotent: true`, applying it twice equals applying it once, under the
// equality rule. The idempotent set is read from the declarations by
// `idempotent_ops`, so this tests the keyboard rather than a second list.
//
// Validates: Requirements 6.10
#[test]
fn a_declared_idempotent_operator_is_idempotent() {
    assert!(!idempotent_ops().is_empty(), "the keyboard declares at least one idempotent Operator");
    for_all::<Idempotent>(SEED, CASES, |case| {
        let once = run(case.index, case.arg);
        let twice = match &once {
            Ok(y) => run(case.index, *y),
            Err(_) => return true, // an operand the Operator faults on faults the same both ways
        };
        match (once, twice) {
            (Ok(y), Ok(yy)) => equal(&y, &yy),
            _ => false,
        }
    })
    .unwrap();
}

// --- Property 12 -----------------------------------------------------------

/// One case of Property 12: an inverse pair `(op, inv, case)` from
/// [`inverse_pairs`] and a start operand of `op`'s operand type. The phasor
/// starts of the approximate `quantise` direction are swept over the full
/// circle, and the class starts of the approximate `lift` direction are
/// genuine sectors in `0..64`, so the round trip is the one the tolerance is
/// declared over.
#[derive(Debug)]
struct Roundtrip {
    op: u8,
    inv: u8,
    case: Inverse,
    arg: Operand,
}

impl Gen for Roundtrip {
    fn draw(g: &mut Seeded) -> Self {
        let pairs = inverse_pairs();
        let (op, inv, case) = *g.pick(&pairs);
        let operand = table()[op as usize].operands[0];
        let arg = match (case, operand) {
            // Sweep the full circle for the phasor that quantise reads.
            (Inverse::Approximate { .. }, Type::Phasor) => {
                Operand::Phasor(c64::from_polar(1.0, g.phase()))
            }
            // A genuine sector for the class that lift reads, so the symbolic
            // round trip is exact.
            (Inverse::Approximate { .. }, Type::Class) => {
                Operand::Class(g.below(CHANNELS as u64) as u16)
            }
            _ => arbitrary(g, operand),
        };
        Roundtrip { op, inv, case, arg }
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 12: An exact inverse
// round-trips, and an approximate inverse lands within its declared
// tolerance. The pairs are read from the declarations by `inverse_pairs`; an
// exact pair is checked to the bit under the equality rule, and q64's
// approximate `quantise`↔`lift` pair is checked within its declared tolerance
// with phases sweeping the full circle.
//
// Validates: Requirements 6.11, 6.17
#[test]
fn an_inverse_round_trips_within_its_declared_tolerance() {
    assert!(!inverse_pairs().is_empty(), "the keyboard declares at least one inverse");
    for_all::<Roundtrip>(SEED, CASES, |case| {
        let forward = match run(case.op, case.arg) {
            Ok(y) => y,
            Err(_) => return false, // a declared inverse must apply to a legal operand
        };
        let back = match run(case.inv, forward) {
            Ok(z) => z,
            Err(_) => return false,
        };
        match case.case {
            // Exact: the round trip returns the start, byte for byte.
            Inverse::Exact => equal(&back, &case.arg),
            // Approximate: a phasor result is judged by the resonance distance
            // the tolerance is declared in; any other result type round-trips
            // exactly (the sector of the lift direction) and is judged so.
            Inverse::Approximate { tolerance } => match (&case.arg, &back) {
                (Operand::Phasor(x), Operand::Phasor(z)) => {
                    1.0 - (z.arg() - x.arg()).cos() <= tolerance + 1e-12
                }
                _ => equal(&back, &case.arg),
            },
            Inverse::None => false,
        }
    })
    .unwrap();
}

// --- Property 13 -----------------------------------------------------------

/// One case of Property 13: a composition chain `(f, g, h)` from
/// [`composition_chains`] and a start operand of `h`'s operand type — the
/// value the whole chain is applied to, `h` first.
#[derive(Debug)]
struct Chain {
    f: u8,
    g: u8,
    h: u8,
    start: Operand,
}

impl Gen for Chain {
    fn draw(gen: &mut Seeded) -> Self {
        let chains = composition_chains();
        let (f, g, h) = *gen.pick(&chains);
        let start_ty = table()[h as usize].operands[0];
        Chain { f, g, h, start: arbitrary(gen, start_ty) }
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 13: Composition of
// arity-1 Operators is associative — over ordered triples of arity-1
// Operators whose result types match the next operand type (enumerated by
// `composition_chains` from the type table), bracketing the three as
// `(f ∘ g) ∘ h` and as `f ∘ (g ∘ h)` applied to the same start gives equal
// results (or the same fault), judged under the equality rule.
//
// Validates: Requirements 6.12
#[test]
fn composition_of_arity_1_operators_is_associative() {
    assert!(!composition_chains().is_empty(), "the type table admits at least one chain");
    for_all::<Chain>(SEED, CASES, |case| {
        // Left bracketing: form (f ∘ g), then apply it to h(start).
        let left = bracket_left(case.f, case.g, case.h, case.start);
        // Right bracketing: form (g ∘ h), then apply f to its result.
        let right = bracket_right(case.f, case.g, case.h, case.start);
        same(&left, &right)
    })
    .unwrap();
}

/// `(f ∘ g) ∘ h` at `start`: run `h`, then the composite `g` then `f`.
fn bracket_left(f: u8, g: u8, h: u8, start: Operand) -> Result<Operand, crate::keyboard::Fault> {
    let after_h = run(h, start)?;
    let fg = |x: Operand| run(g, x).and_then(|y| run(f, y));
    fg(after_h)
}

/// `f ∘ (g ∘ h)` at `start`: run the composite `h` then `g`, then `f`.
fn bracket_right(f: u8, g: u8, h: u8, start: Operand) -> Result<Operand, crate::keyboard::Fault> {
    let gh = |x: Operand| run(h, x).and_then(|y| run(g, y));
    let after_gh = gh(start)?;
    run(f, after_gh)
}

/// Two round-trip outcomes agree: both faulted with the same fault, or both
/// succeeded with results equal under the equality rule.
fn same(a: &Result<Operand, crate::keyboard::Fault>, b: &Result<Operand, crate::keyboard::Fault>) -> bool {
    match (a, b) {
        (Ok(x), Ok(y)) => equal(x, y),
        (Err(x), Err(y)) => x == y,
        _ => false,
    }
}
