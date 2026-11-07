//! A clause unfolded, and rows read back as an act.
//!
//! [`ground`](super::ground) names a clause filling a role and stops: one
//! call, one frame, so the row count holds (R5.12). A told sentence with a
//! clause in it — "sarah wants to go home" — needs the clause's own rows as
//! well, or the question "what does sarah want?" can answer only with the
//! sub-event's name. [`unfold`] grounds the act and then every clause under
//! the name [`nested`](super::nested) gave it, with [`CLAUSE`] for the kind
//! of its predicate row — a clause is not asserted by the speaker, so it
//! does not take the sentence's force.
//!
//! [`unground`] is the way back: the rows about an event, read as an act, so
//! a sub-event the machine answers with can be said in English again. It is
//! structural, like `told`'s reading of a preposition role: a row's kind is
//! a force, a feature, [`CLAUSE`], or else a role; a role's filler that is
//! itself an event the rows describe is a clause.

use crate::ir::{Act, Aspect, Filler, Force, Frame, Modal, Polarity, Role, Tense, Voice};

use super::{features, ground, nested, FEATURES};

/// The kind of a clause's predicate row: the sub-event under its parent's
/// role, with no force of its own.
pub const CLAUSE: &str = "clause";

/// The act as rows about `event`, and every clause in it as rows about its
/// sub-event, to any depth.
pub fn unfold(act: &Act, event: &str) -> Vec<phiecs::Row> {
    let mut out = ground(act, event);
    clauses(&act.frame, event, &mut out);
    out
}

fn clauses(frame: &Frame, event: &str, out: &mut Vec<phiecs::Row>) {
    for (role, filler) in &frame.slots {
        let Filler::Clause(inner) = filler else { continue };
        let sub = nested(event, role);
        out.push(phiecs::Row::new(&sub, CLAUSE, &inner.pred));
        for (role, filler) in &inner.slots {
            let other = match filler {
                Filler::Clause(_) => nested(&sub, role),
                Filler::Thing(thing) => thing.head.clone(),
                Filler::Name(s) | Filler::Pronoun(s) | Filler::Word(s) => s.clone(),
                Filler::Hole(wh) => format!("?{}", wh.name()),
            };
            out.push(phiecs::Row::new(&sub, role.name(), &other));
        }
        for (feature, value) in features(inner) {
            out.push(phiecs::Row::new(&sub, feature, value));
        }
        clauses(inner, &sub, out);
    }
}

/// The force a predicate row's kind names: `ground` writes it in lower case.
fn force_named(kind: &str) -> Option<Force> {
    Force::ALL.into_iter().find(|f| f.name().eq_ignore_ascii_case(kind))
}

/// Whether `rows` hold anything about `event`.
pub fn is_event(event: &str, rows: &[phiecs::Row]) -> bool {
    rows.iter().any(|r| r.about == event)
}

/// The rows about `event`, read back as an act. `None` when no row gives
/// the event a predicate. A clause's force is `assert`, having none of its
/// own.
pub fn unground(event: &str, rows: &[phiecs::Row]) -> Option<Act> {
    let mine: Vec<&phiecs::Row> = rows.iter().filter(|r| r.about == event).collect();
    let (force, pred) = mine.iter().find_map(|r| match r.kind.as_str() {
        CLAUSE => Some((Force::Assert, r.other.clone())),
        kind => force_named(kind).map(|f| (f, r.other.clone())),
    })?;
    let mut frame = Frame::new(&pred, Tense::default());
    for r in &mine {
        match r.kind.as_str() {
            "tense" => frame.tense = Tense::named(&r.other).unwrap_or_default(),
            "aspect" => frame.aspect = Aspect::named(&r.other).unwrap_or_default(),
            "modal" => frame.modal = Modal::named(&r.other),
            "voice" => frame.voice = Voice::named(&r.other).unwrap_or_default(),
            "polarity" => frame.polarity = Polarity::named(&r.other).unwrap_or_default(),
            kind if kind == CLAUSE || force_named(kind).is_some() || FEATURES.contains(&kind) => {}
            role => {
                let filler = match is_event(&r.other, rows) {
                    true => unground(&r.other, rows).map(|a| Filler::Clause(Box::new(a.frame))).unwrap_or_else(|| Filler::Word(r.other.clone())),
                    false => Filler::Word(r.other.clone()),
                };
                frame = frame.with(Role::named(role), filler);
            }
        }
    }
    Some(Act::new(force, frame))
}
