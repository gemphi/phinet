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
