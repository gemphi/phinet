//! Property 17's host half and Property 19, plus the examples that pin each
//! clause of Requirement 7 criteria 2 to 4. The property drivers are
//! `phistd::gen`'s seeded `for_all`, so a failure replays from
//! `(seed, case)` alone.
//!
//! Property 17's other half — that the *lowered* form of an operation is a
//! `define` of a `lambda` carrying 1 to 64 O-code operations — is the
//! lowering's, and the emission bound it rests on is Property 18's in
//! `machine::lower`. What is held here is the shape and the arity: a
//! `define` of a `lambda` of an Operator's declared argument count parses on
//! phiclm's reader and evaluates when applied, over all 64 Operators.

use super::*;
use phical::keyboard::table;
use phiclm::lisp::read;
use phistd::gen::{for_all, Gen, Seeded, CASES};

fn row(about: &str, kind: &str, other: &str) -> phiecs::Row {
    phiecs::Row::new(about, kind, other)
}

/// The fixture Kind set: two Kinds with rows, and one told with none.
fn fixture() -> KindRows {
    KindRows::of(&[
        row("king", "is-a", "man"),
        row("queen", "is-a", "woman"),
        row("sky", "colour", "blue"),
    ])
    .told("empty", &[])
}

/// The three Kind names the fixture holds — the oracle for Property 19.
const HELD: [&str; 3] = ["colour", "empty", "is-a"];

fn host() -> PhisorHost {
    PhisorHost::new(Arena::new(0x11), fixture()).expect("the five phisor- names are free")
}

// ----- the examples -----

#[test]
fn every_host_name_is_prefixed_and_absent_from_the_39() {
    assert_eq!(PhisorHost::core_names().len(), 39);
    for name in NAMES {
        assert!(name.starts_with("phisor-"), "{name} carries no prefix");
    }
    assert_eq!(PhisorHost::check_names(&NAMES), Ok(()));
    assert_eq!(host().names(), &NAMES[..]);
}

#[test]
fn a_kind_holding_no_rows_answers_an_empty_row_sequence() {
    let mut h = host();
    assert_eq!(h.kinds().rows("empty"), Ok(&[][..]));
    assert_eq!(
        h.call("phisor-rows", vec![Value::Str("empty".into())]),
        Ok(Value::List(Vec::new()))
    );
}

#[test]
fn a_kind_the_set_does_not_hold_errors_and_names_it() {
    let mut h = host();
    let refused = h
        .call("phisor-rows", vec![Value::Str("regal".into())])
        .unwrap_err();
    assert!(refused.contains("regal"), "{refused}");
    assert_eq!(h.kinds().rows("regal").unwrap_err(), refused);
    // The set holds the three Kinds told and no fourth.
    assert_eq!(h.kinds().names(), HELD.to_vec());
    assert_eq!(h.kinds().len(), 3);
    assert!(!h.kinds().is_empty());
}

/// A host that shadows a core name — what criterion 4 is written against.
struct Shadow;

impl Host for Shadow {
    fn names(&self) -> &'static [&'static str] {
        &["union", "phisor-rows"]
    }

    fn call(&mut self, _: &'static str, _: Vec<Value>) -> Result<Value, String> {
        unreachable!("a shadowing host builds no machine, so nothing can call it")
    }
}

#[test]
fn a_collision_names_both_and_constructs_no_machine() {
    let Err(refused) = checked(Shadow) else {
        panic!("a shadowed core name must be refused")
    };
    assert_eq!(
        refused,
        HostError::CoreNameCollision {
            primitive: "union",
            core: "union"
        }
    );
    let said = refused.to_string();
    assert!(
        said.contains("union") && said.contains("reserved"),
        "{said}"
    );
    // And the message arrives in place of the assert: the real host's five
    // are free, so the real machine still builds.
    assert!(checked(host()).is_ok());
}

#[test]
fn the_set_algebra_reads_phisor_rows_with_no_conversion() {
    let mut m = checked(host()).expect("the five phisor- names are free");
    // `king is-a man` and `queen is-a woman` are held, so one referent of
    // `man` answers under the Kind `is-a`.
    assert_eq!(
        m.run("(count (referents (phisor-rows \"is-a\") man))"),
        Ok(Value::Num(1.0))
    );
    assert_eq!(
        m.run("(count (phisor-rows \"empty\"))"),
        Ok(Value::Num(0.0))
    );
}

// ----- Property 17: a lambda the machine accepts, and no name collides -----

/// A drawn set of candidate host primitive names, one to five of them: a
/// core name straight from the 39, a case variant of one (`Union` against
/// `union`), a dash variant (`union-`, `-union`, `restrictdomain`), or a
/// `phisor-` name known free.
#[derive(Debug)]
struct Names(Vec<String>);

