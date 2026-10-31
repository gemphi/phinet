//! `ground` — an Act as rows: the event is an entity, and what the sentence
//! says about it is one row for the predicate, one row for each role and one
//! row for each feature (R5.12).
//!
//! This is the neo-Davidsonian move. "A girl was wearing a blue suit" is not
//! a single fact but an event with a name: the event is a wearing, the girl
//! is its agent, the suit its theme, and it was past. Every row is *about*
//! that one event, which is why the event has to be an entity before the
//! sentence can be rows at all:
//!
//! ```text
//! e1  assert    wear      1.0   the predicate, with the force as its kind
//! e1  agent     girl      1.0   one row per role, in the frame's own order
//! e1  theme     suit      1.0
//! e1  tense     past      1.0   one row per marked feature
//! e1  aspect    progressive 1.0
//! ```
//!
//! Three choices this module makes, each of which the row count rests on.
//!
//! **The force rides on the predicate row's kind.** R5.12 counts the rows —
//! one predicate, the roles, the features — so the force cannot have a row
//! of its own without breaking the count, and dropping it would lose what
//! the sentence is *for*. The relation the event bears to its predicate is
//! exactly the force: `e1 assert wear` says this event is asserted to be a
//! wearing, `e1 ask wear` that it is asked after.
//!
//! **A feature is grounded when it is marked.** A frame always carries five
//! features, but present tense, simple aspect, no modal, the active voice
//! and positive polarity are the defaults, and a row saying so states
//! nothing the event does not already say. So a frame grounds 0 to 5 feature
//! rows, which is the range Property 26 is generated over.
//!
//! **One call grounds one frame.** A clause filling a role is an event in
//! its own right, so its role row names it rather than unfolding it:
//! [`nested`] gives that sub-event its name, and grounding it is a second
//! call under that name. An attribute of a filler (`suit{colour: blue}`)
//! belongs to the filler as an entity, not to this event, for the same
//! reason.
//!
//! | file | what |
//! |---|---|
//! | `canon` | the [`Canon`](phical::keyboard::Canon) forms: `Act`, `Frame`, [`Row`], [`Word`] |
//! | `write` | the frame body's canonical bytes, with a clause minted as a handle |
//! | `read` | the same bytes back, over a cursor that refuses a short or long form |

pub mod canon;
mod read;
mod write;

#[cfg(test)]
mod tests;

pub use canon::{mint, Row, Word};

use crate::ir::{Act, Aspect, Filler, Frame, Polarity, Role, Tense, Voice};

/// The Act as rows about `event`: the predicate, then each role in the
/// frame's own slot order, then each marked feature. Exactly
/// `1 + roles + features` rows, every one of them about the event (R5.12).
pub fn ground(act: &Act, event: &str) -> Vec<phiecs::Row> {
    let frame = &act.frame;
    let mut out = Vec::with_capacity(1 + frame.slots.len() + 5);
    out.push(phiecs::Row::new(event, &act.force.name().to_lowercase(), &frame.pred));
    for (role, filler) in &frame.slots {
        out.push(phiecs::Row::new(event, role.name(), &other(event, role, filler)));
    }
    for (feature, value) in features(frame) {
        out.push(phiecs::Row::new(event, feature, value));
    }
    out
}

/// The frame's marked features, in the order the rows carry them: a feature
/// is marked when it departs from its default, since the default says
/// nothing the event does not already say. 0 to 5 of them.
pub fn features(frame: &Frame) -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::with_capacity(5);
    if frame.tense != Tense::default() {
        out.push(("tense", frame.tense.name()));
    }
    if frame.aspect != Aspect::default() {
        out.push(("aspect", frame.aspect.name()));
    }
    if let Some(modal) = frame.modal {
        out.push(("modal", modal.name()));
    }
    if frame.voice != Voice::default() {
        out.push(("voice", frame.voice.name()));
    }
    if frame.polarity != Polarity::default() {
        out.push(("polarity", frame.polarity.name()));
    }
    out
}

/// The name a clause filling `role` of `event` is grounded under: the
/// sub-event is an entity too, and this is the only name both calls agree on
/// without either one reading the other's rows.
pub fn nested(event: &str, role: &Role) -> String {
    format!("{event}/{}", role.name())
}

/// What a role row's third field holds: the filler, named. A thing is its
/// head, a name, a pronoun and a bare word are themselves, a hole is the
/// question it asks, and a clause is the sub-event [`nested`] names.
fn other(event: &str, role: &Role, filler: &Filler) -> String {
    match filler {
        Filler::Thing(thing) => thing.head.clone(),
        Filler::Name(s) | Filler::Pronoun(s) | Filler::Word(s) => s.clone(),
        Filler::Hole(wh) => format!("?{}", wh.name()),
        Filler::Clause(_) => nested(event, role),
    }
}
