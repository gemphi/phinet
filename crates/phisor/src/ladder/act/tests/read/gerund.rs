//! Reading gerund complements and purpose infinitives.

use super::*;

#[test]
fn a_gerund_complement_reads_and_round_trips() {
    use crate::ir::{Aspect, Filler, Role};
    // A gerund verb + an -ing form is a clause sharing the agent, kept as
    // progressive aspect so it is told from a control infinitive.
    let act = read("i enjoyed reading the book.").expect("a gerund complement is covered");
    assert_eq!(act.frame.pred, "enjoy");
    let Some(Filler::Clause(inner)) = act.frame.get(&Role::Theme) else { panic!("theme is not a clause") };
    assert_eq!((inner.pred.as_str(), inner.aspect), ("read", Aspect::Progressive));
    assert_eq!(act.frame.get(&Role::Agent), inner.get(&Role::Agent));
    assert_eq!(realise(&act), "i enjoyed reading the book.");
    let act = read("they avoided stealing the money.").expect("a past gerund complement is covered");
    assert_eq!(act.frame.pred, "avoid");
    assert_eq!(realise(&act), "they avoided stealing the money.");
}

#[test]
fn a_gerund_is_not_read_after_a_verb_that_does_not_take_one() {
    // "walk" is not a gerund verb, and an unfinished tail leaves no frame.
    assert!(read("i walk reading the book.").is_none());
    assert!(read("i enjoy reading the.").is_none());
}

#[test]
fn a_purpose_infinitive_reads_and_round_trips() {
    use crate::ir::{Filler, Role};
    // "to" + a bare verb after a complete predicate is a purpose clause stored
    // under `to`, not a theme and not a prepositional phrase.
    let act = read("he went to the library to read a book.").expect("a purpose infinitive is covered");
    assert!(matches!(act.frame.get(&Role::Via("to".into())), Some(Filler::Thing(_))));
    assert!(act.frame.slots.iter().any(|(r, f)| *r == Role::Via("to".into()) && matches!(f, Filler::Clause(_))));
    assert!(act.frame.get(&Role::Theme).is_none());
    assert_eq!(realise(&act), "he went to the library to read a book.");
    // Directly after the verb, with a complement of its own.
    let act = read("she came to open the door.").expect("a bare purpose infinitive is covered");
    assert_eq!(realise(&act), "she came to open the door.");
}

#[test]
fn a_control_infinitive_is_not_also_read_as_a_purpose() {
    use crate::ir::{Filler, Role};
    // "want" owns its `to`: the clause is the theme and nothing sits under `to`.
    let act = read("i want to open the door.").expect("a control infinitive is covered");
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Clause(_))));
    assert!(!act.frame.slots.iter().any(|(r, _)| *r == Role::Via("to".into())));
}

#[test]
fn a_noun_modifying_infinitive_is_not_read_as_a_purpose() {
    use crate::ir::{Filler, Role};
    // "somewhere to read" modifies the adverb; reading it as purpose would
    // round-trip the words and state the wrong structure. The host guard stops
    // the purpose fallback; the infinitive is read on the host instead.
    let act = read("he went somewhere to read a book.").expect("a host infinitive is covered");
    assert!(!act.frame.slots.iter().any(|(_, f)| matches!(f, Filler::Clause(_))));
    assert!(act.frame.get(&Role::Via("to".into())).is_none());
    assert_eq!(realise(&act), "he went somewhere to read a book.");
    // The same tail without the host reads as a purpose.
    assert!(read("he went there to read a book.").is_some());
}

#[test]
fn to_read_after_a_host_is_to_plus_a_verb_not_a_noun() {
    use crate::ir::{Attr, Filler, Role};
    // `read` is the verb of an infinitive on the host, not the noun of a `to`
    // phrase on the matrix verb.
    let act = read("he has something to read.").expect("a host infinitive is covered");
    assert_eq!(act.frame.pred, "have");
    assert!(act.frame.get(&Role::Via("to".into())).is_none());
    let Some(Filler::Thing(t)) = act.frame.get(&Role::Theme) else { panic!("theme is not a thing") };
    assert_eq!(t.head, "something");
    let Some((_, Filler::Clause(inner))) = t.attrs.iter().find(|(a, _)| *a == Attr::Via("to".into())) else {
        panic!("no infinitive on the host")
    };
    assert_eq!(inner.pred, "read");
    assert_eq!(realise(&act), "he has something to read.");
}

#[test]
fn a_prepositional_to_is_not_disturbed_by_the_purpose_rule() {
    use crate::ir::Role;
    let act = read("she went to london.").expect("a prepositional to is covered");
    assert!(!act.frame.slots.iter().any(|(_, f)| matches!(f, crate::ir::Filler::Clause(_))));
    assert!(act.frame.slots.iter().any(|(r, _)| *r == Role::Via("to".into())));
    assert_eq!(realise(&act), "she went to london.");
}

#[test]
fn a_purpose_tail_that_does_not_read_leaves_no_partial_frame() {
    // The clause ends in a preposition with nothing after it, so it cannot be
    // placed and the whole sentence abstains rather than reading "went to the
    // library" alone.
    assert!(read("he went to the library to read a book at.").is_none());
}
