//! The same bytes back. The reader is a cursor that refuses a form it cannot
//! account for: a form too short for the field it is asked for, a form with
//! bytes left over, and a tag no variant has are each
//! [`Fault::NotCanonical`] naming what was asked for and what was there.
//!
//! That strictness is the point. The Arena's bytes are the only place a
//! value lives once it is interned, so a reader that guessed at a short form
//! would hand back a value that was never written, and byte identity would
//! stop meaning structural equality.

use crate::ir::filler::{Attr, Det, Thing, Wh};
use crate::ir::{Aspect, Filler, Frame, Modal, Polarity, Role, Tense, Voice};
use phical::keyboard::{Arena, Fault, Handle, Type};

/// A reader over one canonical form, at a position in it.
pub(super) struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Cursor { bytes, at: 0 }
    }

    fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8], Fault> {
        let end = self.at + n;
        match self.bytes.get(self.at..end) {
            Some(field) => {
                self.at = end;
                Ok(field)
            }
            None => Err(Fault::NotCanonical {
                text: format!("{} bytes", self.bytes.len()),
                reason: format!("too short for {what} at byte {}: wanted {n} more", self.at),
            }),
        }
    }

    pub(super) fn u8(&mut self) -> Result<u8, Fault> {
        Ok(self.take(1, "a tag")?[0])
    }

    pub(super) fn u32(&mut self) -> Result<u32, Fault> {
        let field = self.take(4, "a count")?;
        Ok(u32::from_le_bytes([field[0], field[1], field[2], field[3]]))
    }

    pub(super) fn f64(&mut self) -> Result<f64, Fault> {
        let field = self.take(8, "a weight")?;
        let mut bits = [0u8; 8];
        bits.copy_from_slice(field);
        Ok(f64::from_bits(u64::from_le_bytes(bits)))
    }

    pub(super) fn handle(&mut self, ty: Type) -> Result<Handle, Fault> {
        Ok(Handle::new(ty, self.u32()?))
    }

    /// The text of the word the next handle names.
    pub(super) fn word(&mut self, arena: &Arena) -> Result<String, Fault> {
        let handle = self.handle(Type::Word)?;
        Ok(super::canon::read::<super::Word>(arena, handle)?.0)
    }

    /// Nothing left over: a form carrying bytes the reader did not account
    /// for is not the form the writer wrote.
    pub(super) fn end(&self) -> Result<(), Fault> {
        match self.bytes.len() - self.at {
            0 => Ok(()),
            over => Err(Fault::NotCanonical {
                text: format!("{} bytes", self.bytes.len()),
                reason: format!("{over} bytes left over after the form"),
            }),
        }
    }
}

/// The variant a tag names: its position in its type's declared `ALL` table.
pub(super) fn untag<T: Copy>(all: &[T], tag: u8, what: &str) -> Result<T, Fault> {
    match all.get(tag as usize) {
        Some(&x) => Ok(x),
        None => Err(Fault::NotCanonical {
            text: format!("tag {tag}"),
            reason: format!("no {what} has that tag; the table holds {}", all.len()),
        }),
    }
}

/// The frame the form holds: the predicate, the five features, then the
/// slots in the order they were written.
pub(super) fn frame(arena: &Arena, at: &mut Cursor) -> Result<Frame, Fault> {
    let pred = at.word(arena)?;
    let tense = untag(&Tense::ALL, at.u8()?, "tense")?;
    let mut f = Frame::new(&pred, tense);
    f.aspect = untag(&Aspect::ALL, at.u8()?, "aspect")?;
    f.modal = match at.u8()? {
        0 => None,
        tag => Some(untag(&Modal::ALL, tag - 1, "modal")?),
    };
    f.voice = untag(&Voice::ALL, at.u8()?, "voice")?;
    f.polarity = untag(&Polarity::ALL, at.u8()?, "polarity")?;
    let slots = at.u32()?;
    for _ in 0..slots {
        let role = Role::named(&at.word(arena)?);
        let fill = filler(arena, at)?;
        f.slots.push((role, fill));
    }
    Ok(f)
}

/// The filler the form holds. A clause resolves its frame handle, so the DAG
/// the writer minted is walked back into the tree it came from.
pub(super) fn filler(arena: &Arena, at: &mut Cursor) -> Result<Filler, Fault> {
    match at.u8()? {
        0 => Ok(Filler::Thing(thing(arena, at)?)),
        1 => Ok(Filler::name(&at.word(arena)?)),
        2 => Ok(Filler::pronoun(&at.word(arena)?)),
        3 => Ok(Filler::word(&at.word(arena)?)),
        4 => Ok(Filler::Hole(untag(&Wh::ALL, at.u8()?, "hole")?)),
        5 => {
            let handle = at.handle(Type::Frame)?;
            Ok(Filler::clause(super::canon::read::<Frame>(arena, handle)?))
        }
        tag => Err(Fault::NotCanonical {
            text: format!("tag {tag}"),
            reason: "no filler has that tag; the table holds 6".to_string(),
        }),
    }
}

fn thing(arena: &Arena, at: &mut Cursor) -> Result<Thing, Fault> {
    let head = at.word(arena)?;
    let mut t = Thing::new(&head);
    t.plural = at.u8()? != 0;
    t.det = det(arena, at)?;
    let attrs = at.u32()?;
    for _ in 0..attrs {
        let a = attr(arena, at)?;
        let value = filler(arena, at)?;
        t.attrs.push((a, value));
    }
    Ok(t)
}

fn det(arena: &Arena, at: &mut Cursor) -> Result<Option<Det>, Fault> {
    match at.u8()? {
        0 => Ok(None),
        1 => Ok(Some(Det::Definite)),
        2 => Ok(Some(Det::Indefinite)),
        3 => Ok(Some(Det::Near)),
        4 => Ok(Some(Det::Far)),
        5 => Ok(Some(Det::Quant(at.word(arena)?))),
        6 => Ok(Some(Det::Owner(at.word(arena)?))),
        tag => Err(Fault::NotCanonical {
            text: format!("tag {tag}"),
            reason: "no determiner has that tag; the table holds 7 with the absent case"
                .to_string(),
        }),
    }
}

fn attr(arena: &Arena, at: &mut Cursor) -> Result<Attr, Fault> {
    match at.u8()? {
        0 => Ok(Attr::Mod),
        1 => Ok(Attr::Order),
        2 => Ok(Attr::Count),
        3 => Ok(Attr::Kind(at.word(arena)?)),
        4 => Ok(Attr::Via(at.word(arena)?)),
        tag => Err(Fault::NotCanonical {
            text: format!("tag {tag}"),
            reason: "no attribute has that tag; the table holds 5".to_string(),
        }),
    }
}
