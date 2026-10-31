//! The Helix family — the third block of the keyboard, indices 22 to 27.
//!
//! Six Operators over `phical::helix::Codon`, the 64-fold correspondence's
//! whole reading of one channel: its codon (three bases of four), its
//! hexagram, the sphere harmonic's degree and order, and its place on the
//! B-DNA helix. All six are lifts — `Codon::of` and its five field reads —
//! and no arithmetic is added: each `compute` resolves its operand, calls
//! the existing function, and repacks.
//!
//! **How the nine types carry a codon.** A channel is a [`Operand::Class`]:
//! the 64 channels are a dense id space, and the id is read modulo
//! [`CODONS`] so every `u16` a class may hold names a channel, as a sector
//! is read in the Q64 family. A scalar field (the degree, the order) is
//! carried on the real axis as `Phasor(v + 0i)` by the convention the Circle
//! and Q64 families already use. The three remaining values are a `String`
//! and three `f64`, which no inline type holds, so each is interned and
//! named by a [`Operand::Word`] handle — the handle types being exactly how
//! a value wider than any inline entry of the type table gets one fixed
//! width.
//!
//! **A code is carried by its channel.** Every field the five reads return
//! is a function of the channel alone, so a code's canonical byte form *is*
//! its channel: the tag [`CODE_TAG`] then the channel as a `u16`
//! little-endian, [`CODE_WIDTH`] bytes in all. `codon_of` interns that form,
//! so two equal codes share one handle, and the five reads resolve it back.
//! The tag is what keeps the three byte forms apart: a handle naming a
//! codon's text, a position's coordinates, or a form phisor interned is
//! refused as [`Fault::NotCanonical`] rather than read as a channel.
//!
//! The rows, in index order: 22 codon_of, 23 codon, 24 hexagram, 25 degree,
//! 26 order, 27 position.

use crate::c64;
use crate::helix::Codon;
use phistd::codon::CODONS;

use super::super::arena::{Arena, Fault};
use super::super::operator::{Family, Inverse, Operator};
use super::super::types::{Handle, Operand, Type};

/// The byte opening a helix code's canonical form: `0x48`, ASCII `H`. No
/// codon text (`A`, `C`, `G` or `T` first) and no hexagram glyph (`0xE4`
/// first, the glyphs being three-byte UTF-8) begins with it, so a code is
/// never confused with what a read off it returns.
pub const CODE_TAG: u8 = 0x48;

/// A helix code's canonical width in bytes: the tag, then the channel as a
/// `u16` little-endian.
pub const CODE_WIDTH: usize = 3;

/// A helix position's canonical width in bytes: three `f64`, each as
/// `to_bits` little-endian, in `[x, y, z]` order — the same little-endian
/// rule the type table's phasor follows.
pub const POSITION_WIDTH: usize = 24;

/// The six Helix Operators in index order, 22 to 27.
pub fn table() -> Vec<Operator> {
    vec![
        op(22, "codon_of", &[Type::Class], Type::Word, codon_of),
        op(23, "codon", &[Type::Word], Type::Word, codon),
        op(24, "hexagram", &[Type::Word], Type::Word, hexagram),
        op(25, "degree", &[Type::Word], Type::Phasor, degree),
        op(26, "order", &[Type::Word], Type::Phasor, order),
        op(27, "position", &[Type::Word], Type::Word, position),
    ]
}

/// One Helix row. Every Operator here is arity 1 — a code built from a
/// channel, or one field read off a code — and the arity is the operand
/// list's length, so the two cannot drift apart.
///
/// None is idempotent: the three `word → word` rows take a code and return
/// text or coordinates, which are not codes, so a second application faults
/// rather than returning what it was given. None declares an inverse either:
/// the keyboard holds no Operator reading a channel back off a codon's text,
/// so there is no partner to declare, even though the codon and hexagram
/// reads are injective over the 64 channels. The gates of Requirement 6
/// criterion 15 are task 3.13's to measure and stay `None` here.
fn op(
    index: u8,
    name: &'static str,
    operands: &'static [Type],
    result: Type,
    compute: fn(&[Operand], &mut Arena) -> Result<Operand, Fault>,
) -> Operator {
    Operator {
        index,
        name,
        family: Family::Helix,
        arity: operands.len() as u8,
        operands,
        result,
        idempotent: false,
        inverse: Inverse::None,
        gate: None,
        compute,
    }
}

