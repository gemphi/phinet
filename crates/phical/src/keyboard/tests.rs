//! The keyboard's table-and-apply properties: the table is a complete
//! bijection partitioned into families (7), a well-typed application is
//! deterministic and returns the declared result type (8), an ill-typed one
//! errors by name and returns no value (9), and the identity preserves bytes
//! (10).
//!
//! All four run on the workspace's seeded driver [`phistd::gen`] and on no
//! outside crate.

use super::operator::{apply, identity, table, Family, COUNT};
use super::types::draw::arbitrary;
use super::{equal, Arena, Fault, Operand, Type};
use phistd::gen::{for_all, Gen, Seeded, CASES};

/// A seed with no meaning of its own; two Arenas and every property share it.
const SEED: u64 = 0x5EED_0E03;

// Feature: phinet-64-operators-learning-ladder, Property 7: The Operator
// table is complete — 64 rows — its indices are a bijection onto 0..64 with
// each Operator's own index matching its position, and its families are a
// partition laid out in Family::ALL order.
//
// All six families (tasks 3.4–3.9) have landed, so `table()` now holds the
// whole 64 and the completeness claim holds. The bijection and partition over
// the set are also checked unconditionally by
// `the_table_prefix_is_a_bijection_partitioned_by_family` below, and by the
// assertions inside `table()` itself.
//
// Validates: Requirements 6.1, 6.2
#[test]
fn the_operator_table_is_a_complete_bijection_partitioned_by_family() {
    let ops = table();
    assert_eq!(ops.len(), COUNT, "the keyboard declares exactly {COUNT} Operators");

    let mut seen = vec![false; COUNT];
    for op in &ops {
        let i = op.index as usize;
        assert!(i < COUNT, "index {i} is outside 0..{COUNT}");
        assert!(!seen[i], "index {i} is declared twice");
        seen[i] = true;
    }
    assert!(seen.into_iter().all(|hit| hit), "every index 0..{COUNT} is declared once");

    assert_families_cover(&ops);
    assert_names_unique(&ops);
}

// The structural facts that must hold at every stage, prefix or full: each
// Operator sits at its own declared index, the families appear in ALL order
// as contiguous blocks, and the names are distinct.
#[test]
fn the_table_prefix_is_a_bijection_partitioned_by_family() {
    let ops = table();
    assert!(!ops.is_empty(), "the table always holds at least the identity");
    for (at, op) in ops.iter().enumerate() {
        assert_eq!(op.index as usize, at, "{:?} is out of index order", op.name);
        assert_eq!(op.operands.len(), op.arity as usize, "{:?} arity/operands", op.name);
    }
    assert_families_cover(&ops);
    assert_names_unique(&ops);
}

/// Every family present appears as one contiguous block, in [`Family::ALL`]
/// order — the partition laid out in index order.
fn assert_families_cover(ops: &[super::Operator]) {
    let mut reached = 0usize;
    for op in ops {
        let rank = Family::ALL.iter().position(|&f| f == op.family).unwrap();
        assert!(rank >= reached, "{:?} breaks the family partition order", op.name);
        reached = rank;
    }
}

fn assert_names_unique(ops: &[super::Operator]) {
    let mut names: Vec<&str> = ops.iter().map(|op| op.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), ops.len(), "two Operators share a name");
}

/// One case of Property 8: an Operator drawn from the table, and one operand
/// of each of its declared types, so the application is always well typed.
#[derive(Debug)]
struct WellTyped {
    index: usize,
    args: Vec<Operand>,
}

