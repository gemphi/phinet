//! The Operator declaration and the one entry point every application goes
//! through. An [`Operator`] is a row of the keyboard's frozen index table:
//! its index, its name, the family it partitions into, its arity, the
//! declared type of each operand and of its result, and the laws it is held
//! to — idempotence, an inverse case, and (for a spectral gate) a gate
//! correspondence.
//!
//! [`apply`] is the one door. It checks the arity and then every operand's
//! type against the declaration *before* the family computes anything, so an
//! ill-typed application returns a [`Fault`] naming exactly what mismatched
//! and carries no result value (Requirement 6 criteria 7, 8, 9).

use super::arena::{Arena, Fault};
use super::types::{Operand, Type};

/// The six families the 64 Operators partition into. Every Operator belongs
/// to exactly one, so the families are a partition of the index space and no
/// Operator is counted twice or left out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Family {
    Circle,
    Q64,
    Helix,
    Classes,
    Liskov,
    Relate,
}

impl Family {
    /// The six families in declaration order, which is also the order
    /// [`table`] lays their index ranges out in.
    pub const ALL: [Family; 6] = [
        Family::Circle,
        Family::Q64,
        Family::Helix,
        Family::Classes,
        Family::Liskov,
        Family::Relate,
    ];

    /// The family's name as the index table and the run record spell it.
    pub const fn name(self) -> &'static str {
        match self {
            Family::Circle => "circle",
            Family::Q64 => "q64",
            Family::Helix => "helix",
            Family::Classes => "classes",
            Family::Liskov => "liskov",
            Family::Relate => "relate",
        }
    }
}

/// An Operator's inverse case. An exact inverse round-trips to the bit; an
/// approximate one lands within a declared tolerance (q64's quantise–lift,
/// whose π/64 bound is at or below 0.0099); and most Operators declare none.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Inverse {
    Exact,
    Approximate { tolerance: f64 },
    None,
}

/// The spectral-gate correspondence a few Operators carry: the gate's
/// six-line index, and — once measured — its next-word top-1 accuracy beside
/// the most-frequent baseline. Left `None` until task 3.13 measures it, which
/// is why the fields are optional rather than planted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gate {
    pub lines: u8,
    pub measured_top1: Option<f64>,
    pub baseline_top1: Option<f64>,
}

/// One row of the keyboard's frozen index table. The `compute` field is the
/// family's own closure, reached only after [`apply`] has checked the arity
/// and every operand type, so a family never sees an ill-shaped call.
#[derive(Clone)]
pub struct Operator {
    pub index: u8,
    pub name: &'static str,
    pub family: Family,
    pub arity: u8,
    pub operands: &'static [Type],
    pub result: Type,
    pub idempotent: bool,
    pub inverse: Inverse,
    pub gate: Option<Gate>,
    /// The family's computation, called only on an application already
    /// checked against this row's arity and operand types.
    pub compute: fn(&[Operand], &mut Arena) -> Result<Operand, Fault>,
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Operator")
            .field("index", &self.index)
            .field("name", &self.name)
            .field("family", &self.family)
            .field("arity", &self.arity)
            .field("operands", &self.operands)
            .field("result", &self.result)
            .field("idempotent", &self.idempotent)
            .field("inverse", &self.inverse)
            .field("gate", &self.gate)
            .finish_non_exhaustive()
    }
}

/// The designed, closed Operator count. Growth in phisor's learned counts
/// moves none of these 64 indices.
pub const COUNT: usize = 64;

/// The whole keyboard, assembled from the six family tables in index order.
///
/// It asserts the two structural facts the table rests on, so a family file
/// that renumbers or mislabels an Operator fails here rather than silently:
///
/// - **the index bijection** — the indices are exactly `0..table.len()`, each
///   once, and each Operator's own `index` field matches its position;
/// - **the family partition** — the families appear in [`Family::ALL`] order
///   as contiguous blocks, so every Operator lands in exactly one family.
///
/// Until families 3.4 to 3.9 land, [`super::family::all`] returns however many
/// Operators have been written, so this assembles a prefix of the final table
/// and the two assertions hold over that prefix. The 64-row completeness claim
/// is Property 7's, which stays `#[ignore]` until the prefix is the whole 64.
pub fn table() -> Vec<Operator> {
    let ops = super::family::all();

    for (at, op) in ops.iter().enumerate() {
        assert_eq!(
            op.index as usize, at,
            "operator {:?} sits at position {at} but declares index {}",
            op.name, op.index
        );
        assert_eq!(
            op.operands.len(),
            op.arity as usize,
            "operator {:?} declares arity {} but lists {} operand types",
            op.name,
            op.arity,
            op.operands.len()
        );
    }

    assert_families_partition(&ops);
    ops
}

/// The families must appear as contiguous blocks in [`Family::ALL`] order:
/// once a later family begins, no earlier one may reappear. That is what
/// makes the families a partition laid out in index order rather than
/// interleaved.
fn assert_families_partition(ops: &[Operator]) {
    let mut reached = 0usize;
    for op in ops {
        let rank = Family::ALL.iter().position(|&f| f == op.family).unwrap();
        assert!(
            rank >= reached,
            "operator {:?} in family {} breaks the index-order family partition",
            op.name,
            op.family.name()
        );
        reached = rank;
    }
}

/// The one Operator declared the identity: the arity-1 Operator whose result
/// is its operand, byte for byte. [`super::family::identity`] names it, so a
/// family file renaming or moving it is a single edit there.
pub fn identity() -> Operator {
    super::family::identity()
}

/// The one door every application goes through. It checks, in order and
/// before `op.compute` runs:
///
/// 1. the **arity** — `args.len()` against `op.arity`, returning
///    [`Fault::Arity`] with the Operator name, the declared arity and the
///    count given;
/// 2. each operand's **type** — `args[i].kind()` against `op.operands[i]`,
///    returning [`Fault::OperandType`] with the Operator name, the position,
///    the declared type and the type given.
///
/// Only once both hold does the family compute, so a faulted application
/// carries no result value — the error is returned in place of one, not
/// beside one.
pub fn apply(op: &Operator, args: &[Operand], arena: &mut Arena) -> Result<Operand, Fault> {
    if args.len() != op.arity as usize {
        return Err(Fault::Arity { operator: op.name, declared: op.arity, given: args.len() });
    }
    for (position, (arg, &declared)) in args.iter().zip(op.operands).enumerate() {
        let given = arg.kind();
        if given != declared {
            return Err(Fault::OperandType { operator: op.name, position, declared, given });
        }
    }
    (op.compute)(args, arena)
}
