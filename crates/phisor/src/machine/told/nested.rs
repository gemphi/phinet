//! A hole inside a clause: "where does sarah want to go?" asks about the
//! sub-event under `want`'s theme, not about the wanting.
//!
//! The question is read as levels: the top frame with its fixed roles, then
//! each clause on the way down with the role it fills in its parent, until
//! the frame that holds the hole (or the deepest clause, for a yes/no). The
//! query descends the same way: the events the top level names; under them,
//! through the role's rows, the sub-events that meet the clause's own roles
//! (`told-under`); and at the bottom the hole's role read off, or whether
//! there are any (`told-answer-of`, `told-whether-of`). Every step is a card
//! in `data/ops/core.lisp`; `ground::unfold` wrote the rows it reads.

use phiclm::lisp::Value;

use crate::ground::CLAUSE;
use crate::ir::{Act, Filler, Force, Frame, Role, Wh};

use super::super::ask::{call, relation, AskError, Constraint};

/// The role a question's hole fills, and whether it is a `where` under a
/// preposition, which any preposition role may answer.
pub(super) struct Hole {
    pub role: String,
    pub anywhere: bool,
}

/// One frame on the way to the hole: its predicate, its fixed roles, and
/// the role it fills in its parent (`None` at the top).
pub(super) struct Level {
    pub pred: String,
    pub filters: Vec<Constraint>,
    pub under: Option<String>,
}

/// The levels from the act down to the frame that holds the hole, and the
/// hole; no hole for a yes/no question. A clause beside a hole at the same
/// level is passed over, as before.
pub(super) fn levels(act: &Act) -> Result<(Vec<Level>, Option<Hole>), AskError> {
    if act.force != Force::Ask {
        return Err(AskError::NotAsk { given: act.force });
    }
    let mut out = Vec::new();
    let hole = descend(&act.frame, None, &mut out)?;
    Ok((out, hole))
}

fn descend(frame: &Frame, under: Option<String>, out: &mut Vec<Level>) -> Result<Option<Hole>, AskError> {
    let mut filters = Vec::new();
    let mut holes = Vec::new();
    let mut clause: Option<(String, &Frame)> = None;
    for (role, filler) in &frame.slots {
        let other = match filler {
            Filler::Hole(wh) => {
                holes.push(Hole { role: role.name().to_string(), anywhere: matches!(wh, Wh::Where) && matches!(role, Role::Via(_)) });
                continue;
            }
            Filler::Clause(inner) => {
                clause = clause.or(Some((role.name().to_string(), inner)));
                continue;
            }
            Filler::Thing(thing) => thing.head.clone(),
            Filler::Name(s) | Filler::Pronoun(s) | Filler::Word(s) => s.clone(),
        };
        filters.push(Constraint { kind: role.name().to_string(), other });
    }
    out.push(Level { pred: frame.pred.clone(), filters, under });
    match (holes.len(), clause) {
        (1, _) => Ok(Some(holes.remove(0))),
        (0, Some((role, inner))) => descend(inner, Some(role), out),
        (0, None) => Ok(None),
        (n, _) => Err(AskError::HoleCount { given: n }),
    }
}

/// The kinds a nested query reads, so each may be declared before the run.
pub(super) fn kinds(levels: &[Level]) -> Vec<&str> {
    let mut out: Vec<&str> = levels.iter().flat_map(|l| l.filters.iter().map(|f| f.kind.as_str()).chain(l.under.as_deref())).collect();
    if levels.len() > 1 {
        out.push(CLAUSE);
    }
    out
}

/// The query for two or more levels: `(events …)` at the top, `told-under`
/// for each clause, and the hole's roles read off the bottom, or whether
/// the bottom holds any event.
pub(super) fn query(levels: &[Level], under_forces: Value, roles: &[String]) -> Value {
    let top = &levels[0];
    let mut rels = vec![under_forces];
    rels.extend(top.filters.iter().map(relation));
    let mut events = call("events", vec![call("list", rels)]);
    for level in &levels[1..] {
        let role = level.under.clone().unwrap_or_default();
        let mut inner = vec![relation(&Constraint { kind: CLAUSE.into(), other: level.pred.clone() })];
        inner.extend(level.filters.iter().map(relation));
        let role_rows = call("list", vec![call("phisor-rows", vec![Value::Str(role)])]);
        events = call("told-under", vec![events, role_rows, call("list", inner)]);
    }
    match roles.is_empty() {
        false => {
            let rows: Vec<Value> = roles.iter().map(|r| call("phisor-rows", vec![Value::Str(r.clone())])).collect();
            call("told-answer-of", vec![call("list", rows), events])
        }
        true => call("told-whether-of", vec![events]),
    }
}
