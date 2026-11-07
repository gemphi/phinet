use super::*;
use crate::ground::ground;
use crate::ir::{Frame, Role, Tense, Thing, Wh};
use crate::ladder::read;

/// Told two of the guide's own sentences in English, it answers questions
/// about them in English: the reader makes each an act, `ground` makes the
/// told ones rows about an event, and the question is answered over those
/// events. (The reader as it stands does not cover "a boy was wearing a
/// white cap": "boy" is outside its unprimed words, and primed it refuses
/// "wearing" as noun-and-verb. That gap is the reader's, recorded in
/// docs/211, not hidden here.)
#[test]
fn told_english_it_answers_english_about_what_it_was_told() {
    let story = ["sarah is driving to work.", "tom was reading a book.", "sarah lives in london."];
    let mut rows = Vec::new();
    for (i, s) in story.iter().enumerate() {
        let act = read(s).unwrap_or_else(|| panic!("the reader covers {s:?}"));
        rows.extend(ground(&act, &format!("e{}", i + 1)));
    }
    let answers = |q: &str| {
        let act = read(q).unwrap_or_else(|| panic!("the reader covers {q:?}"));
        answer_told(&act, &rows).unwrap_or_else(|e| panic!("{q:?}: {e}")).members
    };
    // "to work": the reader puts a where-hole under "in", the sentence put
    // its place under "to"; a where is read off every preposition role.
    assert_eq!(answers("where is sarah driving?"), ["work"]);
    assert_eq!(answers("what was tom reading?"), ["book"]);
    assert_eq!(answers("where does sarah live?"), ["london"]);
}

/// Built by hand, so the algebra is tested apart from the reader: three
/// events, and the hole's role projected over the ones the fixed roles
/// select.
#[test]
fn the_hole_is_projected_over_the_events_the_fixed_roles_name() {
    let rows = vec![
        phiecs::Row::new("e1", "assert", "wear"),
        phiecs::Row::new("e1", "agent", "girl"),
        phiecs::Row::new("e1", "theme", "suit"),
        phiecs::Row::new("e2", "assert", "wear"),
        phiecs::Row::new("e2", "agent", "boy"),
        phiecs::Row::new("e2", "theme", "cap"),
        phiecs::Row::new("e3", "assert", "eat"),
        phiecs::Row::new("e3", "agent", "girl"),
        phiecs::Row::new("e3", "theme", "apple"),
    ];
    let ask = |frame: Frame| Act::new(Force::Ask, frame);
    let what_girl_wore = ask(Frame::new("wear", Tense::Past).with(Role::Agent, Filler::Name("girl".into())).with(Role::Theme, Filler::Hole(Wh::What)));
    assert_eq!(answer_told(&what_girl_wore, &rows).unwrap().members, ["suit"]);
    let who_wore_cap = ask(Frame::new("wear", Tense::Past).with(Role::Agent, Filler::Hole(Wh::Who)).with(Role::Theme, Filler::Thing(Thing::new("cap"))));
    assert_eq!(answer_told(&who_wore_cap, &rows).unwrap().members, ["boy"]);
    // Nobody was told a wearing of a hat: an empty class, not an error.
    let who_wore_hat = ask(Frame::new("wear", Tense::Past).with(Role::Agent, Filler::Hole(Wh::Who)).with(Role::Theme, Filler::Thing(Thing::new("hat"))));
    assert_eq!(answer_told(&who_wore_hat, &rows).unwrap().count, 0);
    // Two holes, or no hole, is refused.
    let two = ask(Frame::new("wear", Tense::Past).with(Role::Agent, Filler::Hole(Wh::Who)).with(Role::Theme, Filler::Hole(Wh::What)));
    assert_eq!(answer_told(&two, &rows), Err(AskError::HoleCount { given: 2 }));
}

