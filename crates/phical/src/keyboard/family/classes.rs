//! The Classes family — the fourth block of the keyboard, indices 28 to 39.
//!
//! Twelve Operators over `phical::classes`, the word classes grown by
//! exchange. Six are lifted to the declared ABI — `likelihood`, `move_gain`,
//! `apply_move`, `split_cost`, `halve` and `Halving::cut` — and six are
//! declared new: the Mode cut at 4, 5 and 6 bits (`mode`, `inside`, `size`),
//! the halving's two steps along a code (`sibling`, `parent`), and the keep
//! rule a split is judged by (`pay`). No arithmetic is repeated here: each
//! row unpacks its operands, calls the existing function, and repacks.
//!
//! **How the nine types carry a count, a code and a depth.** A count — a
//! bigram total, a likelihood, a gain, a cost — is carried on the real axis
//! as `Phasor(value + 0i)`, the convention the Circle and Q64 families carry
//! a scalar by, and a yes-or-no answer rides that same axis as `1.0` or
//! `0.0`, there being no boolean among the nine types. A **code** is a
//! [`Operand::Class`]: the dense id read modulo the width of the depth it
//! stands at, so every `u16` a class may hold is a legal code, exactly as
//! every `u16` is a legal Q64 sector. A **depth** is a [`Operand::Level`],
//! its byte read modulo [`DEPTHS`] — one past `classes::MAX_DEPTH`, the
//! deepest a code may grow — so every byte a level may hold names a depth a
//! code can stand at and no cut ever shifts a `u16` past its own width. The
//! three Modes' levels, 4, 5 and 6, are below that bound and read unchanged.
//!
//! **The split table.** `likelihood`, `move_gain` and `apply_move` read a
//! k×k class bigram table, and the move reads one unit's counts against it
//! besides — far more than three operands can name. The shape they are
//! lifted at is therefore the smallest one the halving actually moves a unit
//! in: the **two halves of one split**, each half's own-pair count on the
//! diagonal and nothing across, the table two halves hold once they are
//! fully separated. The move is then the whole of one half into its sibling,
//! which is the merge `classes::pay` judges a split by, and the three rows
//! compose exactly — `apply_move` is `likelihood` plus `move_gain`.
//!
//! The rows, in index order: 28 likelihood, 29 move_gain, 30 apply_move, 31
//! split_cost, 32 halve, 33 cut, 34 mode, 35 inside, 36 sibling, 37 parent,
//! 38 pay, 39 size.

use crate::c64;
// `base` is `phical::classes`, aliased so a lifted call reads apart from
// this module's own row of the same name.
use crate::classes as base;
use crate::classes::{Grow, Halving, WordCounts, MAX_DEPTH};

use super::super::arena::{Arena, Fault};
use super::super::operator::{Family, Inverse, Operator};
use super::super::types::{Operand, Type};

/// One past `classes::MAX_DEPTH`, the deepest a halving code may grow. A
/// level's byte is read modulo this, so every byte names a legal depth.
pub const DEPTHS: u8 = MAX_DEPTH as u8 + 1;

/// The full word code's width: Mode 64 is the six-bit code, and Mode 16 and
/// Mode 32 are that one code cut to 4 and 5 bits (Requirement 5 criterion 3).
pub const MODE_BITS: u32 = 6;

/// The most units one `halve` application grows a code over: the unit count
/// is read modulo this, so every `u16` a class may hold names a legal count
/// and one application stays cheap. A corpus-sized halving is the
/// Ladder_Runner's, through `classes::halve` itself.
pub const UNIT_CAP: u16 = 32;

