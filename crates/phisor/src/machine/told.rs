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
//!
//! A question with no hole - "is sarah driving to work?" - asks whether
//! the events its roles name are more than none: the `told-whether` card,
//! answering `yes` or `no`, where `no` means not among what was told
//! (docs/phisor/runs/mill-usage-2026-11-08, the operation the count found
//! missing).
//!
//! A hole inside a clause - "where does sarah want to go?" - descends:
//! the events the top level names, the sub-events under its role that meet
//! the clause's own roles, and the hole read off the bottom (`nested`,
//! the `told-under` card).

use phical::keyboard::Arena;
use phiclm::lisp::{Env, Host, Value};

use crate::ground::{is_event, unground, CLAUSE, FEATURES};
use crate::ir::{Act, Force, Role};

use super::ask::{call, relation, Answered, AskError, Constraint};
use super::{checked, KindRows, PhisorHost};

mod nested;

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
    answer_told_traced_kinds(act, kinds, None).map(|run| run.answered)
}

/// As [`answer_told_kinds`], with Note G's trace beside the answer, and
/// optionally a `card` — a `(define told-answer (lambda …))` — that replaces
/// the shipped card for this run (`engine::arrange` tries several).
pub fn answer_told_traced_kinds(act: &Act, mut kinds: KindRows, card: Option<&str>) -> Result<super::engine::Run, AskError> {
    let (levels, hole) = nested::levels(act)?;
    let mut roles = Vec::new();
    if let Some(hole) = &hole {
        roles.push(hole.role.clone());
        // A `where` reads every preposition role; inside a clause, every role
        // the question left open ("go home" keeps its place under `theme`).
        if hole.anywhere {
            let more = match levels.len() > 1 {
                true => open_roles(&kinds, &levels[levels.len() - 1].filters),
                false => preposition_roles(&kinds),
            };
            roles.extend(more.into_iter().filter(|r| *r != hole.role));
        }
    }
    // The predicate under every force the store holds, and under a clause.
    let mut forces: Vec<String> = force_kinds(&kinds);
    if forces.is_empty() {
        forces.push("assert".into());
    }
    for kind in nested::kinds(&levels).iter().map(String::as_str).chain(roles.iter().map(String::as_str)).chain(forces.iter().map(String::as_str)) {
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
    // Loading the cards is the machine's birth, not the run: the tally starts here.
    machine.take_tally();
    let top = &levels[0];
    let query = match (levels.len(), &hole) {
        (1, Some(_)) => query_told(&top.pred, &forces, &top.filters, &roles),
        (1, None) => query_whether(&top.pred, &forces, &top.filters),
        _ => nested::query(&levels, under_forces(&top.pred, &forces), &roles),
    };
    machine.eval(&define_told(query), &mut environment).map_err(AskError::Machine)?;
    let mut value = machine.eval(&call(BINDING, Vec::new()), &mut environment).map_err(AskError::Machine)?;
    // Nothing at this level, a clause beside the hole: the hole moves down. A
    // `where` there reads every role of the sub-event the question left open
    // - "go home" keeps its place under `theme`, not a preposition.
    if let (Value::Class(c), Some(deeper), Some(h)) = (&value, nested::deeper(&levels), &hole) {
        if c.is_empty() {
            let roles = match h.anywhere {
                true => open_roles(machine.host.kinds(), &deeper[deeper.len() - 1].filters),
                false => roles.clone(),
            };
            machine.eval(&define_told(nested::query(&deeper, under_forces(&top.pred, &forces), &roles)), &mut environment).map_err(AskError::Machine)?;
            value = machine.eval(&call(BINDING, Vec::new()), &mut environment).map_err(AskError::Machine)?;
        }
    }
    let steps = machine.host.take_trace();
    let usage = super::usage::Usage::of(machine.take_tally(), &steps, machine.host.names());
    match value {
        Value::Class(class) => {
            let rows = machine.host.kinds().all();
            let members: Vec<String> = class.into_iter().map(|m| said(&m, &rows)).collect();
            let count = members.len();
            Ok(super::engine::Run { answered: Answered { members, count }, steps, usage })
        }
        // `told-whether` answers a word: yes, or no.
        Value::Sym(word) => Ok(super::engine::Run { answered: Answered { members: vec![word], count: 1 }, steps, usage }),
        other => Err(AskError::Machine(format!("{BINDING} returned {other}, not a class or a word"))),
    }
}

/// An answer that is an event the rows describe, said in English through
/// the realiser; any other answer is itself. A sub-event that fills a told
/// event's theme is said as that event's complement - "to go home" - and
/// any other event as a statement without its stop, an answer being a phrase.
fn said(member: &str, rows: &[phiecs::Row]) -> String {
    if !is_event(member, rows) {
        return member.to_string();
    }
    let Some(act) = unground(member, rows) else { return member.to_string() };
    let matrix = rows.iter().find(|r| r.other == member && r.kind == Role::Theme.name()).and_then(|r| unground(&r.about, rows));
    match matrix {
        Some(parent) => crate::ladder::realise_complement(&parent.frame, &act.frame),
        None => crate::ladder::realise(&act).trim_end_matches('.').to_string(),
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

/// Every role kind the store holds - named or preposition - that `fixed`
/// does not fix: what a hole moved down into a clause may read.
fn open_roles(kinds: &KindRows, fixed: &[Constraint]) -> Vec<String> {
    let forces: Vec<String> = Force::ALL.iter().map(|f| f.name().to_lowercase()).collect();
    kinds
        .names()
        .into_iter()
        .filter(|k| !FEATURES.contains(k) && *k != CLAUSE && !forces.iter().any(|f| f == k) && !fixed.iter().any(|f| f.kind == *k))
        .map(str::to_string)
        .collect()
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
    let mut relations: Vec<Value> = vec![under_forces(pred, forces)];
    relations.extend(filters.iter().map(relation));
    let rows: Vec<Value> = roles.iter().map(|r| call("phisor-rows", vec![Value::Str(r.clone())])).collect();
    call("told-answer", vec![call("list", rows), call("list", relations)])
}

/// `(told-whether (list relations…))`: yes if the events the roles name are
/// more than none, else no.
fn query_whether(pred: &str, forces: &[String], filters: &[Constraint]) -> Value {
    let mut relations: Vec<Value> = vec![under_forces(pred, forces)];
    relations.extend(filters.iter().map(relation));
    call("told-whether", vec![call("list", relations)])
}

/// `(union-all (list (identity (referents (phisor-rows force) pred))…))`: the
/// predicate's events under every force the store holds.
fn under_forces(pred: &str, forces: &[String]) -> Value {
    let under: Vec<Value> = forces.iter().map(|f| relation(&Constraint { kind: f.clone(), other: pred.to_string() })).collect();
    call("union-all", vec![call("list", under)])
}

fn define_told(body: Value) -> Value {
    let lambda = Value::List(vec![Value::Sym("lambda".into()), Value::Nil, body]);
    Value::List(vec![Value::Sym("define".into()), Value::Sym(BINDING.into()), lambda])
}

#[cfg(test)]
mod tests;
