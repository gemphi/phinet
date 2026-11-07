//! The predict stage of the learned layer (design section 5, stage 2): from
//! the classes before a position, predict the next class, then a word inside
//! that class from the words the data has shown there. When the evidence is
//! thin it abstains instead of guessing.
//!
//! Two control arms run through the same scorer:
//! - [`Arm::Shuffled`] keeps every context but permutes the next-class and
//!   word labels, so any lift the learned arm shows over it is the
//!   context-to-label link and not the label mix.
//! - [`Arm::Frequency`] ignores the context and predicts the most frequent
//!   class and its most frequent word (the majority-class control).
//!
//! Every arm is scored leave-one-sentence-out: while a sentence is scored,
//! its own pairs are out of the counts. The scored sentences are the held out
//! part; the counts pool is the train part plus the scored sentences, each
//! scored one left out in turn. Nothing here reads the Gold_Set or trains a
//! Force classifier. It is off with [`Config::default`] like the abstract
//! stage: [`predict`] returns `None` until `enabled` is set.
use super::{Abstraction, Config};
use crate::guide::{Part, Split};
use std::collections::HashMap;

/// The context of the first position of a sentence.
const START: u16 = u16::MAX;

/// Which model scores the held out sentences.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    /// Context to next class to word.
    Learned,
    /// Control: contexts kept, next-class and word labels shuffled.
    Shuffled,
    /// Control: no context, most frequent class and word.
    Frequency,
}

/// When to abstain: the context must have been seen `min_support` times and
/// its top class must hold at least `min_share` of them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rule {
    pub min_support: u32,
    pub min_share: f64,
}

impl Default for Rule {
    fn default() -> Self {
        Rule { min_support: 2, min_share: 0.5 }
    }
}

/// What one arm scored. Hit rates are over the positions it answered; a
/// figure that cannot be computed is `None`, not zero.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scores {
    /// Positions scored.
    pub total: usize,
    /// Positions answered.
    pub predicted: usize,
    /// Positions where it abstained.
    pub abstained: usize,
    /// Answers with the right class.
    pub abstract_hits: usize,
    /// Answers with the right class and word.
    pub surface_hits: usize,
    pub abstract_top1: Option<f64>,
    pub surface_top1: Option<f64>,
    /// Over all positions, answered or not.
    pub abstract_perplexity: Option<f64>,
    pub surface_perplexity: Option<f64>,
}

struct Pair {
    sentence: usize,
    ctx: u16,
    class: u16,
    word: String,
}

#[derive(Default)]
struct Counts {
    next: HashMap<u16, HashMap<u16, i64>>,
    words: HashMap<u16, HashMap<String, i64>>,
}

fn bump<K: std::hash::Hash + Eq, J: std::hash::Hash + Eq>(map: &mut HashMap<K, HashMap<J, i64>>, k: K, j: J, by: i64) {
    *map.entry(k).or_default().entry(j).or_insert(0) += by;
}

impl Counts {
    fn add(&mut self, p: &Pair, by: i64) {
        bump(&mut self.next, p.ctx, p.class, by);
        bump(&mut self.words, p.class, p.word.clone(), by);
    }
}

/// The key with the highest positive count; ties go to the smallest key.
fn top<K: Ord + Clone>(m: Option<&HashMap<K, i64>>) -> Option<(K, i64)> {
    let m = m?;
    m.iter()
        .filter(|(_, &n)| n > 0)
        .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(a.0)))
        .map(|(k, &n)| (k.clone(), n))
}

fn sum<K>(m: Option<&HashMap<K, i64>>) -> i64 {
    m.map_or(0, |m| m.values().sum())
}

fn count<K: std::hash::Hash + Eq + Clone, Q: ?Sized + std::hash::Hash + Eq>(
    m: Option<&HashMap<K, i64>>,
    k: &Q,
) -> i64
where
    K: std::borrow::Borrow<Q>,
{
    m.and_then(|m| m.get(k)).copied().unwrap_or(0)
}

/// A fixed shuffle: same seed, same order.
fn shuffle<T>(items: &mut [T], mut state: u64) {
    state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) | 1;
    for i in (1..items.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        items.swap(i, (state % (i as u64 + 1)) as usize);
    }
}

impl Abstraction {
    fn pairs(&self, split: &Split, i: usize, blind: bool) -> Vec<Pair> {
        let words = &split.sentences[i].words;
        let classes = self.classes(words);
        (0..words.len())
            .map(|j| {
                let ctx = match (blind, j) {
                    (true, _) => START,
                    (false, 0) => START,
                    (false, _) => classes[j - 1],
                };
                Pair { sentence: i, ctx, class: classes[j], word: words[j].clone() }
            })
            .collect()
    }

