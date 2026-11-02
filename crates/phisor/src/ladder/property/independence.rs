//! Property 61: the Operator table is independent of every learned count.
//!
//! phisor grows uncapped counts - templates, word classes, kinds - as far as
//! the data pays. The 64 Operators are the designed keyboard, not one of those
//! grown things, and R6.14 holds that growing a different number of them moves
//! no index, name, type, arity or family of the table. This runs the Ladder at
//! corpora shaped to grow counts well below and well above 64 and asserts the
//! keyboard fingerprint does not move between them.

use crate::guide::{Function, Mark, Sentence, Split};
use crate::ladder::{Arm, Ladder, Seeds};
use phical::keyboard::{table, Operator};
use phistd::gen::{for_all, Gen, Seeded, CASES_MACHINE};
use phiton::p64::variants::Level;

/// A fingerprint of the Operator table over exactly the fields the frozen
/// index record carries: index, name, family, arity, each operand type and
/// the result type. No learned count has a field here, so if the table is
/// independent of the counts this number is constant across runs.
fn keyboard_fingerprint(ops: &[Operator]) -> u64 {
    let mut h = 0u64;
    for op in ops {
        h = fold(h, op.index as u64);
        h = bytes(h, op.name.as_bytes());
        h = bytes(h, op.family.name().as_bytes());
        h = fold(h, op.arity as u64);
        for ty in op.operands {
            h = bytes(h, ty.name().as_bytes());
        }
        h = bytes(h, op.result.name().as_bytes());
    }
    h
}

fn fold(h: u64, x: u64) -> u64 {
    phistd::mix64(h ^ x)
}

fn bytes(mut h: u64, bs: &[u8]) -> u64 {
    for &b in bs {
        h = phistd::mix64(h ^ b as u64);
    }
    fold(h, 0x1F)
}

/// A corpus drawn to grow either few classes or many: `wide` picks a large
/// vocabulary of distinct words in distinct contexts (many grown classes),
/// `narrow` a tiny one (few). Either way the keyboard is untouched.
#[derive(Debug)]
struct Shape {
    wide: bool,
    units: usize,
}

impl Gen for Shape {
    fn draw(g: &mut Seeded) -> Self {
        Shape { wide: g.below(2) == 0, units: 20 + g.below(180) as usize }
    }
}

fn corpus(shape: &Shape) -> Vec<Sentence> {
    let vocab = if shape.wide { 400 } else { 6 };
    (0..shape.units)
        .map(|u| {
            let words: Vec<String> = (0..6)
                .map(|i| format!("w{}", (u * 6 + i) % vocab))
                .collect();
            Sentence {
                words,
                function: Function::Explain,
                mark: Mark::Stop,
                unit: u,
                run: 0,
            }
        })
        .collect()
}

/// The count of distinct word classes a corpus grows at Mode 64 in the
/// Learned arm - the grown word-class count itself, read straight off the
/// coder. The property uses it only to show its two shapes differ widely in
/// what they grow, so Property 61 is not quietly testing one size twice.
fn grown_classes(sentences: &[Sentence]) -> usize {
    let split = Split::new(sentences.to_vec());
    let ladder = Ladder::new(&split, Seeds::DECLARED);
    let coder = ladder.coder(Arm::Learned, phical::classes::Grow::Paid);
    let mut seen = std::collections::BTreeSet::new();
    for sentence in &split.sentences {
        for class in coder.write(&sentence.words, Level::P64) {
            seen.insert(class);
        }
    }
    seen.len()
}

#[test]
fn property_61_the_operator_table_is_independent_of_every_learned_count() {
    // **Property 61: The operator table is independent of every learned count**
    // **Validates: Requirements 6.14**
    let base = keyboard_fingerprint(&table());
    let run = for_all::<Shape>(0x61, CASES_MACHINE, |shape| {
        let sentences = corpus(shape);
        let split = Split::new(sentences.clone());
        let ladder = Ladder::new(&split, Seeds::DECLARED);
        // Run the Ladder: write every sentence in Mode 64 in the Learned arm,
        // which grows the code. The keyboard is read before and after.
        let coder = ladder.coder(Arm::Learned, phical::classes::Grow::Paid);
        for sentence in &split.sentences {
            let _ = coder.write(&sentence.words, Level::P64);
        }
        // Whatever grew, the table did not move.
        keyboard_fingerprint(&table()) == base
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

#[test]
fn the_two_shapes_grow_widely_different_counts() {
    // The wide corpus grows many more word classes than the narrow one, so
    // Property 61 holds the keyboard fixed across a real range of grown
    // counts rather than one size tested twice.
    let wide = grown_classes(&corpus(&Shape { wide: true, units: 180 }));
    let narrow = grown_classes(&corpus(&Shape { wide: false, units: 180 }));
    assert!(narrow <= 8, "the narrow corpus grew {narrow} classes, not a small count");
    assert!(wide > narrow, "the wide corpus grew {wide}, not above the narrow {narrow}");
}