/// The channel a class operand names: the dense class id modulo the 64
/// codons, so every `u16` a class may hold is a legal channel.
fn channel(a: &Operand) -> usize {
    match a {
        Operand::Class(c) => *c as usize % CODONS,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// The handle a word operand carries.
fn named(a: &Operand) -> Handle {
    match a {
        Operand::Word(h) => *h,
        _ => unreachable!("apply checked the operand type"),
    }
}

/// Repack a scalar field (the degree, the order) on the real axis.
fn scalar(v: f64) -> Operand {
    Operand::Phasor(c64::new(v, 0.0))
}

/// The three coordinates read back off a position's canonical bytes, so a
/// reader of the handle `position` returns is not left to guess the layout.
/// `None` when the bytes are not [`POSITION_WIDTH`] long.
pub fn coordinates(bytes: &[u8]) -> Option<[f64; 3]> {
    let words: [[u8; 8]; 3] = match bytes.len() {
        POSITION_WIDTH => [0, 8, 16].map(|at| bytes[at..at + 8].try_into().unwrap()),
        _ => return None,
    };
    Some(words.map(|w| f64::from_bits(u64::from_le_bytes(w))))
}

/// The canonical byte form of the code for `channel`.
fn canon(channel: usize) -> [u8; CODE_WIDTH] {
    let [lo, hi] = (channel as u16).to_le_bytes();
    [CODE_TAG, lo, hi]
}

/// The code a word operand names, read back off its canonical bytes and
/// rebuilt by the existing `Codon::of`. A handle naming anything else is
/// refused rather than read as a channel; a handle naming nothing in this
/// Arena is the Arena's own [`Fault::NoSuchHandle`].
fn code(a: &Operand, arena: &Arena) -> Result<Codon, Fault> {
    let bytes = arena.resolve(named(a))?;
    let k = match bytes {
        [CODE_TAG, lo, hi] => u16::from_le_bytes([*lo, *hi]) as usize,
        _ => return Err(not_canonical(bytes)),
    };
    match k < CODONS {
        true => Ok(Codon::of(k)),
        false => Err(not_canonical(bytes)),
    }
}

/// The refusal, naming the bytes offered and the form expected.
fn not_canonical(bytes: &[u8]) -> Fault {
    Fault::NotCanonical {
        text: bytes.iter().map(|b| format!("{b:02x}")).collect(),
        reason: format!(
            "a helix code is {CODE_WIDTH} bytes: the tag {CODE_TAG:#04x}, \
             then the channel as a u16 little-endian below {CODONS}"
        ),
    }
}

// --- the six Operators: `codon_of` interns a channel's code, and the five
// field reads resolve it and return one field of the existing `Codon`. ---

/// `Codon::of` lifted: the channel's whole code, interned and named by a
/// handle. The construction is the Arena's interning, so minting the same
/// channel twice returns the one handle already minted.
fn codon_of(a: &[Operand], arena: &mut Arena) -> Result<Operand, Fault> {
    let form = canon(channel(&a[0]));
    Ok(Operand::Word(arena.mint(Type::Word, &form)?))
}

/// The code's three bases, interned as text.
fn codon(a: &[Operand], arena: &mut Arena) -> Result<Operand, Fault> {
    let text = code(&a[0], arena)?.codon;
    Ok(Operand::Word(arena.mint(Type::Word, text.as_bytes())?))
}

/// The code's hexagram glyph, interned as text.
fn hexagram(a: &[Operand], arena: &mut Arena) -> Result<Operand, Fault> {
    let text = code(&a[0], arena)?.hexagram;
    Ok(Operand::Word(arena.mint(Type::Word, text.as_bytes())?))
}

/// The sphere harmonic's degree, on the real axis.
fn degree(a: &[Operand], arena: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(code(&a[0], arena)?.degree as f64))
}

/// The sphere harmonic's order, on the real axis. It is signed, in
/// `−degree..=degree`, which the real axis carries as it stands.
fn order(a: &[Operand], arena: &mut Arena) -> Result<Operand, Fault> {
    Ok(scalar(code(&a[0], arena)?.order as f64))
}

/// The code's place on the B-DNA helix: three coordinates, interned under
/// the layout [`coordinates`] reads back.
fn position(a: &[Operand], arena: &mut Arena) -> Result<Operand, Fault> {
    let place = code(&a[0], arena)?.position;
    let mut form = [0u8; POSITION_WIDTH];
    for (slot, v) in form.chunks_mut(8).zip(place) {
        slot.copy_from_slice(&v.to_bits().to_le_bytes());
    }
    Ok(Operand::Word(arena.mint(Type::Word, &form)?))
}

#[cfg(test)]
mod tests;
