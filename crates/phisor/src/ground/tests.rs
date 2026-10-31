//! Grounding's tests: Property 26 over generated Acts, and the canonical
//! forms' round trip, interning and two refusals. The property driver is
//! `phistd::gen`'s seeded `for_all`, so a failure replays from
//! `(seed, case)` alone.

use super::*;
use crate::ir::filler::{Attr, Det, Thing, Wh};
use crate::ir::{Force, Modal};
use phical::keyboard::{Arena, Fault, MAX_CLAUSE_DEPTH};
use phistd::gen::{for_all, Gen, Seeded, CASES};

/// The event every case grounds under.
const EVENT: &str = "e1";

const PREDS: [&str; 4] = ["wear", "sing", "go", "be"];
const HEADS: [&str; 4] = ["girl", "suit", "cap", "boy"];

/// The role pool cases draw distinct roles from: the four reserved names and
/// two prepositions of their own. `Role::named` is total, so a preposition
/// reads back as the [`Role::Via`] it was written as.
fn pool() -> Vec<Role> {
    vec![Role::Agent, Role::Theme, Role::Recipient, Role::As, Role::named("in"), Role::named("of")]
}

// ----- Property 26: one Row per predicate, role and feature -----

/// A drawn Act over all seven Forces, with 0 to 4 roles and 0 to 5 marked
/// features, a role's filler sometimes a clause, and clause nesting to
/// depth 3. The two counts are carried beside the Act so the oracle is the
/// generator's own arithmetic and not the module's.
#[derive(Debug)]
struct Case {
    act: Act,
    roles: usize,
    features: usize,
}

impl Gen for Case {
    fn draw(g: &mut Seeded) -> Self {
        let force = *g.pick(&Force::ALL);
        let frame = draw_frame(g, 3);
        let roles = frame.slots.len();
        let features = super::features(&frame).len();
        Case { act: Act::new(force, frame), roles, features }
    }
}

/// A frame with each feature drawn independently, so a case lands on 0
/// marked features and on all 5. `depth` is the clause nesting left.
fn draw_frame(g: &mut Seeded, depth: usize) -> Frame {
    let mut f = Frame::new(g.pick(&PREDS), *g.pick(&Tense::ALL));
    f.aspect = *g.pick(&Aspect::ALL);
    f.modal = match g.below(2) {
        0 => None,
        _ => Some(*g.pick(&Modal::ALL)),
    };
    f.voice = *g.pick(&Voice::ALL);
    f.polarity = *g.pick(&Polarity::ALL);
    // Distinct roles, taken from the pool at a drawn offset: 0 to 4 of 6.
    let pool = pool();
    let count = g.below(5) as usize;
    let offset = g.below(pool.len() as u64) as usize;
    for i in 0..count {
        let role = pool[(offset + i) % pool.len()].clone();
        f.slots.push((role, draw_filler(g, depth)));
    }
    f
}

fn draw_filler(g: &mut Seeded, depth: usize) -> Filler {
    match g.below(6) {
        0 => Filler::Thing(
            Thing::new(g.pick(&HEADS))
                .det(Det::Definite)
                .with(Attr::Kind("colour".to_string()), Filler::word("blue")),
        ),
        1 => Filler::name(g.pick(&HEADS)),
        2 => Filler::pronoun("she"),
        3 => Filler::word("blue"),
        4 => Filler::Hole(*g.pick(&Wh::ALL)),
        _ if depth > 0 => Filler::clause(draw_frame(g, depth - 1)),
        _ => Filler::thing(g.pick(&HEADS)),
    }
}

