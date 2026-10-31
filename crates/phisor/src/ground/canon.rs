//! The canonical byte forms phisor hands the Arena, for the four of the
//! keyboard's nine types that are handles: `act`, `frame`, `row` and `word`.
//!
//! The Arena interns bytes and never learns what they mean — phical cannot
//! see `phisor::ir::Act`, since phisor depends on phical and not the reverse
//! — so the meaning lives here, on this side of the edge. The one thing the
//! forms owe R6.16 is that byte identity be structural equality: two
//! structurally equal values must write the same bytes, and two different
//! ones must not. Hence a *canonical* form rather than any form: a nested
//! value is a minted handle, every multi-byte field is little-endian, every
//! tag is a position in a declared `ALL` table, and text that would not
//! survive the row form is refused rather than silently repaired.
//!
//! | type | form |
//! |---|---|
//! | [`Word`] | the text's UTF-8 bytes, nothing around them |
//! | [`Row`] | three word handles (about, kind, other) then the weight's `f64` bits, 20 bytes |
//! | `Act` | the force's tag then the frame's handle, 5 bytes |
//! | `Frame` | the predicate's handle, the five features, the slot count, then the slots |

use super::read::Cursor;
use super::write;
use crate::ir::{Act, Force, Frame};
use phical::keyboard::{Arena, Canon, Fault, Handle, Type};

/// phisor's `word`: one word of text, as the keyboard's `word` type holds
/// it. A newtype rather than a bare `String` because [`Canon`] is phical's
/// trait and the form it declares is phisor's.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Word(pub String);

/// phisor's `row`: the workspace's one interchange row, as the keyboard's
/// `row` type holds it. The three fields are word handles, so two rows over
/// the same three words share their bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct Row(pub phiecs::Row);

/// Mint the handle a value's canonical form interns to. Interning means a
/// structurally equal value gets the handle already minted, so the handle a
/// second call returns is the first call's handle.
pub fn mint<C: Canon>(arena: &mut Arena, value: &C) -> Result<Handle, Fault> {
    let mut bytes = Vec::new();
    value.write(arena, &mut bytes)?;
    arena.mint(C::TYPE, &bytes)
}

/// Read back the value a handle names.
pub fn read<C: Canon>(arena: &Arena, handle: Handle) -> Result<C, Fault> {
    C::read(arena, arena.resolve(handle)?)
}

impl Canon for Word {
    const TYPE: Type = Type::Word;

    fn write(&self, _arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault> {
        out.extend_from_slice(text(&self.0)?.as_bytes());
        Ok(())
    }

    fn read(_arena: &Arena, bytes: &[u8]) -> Result<Self, Fault> {
        match std::str::from_utf8(bytes) {
            Ok(s) => Ok(Word(s.to_string())),
            Err(e) => Err(Fault::NotCanonical {
                text: format!("{} bytes", bytes.len()),
                reason: format!("not UTF-8: {e}"),
            }),
        }
    }
}

/// The canonicality rule for a word's text, and R7.13's second trigger. A
/// word reaches the world as a field of a row, and the row form is
/// `about \t kind \t other \t weight` with every field trimmed: so text
/// holding a separator, or carrying space the form would strip, would come
/// back as different text than it went in as. That is exactly what a
/// canonical form may not do, and it is refused here rather than repaired,
/// because repairing it would map two words onto one handle.
fn text(s: &str) -> Result<&str, Fault> {
    let bad = |reason: String| Fault::NotCanonical { text: s.to_string(), reason };
    if s.is_empty() {
        return Err(bad("empty: the row form skips a field with nothing in it".to_string()));
    }
    if s.trim() != s {
        return Err(bad("untrimmed: the row form trims every field".to_string()));
    }
    match s.chars().find(|c| matches!(c, '\t' | '\n' | '\r' | '\0')) {
        Some(c) => Err(bad(format!("holds {c:?}, which the row form separates on"))),
        None => Ok(s),
    }
}

impl Canon for Row {
    const TYPE: Type = Type::Row;

    fn write(&self, arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault> {
        for field in [&self.0.about, &self.0.kind, &self.0.other] {
            write::word(arena, out, field)?;
        }
        let weight = self.0.weight;
        if !(0.0..=1.0).contains(&weight) {
            return Err(Fault::NotCanonical {
                text: weight.to_string(),
                reason: "a weight outside 0 to 1 inclusive is clamped by the row form".to_string(),
            });
        }
        out.extend_from_slice(&weight.to_bits().to_le_bytes());
        Ok(())
    }

    fn read(arena: &Arena, bytes: &[u8]) -> Result<Self, Fault> {
        let mut at = Cursor::new(bytes);
        let about = at.word(arena)?;
        let kind = at.word(arena)?;
        let other = at.word(arena)?;
        let weight = at.f64()?;
        at.end()?;
        Ok(Row(phiecs::Row::weighted(&about, &kind, &other, weight)))
    }
}

impl Canon for Act {
    const TYPE: Type = Type::Act;

    fn write(&self, arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault> {
        out.push(write::tag(&Force::ALL, &self.force, "force"));
        let frame = mint(arena, &self.frame)?;
        out.extend_from_slice(&frame.index.to_le_bytes());
        Ok(())
    }

    fn read(arena: &Arena, bytes: &[u8]) -> Result<Self, Fault> {
        let mut at = Cursor::new(bytes);
        let force = super::read::untag(&Force::ALL, at.u8()?, "force")?;
        let frame = at.handle(Type::Frame)?;
        at.end()?;
        Ok(Act::new(force, read::<Frame>(arena, frame)?))
    }
}

impl Canon for Frame {
    const TYPE: Type = Type::Frame;

    /// The top frame of an Act is at clause depth 0; a clause inside it
    /// mints a frame of its own one depth down, up to `MAX_CLAUSE_DEPTH`.
    fn write(&self, arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault> {
        write::frame(self, arena, out, 0)
    }

    fn read(arena: &Arena, bytes: &[u8]) -> Result<Self, Fault> {
        let mut at = Cursor::new(bytes);
        let frame = super::read::frame(arena, &mut at)?;
        at.end()?;
        Ok(frame)
    }
}
