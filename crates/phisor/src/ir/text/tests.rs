use super::*;
use crate::ir::{
    Aspect, Attr, Det, Filler, Force, Frame, Modal, Polarity, Role, Tense, Thing, Voice, Wh,
};
use phical::keyboard::MAX_CLAUSE_DEPTH;

fn wait() -> Act {
    Act::new(
        Force::Ask,
        Frame::new("wait", Tense::Past)
            .aspect(Aspect::Progressive)
            .with(Role::Agent, Filler::thing("you"))
            .with(Role::Via("in".to_string()), Filler::thing("rain")),
    )
}

#[test]
fn the_declared_readable_example_writes_and_reads() {
    let act = wait();
    let written = write(&act);
    assert_eq!(
        written,
        "ASK(wait(agent: you, in: rain), past, progressive)"
    );
    assert_eq!(read(&written), Ok(act.clone()));
    assert_eq!(written.parse::<Act>(), Ok(act));
}

#[test]
fn variant_markers_determiners_and_attributes_lose_nothing() {
    let inner = Frame::new("visit", Tense::Present)
        .modal(Modal::May)
        .with(Role::Agent, Filler::Pronoun("she".to_string()));
    let thing = Thing::new("suit")
        .det(Det::Owner("their own".to_string()))
        .plural()
        .with(Attr::Mod, Filler::Word("blue".to_string()))
        .with(Attr::Kind("mod".to_string()), Filler::Hole(Wh::Which))
        .with(
            Attr::Via("of".to_string()),
            Filler::Name("France".to_string()),
        )
        .with(Attr::Count, Filler::Clause(Box::new(inner)));
    let mut frame = Frame::new("name", Tense::Past)
        .aspect(Aspect::PerfectProgressive)
        .modal(Modal::Must)
        .passive()
        .negative();
    frame.slots = vec![
        (Role::Agent, Filler::Name("Helen".to_string())),
        (Role::Theme, Filler::Thing(thing)),
        (Role::Recipient, Filler::Pronoun("they".to_string())),
        (
            Role::Via("agent".to_string()),
            Filler::Word("answer".to_string()),
        ),
    ];
    let act = Act::new(Force::Declare, frame);
    let written = write(&act);
    assert!(written.contains("*Helen"));
    assert!(written.contains("owner(\"their own\") suit·pl"));
    assert!(written.contains("kind(mod): ?which"));
    assert!(written.contains("~of: *France"));
    assert!(written.contains("~agent: 'answer"));
    assert_eq!(read(&written), Ok(act));
}

#[test]
fn quoted_atoms_preserve_delimiters_escapes_controls_and_empty_text() {
    let act = Act::new(
        Force::Express,
        Frame::new("say, \"now\"\n", Tense::Present)
            .with(
                Role::Via("with:role".to_string()),
                Filler::Name(String::new()),
            )
            .with(Role::Theme, Filler::Word("a\\b\tc".to_string()))
            .with(Role::As, Filler::Thing(Thing::new("two words"))),
    );
    let written = write(&act);
    assert!(written.contains("\"say, \\\"now\\\"\\n\""));
    assert!(written.contains("\"with:role\""));
    assert!(written.contains("*\"\""));
    assert_eq!(read(&written), Ok(act));
}

fn deep(depth: usize) -> Act {
    let mut frame = Frame::new("sing", Tense::Present);
    for _ in 0..depth {
        frame =
            Frame::new("wish", Tense::Present).with(Role::Theme, Filler::Clause(Box::new(frame)));
    }
    Act::new(Force::Wish, frame)
}

#[test]
fn the_reader_accepts_the_clause_cap_and_refuses_the_next_depth() {
    let at_cap = deep(MAX_CLAUSE_DEPTH);
    assert_eq!(read(&write(&at_cap)), Ok(at_cap));
    let error = read(&write(&deep(MAX_CLAUSE_DEPTH + 1))).unwrap_err();
    assert!(error
        .message
        .contains(&format!("exceeds the cap {MAX_CLAUSE_DEPTH}")));
}

#[test]
fn unreadable_text_names_where_reading_stopped() {
    let unknown = read("SUGGEST(wait(), present)").unwrap_err();
    assert_eq!(unknown.offset, 0);
    assert!(unknown.message.contains("unknown force"));
    assert!(read("ASK(wait(), future)")
        .unwrap_err()
        .message
        .contains("unknown tense"));
    assert!(read("ASK(wait(), present, perhaps)")
        .unwrap_err()
        .message
        .contains("unknown frame feature"));
    assert!(read("ASK(wait(), present) trailing")
        .unwrap_err()
        .message
        .contains("unexpected trailing"));
}

#[test]
fn the_share_is_named_and_judged_against_one_hundred_percent() {
    let good = wait();
    let too_deep = deep(MAX_CLAUSE_DEPTH + 1);
    let pass = round_trip([&good]);
    assert_eq!(
        pass,
        RoundTrip {
            name: ROUND_TRIP_NAME,
            matched: 1,
            total: 1,
            share: 100.0,
            bar: ROUND_TRIP_BAR,
            shortfall: 0.0,
        }
    );
    assert!(pass.meets_bar());
    let miss = round_trip([&good, &too_deep]);
    assert_eq!(
        (miss.matched, miss.total, miss.share, miss.shortfall),
        (1, 2, 50.0, 50.0)
    );
    assert!(!miss.meets_bar());
    assert!(!round_trip(std::iter::empty::<&Act>()).meets_bar());
}

#[test]
fn explicit_default_features_are_accepted_without_changing_the_act() {
    let parsed = read("ASSERT(rain(), present, simple, active, positive)").unwrap();
    assert_eq!(parsed.frame.aspect, Aspect::Simple);
    assert_eq!(parsed.frame.voice, Voice::Active);
    assert_eq!(parsed.frame.polarity, Polarity::Positive);
    assert_eq!(write(&parsed), "ASSERT(rain(), present)");
}
