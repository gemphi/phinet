//! The closed nine-type table of the keyboard: one fixed width in bytes per
//! type, one little-endian layout per type, and the one equality rule the
//! whole set is judged under.
//!
//! The table is closed. These nine types are every type an Operator may
//! declare for an operand or a result, and nothing in phisor's learned
//! counts — templates, word classes, Kinds — moves a width or a layout.
//!
//! | type | inline / handle | width | layout, byte by byte |
//! |---|---|---|---|
//! | [`Type::Phasor`] | inline | 16 | `[0..8)` re as `f64::to_bits` LE; `[8..16)` im as `f64::to_bits` LE |
//! | [`Type::Class`] | inline | 2 | `u16` LE: the dense class id |
//! | [`Type::Level`] | inline | 1 | `u8`: 4, 5 or 6 — the code's `bits()` |
//! | [`Type::Variation`] | inline | 8 | `[0..2)` first class `u16` LE; `[2..4)` second; `[4..6)` last; `[6]` level bits `u8`; `[7]` pad `0x00` |
//! | [`Type::Kind`] | inline | 2 | `u16` LE: the dense kind id |
//! | [`Type::Word`] | handle | 4 | `u32` LE: an index into the Arena's words |
//! | [`Type::Row`] | handle | 4 | `u32` LE: an index into the Arena's rows |
//! | [`Type::Frame`] | handle | 4 | `u32` LE: an index into the Arena's frames |
//! | [`Type::Act`] | handle | 4 | `u32` LE: an index into the Arena's acts |
//!
//! Every multi-byte field is little-endian and every pad byte is written as
//! `0x00`, so an encoding is byte-exact and the byte identity the equality
//! rule rests on means what it says.

use crate::c64;

pub mod code;
pub mod draw;
pub mod edges;

pub use code::BadWidth;

/// The closed nine-type table. The declaration order is the table's order,
/// and [`Type::ALL`] holds it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Phasor,
    Class,
    Level,
    Variation,
    Kind,
    Act,
    Frame,
    Row,
    Word,
}

impl Type {
    /// The nine types in declaration order.
    pub const ALL: [Type; 9] = [
        Type::Phasor,
        Type::Class,
        Type::Level,
        Type::Variation,
        Type::Kind,
        Type::Act,
        Type::Frame,
        Type::Row,
        Type::Word,
    ];

    /// The type's declared width in bytes, in 1 to 64.
    pub const fn width(self) -> usize {
        match self {
            Type::Phasor => 16,
            Type::Variation => 8,
            Type::Class | Type::Kind => 2,
            Type::Level => 1,
            Type::Act | Type::Frame | Type::Row | Type::Word => 4,
        }
    }

    /// The type's position in [`Type::ALL`], for the per-type tables the
    /// frozen operand set and the index table are held in.
    pub const fn index(self) -> usize {
        match self {
            Type::Phasor => 0,
            Type::Class => 1,
            Type::Level => 2,
            Type::Variation => 3,
            Type::Kind => 4,
            Type::Act => 5,
            Type::Frame => 6,
            Type::Row => 7,
            Type::Word => 8,
        }
    }

    /// The type's name as the index table and the run record spell it.
    pub const fn name(self) -> &'static str {
        match self {
            Type::Phasor => "phasor",
            Type::Class => "class",
            Type::Level => "level",
            Type::Variation => "variation",
            Type::Kind => "kind",
            Type::Act => "act",
            Type::Frame => "frame",
            Type::Row => "row",
            Type::Word => "word",
        }
    }

    /// Whether the type's value is a handle into the Arena rather than the
    /// value itself. The four handle types are how a recursive value — an
    /// Act holding a clause holding a Frame — gets one fixed width.
    pub const fn is_handle(self) -> bool {
        matches!(self, Type::Act | Type::Frame | Type::Row | Type::Word)
    }

}

// Each type's declared minimum, maximum and zero are in `edges`.

/// A stable index into one of the Arena's four tables, carrying the type of
/// the table it indexes. The value of a handle type *is* the handle, which
/// is why the Arena interns rather than merely allocates: byte equality of
/// handles is then structural equality of what they name.
///
/// The minting and resolving live in `keyboard::arena`; the handle is
/// declared here because its four bytes are part of the type table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Handle {
    pub index: u32,
    pub ty: Type,
}

impl Handle {
    pub const fn new(ty: Type, index: u32) -> Self {
        Handle { index, ty }
    }
}

/// A value of one of the nine types, as the Rust reader holds it. The four
/// handle types hold a [`Handle`], never a tree, so every operand has the
/// fixed width its type declares.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    Phasor(c64),
    Class(u16),
    Level(u8),
    /// The first, second and last class of the variation, then the level's
    /// bit count.
    Variation([u16; 3], u8),
    Kind(u16),
    Act(Handle),
    Frame(Handle),
    Row(Handle),
    Word(Handle),
}

impl Operand {
    /// The operand's type. The variant is authoritative: a handle built for
    /// the wrong table is corrected here and on encoding.
    pub const fn kind(&self) -> Type {
        match self {
            Operand::Phasor(_) => Type::Phasor,
            Operand::Class(_) => Type::Class,
            Operand::Level(_) => Type::Level,
            Operand::Variation(..) => Type::Variation,
            Operand::Kind(_) => Type::Kind,
            Operand::Act(_) => Type::Act,
            Operand::Frame(_) => Type::Frame,
            Operand::Row(_) => Type::Row,
            Operand::Word(_) => Type::Word,
        }
    }

    /// The operand of a handle type `ty` naming `index`. Panics on an inline
    /// type: there is no handle to build.
    pub fn handle(ty: Type, index: u32) -> Operand {
        let h = Handle::new(ty, index);
        match ty {
            Type::Act => Operand::Act(h),
            Type::Frame => Operand::Frame(h),
            Type::Row => Operand::Row(h),
            Type::Word => Operand::Word(h),
            inline => panic!("{} is an inline type, not a handle", inline.name()),
        }
    }
}

/// The declared phasor tolerance: two phasors are equal when the absolute
/// difference of each of their two components is at most this.
pub const PHASOR_EPSILON: f64 = 1e-12;

/// The one equality rule for results. Two operands of different types are
/// never equal. Two values of the eight types other than `phasor` are equal
/// exactly when their encoded bytes are identical; two phasors are equal
/// when each component's absolute difference is at most [`PHASOR_EPSILON`].
pub fn equal(a: &Operand, b: &Operand) -> bool {
    match (a, b) {
        (Operand::Phasor(x), Operand::Phasor(y)) => {
            component_equal(x.re, y.re) && component_equal(x.im, y.im)
        }
        _ if a.kind() != b.kind() => false,
        _ => a.encode() == b.encode(),
    }
}

/// One phasor component. The tolerance clause decides every pair whose
/// difference is a number; the bit-identity clause is what makes the rule
/// reflexive on the values where the difference is not — a NaN against
/// itself, an infinity against itself — so that `equal(x, x)` holds for
/// every phasor and no round trip passes vacuously.
fn component_equal(x: f64, y: f64) -> bool {
    x.to_bits() == y.to_bits() || (x - y).abs() <= PHASOR_EPSILON
}

#[cfg(test)]
mod tests;