    /// Score the held out sentences with one arm, leave-one-sentence-out.
    /// `seed` fixes the shuffle of the [`Arm::Shuffled`] control.
    pub fn predict(&self, split: &Split, arm: Arm, rule: Rule, seed: u64) -> Scores {
        let scored = split.of(Part::Test);
        let mut pool: Vec<usize> = self.train().iter().chain(scored.iter()).copied().collect();
        pool.sort_unstable();
        pool.dedup();
        let mut pairs: Vec<Pair> = pool.iter().flat_map(|&i| self.pairs(split, i, arm == Arm::Frequency)).collect();
        if arm == Arm::Shuffled {
            let mut labels: Vec<(u16, String)> = pairs.iter().map(|p| (p.class, p.word.clone())).collect();
            shuffle(&mut labels, seed);
            for (p, (class, word)) in pairs.iter_mut().zip(labels) {
                p.class = class;
                p.word = word;
            }
        }
        let mut counts = Counts::default();
        pairs.iter().for_each(|p| counts.add(p, 1));
        let report = self.report();
        let (k, v) = (report.classes.max(1) as f64, (report.words + 1) as f64);
        let mut s = Scores {
            total: 0,
            predicted: 0,
            abstained: 0,
            abstract_hits: 0,
            surface_hits: 0,
            abstract_top1: None,
            surface_top1: None,
            abstract_perplexity: None,
            surface_perplexity: None,
        };
        let (mut log_abstract, mut log_surface) = (0.0f64, 0.0f64);
        for &sentence in &scored {
            let mine: Vec<&Pair> = pairs.iter().filter(|p| p.sentence == sentence).collect();
            mine.iter().for_each(|p| counts.add(p, -1));
            // Score against the truth, which is never shuffled.
            for (p, real) in mine.iter().zip(self.pairs(split, sentence, arm == Arm::Frequency)) {
                let next = counts.next.get(&p.ctx);
                let n = sum(next) as f64;
                let pc = (count(next, &real.class) as f64 + 1.0) / (n + k);
                let in_class = counts.words.get(&real.class);
                let pw = (count(in_class, real.word.as_str()) as f64 + 1.0) / (sum(in_class) as f64 + v);
                log_abstract += pc.ln();
                log_surface += (pc * pw).ln();
                s.total += 1;
                let guess = top(next).filter(|&(_, hits)| hits >= rule.min_support as i64 && hits as f64 >= rule.min_share * n);
                match guess.and_then(|(class, _)| top(counts.words.get(&class)).map(|(w, _)| (class, w))) {
                    None => s.abstained += 1,
                    Some((class, word)) => {
                        s.predicted += 1;
                        s.abstract_hits += (class == real.class) as usize;
                        s.surface_hits += (class == real.class && word == real.word) as usize;
                    }
                }
            }
            mine.iter().for_each(|p| counts.add(p, 1));
        }
        let rate = |hits: usize| (s.predicted > 0).then(|| hits as f64 / s.predicted as f64);
        let perplexity = |log: f64| (s.total > 0).then(|| (-log / s.total as f64).exp());
        s.abstract_top1 = rate(s.abstract_hits);
        s.surface_top1 = rate(s.surface_hits);
        s.abstract_perplexity = perplexity(log_abstract);
        s.surface_perplexity = perplexity(log_surface);
        s
    }
}

/// Learn the abstract stage and score one arm, or `None` when the flag is off.
pub fn predict(split: &Split, config: Config, arm: Arm, rule: Rule, seed: u64) -> Option<Scores> {
    Abstraction::learn(split, config).map(|a| a.predict(split, arm, rule, seed))
}

#[cfg(test)]
mod check {
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
    fn off_by_default() {
        assert!(predict(&split(), Config::default(), Arm::Learned, Rule::default(), 1).is_none());
    }

    #[test]
    fn learned_beats_both_controls_on_perplexity() {
        let run = |arm| predict(&split(), on(), arm, Rule::default(), 7).unwrap();
        let (learned, shuffled, frequency) = (run(Arm::Learned), run(Arm::Shuffled), run(Arm::Frequency));
        assert_eq!(learned.total, shuffled.total);
        assert_eq!(learned.total, frequency.total);
        assert!(learned.total > 0);
        let p = |s: Scores| s.abstract_perplexity.unwrap();
        assert!(p(learned) < p(shuffled), "{learned:?} vs {shuffled:?}");
        assert!(p(learned) < p(frequency), "{learned:?} vs {frequency:?}");
    }

    #[test]
    fn abstains_when_support_is_out_of_reach() {
        let rule = Rule { min_support: u32::MAX, min_share: 0.0 };
        let s = predict(&split(), on(), Arm::Learned, rule, 1).unwrap();
        assert_eq!((s.predicted, s.abstained), (0, s.total));
        assert!(s.abstract_top1.is_none() && s.abstract_perplexity.is_some());
    }

    #[test]
    fn shuffle_is_fixed_by_seed() {
        let run = |seed| predict(&split(), on(), Arm::Shuffled, Rule::default(), seed).unwrap();
        assert_eq!(run(3), run(3));
    }
}
