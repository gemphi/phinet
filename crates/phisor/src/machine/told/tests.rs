use super::*;
use super::super::answer_usage;
use crate::ground::{ground, unfold};
use crate::ir::{Filler, Frame, Role, Tense, Thing, Wh};
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


/// A told predicate is found under whatever force it was told with, and a
/// clause filling the hole's role is said in English, not named.
#[test]
fn any_force_and_a_clause_said_in_english() {
    use crate::ground::CLAUSE;
    // Told by hand: a wish, and a want whose theme is a clause.
    let wish = Act::new(Force::Wish, Frame::new("have", Tense::Past).with(Role::Agent, Filler::pronoun("i")).with(Role::Theme, Filler::word("money")));
    let want = Act::new(
        Force::Assert,
        Frame::new("want", Tense::Present)
            .with(Role::Agent, Filler::Name("sarah".into()))
            .with(Role::Theme, Filler::clause(Frame::new("go", Tense::Present).with(Role::Via("to".into()), Filler::word("home")))),
    );
    let mut rows = unfold(&wish, "e1");
    rows.extend(unfold(&want, "e2"));
    assert!(rows.iter().any(|r| r.about == "e2/theme" && r.kind == CLAUSE && r.other == "go"), "the clause is unfolded: {rows:?}");
    let ask = |frame: Frame| Act::new(Force::Ask, frame);
    // The wish's predicate answers a question that does not say "wish".
    let what_had = ask(Frame::new("have", Tense::Past).with(Role::Agent, Filler::pronoun("i")).with(Role::Theme, Filler::Hole(Wh::What)));
    assert_eq!(answer_told(&what_had, &rows).unwrap().members, ["money"]);
    // The want's theme is a sub-event, said in English.
    let what_wants = ask(Frame::new("want", Tense::Present).with(Role::Agent, Filler::Name("sarah".into())).with(Role::Theme, Filler::Hole(Wh::What)));
    let said = answer_told(&what_wants, &rows).unwrap().members;
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(said[0].contains("go") && said[0].contains("home") && !said[0].contains("e2/") && !said[0].ends_with('.'), "the clause in English, not its name: {said:?}");
}

/// A question with no hole is answered yes or no: whether the events its
/// roles name are more than none among what was told (the `told-whether`
/// card, the operation the mill-usage count found missing).
#[test]
fn a_yes_no_question_is_membership_over_what_was_told() {
    let mut rows = unfold(&read("sarah is driving to work.").unwrap(), "e1");
    rows.extend(unfold(&read("tom was reading a book.").unwrap(), "e2"));
    let whether = |q: &str| answer_told(&read(q).expect(q), &rows).unwrap().members;
    assert_eq!(whether("is sarah driving to work?"), ["yes"]);
    assert_eq!(whether("was tom reading a book?"), ["yes"]);
    assert_eq!(whether("was tom reading a letter?"), ["no"], "not among what was told");
    assert_eq!(whether("is tom driving to work?"), ["no"]);
    let run = answer_usage(&read("is sarah driving to work?").unwrap(), &rows).unwrap();
    assert!(run.usage.cards.contains_key("told-whether") && !run.usage.cards.contains_key("told-answer"), "{:?}", run.usage.cards);
    assert!(run.usage.operators.is_empty(), "a card over primitives the mill has, not an Operator");
}

/// A hole inside a clause: told "sarah wants to go home", asked the same
/// with "home" a hole, it answers off the sub-event; and whether she wants
/// to go there is yes, elsewhere no. (The reader does not yet read "where
/// does sarah want to go?", so the question is the told act with its hole.)
#[test]
fn a_hole_inside_a_clause_is_read_off_the_sub_event() {
    let want = read("sarah wants to go home.").expect("the reader reads it");
    let mut rows = unfold(&want, "e1");
    if let Some(other) = read("tom wants to go to work.") {
        rows.extend(unfold(&other, "e2"));
    }
    let (role, inner) = want.frame.slots.iter().find_map(|(r, f)| match f {
        Filler::Clause(i) => Some((r.clone(), i.as_ref().clone())),
        _ => None,
    }).expect("a clause fills a role");
    let is_home = |f: &Filler| matches!(f, Filler::Thing(t) if t.head == "home") || matches!(f, Filler::Word(w) | Filler::Name(w) if w == "home");
    assert!(inner.slots.iter().any(|(_, f)| is_home(f)), "home is in the clause: {inner:?}");
    let with = |home: Filler| {
        let mut q = inner.clone();
        for (_, f) in q.slots.iter_mut() {
            if is_home(f) {
                *f = home.clone();
            }
        }
        Act::new(Force::Ask, Frame::new("want", Tense::Present).with(Role::Agent, Filler::Name("sarah".into())).with(role.clone(), Filler::Clause(Box::new(q))))
    };
    assert_eq!(answer_told(&with(Filler::Hole(Wh::Where)), &rows).unwrap().members, ["home"]);
    // And from the English itself, now the reader puts the hole in the clause.
    assert_eq!(answer_told(&read("where does sarah want to go?").expect("read"), &rows).unwrap().members, ["home"]);
    // "to do what" asks for the clause itself, said in English.
    let what_to_do = answer_told(&read("what does sarah want to do?").expect("read"), &rows).unwrap().members;
    assert_eq!(what_to_do.len(), 1, "{what_to_do:?}");
    assert!(what_to_do[0].contains("go") && what_to_do[0].contains("home") && !what_to_do[0].contains("e1/"), "the going, in English: {what_to_do:?}");
    assert_eq!(answer_told(&with(Filler::Word("home".into())), &rows).unwrap().members, ["yes"]);
    assert_eq!(answer_told(&with(Filler::Word("work".into())), &rows).unwrap().members, ["no"]);
    let run = answer_usage(&with(Filler::Hole(Wh::Where)), &rows).unwrap();
    assert!(run.usage.cards.contains_key("told-under"), "{:?}", run.usage.cards);
    assert!(run.usage.operators.is_empty());
}

/// The reader puts "where" on the frame it reads - `want`, for "where does
/// sarah want to go?" - beside the clause. Nothing is under `want`, so the
/// hole moves down into the going and finds home.
#[test]
fn a_hole_beside_a_clause_moves_down_when_its_level_has_nothing() {
    let want = read("sarah wants to go home.").expect("the reader reads it");
    let rows = unfold(&want, "e1");
    let (role, inner) = want.frame.slots.iter().find_map(|(r, f)| match f {
        Filler::Clause(i) => Some((r.clone(), i.as_ref().clone())),
        _ => None,
    }).expect("a clause fills a role");
    let mut bare = inner.clone();
    bare.slots.retain(|(_, f)| !matches!(f, Filler::Thing(t) if t.head == "home") && !matches!(f, Filler::Word(w) | Filler::Name(w) if w == "home"));
    let asked = Act::new(Force::Ask, Frame::new("want", Tense::Present)
        .with(Role::Agent, Filler::Name("sarah".into()))
        .with(role, Filler::Clause(Box::new(bare)))
        .with(Role::named("in"), Filler::Hole(Wh::Where)));
    let run = answer_usage(&asked, &rows).unwrap();
    assert_eq!(run.answered.members, ["home"], "rows {rows:?}
steps {:?}", run.steps);
    assert_eq!(run.usage.cards.get("told-answer"), Some(&1), "tried at its own level first: {:?}", run.usage.cards);
    assert_eq!(run.usage.cards.get("told-under"), Some(&1), "then one level down");
}
