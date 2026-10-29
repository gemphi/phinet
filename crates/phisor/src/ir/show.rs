//! The written form, in the user's notation:
//! `ASSERT(wear(agent: girl, theme: suit{colour: blue}), past, progressive)`.
//!
//! An act is its force around its frame and the frame's features: the tense
//! always, then the aspect unless simple, the modal, `passive`, `not`. A
//! frame inside a frame is bracketed with its own features:
//! `WISH(want(agent: I, theme: [go(agent: I, to: London), present]), present)`.
//! Determiners are not written; a plural noun carries `·pl`.
//!
//! The same writer, blinded, is the template: each filler becomes its
//! shape (`$N`, `$N{$A}`, `?who`), and for the construction the predicate
//! becomes `$V` as well.

use super::filler::{Attr, Filler, Thing};
use super::{Act, Frame};
use std::fmt::{self, Write};

/// How much of the act is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Everything: the act as read.
    Full,
    /// The fillers as their shapes; the predicate kept: the lexical template.
    Shape,
    /// The fillers and the predicate as shapes: the construction.
    Construction,
}

pub fn act(out: &mut impl Write, a: &Act, mode: Mode) -> fmt::Result {
    write!(out, "{}(", a.force.name())?;
    frame_with_features(out, &a.frame, mode)?;
    write!(out, ")")
}

fn frame_with_features(out: &mut impl Write, f: &Frame, mode: Mode) -> fmt::Result {
    frame(out, f, mode)?;
    write!(out, ", {}", f.tense.name())?;
    match f.aspect {
        super::Aspect::Simple => {}
        a => write!(out, ", {}", a.name())?,
    }
    match f.modal {
        Some(m) => write!(out, ", {}", m.name())?,
        None => {}
    }
    match f.voice {
        super::Voice::Passive => write!(out, ", passive")?,
        super::Voice::Active => {}
    }
    match f.polarity {
        super::Polarity::Negative => write!(out, ", not"),
        super::Polarity::Positive => Ok(()),
    }
}

fn frame(out: &mut impl Write, f: &Frame, mode: Mode) -> fmt::Result {
    match mode {
        Mode::Construction => write!(out, "$V(")?,
        Mode::Full | Mode::Shape => write!(out, "{}(", f.pred)?,
    }
    for (i, (role, filler)) in f.slots.iter().enumerate() {
        match i {
            0 => {}
            _ => write!(out, ", ")?,
        }
        write!(out, "{}: ", role.name())?;
        self::filler(out, filler, mode)?;
    }
    write!(out, ")")
}

pub fn filler(out: &mut impl Write, f: &Filler, mode: Mode) -> fmt::Result {
    match (f, mode) {
        (Filler::Thing(t), _) => thing(out, t, mode),
        (Filler::Name(s) | Filler::Pronoun(s) | Filler::Word(s), Mode::Full) => write!(out, "{s}"),
        (Filler::Name(_) | Filler::Pronoun(_), _) => write!(out, "$N"),
        (Filler::Word(_), _) => write!(out, "$A"),
        (Filler::Hole(wh), _) => write!(out, "?{}", wh.name()),
        (Filler::Clause(inner), _) => {
            write!(out, "[")?;
            frame_with_features(out, inner, mode)?;
            write!(out, "]")
        }
    }
}

fn thing(out: &mut impl Write, t: &Thing, mode: Mode) -> fmt::Result {
    match (mode, t.plural) {
        (Mode::Full, true) => write!(out, "{}·pl", t.head)?,
        (Mode::Full, false) => write!(out, "{}", t.head)?,
        _ => write!(out, "$N")?,
    }
    match t.attrs.is_empty() {
        true => return Ok(()),
        false => write!(out, "{{")?,
    }
    for (i, (attr, value)) in t.attrs.iter().enumerate() {
        match i {
            0 => {}
            _ => write!(out, ", ")?,
        }
        // An adjective's kind is the world's, not the sentence's: blinded,
        // every adjective is one shape.
        match (attr, mode) {
            (Attr::Mod | Attr::Kind(_), Mode::Shape | Mode::Construction) => {}
            _ => write!(out, "{}: ", attr.name())?,
        }
        filler(out, value, mode)?;
    }
    write!(out, "}}")
}

impl fmt::Display for Act {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        act(f, self, Mode::Full)
    }
}

impl fmt::Display for Filler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        filler(f, self, Mode::Full)
    }
}
