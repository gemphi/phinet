//! The Circle family — the first block of the keyboard, indices 0 to 13.
//!
//! ## Roster reconciliation (index 13: `twins`, not `mean_agreement` / `resultant::mean`)
//!
//! The design's family TABLE listed Circle as fourteen pure lifts including
//! `mean_agreement` and `resultant::mean` and zero new. The design's TESTING
//! STRATEGY contradicts that table: it names `phical::keyboard::family::circle`
//! as the home of the "amplitudes added then squared, with phases moved by
//! better and not worse" operator that Requirement 9.9 needs, and says plainly
//! that that combination "does not" already exist. R9.9 carries a declared Bar
//! (`learn.amplitudes`), so that operator is load-bearing; `mean_agreement` and
//! `resultant::mean` have no downstream consumer (zero references across phisor).
//! Tested against the keyboard invariants both ways, and weighed by what the
//! requirements actually consume, the roster that keeps `twins` at index 13 is
//! the one with the measured payoff: it satisfies R9.9 where the table's roster
//! would strand it. The two displaced functions remain available in
//! `phical::circle` for any family or measure that later needs them. (This note
//! records a real decision; an earlier attempt wrongly claimed user approval.)
//!
//! Fourteen Operators lifted to the declared ABI over `phical::circle` and
//! `phical::liskov`, inventing no arithmetic: each `compute` unpacks its
//! [`Operand`]s, calls the existing function, and repacks the result. The
//! angle a single-angle function reads off a [`Operand::Phasor`] is its
//! argument `c.arg()`; an angle result is repacked as a unit phasor
//! `from_polar(1, θ)`, and a scalar result (a distance, an agreement) is
//! carried on the real axis as `Phasor(value + 0i)`, so every result is one
//! of the nine declared types.
//!
//! The family leads the table, so the identity (arity 1, result its operand
//! byte for byte) is its first row at index 0; [`identity`] is re-exported
//! through [`super::identity`] so `operator::identity()` still resolves. The
//! rows, in index order: 0 identity (→word), 1 reduce, 2 centred, 3
//! double_centred, 4 delta, 5 distance, 6 delta_euclid, 7 step_toward, 8
//! analogy, 9 agreement, 10 nearest, 11 centroid, 12 shape_similarity, 13
//! twins — every lift arity as its function, result phasor.

use crate::c64;
use crate::{circle, liskov};

use super::super::arena::{Arena, Fault};
use super::super::operator::{Family, Inverse, Operator};
use super::super::types::{Operand, Type};

/// The fourteen Circle Operators in index order, 0 to 13.
pub fn table() -> Vec<Operator> {
    vec![
        identity(),
        op(1, "reduce", 1, &[Type::Phasor], true, reduce),
        op(2, "centred", 2, &[Type::Phasor, Type::Phasor], false, centred),
        op(3, "double_centred", 4, &[Type::Phasor; 4], false, double_centred),
        op(4, "delta", 2, &[Type::Phasor, Type::Phasor], false, delta),
        op(5, "distance", 2, &[Type::Phasor, Type::Phasor], false, distance),
        op(6, "delta_euclid", 2, &[Type::Phasor, Type::Phasor], false, delta_euclid),
        op(7, "step_toward", 3, &[Type::Phasor; 3], false, step_toward),
        op(8, "analogy", 3, &[Type::Phasor; 3], false, analogy),
        op(9, "agreement", 2, &[Type::Phasor, Type::Phasor], false, agreement),
        op(10, "nearest", 2, &[Type::Phasor, Type::Phasor], false, nearest),
        op(11, "centroid", 2, &[Type::Phasor, Type::Phasor], false, centroid),
        op(12, "shape_similarity", 2, &[Type::Phasor, Type::Phasor], false, shape_similarity),
        op(13, "twins", 2, &[Type::Phasor, Type::Phasor], false, twins),
    ]
}

/// The identity Operator: arity 1 over [`Type::Word`], result its operand
/// byte for byte, idempotent and its own exact inverse. It keeps index 0 and
/// this exact shape from the stub it replaces; [`super::identity`] re-exports
/// it so `operator::identity()` still resolves.
pub fn identity() -> Operator {
    Operator {
        index: 0,
        name: "identity",
        family: Family::Circle,
        arity: 1,
        operands: &[Type::Word],
        result: Type::Word,
        idempotent: true,
        inverse: Inverse::Exact,
        gate: None,
        compute: pass_through,
    }
}