/// The twelve Classes Operators in index order, 28 to 39.
pub fn table() -> Vec<Operator> {
    vec![
        op(28, "likelihood", &[Type::Phasor; 2], Type::Phasor, Inverse::None, likelihood),
        op(29, "move_gain", MOVE, Type::Phasor, Inverse::None, move_gain),
        op(30, "apply_move", MOVE, Type::Phasor, Inverse::None, apply_move),
        op(31, "split_cost", &[Type::Class, Type::Phasor], Type::Phasor, Inverse::None, split_cost),
        op(32, "halve", &[Type::Class, Type::Level], Type::Class, Inverse::None, halve),
        op(33, "cut", &[Type::Class, Type::Level, Type::Level], Type::Class, Inverse::None, cut),
        op(34, "mode", &[Type::Class, Type::Level], Type::Class, Inverse::None, mode),
        op(35, "inside", &[Type::Class, Type::Class, Type::Level], Type::Phasor, Inverse::None, inside),
        // The last bit flipped twice is the code it was, so `sibling` is its
        // own exact inverse — the one declared inverse in this family.
        op(36, "sibling", &[Type::Class], Type::Class, Inverse::Exact, sibling),
        op(37, "parent", &[Type::Class], Type::Class, Inverse::None, parent),
        op(38, "pay", MOVE, Type::Phasor, Inverse::None, pay),
        op(39, "size", &[Type::Level], Type::Class, Inverse::None, size),
    ]
}

/// The three operands a split's two halves and the half that moves take:
/// two counts and a class. `move_gain`, `apply_move` and `pay` all read it.
const MOVE: &[Type] = &[Type::Phasor, Type::Phasor, Type::Class];

/// One Classes row. The arity is the operand list's length, so the two
/// cannot drift apart; no row is idempotent — the two whose result type
/// matches their operand, `sibling` and `parent`, both move the code again
/// when applied again — and none corresponds to a spectral gate.
fn op(
    index: u8,
    name: &'static str,
    operands: &'static [Type],
    result: Type,
    inverse: Inverse,
    compute: fn(&[Operand], &mut Arena) -> Result<Operand, Fault>,
) -> Operator {
    Operator {
        index,
        name,
        family: Family::Classes,
        arity: operands.len() as u8,
        operands,
        result,
        idempotent: false,
        inverse,
        gate: None,
        compute,
    }
}