#[test]
fn property_26_grounding_an_act_emits_one_row_per_predicate_role_and_feature() {
    // Property 26: Grounding an Act emits one Row per predicate, role and
    // feature, with the event as an entity named by the predicate row and by
    // every role row.
    // **Validates: Requirements 5.12**
    let run = for_all::<Case>(0x26, CASES, |c| {
        let rows = ground(&c.act, EVENT);
        // One for the predicate, one per role, one per marked feature.
        if rows.len() != 1 + c.roles + c.features {
            return false;
        }
        // The event is an entity, and every row is about it.
        if !rows.iter().all(|r| r.about == EVENT && r.weight == 1.0) {
            return false;
        }
        // The predicate row names the predicate and carries the force.
        let pred = &rows[0];
        if pred.other != c.act.frame.pred || pred.kind != c.act.force.name().to_lowercase() {
            return false;
        }
        // One row per role, in the frame's own slot order, each naming
        // something.
        let roles = rows[1..1 + c.roles]
            .iter()
            .zip(&c.act.frame.slots)
            .all(|(r, (role, _))| r.kind == role.name() && !r.other.is_empty());
        // One row per marked feature, in the feature order.
        let features = rows[1 + c.roles..]
            .iter()
            .zip(super::features(&c.act.frame))
            .all(|(r, (name, value))| r.kind == name && r.other == value);
        roles && features
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

// ----- the rows a sentence becomes -----

/// "A girl was wearing a blue suit", and a wish with a clause in it.
fn wearing() -> Act {
    let suit = Thing::new("suit")
        .det(Det::Indefinite)
        .with(Attr::Kind("colour".to_string()), Filler::word("blue"));
    Act::new(
        Force::Assert,
        Frame::new("wear", Tense::Past)
            .aspect(Aspect::Progressive)
            .with(Role::Agent, Filler::thing("girl"))
            .with(Role::Theme, Filler::Thing(suit)),
    )
}

#[test]
fn an_act_grounds_as_the_predicate_its_roles_and_its_marked_features() {
    let rows = ground(&wearing(), EVENT);
    let shown: Vec<(&str, &str, &str)> =
        rows.iter().map(|r| (r.about.as_str(), r.kind.as_str(), r.other.as_str())).collect();
    assert_eq!(
        shown,
        vec![
            ("e1", "assert", "wear"),
            ("e1", "agent", "girl"),
            ("e1", "theme", "suit"),
            ("e1", "tense", "past"),
            ("e1", "aspect", "progressive"),
        ]
    );
    // The attribute of a filler is the filler's row, not this event's: five
    // rows, and `colour: blue` is not among them (R5.12's count).
    assert!(rows.iter().all(|r| r.other != "blue"));
}

#[test]
fn a_clause_filling_a_role_is_named_not_unfolded() {
    let act = Act::new(
        Force::Wish,
        Frame::new("want", Tense::Present)
            .with(Role::Agent, Filler::pronoun("I"))
            .with(Role::Theme, Filler::clause(Frame::new("sing", Tense::Present))),
    );
    let rows = ground(&act, EVENT);
    // Two roles, no marked feature: present, simple, active, positive.
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[2].other, nested(EVENT, &Role::Theme));
    assert_eq!(rows[2].other, "e1/theme");
    // The sub-event's own rows come from grounding it under that name.
    let inner = ground(&Act::new(Force::Assert, Frame::new("sing", Tense::Present)), &rows[2].other);
    assert_eq!(inner.len(), 1);
    assert_eq!((inner[0].about.as_str(), inner[0].other.as_str()), ("e1/theme", "sing"));
}

// ----- the canonical forms -----

#[test]
fn an_act_survives_the_arena_and_interns() {
    let mut arena = Arena::new(0x26);
    let act = wearing();
    let handle = mint(&mut arena, &act).expect("the act canonicalises");
    assert_eq!(canon::read::<Act>(&arena, handle).expect("it reads back"), act);
    // Interning: a structurally equal value gets the handle already minted,
    // which is what R6.16's byte identity rests on.
    assert_eq!(mint(&mut arena, &wearing()).unwrap(), handle);
    let asked = Act::new(Force::Ask, wearing().frame);
    assert_ne!(mint(&mut arena, &asked).unwrap(), handle);
}

#[test]
fn a_row_and_a_word_survive_the_arena() {
    let mut arena = Arena::new(0x26);
    let row = Row(phiecs::Row::weighted("e1", "agent", "girl", 0.5));
    let handle = mint(&mut arena, &row).expect("the row canonicalises");
    assert_eq!(canon::read::<Row>(&arena, handle).expect("it reads back"), row);
    let word = Word("girl".to_string());
    let minted = mint(&mut arena, &word).unwrap();
    assert_eq!(canon::read::<Word>(&arena, minted).unwrap(), word);
    // The row's own fields were interned as words, so the head's handle is
    // the one the row already minted.
    assert_eq!(mint(&mut arena, &Word("girl".to_string())).unwrap(), minted);
}

/// An act whose theme is a clause, nested `deep` times.
fn deep(deep: usize) -> Act {
    let mut frame = Frame::new("sing", Tense::Present);
    for _ in 0..deep {
        frame = Frame::new("want", Tense::Present).with(Role::Theme, Filler::clause(frame));
    }
    Act::new(Force::Wish, frame)
}

#[test]
fn a_clause_nested_past_the_cap_is_refused_with_the_depth_and_the_cap() {
    let mut arena = Arena::new(0x26);
    // At the cap the Act canonicalises; one clause further it does not.
    assert!(mint(&mut arena, &deep(MAX_CLAUSE_DEPTH)).is_ok());
    assert_eq!(
        mint(&mut arena, &deep(MAX_CLAUSE_DEPTH + 1)),
        Err(Fault::TooDeep { depth: MAX_CLAUSE_DEPTH + 1, cap: MAX_CLAUSE_DEPTH })
    );
}

#[test]
fn text_the_row_form_would_change_will_not_canonicalise() {
    let mut arena = Arena::new(0x26);
    let mut reason = |s: &str| match mint(&mut arena, &Word(s.to_string())) {
        Err(Fault::NotCanonical { text, reason }) => {
            assert_eq!(text, s);
            reason
        }
        other => panic!("expected {s:?} to be refused, got {other:?}"),
    };
    assert!(reason("").starts_with("empty"));
    assert!(reason(" girl").starts_with("untrimmed"));
    assert!(reason("girl\tagent").starts_with("holds"));
    // A weight the row form would clamp is refused for the same reason: it
    // would not come back as it went in.
    let row = Row(phiecs::Row::weighted("e1", "agent", "girl", 2.0));
    match mint(&mut arena, &row) {
        Err(Fault::NotCanonical { text, .. }) => assert_eq!(text, "2"),
        other => panic!("expected a weight outside 0 to 1 to be refused, got {other:?}"),
    }
}
