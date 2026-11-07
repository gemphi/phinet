use super::*;
use crate::machine::{checked, KindRows, PhisorHost};
use phical::keyboard::Arena;

fn machine() -> Machine<PhisorHost> {
    checked(PhisorHost::new(Arena::new(0x0ca5), KindRows::new()).expect("free names")).expect("a machine")
}

#[test]
fn a_machine_is_born_knowing_the_keyboard_and_the_cards() {
    let mut m = machine();
    let mut env = Env::new();
    let loaded = load(&mut m, &mut env).expect("the cards load");
    assert_eq!(loaded.keyboard, 64);
    for card in ["meet-all", "union-all", "events", "told-answer"] {
        assert!(loaded.cards.iter().any(|c| c == card), "core.lisp defines {card}: {:?}", loaded.cards);
    }
    // A keyboard Operator by its card name: 0 is identity.
    assert_eq!(m.run("(phisor-op-0-identity 5)"), Ok(Value::Num(5.0)));
    // A core card over the primitives: the meet of three relations.
    let v = m.run("(count (meet-all (list (list (list a b) (list c d)) (list (list a b)) (list (list a b) (list e f)))))");
    assert_eq!(v, Ok(Value::Num(1.0)));
}

#[test]
fn a_punched_card_is_known_now_and_by_the_next_machine() {
    let dir = std::env::temp_dir().join(format!("phisor-ops-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::copy(super::dir().join("core.lisp"), dir.join("core.lisp")).unwrap();
    let learned = dir.join("learned.lisp");

    let mut m = machine();
    let mut env = Env::new();
    let before = load_from(&mut m, &mut env, &dir).unwrap();
    assert!(!before.cards.iter().any(|c| c == "twice"), "no card yet");
    let name = add_to(&mut m, &mut env, &learned, "(define twice (lambda (x) (+ x x)))").expect("the card punches");
    assert_eq!(name, "twice");
    assert_eq!(m.run("(twice 4)"), Ok(Value::Num(8.0)), "known now");

    let mut next = machine();
    let mut env = Env::new();
    let loaded = load_from(&mut next, &mut env, &dir).unwrap();
    assert!(loaded.cards.iter().any(|c| c == "twice"), "the next machine is born knowing it");
    assert_eq!(next.run("(twice 21)"), Ok(Value::Num(42.0)));

    assert!(add_to(&mut m, &mut env, &learned, "(twice 1)").is_err(), "not a define of a lambda");
    assert!(add_to(&mut m, &mut env, &learned, "(define a (lambda (x) x)) (define b (lambda (x) x))").is_err(), "one form per card");
    let _ = add_to(&mut m, &mut env, &learned, "(define twice (lambda (x) (+ x x)))");
    assert_eq!(phistd::lexfile::read(&learned).len(), 1, "not doubled");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn note_g_runs_on_the_mill() {
    // Lovelace's Bernoulli table (Note G), her numbering: B1 = 1/6, B3 = -1/30, B5 = 1/42, B7 = -1/30.
    let mut m = machine();
    let mut env = Env::new();
    let loaded = load(&mut m, &mut env).expect("the cards load");
    assert!(loaded.cards.iter().any(|c| c == "bernoulli"), "bernoulli.lisp is a card file: {:?}", loaded.cards);
    for (m_, want) in [(1, 1.0 / 6.0), (3, -1.0 / 30.0), (5, 1.0 / 42.0), (7, -1.0 / 30.0)] {
        let Ok(Value::Num(got)) = m.run(&format!("(bernoulli {m_})")) else { panic!("B{m_} is a number") };
        assert!((got - want).abs() < 1e-9, "B{m_}: {got} against {want}");
    }
    assert!(m.run("(/ 1 0)").is_err(), "the engine stops on an impossible order");
}
