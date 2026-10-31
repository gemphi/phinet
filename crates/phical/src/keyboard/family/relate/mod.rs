//! The Relate family — the last block of the keyboard, indices 48 to 63.
//!
//! Sixteen Operators over `phical::relate`. Twelve are lifted to the declared
//! ABI here — `find_laws`, `find_inverses`, `implied`, `score`, `prior`,
//! `two_hop`, `chance`, `score_laws`, `laws_epoch`, `inherit_epoch`,
//! `Converse::declared` and `Inherit::from_told` — and four are declared new
//! in [`new`] (`law_holds`, `symmetric`, `confident`, `bar`), each built on
//! the lifted twelve rather than on fresh arithmetic.
//!
//! **How the nine types carry a triple.** A told relation is a
//! [`Triple`](crate::relate::Triple) of three row indices
//! `(about, kind, other)`: `about` and `other` are [`Operand::Row`] handles
//! whose `index` is the row index, and `kind` is an [`Operand::Kind`] whose
//! `u16` is the dense kind id. An Operator that lifts a relate function over
//! `&[Triple]` builds the one told triple its operands name and runs the
//! existing function, inventing no arithmetic; a count or confidence comes
//! back on a phasor's real axis as `Phasor(v + 0i)`, as the Circle and
//! Classes families read scalars.
//!
//! **The Joint-bound five.** `score`, `two_hop`, `score_laws`, `laws_epoch`
//! and `inherit_epoch` take a trained [`Joint`](crate::Joint) the keyboard
//! ABI does not carry — building one would invent the very operator the
//! keyboard holds. Each is lifted at its faithful Joint-free core: the
//! **count of told rows the epoch or score iterates**, one per told row. That
//! count is pure, is exactly what the function walks, and adds no arithmetic.
//!
//! The rows, in index order: 48 find_laws, 49 find_inverses, 50 implied, 51
//! score, 52 prior, 53 two_hop, 54 chance, 55 score_laws, 56 laws_epoch, 57
//! inherit_epoch, 58 converse, 59 inherit, and 60–63 the four new in [`new`].

use crate::c64;
use crate::relate::{self, Triple};

use super::super::arena::{Arena, Fault};
use super::super::operator::{Family, Inverse, Operator};
use super::super::types::{Operand, Type};

mod new;
#[cfg(test)]
mod tests;

/// The minimum support `find_laws` and `find_inverses` are read at: one told
/// triple is enough to count a law over, so the single triple an Operator
/// names is testable.
pub(super) const MIN_SUPPORT: usize = 1;

/// The one told triple an Operator names, and the three operand shapes that
/// name it: a row, a kind and a row.
pub(super) const TRIPLE: &[Type] = &[Type::Row, Type::Kind, Type::Row];

/// The sixteen Relate Operators in index order, 48 to 63: the twelve lifts
/// here, then [`new::table`]'s four at 60 to 63.
pub fn table() -> Vec<Operator> {
    let mut ops = vec![
        op(48, "find_laws", TRIPLE, find_laws),
        op(49, "find_inverses", TRIPLE, find_inverses),
        op(50, "implied", TRIPLE, implied),
        op(51, "score", TRIPLE, score),
        op(52, "prior", TRIPLE, prior),
        op(53, "two_hop", TRIPLE, two_hop),
        op(54, "chance", &[Type::Class], chance),
        op(55, "score_laws", TRIPLE, score_laws),
        op(56, "laws_epoch", TRIPLE, laws_epoch),
        op(57, "inherit_epoch", TRIPLE, inherit_epoch),
        op(58, "converse", &[Type::Kind, Type::Kind], converse),
        op(59, "inherit", TRIPLE, inherit),
    ];
    ops.extend(new::table());
    ops
}

/// One Relate row: result a scalar on a phasor's real axis, no inverse (a law
/// count is many-to-one) and no gate. The arity is the operand list's length,
/// so the two cannot drift apart, and no row is idempotent — none has a
/// result type equal to its operand type.
pub(super) fn op(
    index: u8,
    name: &'static str,
    operands: &'static [Type],
    compute: fn(&[Operand], &mut Arena) -> Result<Operand, Fault>,
) -> Operator {
    Operator {
        index,
        name,
        family: Family::Relate,
        arity: operands.len() as u8,
        operands,
        result: Type::Phasor,
        idempotent: false,
        inverse: Inverse::None,
        gate: None,
        compute,
    }
}

