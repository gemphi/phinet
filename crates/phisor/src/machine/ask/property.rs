use std::collections::BTreeSet;

use phiecs::quest::{Oracle, OracleError};
use phistd::gen::{for_all, Gen, Seeded, CASES};

use super::*;
use crate::ir::{Attr, Role, Tense, Thing, Wh};

const ENTITIES: [&str; 12] = [
    "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "e10", "e11",
];
const KINDS: [&str; 6] = ["is-a", "colour", "of", "order", "near", "used-for"];
const VALUES: [&str; 8] = [
    "king", "blue", "France", "first", "river", "tool", "animal", "round",
];

/// A filter retained independently of machine::ask's private Constraint.
/// The model oracle reads these generated pairs directly rather than asking
/// the implementation how it interpreted the Act.
#[derive(Clone, Debug)]
struct Filter {
    kind: String,
    other: String,
}

/// One row set (0..=200 rows), one ASK with a hole in a present role, and
/// the filters from which that ASK was built.
#[derive(Debug)]
struct Case {
    rows: Vec<phiecs::Row>,
    act: Act,
    filters: Vec<Filter>,
}

impl Gen for Case {
    fn draw(g: &mut Seeded) -> Self {
        let count = 1 + g.below(4) as usize;
        let offset = g.below(KINDS.len() as u64) as usize;
        let mut filters = Vec::with_capacity(count);
        for i in 0..count {
            let kind = KINDS[(offset + i) % KINDS.len()];
            let other = VALUES[g.below(VALUES.len() as u64) as usize];
            filters.push(Filter {
                kind: kind.to_string(),
                other: other.to_string(),
            });
        }

        // Half the cases use the documented Thing shape (`is-a` plus
        // attributes); half use fixed role fillers directly. Both lower to
        // the same generated filter list.
        let thing_shape = g.below(2) == 0;
        if thing_shape {
            filters[0].kind = "is-a".to_string();
        }
        let hole_roles = [
            Role::Agent,
            Role::Theme,
            Role::Recipient,
            Role::As,
            Role::Via("where".into()),
        ];
        let hole = g.pick(&hole_roles).clone();
        let mut frame = Frame::new("be", Tense::Past).with(hole, Filler::Hole(Wh::Who));
        if thing_shape {
            let mut thing = Thing::new(&filters[0].other);
            for filter in filters.iter().skip(1) {
                thing = thing.with(
                    Attr::Kind(filter.kind.clone()),
                    Filler::word(&filter.other),
                );
            }
            frame = frame.with(Role::As, Filler::Thing(thing));
        } else {
            for filter in &filters {
                frame = frame.with(
                    Role::Via(filter.kind.clone()),
                    Filler::word(&filter.other),
                );
            }
        }
        let act = Act::new(Force::Ask, frame);

        // One case in three plants a relation with no matching pair, so the
        // empty class is exercised even when other random rows happen to
        // intersect. Other cases plant one witness shared by every filter.
        let empty = g.below(3) == 0;
        let blocked = if empty { Some(filters.len() - 1) } else { None };
        let reserve = if empty { 0 } else { filters.len() };
        let random_count = g.below((201 - reserve) as u64) as usize;
        let mut rows = Vec::with_capacity(random_count + reserve);
        for _ in 0..random_count {
            let entity = *g.pick(&ENTITIES);
            let selected = g.below(filters.len() as u64) as usize;
            let filter = &filters[selected];
            let mut other = if g.below(2) == 0 {
                filter.other.clone()
            } else {
                format!("value{}", g.below(12))
            };
            if blocked == Some(selected) && other == filter.other {
                other.push_str("-other");
            }
            rows.push(phiecs::Row::new(entity, &filter.kind, &other));
        }
        if !empty {
            for filter in &filters {
                rows.push(phiecs::Row::new("witness", &filter.kind, &filter.other));
            }
        }
        Case { rows, act, filters }
    }
}

/// The model-based oracle: for each filter, collect matching `about` values
/// directly from the raw rows, then intersect those sets with plain Rust set
/// membership. It calls no phiclm function and shares no query-builder code
/// with the implementation.
fn direct(case: &Case) -> BTreeSet<String> {
    let mut answer: Option<BTreeSet<String>> = None;
    for filter in &case.filters {
        let matching: BTreeSet<String> = case
            .rows
            .iter()
            .filter(|row| row.kind == filter.kind && row.other == filter.other)
            .map(|row| row.about.clone())
            .collect();
        answer = Some(match answer {
            None => matching,
            Some(current) => current
                .into_iter()
                .filter(|entity| matching.contains(entity))
                .collect(),
        });
    }
    answer.unwrap_or_default()
}

/// Installed for every generated case. Any external consultation fails the
/// property immediately instead of quietly returning convenient test data.
struct PanicOracle;

impl Oracle for PanicOracle {
    fn ask(&mut self, prompt: &str) -> Result<String, OracleError> {
        panic!("external Oracle was consulted for local ASK: {prompt}")
    }

    fn name(&self) -> String {
        "panic-if-consulted".to_string()
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 20: An ASK is
// answered by the set algebra alone. The model oracle computes the same
// intersection directly in Rust, and an installed external Oracle panics if
// consulted.
//
// **Validates: Requirements 7.11**
#[test]
fn property_20_an_ask_is_answered_by_the_set_algebra_alone() {
    let run = for_all::<Case>(0x20, CASES, |case| {
        let expected = direct(case);
        let mut external = PanicOracle;
        let answered = match answer_with_oracle(&case.act, &case.rows, &mut external) {
            Ok(answered) => answered,
            Err(_) => return false,
        };
        let actual: BTreeSet<String> = answered.members.iter().cloned().collect();
        actual == expected
            && answered.count == expected.len()
            && answered.count == answered.members.len()
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
