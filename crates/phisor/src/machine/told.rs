//! Answering an `ASK` over what was TOLD.
//!
//! `ask` answers a question about a thing: "who was the first king of
//! France?" names its answer by descriptors, `king{order: first, of:
//! France}`, and the answer is the referents those descriptors meet on. But a
//! told sentence is not rows about a thing. `crate::ground` makes it rows
//! about an EVENT: "a girl was wearing a blue suit" is `(e assert wear)`,
//! `(e agent girl)`, `(e theme suit)`, `(e tense past)`. Asked "what was the
//! girl wearing?" — `ASK(wear(agent: girl, theme: ?what), past)` — `ask`
//! would look for a referent with `agent: girl`, and find the event, not the
//! suit.
//!
//! So here the fixed roles name the EVENTS, and the hole's role is read off
//! them: the events asserted to be a wearing whose agent is the girl are the
//! domain of the meet of those roles' identities, exactly as a thing's
//! descriptors are in `ask`; and the answer is the image of the hole's role
//! over those events — `image(rows(theme), events)`. The whole of it is one
//! lambda in phiclm's Lisp, as every language operation is, built from the
//! same four named set operations `ask` uses plus `image` and `union`.
//!
//! A `where` is read off every preposition role the events carry, not only
//! the one the reader guessed: "sarah is driving to work" put her place
//! under `to`, and "where is sarah driving?" puts its hole under `in`. A
//! preposition role is told structurally — a kind that is not one of the
//! four named roles, a feature or a force — never from a list of words.
//!
//! What it does not do yet: a clause filling a role (a sub-event) is passed
//! over, and the told force is taken to be `assert`.

use phical::keyboard::Arena;
use phiclm::lisp::{Env, Value};

use crate::ground::FEATURES;
use crate::ir::{Act, Filler, Force, Role, Wh};

use super::ask::{call, relation, Answered, AskError, Constraint};
use super::{checked, KindRows, PhisorHost};

const BINDING: &str = "phisor-answer-told";
const ARENA_SEED: u64 = 0x544f_4c44;

/// The answer to `act` from `rows` that ground told sentences as events.
pub fn answer_told(act: &Act, rows: &[phiecs::Row]) -> Result<Answered, AskError> {
    answer_told_kinds(act, KindRows::of(rows))
}

/// As [`answer_told`], against a prepared set of Kinds. A relation the rows
/// never named is declared empty, so a question about a role nobody told
/// has an empty class, not an error.
pub fn answer_told_kinds(act: &Act, mut kinds: KindRows) -> Result<Answered, AskError> {
    let (filters, hole) = events_and_hole(act)?;
    let mut roles = vec![hole.role.clone()];
    if hole.anywhere {
        roles.extend(preposition_roles(&kinds).into_iter().filter(|r| *r != hole.role));
    }
    for kind in filters.iter().map(|f| f.kind.as_str()).chain(roles.iter().map(String::as_str)) {
        if kinds.rows(kind).is_err() {
            kinds = kinds.told(kind, &[]);
        }
    }
    let host = PhisorHost::new(Arena::new(ARENA_SEED), kinds).map_err(|e| AskError::Host(e.to_string()))?;
    let mut machine = checked(host).map_err(|e| AskError::Host(e.to_string()))?;
    let mut environment = Env::new();
    machine.eval(&define_told(query_told(&filters, &roles)), &mut environment).map_err(AskError::Machine)?;
    let value = machine.eval(&call(BINDING, Vec::new()), &mut environment).map_err(AskError::Machine)?;
    match value {
        Value::Class(class) => {
            let members: Vec<String> = class.into_iter().collect();
            let count = members.len();
            Ok(Answered { members, count })
        }
        other => Err(AskError::Machine(format!("{BINDING} returned {other}, not a class"))),
    }
}

/// The role a question's hole fills, and whether it is a `where` under a
/// preposition, which any preposition role may answer.
struct Hole {
    role: String,
    anywhere: bool,
}

/// The fixed roles as constraints on the event — each role with its filler,
/// and the predicate under `assert` — and the one role the hole fills.
fn events_and_hole(act: &Act) -> Result<(Vec<Constraint>, Hole), AskError> {
    if act.force != Force::Ask {
        return Err(AskError::NotAsk { given: act.force });
    }
    let mut filters = vec![Constraint { kind: "assert".into(), other: act.frame.pred.clone() }];
    let mut holes = Vec::new();
    for (role, filler) in &act.frame.slots {
        let other = match filler {
            Filler::Hole(wh) => {
                holes.push(Hole { role: role.name().to_string(), anywhere: matches!(wh, Wh::Where) && matches!(role, Role::Via(_)) });
                continue;
            }
            Filler::Thing(thing) => thing.head.clone(),
            Filler::Name(s) | Filler::Pronoun(s) | Filler::Word(s) => s.clone(),
            Filler::Clause(_) => continue,
        };
        filters.push(Constraint { kind: role.name().to_string(), other });
    }
    match holes.len() {
        1 => Ok((filters, holes.remove(0))),
        n => Err(AskError::HoleCount { given: n }),
    }
}

/// The kinds that are preposition roles: not a named role, a feature or a
/// force. Structural, so no list of prepositions lives here.
fn preposition_roles(kinds: &KindRows) -> Vec<String> {
    let named: Vec<&str> = Role::NAMED.iter().map(Role::name).collect();
    let forces: Vec<String> = Force::ALL.iter().map(|f| f.name().to_lowercase()).collect();
    kinds
        .names()
        .into_iter()
        .filter(|k| !named.contains(k) && !FEATURES.contains(k) && !forces.iter().any(|f| f == k))
        .map(str::to_string)
        .collect()
}

/// `image(union(phisor-rows(role)…), domain(meet(identity(referents(…)) …)))`.
fn query_told(filters: &[Constraint], roles: &[String]) -> Value {
    let mut relations: Vec<Value> = filters.iter().map(relation).collect();
    let first = relations.remove(0);
    let met = match relations.is_empty() {
        true => call("meet", vec![first.clone(), first]),
        false => relations.into_iter().fold(first, |l, r| call("meet", vec![l, r])),
    };
    let events = call("domain", vec![met]);
    let mut rows: Vec<Value> = roles.iter().map(|r| call("phisor-rows", vec![Value::Str(r.clone())])).collect();
    let head = rows.remove(0);
    let role = rows.into_iter().fold(head, |l, r| call("union", vec![l, r]));
    call("image", vec![role, events])
}

fn define_told(body: Value) -> Value {
    let lambda = Value::List(vec![Value::Sym("lambda".into()), Value::Nil, body]);
    Value::List(vec![Value::Sym("define".into()), Value::Sym(BINDING.into()), lambda])
}

#[cfg(test)]
mod tests;
