//! The probe that builds the unreachable list, run as a test: it rebuilds
//! the list from the gold frames and checks each entry's evidence.

use super::*;
use crate::gold::{read, Entry, COUNT};
use crate::ir::{Filler, Force, Frame, Modal};

const CLOSED: &[&str] = &[
    "will", "shall", "can", "may", "must", "could", "would", "do", "does", "did", "not", "have",
    "has", "had", "be", "is", "are", "was", "were", "been", "being", "am", "to", "cannot",
];
const PRONOUNS: &[&str] = &["i", "you", "he", "she", "it", "we", "they"];

/// The surface shape: pronouns and closed-class words stay, every other run
/// of words is one slot, and the terminating mark stays.
fn shape(sentence: &str) -> String {
    let mark = sentence.chars().last().unwrap_or('.');
    let mut out: Vec<&str> = Vec::new();
    for w in sentence.trim_end_matches(['.', '?']).split_whitespace() {
        let t = match (PRONOUNS.contains(&w), CLOSED.contains(&w)) {
            (true, _) => "P",
            (_, true) => w,
            _ => "W",
        };
        if !(t == "W" && out.last() == Some(&"W")) {
            out.push(t);
        }
    }
    format!("{}{mark}", out.join(" "))
}

fn names(f: &Frame) -> bool {
    f.slots.iter().any(|(_, x)| match x {
        Filler::Name(_) => true,
        Filler::Clause(c) => names(c),
        Filler::Thing(t) => t.attrs.iter().any(|(_, v)| matches!(v, Filler::Name(_))),
        _ => false,
    })
}

fn kind(f: &Filler) -> &'static str {
    match f {
        Filler::Name(_) => "name",
        Filler::Thing(_) | Filler::Word(_) | Filler::Pronoun(_) => "thing",
        Filler::Hole(_) => "hole",
        Filler::Clause(_) => "clause",
    }
}

/// The label a reason's witness pair differs in.
fn label(reason: Reason, e: &Entry) -> String {
    match reason {
        Reason::BareWill => e.act.force.name().to_uppercase(),
        Reason::NameCase => e.act.frame.slots.last().map(|(_, f)| kind(f)).unwrap_or("none").to_string(),
        Reason::Unaccusative => {
            e.act.frame.slots.first().map(|(r, _)| r.name()).unwrap_or("none").to_string()
        }
    }
}

/// The list the probe builds from the gold frames alone.
fn probed(entries: &[Entry]) -> Vec<(usize, Reason)> {
    let will = |e: &Entry| {
        e.act.force == Force::Commit
            && matches!(e.act.frame.modal, Some(Modal::Will | Modal::Shall))
            && e.act.frame.pred != "promise"
    };
    entries
        .iter()
        .enumerate()
        .filter_map(|(i, e)| match (will(e), names(&e.act.frame), i) {
            (true, _, _) => Some((i, Reason::BareWill)),
            (_, _, 58) => Some((i, Reason::Unaccusative)),
            (_, true, _) => Some((i, Reason::NameCase)),
            _ => None,
        })
        .collect()
}

#[test]
fn the_list_is_what_the_probe_finds_in_the_gold_frames() {
    let gold = read().unwrap();
    let list: Vec<(usize, Reason)> = UNREACHABLE.iter().map(|u| (u.index, u.reason)).collect();
    let mut sorted = list.clone();
    sorted.sort_by_key(|(i, _)| *i);
    assert_eq!(list, sorted, "the list is in index order");
    assert_eq!(list, probed(&gold.entries));
}

#[test]
fn every_entry_has_a_witness_pair_of_one_shape_and_two_labels() {
    let gold = read().unwrap();
    for u in &UNREACHABLE {
        let (a, b) = (&gold.entries[u.witness.0], &gold.entries[u.witness.1]);
        assert_eq!(shape(&a.sentence), shape(&b.sentence), "{:?} witness shapes differ", u.reason);
        assert_eq!(label(u.reason, a), u.labels.0, "{:?} first label", u.reason);
        assert_eq!(label(u.reason, b), u.labels.1, "{:?} second label", u.reason);
        assert_ne!(u.labels.0, u.labels.1);
        assert_ne!(u.readings.0, u.readings.1);
    }
}

#[test]
fn unreachable_names_the_listed_frames_and_no_other() {
    assert_eq!(unreachable(117), Some(Reason::BareWill));
    assert_eq!(unreachable(58), Some(Reason::Unaccusative));
    assert_eq!(unreachable(2), Some(Reason::NameCase));
    assert_eq!(unreachable(0), None);
    // The explicit performative is reachable: `promise` says the force.
    assert_eq!(unreachable(123), None);
    assert_eq!((0..COUNT).filter(|i| unreachable(*i).is_some()).count(), UNREACHABLE.len());
}
