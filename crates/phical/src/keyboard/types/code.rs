//! Encoding and decoding under the declared layouts. One function per
//! direction, both reading the table in `super`'s doc comment, so the width
//! an operand encodes to and the width its type declares cannot drift apart.
//!
//! Every multi-byte field is little-endian. Every pad byte is written as
//! `0x00` rather than left as whatever was in the buffer, because the
//! equality rule compares encoded bytes and a pad byte carrying rubbish
//! would make two equal values unequal.

use super::{Handle, Operand, Type};
use crate::c64;

/// A byte slice offered for a type whose declared width it does not have.
/// It names the type, the width declared and the width given, so a caller
/// sees which of the two is wrong.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadWidth {
    pub ty: Type,
    pub declared: usize,
    pub given: usize,
}

impl std::fmt::Display for BadWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} declares a width of {} bytes; {} given",
            self.ty.name(),
            self.declared,
            self.given
        )
    }
}

impl std::error::Error for BadWidth {}

impl Operand {
    /// The operand's canonical bytes: exactly `self.kind().width()` of them.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.kind().width());
        self.write_into(&mut out);
        out
    }

    /// Append the operand's canonical bytes to `out`, for a caller building
    /// a canonical form out of several operands.
    pub fn write_into(&self, out: &mut Vec<u8>) {
        match *self {
            Operand::Phasor(z) => {
                out.extend_from_slice(&z.re.to_bits().to_le_bytes());
                out.extend_from_slice(&z.im.to_bits().to_le_bytes());
            }
            Operand::Class(c) => out.extend_from_slice(&c.to_le_bytes()),
            Operand::Level(bits) => out.push(bits),
            Operand::Variation(classes, bits) => {
                for c in classes {
                    out.extend_from_slice(&c.to_le_bytes());
                }
                out.push(bits);
                // The one pad byte, written as zero.
                out.push(0x00);
            }
            Operand::Kind(k) => out.extend_from_slice(&k.to_le_bytes()),
            Operand::Act(h) | Operand::Frame(h) | Operand::Row(h) | Operand::Word(h) => {
                out.extend_from_slice(&h.index.to_le_bytes())
            }
        }
    }

    /// The operand `bytes` encodes, read as type `ty`. The slice must be the
    /// type's declared width exactly — a shorter one has no value in it and
    /// a longer one holds something the layout does not describe.
    ///
    /// A handle's type comes from `ty`, not from the bytes: the four handle
    /// types share one layout, and which table an index names is carried by
    /// the declaration rather than by the encoding.
    pub fn decode(ty: Type, bytes: &[u8]) -> Result<Operand, BadWidth> {
        if bytes.len() != ty.width() {
            return Err(BadWidth { ty, declared: ty.width(), given: bytes.len() });
        }
        Ok(match ty {
            Type::Phasor => Operand::Phasor(c64::new(f64_at(bytes, 0), f64_at(bytes, 8))),
            Type::Class => Operand::Class(u16_at(bytes, 0)),
            Type::Level => Operand::Level(bytes[0]),
            Type::Variation => Operand::Variation(
                [u16_at(bytes, 0), u16_at(bytes, 2), u16_at(bytes, 4)],
                bytes[6],
            ),
            Type::Kind => Operand::Kind(u16_at(bytes, 0)),
            handle => Operand::handle(handle, u32_at(bytes, 0)),
        })
    }

    /// The handle an operand of a handle type carries, or `None` for the
    /// five inline types.
    pub fn as_handle(&self) -> Option<Handle> {
        match *self {
            Operand::Act(h) | Operand::Frame(h) | Operand::Row(h) | Operand::Word(h) => Some(h),
            _ => None,
        }
    }
}

fn u16_at(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn f64_at(bytes: &[u8], at: usize) -> f64 {
    let mut word = [0u8; 8];
    word.copy_from_slice(&bytes[at..at + 8]);
    f64::from_bits(u64::from_le_bytes(word))
}
