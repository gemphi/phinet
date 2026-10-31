//! The Q64 family — the second block of the keyboard, indices 14 to 21.
//!
//! Eight Operators over the C64/Q64 juncture: the same circle read
//! continuously as a phase or in 64 symbols as a sector. Six are lifted to
//! the declared ABI over `phical::q64` — `quantise`, `lift`, `quantise_word`,
//! `lift_word`, `hamming` and `sector_distance` — and two are declared new,
//! `sector_rotate` and `sector_nearest`, each built on the lifted six rather
//! than on fresh arithmetic.
//!
//! **How the nine types carry a sector and a word.** A sector is a
//! [`Operand::Class`]: the 64 symbols are a dense id space, and the id is
//! read modulo [`CHANNELS`] so every `u16` a class may hold is a legal
//! sector. A phase is the argument of a [`Operand::Phasor`], as in the
//! Circle family, and a scalar result (a Hamming count, a sector distance)
//! is carried on the real axis as `Phasor(value + 0i)` by that same
//! convention. A short Q64 **word** is a [`Operand::Variation`]: its three
//! classes are three channels' sectors and its level is [`SYMBOL_BITS`], the
//! 6 bits one Q64 symbol takes.
//!
//! The rows, in index order: 14 quantise, 15 lift, 16 quantise_word, 17
//! lift_word, 18 hamming, 19 sector_distance, 20 sector_rotate, 21
//! sector_nearest.

use crate::c64;
use crate::{q64, CHANNELS};

use super::super::arena::{Arena, Fault};
use super::super::operator::{Family, Inverse, Operator};
use super::super::types::{Operand, Type};

/// One Q64 symbol's width in bits: `2^6 = 64`, so a word of sectors carries
/// level 6, and that is the level a [`Operand::Variation`] holding sectors
/// declares.
pub const SYMBOL_BITS: u8 = 6;

/// Requirement 6 criterion 17's declared cap: an approximate inverse's
/// tolerance is a number no greater than this.
pub const TOLERANCE_CAP: f64 = 0.0099;

/// The declared tolerance of the one approximate inverse in this family,
/// `quantise`–`lift`.
///
/// `quantise` reads a phasor's argument alone and `lift` returns the sector's
/// central phase, so the round trip moves a phase by at most
/// [`q64::BOUND`] — the already-proven π/64 half-sector bound, which
/// `phical::q64`'s own test shows is tight. The distance the round trip is
/// judged under is the **resonance distance** `1 − cos(Δθ)` between the
/// operand's argument and the lifted sector centre, which is the measure
/// phasors are compared by everywhere else in phical, so the declared
/// tolerance is that bound read as a resonance: `1 − cos(π/64)` ≈ 0.001204.
/// That is comfortably at or below [`TOLERANCE_CAP`], and
/// `the_declared_tolerance_is_within_the_cap_and_the_round_trip_lands_inside_it`
/// proves both halves over a full sweep of the circle.
pub fn tolerance() -> f64 {
    1.0 - q64::BOUND.cos()
}

/// The eight Q64 Operators in index order, 14 to 21.
pub fn table() -> Vec<Operator> {
    // The one approximate inverse: quantise and lift are each other's, under
    // the declared resonance tolerance.
    let round_trip = Inverse::Approximate { tolerance: tolerance() };
    vec![
        op(14, "quantise", &[Type::Phasor], Type::Class, round_trip, quantise),
        op(15, "lift", &[Type::Class], Type::Phasor, round_trip, lift),
        op(
            16,
            "quantise_word",
            &[Type::Phasor; 3],
            Type::Variation,
            Inverse::None,
            quantise_word,
        ),
        op(17, "lift_word", &[Type::Variation], Type::Phasor, Inverse::None, lift_word),
        op(18, "hamming", &[Type::Variation; 2], Type::Phasor, Inverse::None, hamming),
        op(19, "sector_distance", &[Type::Class; 2], Type::Phasor, Inverse::None, sector_distance),
        op(20, "sector_rotate", &[Type::Class; 2], Type::Class, Inverse::None, sector_rotate),
        op(21, "sector_nearest", &[Type::Class; 3], Type::Class, Inverse::None, sector_nearest),
    ]
}