/// A count — a bigram total, a likelihood — on a phasor's real axis.
fn count(a: &Operand) -> f64 {
    match a {
        Operand::Phasor(c) => c.re,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The dense class id a class operand names, as it stands.
fn id(a: &Operand) -> u16 {
    match a {
        Operand::Class(c) => *c,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// A depth in bits: the level's byte read modulo [`DEPTHS`], so 4, 5 and 6
/// read unchanged and no byte names a depth a `u16` code cannot stand at.
fn bits(a: &Operand) -> u32 {
    match a {
        Operand::Level(b) => (*b % DEPTHS) as u32,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The code a class operand names at `depth` bits: its id read modulo the
/// depth's width, so every `u16` is a legal code there.
fn code(a: &Operand, depth: u32) -> u16 {
    (id(a) as u32 % (1u32 << depth)) as u16
}

/// Repack a scalar (a likelihood, a gain, a cost) on the real axis.
fn scalar(v: f64) -> Operand {
    Operand::Phasor(c64::new(v, 0.0))
}

/// A yes-or-no answer on that same real axis: 1.0 for yes, 0.0 for no.
fn truth(held: bool) -> Operand {
    scalar(match held {
        true => 1.0,
        false => 0.0,
    })
}

/// The split table of two counts — `[[n0, 0], [0, n1]]` — with its row and
/// column sums, which are the two counts themselves.
fn split(n0: f64, n1: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    (vec![n0, 0.0, 0.0, n1], vec![n0, n1], vec![n0, n1])
}

/// The half a class operand names — its last bit, the bit `sibling` flips —
/// and the sibling half it would move into.
fn halves(a: &Operand) -> (usize, usize) {
    let from = (id(a) & 1) as usize;
    (from, 1 - from)
}

/// `Halving::cut` over a halving of one unit: the code standing at `depth`
/// bits, cut to `to`. The shift is the method's own.
fn shift(c: u16, depth: u32, to: u32) -> u16 {
    Halving { codes: vec![c], depth, ..Halving::default() }.cut(0, to)
}

/// A code's Mode class: the full six-bit word code cut to the Mode's bits.
fn mode_of(a: &Operand, level: &Operand) -> u16 {
    shift(code(a, MODE_BITS), MODE_BITS, bits(level))
}

// --- the six lifts: each calls the existing `phical::classes` function over
// the split table and repacks. ---

fn likelihood(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (pairs, left, right) = split(count(&a[0]), count(&a[1]));
    Ok(scalar(base::likelihood(&pairs, &left, &right)))
}

/// The gain of moving the named half whole into its sibling — the merge of
/// the two halves, so the gain is a loss and the number `pay` weighs.
fn move_gain(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (pairs, left, right) = split(count(&a[0]), count(&a[1]));
    let (from, to) = halves(&a[2]);
    let zero = [0.0; 2];
    let moving = WordCounts { out: &zero, inn: &zero, own: pairs[from * 2 + from] };
    Ok(scalar(base::move_gain(&pairs, &left, &right, 2, &moving, from, to)))
}

/// The same move made in the tables, read back as the likelihood they then
/// hold — the only number of the moved table one operand can carry, and the
/// one that makes `apply_move` equal `likelihood` plus `move_gain`.
fn apply_move(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (mut pairs, mut left, mut right) = split(count(&a[0]), count(&a[1]));
    let (from, to) = halves(&a[2]);
    let zero = [0.0; 2];
    let moving = WordCounts { out: &zero, inn: &zero, own: pairs[from * 2 + from] };
    base::apply_move(&mut pairs, &mut left, &mut right, 2, &moving, from, to);
    Ok(scalar(base::likelihood(&pairs, &left, &right)))
}

fn split_cost(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(base::split_cost(id(&a[0]) as usize, count(&a[1]))))
}

/// The code grown over the unit count, every unit told once in one run — the
/// simplest corpus the ABI can name — to the depth the level gives, read
/// back as the count of classes it ended with.
fn halve(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let units = (id(&a[0]) % UNIT_CAP) as usize;
    let run: Vec<usize> = (0..units).collect();
    let grown = base::halve(units, &[run], Grow::Fixed(bits(&a[1])));
    Ok(Operand::Class(grown.classes as u16))
}

fn cut(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let depth = bits(&a[1]);
    Ok(Operand::Class(shift(code(&a[0], depth), depth, bits(&a[2]))))
}

// --- the six new rows: the Mode cut, the two steps along a code, and the
// keep rule, each built on the lifts above rather than on fresh arithmetic.

/// New: the Mode class of a word code — the one six-bit code cut at 4, 5 or
/// 6 bits, so Mode 16 is Mode 64 coarsened rather than a second run.
fn mode(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(Operand::Class(mode_of(&a[0], &a[1])))
}

/// New: whether a word code lies inside a coarse class at that Mode — the
/// containment Requirement 5 criterion 3 reports, read one pair at a time.
fn inside(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let level = bits(&a[2]).min(MODE_BITS);
    Ok(truth(mode_of(&a[0], &a[2]) == code(&a[1], level)))
}

/// New: the one class a unit may move to at its depth — its code's last bit
/// flipped, which is the sibling `classes::halve` offers every unit.
fn sibling(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(Operand::Class(id(&a[0]) ^ 1))
}

/// New: the code one depth back, the class a split came from.
fn parent(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(Operand::Class(id(&a[0]) >> 1))
}

/// New: whether a split pays for itself — `classes::pay`'s keep rule as one
/// Operator. The merge's loss is the split table's own `move_gain` the other
/// way about, and the cost is `split_cost` for the class the split adds over
/// the two halves' pairs: 1.0 when the split is kept, 0.0 when it is merged.
fn pay(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (n0, n1) = (count(&a[0]), count(&a[1]));
    let (pairs, left, right) = split(n0, n1);
    let zero = [0.0; 2];
    let moving = WordCounts { out: &zero, inn: &zero, own: n1 };
    let loss = -base::move_gain(&pairs, &left, &right, 2, &moving, 1, 0);
    let leaves = (id(&a[2]) as usize).saturating_sub(1);
    Ok(truth(loss > base::split_cost(leaves, n0 + n1)))
}

/// New: the class count a Mode's level names — 16, 32 and 64 at 4, 5 and 6
/// bits — which is the modulus the Hash_Null arm hashes a spelling by.
fn size(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(Operand::Class(1u16 << bits(&a[0])))
}

#[cfg(test)]
mod tests;
