//! Property 27: the generated categorical product, seeded arbitrary Acts,
//! every supported clause depth and the frozen gold set all use the same
//! printer-reader pair.

mod generate;

use super::{read, round_trip, write, ROUND_TRIP_BAR};
use crate::gold;
use crate::ir::{
    Act, Aspect, Attr, Det, Filler, Force, Frame, Modal, Polarity, Role, Tense, Thing, Voice, Wh,
};
use generate::Case;
use phical::keyboard::MAX_CLAUSE_DEPTH;
use phistd::gen::{for_all, replay, CASES};

const MODALS: [Option<Modal>; 6] = [
    None,
    Some(Modal::Will),
    Some(Modal::Can),
    Some(Modal::May),
    Some(Modal::Must),
    Some(Modal::Shall),
];

fn survives(act: &Act) -> bool {
    read(&write(act)).as_ref() == Ok(act)
}

fn role(index: usize) -> Role {
    match index % 6 {
        0 => Role::Agent,
        1 => Role::Theme,
        2 => Role::Recipient,
        3 => Role::As,
        4 => Role::Via("in".to_string()),
        _ => Role::Via("agent".to_string()),
    }
}

fn filler(index: usize) -> Filler {
    match index % 6 {
        0 => Filler::Thing(
            Thing::new("suit")
                .det(Det::Indefinite)
                .plural()
                .with(Attr::Kind("colour".to_string()), Filler::word("blue")),
        ),
        1 => Filler::Name("France".to_string()),
        2 => Filler::Pronoun("you".to_string()),
        3 => Filler::Word("first".to_string()),
        4 => Filler::Hole(Wh::HowMany),
        _ => Filler::Clause(Box::new(Frame::new("rain", Tense::Present))),
    }
}

fn categorical() -> Vec<Act> {
    let mut out = Vec::new();
    let mut case = 0usize;
    for force in Force::ALL {
        for tense in Tense::ALL {
            for aspect in Aspect::ALL {
                for modal in MODALS {
                    for voice in Voice::ALL {
                        for polarity in Polarity::ALL {
                            for roles in 0..=4 {
                                let mut frame = Frame::new("predicate", tense);
                                frame.aspect = aspect;
                                frame.modal = modal;
                                frame.voice = voice;
                                frame.polarity = polarity;
                                for slot in 0..roles {
                                    frame.slots.push((role(slot), filler(case + slot)));
                                }
                                out.push(Act::new(force, frame));
                                case += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

fn deep(depth: usize) -> Act {
    let mut frame = Frame::new("sing", Tense::Present);
    for _ in 0..depth {
        frame = Frame::new("wish", Tense::Past)
            .aspect(Aspect::PerfectProgressive)
            .modal(Modal::Can)
            .passive()
            .negative()
            .with(Role::Theme, Filler::Clause(Box::new(frame)));
    }
    Act::new(Force::Wish, frame)
}

#[test]
fn property_27_writing_an_act_and_reading_it_back_loses_nothing() {
    // Property 27: Writing an Act and reading it back loses nothing.
    // **Validates: Requirements 10.10**

    // Seeded generated Acts exercise arbitrary payload strings as well as
    // recursive combinations. The first failure reports a replayable case.
    let generated = for_all::<Case>(0x5EED_0E27, CASES, |case| survives(&case.0));
    assert!(generated.is_ok(), "{}", generated.unwrap_err());

    let mut acts = categorical();
    // 7 forces × 2 tenses × 4 aspects × 6 modality states × 2 voices ×
    // 2 polarities × each role count 0 through 4.
    assert_eq!(acts.len(), 7 * 2 * 4 * 6 * 2 * 2 * 5);

    // Every Filler variant is present even independently of the categorical
    // cycling, so a future loop-order change cannot weaken the property.
    for variant in 0..6 {
        acts.push(Act::new(
            Force::Assert,
            Frame::new("hold", Tense::Present).with(Role::Theme, filler(variant)),
        ));
    }

    // The top frame is depth zero; every allowed clause depth is exercised,
    // including MAX_CLAUSE_DEPTH itself.
    acts.extend((0..=MAX_CLAUSE_DEPTH).map(deep));
    acts.extend((0..CASES).map(|case| replay::<Case>(0x5EED_0E27, case).0));

    let frozen = gold::read().expect("the 150 frozen gold frames must retain their fingerprint");
    assert_eq!(frozen.len(), gold::COUNT);
    acts.extend(frozen.acts().cloned());

    let measured = round_trip(acts.iter());
    assert_eq!(measured.matched, measured.total);
    assert_eq!(measured.share, ROUND_TRIP_BAR);
    assert_eq!(measured.shortfall, 0.0);
    assert!(measured.meets_bar());
}