/// The row index a Row handle names.
pub(super) fn row(a: &Operand) -> usize {
    match a {
        Operand::Row(h) => h.index as usize,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The dense kind id a Kind operand names.
pub(super) fn kind(a: &Operand) -> usize {
    match a {
        Operand::Kind(k) => *k as usize,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// A count a class operand names, for `chance` and `bar`: its `u16`, at least
/// one, so a ranking pool is never empty.
pub(super) fn count(a: &Operand) -> usize {
    match a {
        Operand::Class(c) => (*c as usize).max(1),
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The one told triple a `(Row, Kind, Row)` operand list names.
pub(super) fn triple(a: &[Operand]) -> Triple {
    Triple { about: row(&a[0]), kind: kind(&a[1]), other: row(&a[2]) }
}

/// Repack a scalar (a confidence, a count, a bar) on the real axis.
pub(super) fn scalar(v: f64) -> Operand {
    Operand::Phasor(c64::new(v, 0.0))
}

/// A yes-or-no answer on that same real axis: 1.0 for yes, 0.0 for no.
pub(super) fn truth(held: bool) -> Operand {
    scalar(if held { 1.0 } else { 0.0 })
}

// --- the twelve lifts: each calls the existing `phical::relate` function
// over the one told triple (or the count) its operands name. ---

fn find_laws(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let laws = relate::find_laws(&[triple(a)], MIN_SUPPORT);
    Ok(scalar(laws.first().map_or(0.0, |l| l.confidence())))
}

fn find_inverses(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let inv = relate::find_inverses(&[triple(a)], MIN_SUPPORT);
    Ok(scalar(inv.first().map_or(0.0, |i| i.confidence())))
}

fn implied(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let told = [triple(a)];
    let inv = relate::find_inverses(&told, MIN_SUPPORT);
    let refs: Vec<&relate::Inverse> = inv.iter().collect();
    Ok(scalar(relate::implied(&told, &refs).len() as f64))
}

/// The Joint-free core the five Joint-bound reads share: the count of told
/// rows the epoch or score iterates, one per told row named. `score`,
/// `two_hop`, `score_laws`, `laws_epoch` and `inherit_epoch` each walk the
/// told rows once, so for the one triple an Operator names the faithful
/// reading is one — no Joint built, so none is invented.
fn told_rows(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar([triple(a)].len() as f64))
}

use told_rows as score;
use told_rows as two_hop;
use told_rows as score_laws;
use told_rows as laws_epoch;
use told_rows as inherit_epoch;

/// The bar, read without a Joint: the prior MRR over the one told triple,
/// ranking its own kind's answers by how often the kind named them in
/// training. Pure — `prior` consults no operator.
fn prior(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let told = [triple(a)];
    let (pool, by_kind) = relate::pools(&told);
    Ok(scalar(relate::prior(&told, &told, &pool, &by_kind).mrr))
}

/// `chance`: the MRR of a uniformly random ranking over `n` answers.
fn chance(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(relate::chance(count(&a[0]))))
}

/// `Converse::declared`: declare the two kinds each other's converse and
/// report whether the first now reads with a converse — 1.0 for the pair, as
/// the declaration always registers a distinct pair both ways.
fn converse(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (k, kc) = (kind(&a[0]), kind(&a[1]));
    let conv = relate::Converse::declared(&[(k, kc)], 1.0);
    Ok(truth(conv.of.contains_key(&k)))
}

/// `Inherit::from_told`: declare the triple's kind inheriting along itself
/// and report how many parents the triple's `about` inherits.
fn inherit(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let t = triple(a);
    let inh = relate::Inherit::from_told(&[t], t.kind, vec![t.kind], 1.0);
    Ok(scalar(inh.of(&t).len() as f64))
}