impl Gen for Names {
    fn draw(g: &mut Seeded) -> Self {
        let count = 1 + g.below(5) as usize;
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            let core = *g.pick(PhisorHost::core_names());
            out.push(match g.below(4) {
                0 => core.to_string(),
                1 => upper_first(core),
                2 => g
                    .pick(&[
                        format!("{core}-"),
                        format!("-{core}"),
                        core.replace('-', ""),
                    ])
                    .clone(),
                _ => format!("phisor-{core}"),
            });
        }
        Names(out)
    }
}

/// `union` as `Union`: the case variant the comparison must keep apart.
fn upper_first(core: &str) -> String {
    let mut rest = core.chars();
    match rest.next() {
        Some(first) => first.to_uppercase().collect::<String>() + rest.as_str(),
        None => String::new(),
    }
}

#[test]
fn property_17_a_lowered_operation_is_a_lambda_and_no_host_name_collides() {
    // The lambda half, exhaustive over all 64 Operators rather than sampled.
    let mut m = checked(host()).expect("the five phisor- names are free");
    for op in table() {
        let params: Vec<String> = (0..op.arity).map(|i| format!("a{i}")).collect();
        let bound = format!("phisor-op-{}", op.index);
        let lowered = format!(
            "(define {bound} (lambda ({}) (count (phisor-rows \"is-a\"))))",
            params.join(" ")
        );

        let forms = read(&lowered).unwrap_or_else(|e| panic!("{} does not parse: {e}", op.name));
        assert_eq!(
            forms.len(),
            1,
            "{} lowered to {} forms",
            op.name,
            forms.len()
        );
        let Value::List(items) = &forms[0] else {
            panic!("{} is not a form", op.name)
        };
        assert_eq!(
            items.first(),
            Some(&Value::Sym("define".into())),
            "{} is no define",
            op.name
        );
        let Some(Value::List(lambda)) = items.get(2) else {
            panic!("{} defines no list", op.name)
        };
        assert_eq!(
            lambda.first(),
            Some(&Value::Sym("lambda".into())),
            "{} is no lambda",
            op.name
        );

        // And the machine accepts it, applied to its declared argument count.
        assert_eq!(
            m.run(&lowered),
            Ok(Value::Sym(bound.clone())),
            "{} did not define",
            op.name
        );
        let args = vec!["0"; op.arity as usize].join(" ");
        assert_eq!(
            m.run(&format!("({bound} {args})")),
            Ok(Value::Num(2.0)),
            "{} did not apply",
            op.name
        );
    }

    // The name half: construction succeeds exactly when the set is disjoint
    // from the 39, compared as whole names with case significant.
    let run = for_all::<Names>(0x17, CASES, |drawn| {
        let names: Vec<&str> = drawn.0.iter().map(String::as_str).collect();
        let reserved = names
            .iter()
            .find(|name| PhisorHost::core_names().contains(name));
        match (PhisorHost::check_names(&names), reserved) {
            (Ok(()), None) => true,
            (Err(HostError::CoreNameCollision { primitive, core }), Some(&first)) => {
                primitive == core && core == first
            }
            _ => false,
        }
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

// ----- Property 19: a named Kind's rows come back, an unnamed one errors -----

/// A drawn Kind name against the fixture set: one the set holds, one
/// differing only in case, one with a trailing dash, and one free.
#[derive(Debug)]
struct Name(String);

impl Gen for Name {
    fn draw(g: &mut Seeded) -> Self {
        let held = *g.pick(&HELD);
        Name(match g.below(4) {
            0 => held.to_string(),
            1 => held.to_uppercase(),
            2 => format!("{held}-"),
            _ => format!("kind{}", g.below(50)),
        })
    }
}

#[test]
fn property_19_a_named_kinds_rows_come_back_and_an_unnamed_one_errors() {
    let run = for_all::<Name>(0x19, CASES, |drawn| {
        let mut h = host();
        let held = HELD.contains(&drawn.0.as_str());
        let direct = h.kinds().rows(&drawn.0).map(|rows| rows.len());
        match (
            direct,
            h.call("phisor-rows", vec![Value::Str(drawn.0.clone())]),
        ) {
            // A Kind the set holds: the same count through both doors, and
            // empty for the Kind told with no rows.
            (Ok(count), Ok(Value::List(items))) => {
                held && items.len() == count && (drawn.0 != "empty" || items.is_empty())
            }
            // A Kind it does not hold: one error, naming the Kind.
            (Err(direct), Err(through)) => !held && direct == through && direct.contains(&drawn.0),
            _ => false,
        }
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
