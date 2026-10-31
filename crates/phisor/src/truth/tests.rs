//! Properties 37-40 and 43 for the classifier core, plus the example tests
//! that pin each class. The property drivers are `phistd::gen`'s seeded
//! `for_all`, so a failure replays from `(seed, case)` alone.

use super::*;
use phistd::gen::{for_all, Gen, Seeded, CASES};

fn claim(kind: &str, other: &str, weight: f64) -> Claim {
    Claim { kind: kind.to_string(), other: other.to_string(), weight }
}

/// A world holding every `(about, kind, other)` triple given, at full weight.
fn world_of(triples: &[(&str, &str, &str)]) -> World {
    let rows: Vec<phiecs::quest::Row> =
        triples.iter().map(|(a, k, o)| phiecs::quest::Row::new(a, k, o)).collect();
    World::from_rows(&rows)
}

// ----- examples, one per class -----

#[test]
fn an_empty_world_leaves_a_well_formed_claim_unsupported() {
    let mut d = Detector::new(Declared::none());
    assert_eq!(d.classify("cat", &claim("is-a", "mammal", 0.9), &World::new()), Standing::Unsupported);
    // And the unsupported reading carries no agreement value (R8.4).
    assert_eq!(d.seen.last().unwrap().agreement, None);
}

#[test]
fn a_functional_kind_with_a_second_other_contradicts_and_names_the_held_row() {
    let world = world_of(&[("sky", "colour", "blue")]);
    let mut d = Detector::new(Declared::none().functional(&["colour"]));
    let standing = d.classify("sky", &claim("colour", "green", 0.9), &world);
    let Standing::Contradicting { rows } = standing else { panic!("{standing:?}") };
    assert!(rows.contains(&("sky".into(), "colour".into(), "blue".into())), "{rows:?}");
}

#[test]
fn a_one_to_one_kind_with_a_second_about_contradicts() {
    let world = world_of(&[("alice", "capital-of", "france")]);
    let mut d = Detector::new(Declared::none().one_to_one(&["capital-of"]));
    let standing = d.classify("bob", &claim("capital-of", "france", 0.9), &world);
    assert!(matches!(standing, Standing::Contradicting { .. }), "{standing:?}");
}

#[test]
fn a_converse_held_with_a_different_other_contradicts() {
    // `has ⇄ part-of`: `engine part-of car` held says `car has engine`, so a
    // claim `car has wheel` contradicts when has is a one-answer converse.
    let world = world_of(&[("engine", "part-of", "car")]);
    let mut d = Detector::new(Declared::none().converse(&[("has", "part-of")]));
    let standing = d.classify("car", &claim("has", "wheel", 0.9), &world);
    let Standing::Contradicting { rows } = standing else { panic!("{standing:?}") };
    assert!(rows.contains(&("engine".into(), "part-of".into(), "car".into())), "{rows:?}");
}

#[test]
fn an_inherit_edge_closing_a_cycle_contradicts_and_names_the_path() {
    // `cat is-a mammal`, `mammal is-a animal` held; a claim `animal is-a cat`
    // closes the cycle, and every edge on the path is named.
    let world = world_of(&[("cat", "is-a", "mammal"), ("mammal", "is-a", "animal")]);
    let mut d = Detector::new(Declared::none().inherit(&["is-a"]));
    let standing = d.classify("animal", &claim("is-a", "cat", 0.9), &world);
    let Standing::Contradicting { rows } = standing else { panic!("{standing:?}") };
    assert!(rows.contains(&("cat".into(), "is-a".into(), "mammal".into())), "{rows:?}");
    assert!(rows.contains(&("mammal".into(), "is-a".into(), "animal".into())), "{rows:?}");
}

#[test]
fn the_record_names_this_detector_by_crate_and_version() {
    let d = Detector::new(Declared::none());
    assert_eq!(d.name(), format!("phisor::truth v{}", env!("CARGO_PKG_VERSION")));
}

// ----- Property 37: exactly one class, the earliest it qualifies for -----

/// A drawn claim-and-world, built so each precedence rung can be reached.
#[derive(Debug)]
struct Case {
    about: String,
    claim: Claim,
    held: Vec<(String, String, String)>,
    functional: bool,
}

impl Gen for Case {
    fn draw(g: &mut Seeded) -> Self {
        let words = ["cat", "mammal", "animal", "dog", "sky", "blue", "green"];
        let about = g.pick(&words).to_string();
        let kind = g.pick(&["is-a", "colour", "near"]).to_string();
        // One draw in six is unreadable (weight off the scale); one in six is
        // a self-edge; the rest are well-formed.
        let shape = g.below(6);
        let other = if shape == 1 { about.clone() } else { g.pick(&words).to_string() };
        let weight = if shape == 0 { 2.0 } else { g.unit() };
        let held = if g.below(2) == 0 {
            vec![(about.clone(), kind.clone(), g.pick(&words).to_string())]
        } else {
            Vec::new()
        };
        Case { about, claim: Claim { kind, other, weight }, held, functional: true }
    }
}

