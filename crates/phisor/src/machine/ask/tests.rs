use super::*;
use crate::ir::{Attr, Role, Tense, Thing, Wh};

fn king_ask() -> Act {
    let king = Thing::new("king")
        .with(Attr::Order, Filler::word("first"))
        .with(Attr::Via("of".into()), Filler::name("France"));
    Act::new(
        Force::Ask,
        Frame::new("be", Tense::Past)
            .with(Role::Theme, Filler::Hole(Wh::Who))
            .with(Role::As, Filler::Thing(king)),
    )
}

fn row(about: &str, kind: &str, other: &str) -> phiecs::Row {
    phiecs::Row::new(about, kind, other)
}

#[test]
fn the_first_king_of_france_is_answered_by_the_documented_algebra() {
    let rows = vec![
        row("clovis", "is-a", "king"),
        row("clovis", "of", "France"),
        row("clovis", "order", "first"),
        row("charles", "is-a", "king"),
        row("charles", "of", "France"),
        row("charles", "order", "second"),
        row("pepin", "is-a", "man"),
        row("pepin", "of", "France"),
        row("pepin", "order", "first"),
    ];

    assert_eq!(
        answer(&king_ask(), &rows),
        Ok(Answered {
            members: vec!["clovis".into()],
            count: 1,
        })
    );
}

#[test]
fn no_shared_referent_returns_the_empty_class() {
    let rows = vec![
        row("clovis", "is-a", "king"),
        row("clovis", "of", "France"),
        // The requested `order` Kind is absent. ASK treats that relation as
        // empty without changing phisor-rows' direct unnamed-Kind error.
    ];
    assert_eq!(
        answer(&king_ask(), &rows),
        Ok(Answered {
            members: Vec::new(),
            count: 0,
        })
    );
}

#[test]
fn the_query_is_a_lambda_using_only_the_required_set_path() {
    let filters = constraints(&king_ask()).expect("the question has one hole and three filters");
    let form = define(query(&filters));
    let mut heads = Vec::new();
    collect_heads(&form, &mut heads);

    assert_eq!(heads.iter().filter(|&&name| name == "domain").count(), 1);
    assert_eq!(heads.iter().filter(|&&name| name == "meet").count(), 2);
    assert_eq!(heads.iter().filter(|&&name| name == "identity").count(), 3);
    assert_eq!(heads.iter().filter(|&&name| name == "referents").count(), 3);
    assert_eq!(heads.iter().filter(|&&name| name == "phisor-rows").count(), 3);
    assert_eq!(heads.iter().filter(|&&name| name == "lambda").count(), 1);
}

fn collect_heads<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    if let Value::List(items) = value {
        if let Some(Value::Sym(head)) = items.first() {
            out.push(head);
        }
        for item in items {
            collect_heads(item, out);
        }
    }
}

#[test]
fn only_an_ask_with_one_hole_and_a_fixed_descriptor_is_accepted() {
    let fixed = Frame::new("be", Tense::Present)
        .with(Role::Theme, Filler::Hole(Wh::Who))
        .with(Role::As, Filler::thing("king"));
    assert_eq!(
        answer(&Act::new(Force::Assert, fixed.clone()), &[]),
        Err(AskError::NotAsk {
            given: Force::Assert,
        })
    );
    assert_eq!(
        answer(
            &Act::new(
                Force::Ask,
                Frame::new("be", Tense::Present).with(Role::As, Filler::thing("king")),
            ),
            &[],
        ),
        Err(AskError::HoleCount { given: 0 })
    );
    assert_eq!(
        answer(
            &Act::new(
                Force::Ask,
                fixed.with(Role::Agent, Filler::Hole(Wh::What)),
            ),
            &[],
        ),
        Err(AskError::HoleCount { given: 2 })
    );
    assert_eq!(
        answer(
            &Act::new(
                Force::Ask,
                Frame::new("be", Tense::Present)
                    .with(Role::Theme, Filler::Hole(Wh::Who)),
            ),
            &[],
        ),
        Err(AskError::NoConstraints)
    );
}
