use super::*;
use crate::guide::{Function, Mark, Sentence};
fn sentence(i: usize) -> Sentence {
    let noun = ["girl", "boy", "dog", "cat"][i % 4];
    let verb = ["saw", "liked", "fed"][i % 3];
    let words = ["the", noun, verb, "a", "red", "ball"].map(String::from).to_vec();
    Sentence { words, function: Function::Explain, mark: Mark::Stop, unit: 0, run: 0 }
}
fn split() -> Split {
    Split::new((0..60).map(sentence).collect())
}
fn on() -> Config {
    Config { enabled: true, ..Config::default() }
}
#[test]
fn the_flag_defaults_off_and_nothing_is_learned() {
    assert!(!Config::default().enabled);
    assert!(Abstraction::learn(&split(), Config::default()).is_none());
}
#[test]
fn a_variation_is_the_first_two_classes_and_the_last() {
    let split = split();
    let a = Abstraction::learn(&split, on()).unwrap();
    let words = &split.sentences[1].words;
    let c = a.classes(words);
    assert_eq!(a.variation(words), format!("c{} c{} … c{}", c[0], c[1], c[c.len() - 1]));
}
#[test]
fn training_indices_are_the_train_part_only() {
    let split = split();
    let a = Abstraction::learn(&split, on()).unwrap();
    assert_eq!(a.train(), split.of(Part::Train).as_slice());
    let held: Vec<usize> = split.of(Part::Test).into_iter().chain(split.of(Part::Dev)).collect();
    assert!(!held.is_empty() && a.train().iter().all(|i| !held.contains(i)));
}
#[test]
fn the_report_reads_the_class_count_from_the_data() {
    let a = Abstraction::learn(&split(), on()).unwrap();
    let r = a.report();
    assert_eq!(r.classes, a.code.halving.classes);
    assert_eq!(r.assumed, phiton::p64::variants::Level::P64.count());
    assert_eq!(r.reaches_assumed, r.classes >= 64);
    assert_eq!(r.sentences, a.train().len());
}

// Property 7: train-only learning (Requirements 8.6, 8.7). Bounded, seeded
// generation (no new dependency): 40 splits of 20..80 sentences, each drawn
// from a small vocabulary, some marked as the Key's answers (held out).
const VOCAB: [&str; 12] = ["the", "a", "girl", "boy", "dog", "cat", "saw", "liked", "fed", "red", "ball", "tree"];
fn next(s: &mut u64) -> usize {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*s >> 33) as usize
}
fn generated(seed: u64) -> Split {
    let mut s = seed;
    let n = 20 + next(&mut s) % 60;
    let sentences = (0..n)
        .map(|_| {
            let len = 2 + next(&mut s) % 6;
            let words = (0..len).map(|_| VOCAB[next(&mut s) % VOCAB.len()].to_string()).collect();
            let function = if next(&mut s) % 6 == 0 { Function::Answer } else { Function::Explain };
            Sentence { words, function, mark: Mark::Stop, unit: 0, run: 0 }
        })
        .collect();
    Split::new(sentences)
}
/// **Validates: Requirements 8.6, 8.7**
#[test]
fn property_train_indices_never_include_test_or_dev() {
    for seed in 0..40u64 {
        let split = generated(seed);
        let a = Abstraction::learn(&split, on()).unwrap();
        assert_eq!(a.train(), split.of(Part::Train).as_slice(), "seed {seed}");
        for &i in a.train() {
            assert_eq!(split.parts[i], Part::Train, "seed {seed}: index {i} is not train");
        }
    }
}
/// **Validates: Requirements 8.6, 8.7**
#[test]
fn property_changing_a_held_out_sentence_leaves_every_class_unchanged() {
    for seed in 0..40u64 {
        let split = generated(seed);
        let before = Abstraction::learn(&split, on()).unwrap();
        let mut s = seed ^ 0x9e37_79b9;
        let mut changed = generated(seed);
        let held: Vec<usize> = changed.of(Part::Test).into_iter().chain(changed.of(Part::Dev)).collect();
        assert!(!held.is_empty(), "seed {seed}: nothing held out");
        for &i in &held {
            // Rewrite the held-out words, including words the train part never saw.
            changed.sentences[i].words = (0..1 + next(&mut s) % 6)
                .map(|_| if next(&mut s) % 2 == 0 { VOCAB[next(&mut s) % VOCAB.len()].to_string() } else { format!("novel{}", next(&mut s) % 50) })
                .collect();
        }
        let after = Abstraction::learn(&changed, on()).unwrap();
        let probe: Vec<&str> = VOCAB.iter().copied().chain(["novel0", "novel7", "novel49"]).collect();
        for w in probe {
            assert_eq!(before.class(w), after.class(w), "seed {seed}: class of {w:?} moved");
        }
        assert_eq!(before.report(), after.report(), "seed {seed}");
    }
}
