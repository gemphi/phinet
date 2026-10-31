//! The frame body's canonical bytes. A frame is written inline — its
//! predicate and every word in it as a minted word handle, its features and
//! its shape as tags — with one exception: a clause mints a frame handle of
//! its own, so `Filler::Clause(Box<Frame>)`, a recursive tree, becomes a DAG
//! of fixed-width handles rather than an inline struct of unbounded size.
//!
//! The clause is also where R7.13's first trigger lives. Each clause is one
//! depth down, and a frame below `MAX_CLAUSE_DEPTH` is refused with the
//! depth it reached and the cap, rather than written as bytes no fixed-width
//! operand could carry.

use crate::ir::filler::{Attr, Det, Thing, Wh};
use crate::ir::{Aspect, Filler, Frame, Modal, Polarity, Tense, Voice};
use phical::keyboard::{Arena, Fault, Handle, Type, MAX_CLAUSE_DEPTH};

/// A variant's tag: its position in its type's declared `ALL` table, so the
/// tags are the declaration order and nothing else. A variant missing from
/// its own table is a broken table, not a bad input, so it panics.
pub(super) fn tag<T: PartialEq>(all: &[T], x: &T, what: &str) -> u8 {
    match all.iter().position(|y| y == x) {
        Some(i) => i as u8,
        None => panic!("{what} is not in its own ALL table"),
    }
}

/// Mint `s` as a word and write its handle: four little-endian bytes.
pub(super) fn word(arena: &mut Arena, out: &mut Vec<u8>, s: &str) -> Result<(), Fault> {
    let handle = super::canon::mint(arena, &super::Word(s.to_string()))?;
    out.extend_from_slice(&handle.index.to_le_bytes());
    Ok(())
}

/// The predicate's handle, the five features, the slot count, then each slot
/// as its role's handle and its filler.
pub(super) fn frame(
    f: &Frame,
    arena: &mut Arena,
    out: &mut Vec<u8>,
    depth: usize,
) -> Result<(), Fault> {
    word(arena, out, &f.pred)?;
    out.push(tag(&Tense::ALL, &f.tense, "tense"));
    out.push(tag(&Aspect::ALL, &f.aspect, "aspect"));
    // The modal is the one optional feature: 0 for none, else one past its
    // tag, so the absent case cannot be read as `will`.
    out.push(match &f.modal {
        None => 0,
        Some(m) => 1 + tag(&Modal::ALL, m, "modal"),
    });
    out.push(tag(&Voice::ALL, &f.voice, "voice"));
    out.push(tag(&Polarity::ALL, &f.polarity, "polarity"));
    out.extend_from_slice(&(f.slots.len() as u32).to_le_bytes());
    for (role, fill) in &f.slots {
        // `Role::named` is total, so the four reserved names and every
        // preposition's own name both read back as the role they were.
        word(arena, out, role.name())?;
        filler(fill, arena, out, depth)?;
    }
    Ok(())
}

/// The filler's variant tag then its payload. The six tags are the variants'
/// declaration order in [`Filler`].
pub(super) fn filler(
    x: &Filler,
    arena: &mut Arena,
    out: &mut Vec<u8>,
    depth: usize,
) -> Result<(), Fault> {
    match x {
        Filler::Thing(t) => {
            out.push(0);
            thing(t, arena, out, depth)
        }
        Filler::Name(s) => {
            out.push(1);
            word(arena, out, s)
        }
        Filler::Pronoun(s) => {
            out.push(2);
            word(arena, out, s)
        }
        Filler::Word(s) => {
            out.push(3);
            word(arena, out, s)
        }
        Filler::Hole(wh) => {
            out.push(4);
            out.push(tag(&Wh::ALL, wh, "hole"));
            Ok(())
        }
        Filler::Clause(inner) => {
            out.push(5);
            let handle = clause(inner, arena, depth)?;
            out.extend_from_slice(&handle.index.to_le_bytes());
            Ok(())
        }
    }
}

/// A clause as a frame handle, one clause depth down. R7.13's first
/// trigger: past the cap the Act is refused rather than written.
fn clause(f: &Frame, arena: &mut Arena, depth: usize) -> Result<Handle, Fault> {
    let depth = depth + 1;
    if depth > MAX_CLAUSE_DEPTH {
        return Err(Fault::TooDeep { depth, cap: MAX_CLAUSE_DEPTH });
    }
    let mut bytes = Vec::new();
    frame(f, arena, &mut bytes, depth)?;
    arena.mint(Type::Frame, &bytes)
}

/// The head's handle, the number, the determiner, then each attribute with
/// its value. An attribute's value is a filler, so a thing nests too.
fn thing(t: &Thing, arena: &mut Arena, out: &mut Vec<u8>, depth: usize) -> Result<(), Fault> {
    word(arena, out, &t.head)?;
    out.push(u8::from(t.plural));
    det(t.det.as_ref(), arena, out)?;
    out.extend_from_slice(&(t.attrs.len() as u32).to_le_bytes());
    for (a, value) in &t.attrs {
        attr(a, arena, out)?;
        filler(value, arena, out, depth)?;
    }
    Ok(())
}

/// The determiner: 0 for none, then the four closed ones, then the two that
/// carry a word of their own.
fn det(d: Option<&Det>, arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault> {
    let (tag, carried) = match d {
        None => (0, None),
        Some(Det::Definite) => (1, None),
        Some(Det::Indefinite) => (2, None),
        Some(Det::Near) => (3, None),
        Some(Det::Far) => (4, None),
        Some(Det::Quant(s)) => (5, Some(s)),
        Some(Det::Owner(s)) => (6, Some(s)),
    };
    out.push(tag);
    match carried {
        Some(s) => word(arena, out, s),
        None => Ok(()),
    }
}

/// The attribute's kind. `Kind` and `Via` carry a name, and they carry it as
/// a handle beside their own tag rather than as the name alone: `Attr::Mod`
/// and `Attr::Kind("mod")` both answer `"mod"` to `name()`, and one form may
/// not stand for two values.
fn attr(a: &Attr, arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault> {
    let (tag, carried) = match a {
        Attr::Mod => (0, None),
        Attr::Order => (1, None),
        Attr::Count => (2, None),
        Attr::Kind(k) => (3, Some(k)),
        Attr::Via(p) => (4, Some(p)),
    };
    out.push(tag);
    match carried {
        Some(s) => word(arena, out, s),
        None => Ok(()),
    }
}