#[test]
fn property_37_a_claim_gets_exactly_one_earliest_class() {
    // Property 37: A claim gets exactly one class, and the earliest one it
    // qualifies for.
    let run = for_all::<Case>(0x37, CASES, |c| {
        let rows: Vec<phiecs::quest::Row> =
            c.held.iter().map(|(a, k, o)| phiecs::quest::Row::new(a, k, o)).collect();
        let world = World::from_rows(&rows);
        let decl = if c.functional { Declared::none().functional(&["colour", "is-a", "near"]) } else { Declared::none() };
        let mut d = Detector::new(decl);
        let standing = d.classify(&c.about, &c.claim, &world);
        // Exactly one Reading recorded, and its standing is the one returned.
        if d.seen.len() != 1 || d.seen[0].standing != standing {
            return false;
        }
        // Precedence: an unreadable claim is never also degenerate; a
        // degenerate one is never contradicting; the earliest holds.
        let bad_weight = c.claim.weight.is_nan() || !(0.0..=1.0).contains(&c.claim.weight);
        match &standing {
            Standing::Unreadable { .. } => bad_weight,
            Standing::Degenerate { .. } => !bad_weight,
            _ => !bad_weight,
        }
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

// ----- Property 38: a broken Law is contradicting and names the triples -----

#[test]
fn property_38_a_broken_law_is_contradicting_and_names_its_triples() {
    // Property 38: A broken Law is contradicting, and names the triples that
    // broke it.
    let run = for_all::<(u64, u64)>(0x38, CASES, |&(a, b)| {
        let held_other = format!("o{}", a % 7);
        let claim_other = format!("o{}", (b % 7) + 100); // disjoint from held
        let world = world_of(&[("x", "colour", held_other.as_str())]);
        let mut d = Detector::new(Declared::none().functional(&["colour"]));
        let standing = d.classify("x", &claim("colour", &claim_other, 0.5), &world);
        let Standing::Contradicting { rows } = standing else { return false };
        // The held triple that broke the Law is named, and nothing else is.
        rows == vec![("x".into(), "colour".into(), held_other)]
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

// ----- Property 39: unsupported carries no verdict and no agreement -----

#[test]
fn property_39_an_unsupported_claim_carries_no_verdict_and_no_agreement() {
    // Property 39: An unsupported claim carries no verdict and no agreement
    // value — absent, not Some(0.0).
    let run = for_all::<(u64, u64)>(0x39, CASES, |&(a, b)| {
        let kind = ["is-a", "colour", "likes"][(a % 3) as usize];
        let other = format!("w{}", b % 20);
        let mut d = Detector::new(Declared::none());
        let standing = d.classify("subject", &claim(kind, &other, 0.7), &World::new());
        if standing != Standing::Unsupported {
            return false;
        }
        let r = d.seen.last().unwrap();
        r.agreement.is_none() && r.verdict.is_none()
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

// ----- Property 40: a self-edge or empty field is degenerate, recorded -----

#[test]
fn property_40_a_self_edge_or_empty_field_is_degenerate_and_recorded() {
    // Property 40: A self-edge or an empty field is degenerate, and the test
    // that held is recorded.
    let run = for_all::<u64>(0x40, CASES, |&n| {
        let (about, kind, other, want) = match n % 4 {
            0 => ("cat", "is-a", "cat", Degeneracy::SelfEdge), // other == about
            1 => ("cat", "same", "same", Degeneracy::SelfEdge), // kind == other
            2 => ("", "is-a", "mammal", Degeneracy::EmptyField), // empty about
            _ => ("cat", "is-a", "  ", Degeneracy::EmptyField), // empty other
        };
        let mut d = Detector::new(Declared::none());
        let standing = d.classify(about, &claim(kind, other, 0.9), &World::new());
        let Standing::Degenerate { test } = standing else { return false };
        // The test that held is recorded on the Reading too (R8.5).
        test == want && d.seen.last().unwrap().degeneracy == Some(want)
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

// ----- Property 43: unreadable is rejected before any class test -----

#[test]
fn property_43_an_unreadable_claim_is_rejected_before_any_class_test() {
    // Property 43: An unreadable claim is rejected before any class test. A
    // weight off 0..=1, even on a claim that would otherwise be a self-edge
    // or contradict a held Law, reads Unreadable first.
    let run = for_all::<(f64, u64)>(0x43, CASES, |&(w, n)| {
        // Only the off-scale and NaN weights are the unreadable ones here.
        if (0.0..=1.0).contains(&w) {
            return true;
        }
        // Build a world and a declaration that WOULD make this contradict,
        // and an about==other that WOULD make it degenerate, so readability
        // must win to pass.
        let world = world_of(&[("cat", "is-a", "mammal")]);
        let mut d = Detector::new(Declared::none().functional(&["is-a"]));
        let other = if n % 2 == 0 { "cat" } else { "fish" };
        let standing = d.classify("cat", &claim("is-a", other, w), &world);
        matches!(standing, Standing::Unreadable { .. })
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
