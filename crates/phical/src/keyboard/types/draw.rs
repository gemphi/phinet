//! The generators the keyboard's properties are run over, on
//! [`phistd::gen`] and on no outside crate.
//!
//! An [`Operand`] draw covers all nine types and, three draws in eight,
//! lands on the drawn type's declared minimum, maximum or zero — the three
//! values every property over the type table has to see. A phasor's
//! components come from `Seeded::awkward_f64` most of the time, so both
//! signed zeros, the subnormal edges, both infinities and the NaN bit
//! patterns are drawn far more often than chance would offer them; those
//! are the values where a float carried as bits either survives or does not.

use super::{Operand, Type};
use crate::c64;
use phistd::gen::{Gen, Seeded};

impl Gen for Type {
    fn draw(g: &mut Seeded) -> Self {
        *g.pick(&Type::ALL)
    }
}

impl Gen for Operand {
    fn draw(g: &mut Seeded) -> Self {
        let ty = Type::draw(g);
        match g.below(8) {
            0 => ty.minimum(),
            1 => ty.maximum(),
            2 => ty.zero(),
            _ => arbitrary(g, ty),
        }
    }
}

/// An arbitrary value of `ty`, the declared edges aside.
pub fn arbitrary(g: &mut Seeded, ty: Type) -> Operand {
    match ty {
        Type::Phasor => Operand::Phasor(phasor(g)),
        Type::Class => Operand::Class(g.u64() as u16),
        Type::Level => Operand::Level(level_bits(g)),
        Type::Variation => Operand::Variation(
            [g.u64() as u16, g.u64() as u16, g.u64() as u16],
            level_bits(g),
        ),
        Type::Kind => Operand::Kind(g.u64() as u16),
        handle => Operand::handle(handle, g.u64() as u32),
    }
}

/// A phasor: three draws in four from the awkward bit patterns, the fourth
/// an ordinary number of a size a tolerance means something at.
pub fn phasor(g: &mut Seeded) -> c64 {
    c64::new(component(g), component(g))
}

fn component(g: &mut Seeded) -> f64 {
    if g.below(4) == 0 {
        g.unit() * 8.0 - 4.0
    } else {
        g.awkward_f64()
    }
}

/// A level's bit count: 4, 5 or 6, the invalid 0, or — one draw in eight —
/// an arbitrary byte, since decoding a level does not validate it and the
/// round trip has to hold for whatever byte is there.
fn level_bits(g: &mut Seeded) -> u8 {
    if g.below(8) == 0 {
        g.u64() as u8
    } else {
        *g.pick(&[0u8, 4, 5, 6])
    }
}
