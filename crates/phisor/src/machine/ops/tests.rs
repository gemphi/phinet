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
    std::env::set_var("PHISOR_OPS", &dir);

    let mut m = machine();
    let mut env = Env::new();
    let before = load(&mut m, &mut env).unwrap();
    assert!(!before.cards.iter().any(|c| c == "twice"), "no card yet");
    let name = add(&mut m, &mut env, "(define twice (lambda (x) (+ x x)))").expect("the card punches");
    assert_eq!(name, "twice");
    assert_eq!(m.run("(twice 4)"), Ok(Value::Num(8.0)), "known now");

    let mut next = machine();
    let mut env = Env::new();
    let loaded = load(&mut next, &mut env).unwrap();
    assert!(loaded.cards.iter().any(|c| c == "twice"), "the next machine is born knowing it");
    assert_eq!(next.run("(twice 21)"), Ok(Value::Num(42.0)));

    assert!(add(&mut m, &mut env, "(twice 1)").is_err(), "not a define of a lambda");
    assert!(add(&mut m, &mut env, "(define a (lambda (x) x)) (define b (lambda (x) x))").is_err(), "one form per card");
    let _ = add(&mut m, &mut env, "(define twice (lambda (x) (+ x x)))");
    assert_eq!(phistd::lexfile::read(dir.join("learned.lisp")).len(), 1, "not doubled");

    std::env::remove_var("PHISOR_OPS");
    let _ = std::fs::remove_dir_all(&dir);
}