impl Gen for WellTyped {
    fn draw(g: &mut Seeded) -> Self {
        let ops = table();
        let index = g.below(ops.len() as u64) as usize;
        let args = ops[index].operands.iter().map(|&ty| arbitrary(g, ty)).collect();
        WellTyped { index, args }
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 8: A well-typed
// application is deterministic — the same Operator on the same operands in a
// fresh Arena returns the same result — and the result's type is the one the
// Operator declares.
//
// Validates: Requirements 6.4, 6.7
#[test]
fn a_well_typed_application_is_deterministic_and_returns_the_declared_type() {
    for_all::<WellTyped>(SEED, CASES, |case| {
        let ops = table();
        let op = &ops[case.index];

        let mut a = Arena::new(SEED);
        let mut b = Arena::new(SEED);
        let first = apply(op, &case.args, &mut a);
        let second = apply(op, &case.args, &mut b);

        match (first, second) {
            // Determinism is judged under the equality rule (design Property
            // 8), not derived `==`: the rule makes a phasor equal to itself
            // bit for bit, so a NaN-component result an awkward operand drives
            // a circle statistic to is equal to the identical NaN the second
            // run produces, where derived `f64` equality would call them
            // unequal and mislabel a deterministic Operator.
            (Ok(x), Ok(y)) => equal(&x, &y) && x.kind() == op.result,
            // A fault is allowed (an out-of-range handle, say), but it must be
            // the same fault both times — determinism holds for errors too.
            (Err(x), Err(y)) => x == y,
            _ => false,
        }
    })
    .unwrap();
}

/// One case of Property 9: a well-typed call made ill-typed one of two ways —
/// the argument count changed to a wrong length in 0 to 5, or one position's
/// type replaced by each of the other eight types in turn.
#[derive(Debug)]
enum IllTyped {
    WrongArity { index: usize, len: usize },
    WrongType { index: usize, position: usize, swapped: Type },
}

impl Gen for IllTyped {
    fn draw(g: &mut Seeded) -> Self {
        let ops = table();
        let index = g.below(ops.len() as u64) as usize;
        let op = &ops[index];
        // Half the cases attack the arity, half a single operand's type; an
        // arity-0 Operator has no position to swap, so it always takes arity.
        if op.arity == 0 || g.below(2) == 0 {
            // A wrong length in 0..=5, avoiding the one correct length.
            let mut len = g.below(6) as usize;
            if len == op.arity as usize {
                len = (len + 1) % 6;
            }
            IllTyped::WrongArity { index, len }
        } else {
            let position = g.below(op.arity as u64) as usize;
            let declared = op.operands[position];
            // One of the other eight types, never the declared one.
            let others: Vec<Type> = Type::ALL.into_iter().filter(|&t| t != declared).collect();
            IllTyped::WrongType { index, position, swapped: *g.pick(&others) }
        }
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 9: An ill-typed
// application returns an error that names the mismatch — the Operator name,
// the count or position, the declared arity or type and the one given — and
// carries no result value. The arity attack sweeps wrong lengths 0 to 5; the
// type attack replaces one position's type with each of the other eight.
//
// Validates: Requirements 6.8, 6.9
#[test]
fn an_ill_typed_application_errors_by_name_and_returns_no_value() {
    for_all::<IllTyped>(SEED, CASES, |case| {
        let ops = table();
        let mut arena = Arena::new(SEED);
        match case {
            IllTyped::WrongArity { index, len } => {
                let op = &ops[*index];
                // Fill the wrong-length slice with values of the first
                // declared type (or any type when arity 0), so the arity check
                // is what fires, not a type check behind it.
                let filler = op.operands.first().copied().unwrap_or(Type::Word);
                let args: Vec<Operand> =
                    (0..*len).map(|_| filler_value(filler)).collect();
                matches!(
                    apply(op, &args, &mut arena),
                    Err(Fault::Arity { operator, declared, given })
                        if operator == op.name && declared == op.arity && given == *len
                )
            }
            IllTyped::WrongType { index, position, swapped } => {
                let op = &ops[*index];
                // A well-typed argument list with exactly one position swapped
                // to the wrong type, so the type check at that position fires
                // and names it.
                let mut args: Vec<Operand> =
                    op.operands.iter().map(|&ty| filler_value(ty)).collect();
                args[*position] = filler_value(*swapped);
                matches!(
                    apply(op, &args, &mut arena),
                    Err(Fault::OperandType { operator, position: at, declared, given })
                        if operator == op.name
                            && at == *position
                            && declared == op.operands[*position]
                            && given == *swapped
                )
            }
        }
    })
    .unwrap();
}

/// A fixed, well-typed value of `ty` for the arity and type attacks, where the
/// value's contents do not matter — only its type does.
fn filler_value(ty: Type) -> Operand {
    ty.zero()
}

/// One case of Property 10: an operand of the identity's declared type.
#[derive(Debug)]
struct IdOperand(Operand);

impl Gen for IdOperand {
    fn draw(g: &mut Seeded) -> Self {
        IdOperand(arbitrary(g, identity().operands[0]))
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 10: The identity
// Operator preserves bytes exactly — its result encodes to the very bytes its
// operand did, for an operand of the identity's declared type.
//
// Validates: Requirements 6.18
#[test]
fn the_identity_operator_preserves_bytes_exactly() {
    let id = identity();
    for_all::<IdOperand>(SEED, CASES, |case| {
        let mut arena = Arena::new(SEED);
        match apply(&id, &[case.0], &mut arena) {
            Ok(result) => result.encode() == case.0.encode() && result == case.0,
            Err(_) => false,
        }
    })
    .unwrap();
}