/// One Q64 row. The arity is the operand list's length, so the two cannot
/// drift apart; no Operator here is idempotent — not one has a result type
/// equal to its operand type, so applying it twice is not even well typed —
/// and none corresponds to a spectral gate.
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
        family: Family::Q64,
        arity: operands.len() as u8,
        operands,
        result,
        idempotent: false,
        inverse,
        gate: None,
        compute,
    }
}

/// The argument (phase) of a phasor operand, in [−π, π].
fn phase(a: &Operand) -> f64 {
    match a {
        Operand::Phasor(c) => c.arg(),
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The sector a class operand names: the dense class id modulo the 64
/// sectors, so every `u16` a class may hold is a legal sector.
fn sector(a: &Operand) -> u8 {
    match a {
        Operand::Class(c) => (*c as usize % CHANNELS) as u8,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The three channels' sectors a variation operand carries as a short Q64
/// word, each read modulo the 64 sectors. The level byte is the word's
/// symbol width and is not a channel.
fn word(a: &Operand) -> [u8; 3] {
    match a {
        Operand::Variation(classes, _) => {
            [0, 1, 2].map(|i| (classes[i] as usize % CHANNELS) as u8)
        }
        _ => unreachable!("apply checked the operand type"),
    }
}

/// Repack a scalar (a Hamming count, a sector distance) on the real axis.
fn scalar(v: f64) -> Operand {
    Operand::Phasor(c64::new(v, 0.0))
}

// --- the eight Operators: the six lifts call the existing `phical::q64`
// function and repack, and the two new ones are built on those. ---

fn quantise(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(Operand::Class(q64::quantise(phase(&a[0])) as u16))
}

fn lift(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(Operand::Phasor(c64::from_polar(1.0, q64::lift(sector(&a[0])))))
}

fn quantise_word(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let sectors = q64::quantise_word(&[phase(&a[0]), phase(&a[1]), phase(&a[2])]);
    Ok(Operand::Variation(
        [sectors[0] as u16, sectors[1] as u16, sectors[2] as u16],
        SYMBOL_BITS,
    ))
}

/// The word's resultant: its three sectors lifted to unit phasors by
/// `q64::lift_word` and added. The sum's argument is the word's mean
/// direction and its magnitude the word's concentration, so one phasor
/// carries what three channels agreed on. Many-to-one, hence no inverse.
fn lift_word(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let lifted = q64::lift_word(&word(&a[0]));
    Ok(Operand::Phasor(lifted.iter().fold(c64::new(0.0, 0.0), |sum, z| sum + z)))
}

fn hamming(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(q64::hamming(&word(&a[0]), &word(&a[1])) as f64))
}

fn sector_distance(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(q64::sector_distance(sector(&a[0]), sector(&a[1])) as f64))
}

/// New: turn a sector by an offset of sectors, the symbolic reading of a
/// phase shift. The sum wraps the 64 sectors, so the result is a sector like
/// its operands.
fn sector_rotate(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let turned = (sector(&a[0]) as usize + sector(&a[1]) as usize) % CHANNELS;
    Ok(Operand::Class(turned as u16))
}

/// New: of the two candidate sectors, the one the nearer to the target under
/// `q64::sector_distance`, ties going to the first — the sector-space reading
/// of `circle::nearest`. The comparison is the only thing added; the distance
/// is the existing function's.
fn sector_nearest(a: &[Operand], _: &mut Arena) -> Result<Operand, Fault> {
    let (target, first, second) = (sector(&a[0]), sector(&a[1]), sector(&a[2]));
    let keep = q64::sector_distance(first, target) <= q64::sector_distance(second, target);
    Ok(Operand::Class(if keep { first } else { second } as u16))
}

#[cfg(test)]
mod tests;
