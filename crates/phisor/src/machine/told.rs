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
//! same four named set operations `ask` uses plus `image` and `union` -
//! and since the cards (`machine::ops`), the lambda is `told-answer` in
//! `data/ops/core.lisp`, applied to the roles' rows and the fixed roles'
//! relations: the operation is a card in a file, not a shape built here.
//!
//! A `where` is read off every preposition role the events carry, not only
//! the one the reader guessed: "sarah is driving to work" put her place
//! under `to`, and "where is sarah driving?" puts its hole under `in`. A
//! preposition role is told structurally — a kind that is not one of the
//! four named roles, a feature or a force — never from a list of words.
//!
//! The predicate is matched under every force the store holds - told
//! "i wish i had a lot of money", the wish's `have` answers "what did i
//! have?" - and under [`CLAUSE`], so a sub-event unfolded by
//! `ground::unfold` is found too. An answer that is itself an event (a
//! clause filling the hole's role) is said in English again, through
//! `ground::unground` and the realiser, rather than given as a name.

use phical::keyboard::Arena;
use phiclm::lisp::{Env, Value};

use crate::ground::{is_event, unground, CLAUSE, FEATURES};
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
pub fn answer_told_kinds(act: &Act, kinds: KindRows) -> Result<Answered, AskError> {
    answer_told_traced_kinds(act, kinds, None).map(|(answered, _)| answered)
}

/// As [`answer_told_kinds`], with Note G's trace beside the answer, and
/// optionally a `card` — a `(define told-answer (lambda …))` — that replaces
/// the shipped card for this run (`engine::arrange` tries several).
pub fn answer_told_traced_kinds(act: &Act, mut kinds: KindRows, card: Option<&str>) -> Result<(Answered, Vec<super::engine::Step>), AskError> {
    let (pred, filters, hole) = events_and_hole(act)?;
    let mut roles = vec![hole.role.clone()];
    if hole.anywhere {
        roles.extend(preposition_roles(&kinds).into_iter().filter(|r| *r != hole.role));
    }
    // The predicate under every force the store holds, and under a clause.
    let mut forces: Vec<String> = force_kinds(&kinds);
    if forces.is_empty() {
        forces.push("assert".into());
    }
    for kind in filters.iter().map(|f| f.kind.as_str()).chain(roles.iter().map(String::as_str)).chain(forces.iter().map(String::as_str)) {
        if kinds.rows(kind).is_err() {
            kinds = kinds.told(kind, &[]);
        }
    }
    let host = PhisorHost::new(Arena::new(ARENA_SEED), kinds).map_err(|e| AskError::Host(e.to_string()))?;
    let mut machine = checked(host).map_err(|e| AskError::Host(e.to_string()))?;
    let mut environment = Env::new();
    super::ops::load(&mut machine, &mut environment).map_err(AskError::Machine)?;
    if let Some(card) = card {
        for form in phiclm::lisp::read(card).map_err(AskError::Machine)? {
            machine.eval(&form, &mut environment).map_err(AskError::Machine)?;
        }
    }
    machine.eval(&define_told(query_told(&pred, &forces, &filters, &roles)), &mut environment).map_err(AskError::Machine)?;
    let value = machine.eval(&call(BINDING, Vec::new()), &mut environment).map_err(AskError::Machine)?;
    let trace = machine.host.take_trace();
    match value {
        Value::Class(class) => {
            let rows = machine.host.kinds().all();
            let members: Vec<String> = class.into_iter().map(|m| said(&m, &rows)).collect();
            let count = members.len();
            Ok((Answered { members, count }, trace))
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

/// An answer that is an event the rows describe, said in English through
/// the realiser; any other answer is itself.
fn said(member: &str, rows: &[phiecs::Row]) -> String {
    match is_event(member, rows) {
        // The realiser says a statement; an answer is a phrase, so the stop goes.
        true => unground(member, rows).map(|act| crate::ladder::realise(&act).trim_end_matches('.').to_string()).unwrap_or_else(|| member.to_string()),
        false => member.to_string(),
    }
}

/// The force kinds the store holds, and [`CLAUSE`] when it holds one: the
/// kinds a told predicate may sit under.
fn force_kinds(kinds: &KindRows) -> Vec<String> {
    kinds
        .names()
        .into_iter()
        .filter(|k| *k == CLAUSE || Force::ALL.iter().any(|f| f.name().eq_ignore_ascii_case(k)))
        .map(str::to_string)
        .collect()
}

/// The predicate, the fixed roles as constraints on the event - each role
/// with its filler - and the one role the hole fills.
fn events_and_hole(act: &Act) -> Result<(String, Vec<Constraint>, Hole), AskError> {
    if act.force != Force::Ask {
        return Err(AskError::NotAsk { given: act.force });
    }
    let mut filters = Vec::new();
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
        1 => Ok((act.frame.pred.clone(), filters, holes.remove(0))),
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
        .filter(|k| !named.contains(k) && !FEATURES.contains(k) && *k != CLAUSE && !forces.iter().any(|f| f == k))
        .map(str::to_string)
        .collect()
}

/// `(told-answer (list (phisor-rows role)…) (list (identity (referents …))…))`:
/// the card in `data/ops/core.lisp`, which is
/// `image(union-all(roles), domain(meet-all(relations)))`.
fn query_told(pred: &str, forces: &[String], filters: &[Constraint], roles: &[String]) -> Value {
    let under: Vec<Value> = forces.iter().map(|f| relation(&Constraint { kind: f.clone(), other: pred.to_string() })).collect();
    let mut relations: Vec<Value> = vec![call("union-all", vec![call("list", under)])];
    relations.extend(filters.iter().map(relation));
    let rows: Vec<Value> = roles.iter().map(|r| call("phisor-rows", vec![Value::Str(r.clone())])).collect();
    call("told-answer", vec![call("list", rows), call("list", relations)])
}

fn define_told(body: Value) -> Value {
    let lambda = Value::List(vec![Value::Sym("lambda".into()), Value::Nil, body]);
    Value::List(vec![Value::Sym("define".into()), Value::Sym(BINDING.into()), lambda])
}

#[cfg(test)]
mod tests;