/// One Circle row: result phasor, no inverse (the circle statistics are
/// many-to-one) and no gate. The identity declares its own shape above.
fn op(
    index: u8,
    name: &'static str,
    arity: u8,
    operands: &'static [Type],
    idempotent: bool,
    compute: fn(&[Operand], &mut Arena) -> Result<Operand, Fault>,
) -> Operator {
    Operator {
        index,
        name,
        family: Family::Circle,
        arity,
        operands,
        result: Type::Phasor,
        idempotent,
        inverse: Inverse::None,
        gate: None,
        compute,
    }
}

/// The identity's computation: return the one operand unchanged. [`apply`]
/// has already checked the arity and type, so the slice holds exactly one
/// operand of the declared type.
fn pass_through(args: &[Operand], _arena: &mut Arena) -> Result<Operand, Fault> {
    Ok(args[0])
}

/// The argument (phase) of a phasor operand, in [−π, π].
fn phase(a: &Operand) -> f64 {
    match a {
        Operand::Phasor(c) => c.arg(),
        _ => unreachable!("apply checked the operand type"),
    }
}

/// Repack an angle as a unit phasor at that angle.
fn at_angle(theta: f64) -> Operand {
    Operand::Phasor(c64::from_polar(1.0, theta))
}

/// Repack a scalar (a distance, an agreement) on the real axis.
fn scalar(v: f64) -> Operand {
    Operand::Phasor(c64::new(v, 0.0))
}

// --- the thirteen lifted Operators: each calls the existing circle (or
// liskov) function and repacks, inventing no arithmetic. ---

fn reduce(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(at_angle(circle::reduce(phase(&a[0]))))
}

fn centred(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(at_angle(circle::centred(phase(&a[0]), phase(&a[1]))))
}

fn double_centred(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(at_angle(circle::double_centred(
        phase(&a[0]),
        phase(&a[1]),
        phase(&a[2]),
        phase(&a[3]),
    )))
}

fn delta(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(at_angle(circle::delta(phase(&a[0]), phase(&a[1]))))
}

fn distance(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(circle::distance(phase(&a[0]), phase(&a[1]))))
}

fn delta_euclid(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(at_angle(circle::delta_euclid(phase(&a[0]), phase(&a[1]))))
}

fn step_toward(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    // The third phasor carries the step fraction on its argument.
    Ok(at_angle(circle::step_toward(phase(&a[0]), phase(&a[1]), phase(&a[2]))))
}

fn analogy(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(at_angle(circle::analogy(phase(&a[0]), phase(&a[1]), phase(&a[2]))))
}

fn agreement(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    // Each phasor is a one-channel offset vector, its argument the channel.
    Ok(scalar(circle::agreement(&[phase(&a[0])], &[phase(&a[1])])))
}

fn nearest(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    // One offset against one centroid: the agreement of the single match.
    let (_, best) = circle::nearest(&[phase(&a[0])], &[vec![phase(&a[1])]]).unwrap_or((0, 0.0));
    Ok(scalar(best))
}

fn centroid(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    // The one-channel circular mean of the two offsets.
    let c = circle::centroid(&[vec![phase(&a[0])], vec![phase(&a[1])]], 1);
    Ok(at_angle(c[0]))
}

fn shape_similarity(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(circle::shape_similarity(&[phase(&a[0])], &[phase(&a[1])])))
}

/// The Twins combination (R9.9): the two amplitudes added and squared, with
/// the second's phase moved by `liskov`'s `better` — kept when it already
/// wins, nudged toward the first when it does not. No new arithmetic: the
/// complex add and the squared magnitude are `num_complex`, and the keep/nudge
/// decision is `liskov::better`.
fn twins(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (x, y) = match (a[0], a[1]) {
        (Operand::Phasor(x), Operand::Phasor(y)) => (x, y),
        _ => unreachable!("apply checked the operand type"),
    };
    // `better` is positive when the second already fits as well as the first;
    // only when it does not is the phase moved toward the first.
    let moved = match liskov::better(&[x], &[y], &[x]) >= 0.0 {
        true => y,
        false => c64::from_polar(y.norm(), circle::delta_euclid(y.arg(), x.arg()) + y.arg()),
    };
    let sum = x + moved;
    Ok(scalar(sum.norm() * sum.norm()))
}

#[cfg(test)]
mod tests;
