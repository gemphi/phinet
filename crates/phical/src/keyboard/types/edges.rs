//! Each type's declared minimum, maximum and zero — the three values every
//! property over the type table has to see, and the three the frozen operand
//! set is required to carry for each of the nine types.
//!
//! A type's **zero** is the value whose encoding is the type's width in
//! `0x00` bytes. For eight types that is also the minimum; for `level` it is
//! the invalid value `0`, which is not a level at all and is carried
//! deliberately so the type error of R6.8 is exercised rather than assumed.

use super::{Operand, Type};
use crate::c64;

impl Type {
    /// The type's declared minimum.
    pub fn minimum(self) -> Operand {
        match self {
            Type::Phasor => Operand::Phasor(c64::new(f64::MIN, f64::MIN)),
            Type::Class => Operand::Class(0),
            // A level is 4, 5 or 6 bits: 4 is the minimum, and 0 is the
            // invalid zero, which is `zero()` and not this.
            Type::Level => Operand::Level(4),
            Type::Variation => Operand::Variation([0; 3], 4),
            Type::Kind => Operand::Kind(0),
            handle => Operand::handle(handle, 0),
        }
    }

    /// The type's declared maximum.
    pub fn maximum(self) -> Operand {
        match self {
            Type::Phasor => Operand::Phasor(c64::new(f64::MAX, f64::MAX)),
            Type::Class => Operand::Class(u16::MAX),
            Type::Level => Operand::Level(6),
            Type::Variation => Operand::Variation([u16::MAX; 3], 6),
            Type::Kind => Operand::Kind(u16::MAX),
            handle => Operand::handle(handle, u32::MAX),
        }
    }

    /// The type's zero: the width in `0x00` bytes, read back.
    pub fn zero(self) -> Operand {
        match self {
            Type::Phasor => Operand::Phasor(c64::new(0.0, 0.0)),
            Type::Class => Operand::Class(0),
            Type::Level => Operand::Level(0),
            Type::Variation => Operand::Variation([0; 3], 0),
            Type::Kind => Operand::Kind(0),
            handle => Operand::handle(handle, 0),
        }
    }
}
