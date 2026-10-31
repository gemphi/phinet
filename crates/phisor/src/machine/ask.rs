//! Answering an `ASK` Act with phiclm's set algebra (Requirement 7.11).
//!
//! A fixed descriptor `(kind, other)` becomes
//! `identity(referents(phisor-rows(kind), other))`. `meet` intersects those
//! identity relations and `domain` returns their shared referents as a
//! class. The expression is the body of a Lisp lambda and is evaluated by
//! phiclm's machine. No quest path and no Oracle answer this question.

use std::fmt;

use phical::keyboard::Arena;
use phiclm::lisp::{Env, Value};
use phiecs::quest::Oracle;

use crate::ir::{Act, Filler, Force, Frame, Thing};

use super::{checked, KindRows, PhisorHost};

const BINDING: &str = "phisor-answer-local";
const ARENA_SEED: u64 = 0x4153_4b;

/// The class that answers an ASK, and its cardinality.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answered {
    pub members: Vec<String>,
    pub count: usize,
}

/// An Act that cannot denote the finite intersection Requirement 7.11 asks
/// phiclm to compute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AskError {
    NotAsk { given: Force },
    HoleCount { given: usize },
    NoConstraints,
    Host(String),
    Machine(String),
}

impl fmt::Display for AskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AskError::NotAsk { given } => {
                write!(f, "ASK required, {} given", given.name())
            }
            AskError::HoleCount { given } => {
                write!(f, "ASK requires exactly one hole, {given} given")
            }
            AskError::NoConstraints => write!(f, "ASK requires at least one fixed descriptor"),
            AskError::Host(reason) => write!(f, "ASK host: {reason}"),
            AskError::Machine(reason) => write!(f, "ASK machine: {reason}"),
        }
    }
}

impl std::error::Error for AskError {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Constraint {
    kind: String,
    other: String,
}

/// Answer `act` from `rows` without an external source.
pub fn answer(act: &Act, rows: &[phiecs::Row]) -> Result<Answered, AskError> {
    answer_kinds(act, KindRows::of(rows))
}

/// The wider runtime may have an external Oracle installed. It is accepted
/// here only so the no-consultation contract can be exercised: ASK follows
/// exactly the same local path as [`answer`], and never calls the Oracle.
pub fn answer_with_oracle(
    act: &Act,
    rows: &[phiecs::Row],
    _external: &mut dyn Oracle,
) -> Result<Answered, AskError> {
    answer(act, rows)
}

/// Answer against a prepared set of Kinds. A relation absent from the set is
/// declared locally with no rows, making its referent class empty; direct
/// calls to `phisor-rows` retain their Requirement 7.2 error for an unnamed
/// Kind.
pub fn answer_kinds(act: &Act, mut kinds: KindRows) -> Result<Answered, AskError> {
    let filters = constraints(act)?;
    for filter in &filters {
        if kinds.rows(&filter.kind).is_err() {
            kinds = kinds.told(&filter.kind, &[]);
        }
    }

    let host = PhisorHost::new(Arena::new(ARENA_SEED), kinds)
        .map_err(|error| AskError::Host(error.to_string()))?;
    let mut machine = checked(host).map_err(|error| AskError::Host(error.to_string()))?;
    let definition = define(query(&filters));
    let mut environment = Env::new();
    machine
        .eval(&definition, &mut environment)
        .map_err(AskError::Machine)?;
    let value = machine
        .eval(&call(BINDING, Vec::new()), &mut environment)
        .map_err(AskError::Machine)?;

    match value {
        Value::Class(class) => {
            let members: Vec<String> = class.into_iter().collect();
            let count = members.len();
            Ok(Answered { members, count })
        }
        other => Err(AskError::Machine(format!(
            "{BINDING} returned {other}, not a class"
        ))),
    }
}

/// Read one direct role filler as a descriptor. A Thing describes the answer
/// itself: its head is `is-a`, and each fixed attribute contributes its own
/// relation, as in `king{order: first, of: France}`. Atomic fillers use their
/// role as the relation. Clauses are walked in the same way.
fn constraints(act: &Act) -> Result<Vec<Constraint>, AskError> {
    if act.force != Force::Ask {
        return Err(AskError::NotAsk { given: act.force });
    }
    let mut filters = Vec::new();
    let mut holes = 0;
    collect_frame(&act.frame, &mut filters, &mut holes);
    if holes != 1 {
        return Err(AskError::HoleCount { given: holes });
    }
    filters.sort();
    filters.dedup();
    if filters.is_empty() {
        return Err(AskError::NoConstraints);
    }
    Ok(filters)
}

fn collect_frame(frame: &Frame, out: &mut Vec<Constraint>, holes: &mut usize) {
    for (role, filler) in &frame.slots {
        match filler {
            Filler::Hole(_) => *holes += 1,
            Filler::Thing(thing) => collect_thing(thing, out, holes),
            Filler::Name(name) | Filler::Pronoun(name) | Filler::Word(name) => {
                push(out, role.name(), name)
            }
            Filler::Clause(inner) => collect_frame(inner, out, holes),
        }
    }
}

fn collect_thing(thing: &Thing, out: &mut Vec<Constraint>, holes: &mut usize) {
    push(out, "is-a", &thing.head);
    for (attribute, filler) in &thing.attrs {
        match filler {
            Filler::Hole(_) => *holes += 1,
            Filler::Thing(nested) => push(out, attribute.name(), &nested.head),
            Filler::Name(name) | Filler::Pronoun(name) | Filler::Word(name) => {
                push(out, attribute.name(), name)
            }
            Filler::Clause(inner) => collect_frame(inner, out, holes),
        }
    }
}

fn push(out: &mut Vec<Constraint>, kind: &str, other: &str) {
    out.push(Constraint {
        kind: kind.to_string(),
        other: other.to_string(),
    });
}

/// `domain(meet(identity(referents(...)), ...))`. A one-filter ASK meets the
/// relation with itself, so every successful ASK goes through all four named
/// set operations without changing its answer.
fn query(filters: &[Constraint]) -> Value {
    let mut relations: Vec<Value> = filters.iter().map(relation).collect();
    let first = relations.remove(0);
    let intersection = if relations.is_empty() {
        call("meet", vec![first.clone(), first])
    } else {
        relations
            .into_iter()
            .fold(first, |left, right| call("meet", vec![left, right]))
    };
    call("domain", vec![intersection])
}

fn relation(filter: &Constraint) -> Value {
    let rows = call(
        "phisor-rows",
        vec![Value::Str(filter.kind.clone())],
    );
    let referents = call(
        "referents",
        vec![rows, Value::Str(filter.other.clone())],
    );
    call("identity", vec![referents])
}

fn define(body: Value) -> Value {
    let lambda = Value::List(vec![Value::Sym("lambda".into()), Value::Nil, body]);
    Value::List(vec![
        Value::Sym("define".into()),
        Value::Sym(BINDING.into()),
        lambda,
    ])
}

fn call(name: &str, arguments: Vec<Value>) -> Value {
    Value::List(
        std::iter::once(Value::Sym(name.to_string()))
            .chain(arguments)
            .collect(),
    )
}

#[cfg(test)]
mod property;
#[cfg(test)]
mod tests;
