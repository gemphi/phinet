//! Property 17's host half and Property 19, plus the examples that pin each
//! clause of Requirement 7 criteria 2 to 4. The property drivers are
//! `phistd::gen`'s seeded `for_all`, so a failure replays from
//! `(seed, case)` alone.
//!
//! Property 17's other half — that the *lowered* form of an operation is a
//! `define` of a `lambda` carrying 1 to 64 O-code operations — is the
//! lowering's, and the emission bound it rests on is Property 18's in
//! `machine::lower`. What is held here is the shape and the arity: a
//! `define` of a `lambda` of an Operator's declared argument count parses on
//! phiclm's reader and evaluates when applied, over all 64 Operators.

use super::*;
use crate::machine::Lowering;
use phical::keyboard::table;
use phiclm::lisp::read;
use phistd::gen::{for_all, Gen, Seeded, CASES};

mod names;
mod rows;
mod operators;

fn row(about: &str, kind: &str, other: &str) -> phiecs::Row {
    phiecs::Row::new(about, kind, other)
}

/// The fixture Kind set: two Kinds with rows, and one told with none.
fn fixture() -> KindRows {
    KindRows::of(&[
        row("king", "is-a", "man"),
        row("queen", "is-a", "woman"),
        row("sky", "colour", "blue"),
    ])
    .told("empty", &[])
}

/// The three Kind names the fixture holds — the oracle for Property 19.
const HELD: [&str; 3] = ["colour", "empty", "is-a"];

fn host() -> PhisorHost {
    PhisorHost::new(Arena::new(0x11), fixture()).expect("the five phisor- names are free")
}
