//! The word, Mode, Variation and Kind heights' five numbered properties.
//! The drivers are `phistd::gen`'s seeded `for_all`, so a failure replays from
//! `(seed, case)` alone and no outside crate is taken on. The corpus the
//! cases draw over, and the three arms' coders, are the module's own: see
//! `tests.rs`.
//!
//! | property | what it holds |
//! |---|---|
//! | 21 | a text in a Mode carries structure and no words (R5.1) |
//! | 22 | the Modes are one code coarsened, not three codes (R5.3) |
//! | 23 | a Variation is first, second and last class (R5.4) |
//! | 24 | the River pairs adjacent sentences only within runs (R5.5) |
//! | 25 | sentence accounting conserves (R5.15) |
//! | 52 | Mode counts agree exactly within 20% of their mean (R5.10) |

use super::tests::{corpus, first, sentence, VOCAB};
use super::*;
use crate::code::Coder;
use phistd::gen::{for_all, Gen, Seeded, CASES};

mod kinds;
mod measure;
mod nesting;

// ----- Property 21: a text in a Mode carries structure and no words -----

/// A drawn text, the Mode it is written in and the arm that writes it. The
/// vocabulary [`VOCAB`] is drawn from deliberately spells three of its words
/// like class tokens (`c3`, `c7`, `c12`), so a leaked word cannot hide
/// behind the token form.
#[derive(Debug)]
struct Text {
    sentences: Vec<Sentence>,
    level: Level,
    arm: usize,
}

impl Gen for Text {
    fn draw(g: &mut Seeded) -> Self {
        let count = 1 + g.below(40) as usize;
        let sentences = (0..count)
            .map(|u| {
                let words: Vec<&str> = (0..g.below(10)).map(|_| *g.pick(&VOCAB)).collect();
                sentence(&words, u)
            })
            .collect();
        let level = *g.pick(&crate::code::LEVELS);
        Text {
            sentences,
            level,
            arm: g.below(3) as usize,
        }
    }
}

/// The three arms' coders, grown once from the fixed corpus, so a property
/// case writes rather than relearns.
fn arms() -> Vec<Coder> {
    let split = Split::new(corpus());
    let ladder = Ladder::new(&split, Seeds::default());
    Arm::ALL
        .into_iter()
        .map(|arm| ladder.coder(arm, Grow::Fixed(3)))
        .collect()
}

#[test]
fn property_21_a_text_in_a_mode_carries_structure_and_no_words() {
    // Property 21: A text in a Mode carries structure and no words.
    // **Validates: Requirements 5.1**
    let arms = arms();
    let run = for_all::<Text>(0x21, CASES, |t| {
        let coder = &arms[t.arm];
        let Ok(written) = write(&t.sentences, t.level, coder) else {
            return false;
        };
        // The token count equals the word count, sentence by sentence and
        // over the whole text.
        let words: usize = t.sentences.iter().map(|s| s.words.len()).sum();
        let tokens: usize = written.iter().map(Vec::len).sum();
        if written.len() != t.sentences.len() || tokens != words {
            return false;
        }
        // The tokens follow the text's own word order: token i of sentence
        // s is the class of word i of sentence s, and nothing else.
        let ordered = written.iter().zip(&t.sentences).all(|(w, s)| {
            w.len() == s.words.len()
                && w.iter()
                    .zip(&s.words)
                    .all(|(&c, word)| c == coder.class(word, t.level))
        });
        if !ordered {
            return false;
        }
        // Every token matches the class-token form and reads back as the
        // class it was written for.
        let shown = show(&written);
        let formed = shown.iter().zip(&written).all(|(line, w)| {
            let read: Vec<&str> = line.split_whitespace().collect();
            read.len() == w.len()
                && read.iter().zip(w).all(|(token, &c)| {
                    token
                        .strip_prefix('c')
                        .is_some_and(|digits| digits.parse::<u16>() == Ok(c))
                })
        });
        if !formed {
            return false;
        }
        // No word of the text survives: replacing every word by the first
        // word of its own class leaves the write byte-identical, so nothing
        // of a spelling is carried. A word spelled `c7` is written as its
        // own class, which is not generally class 7.
        let renamed: Vec<Sentence> = t
            .sentences
            .iter()
            .map(|s| {
                let words: Vec<&str> = s.words.iter().map(|w| first(coder, w, t.level)).collect();
                sentence(&words, s.unit)
            })
            .collect();
        write(&renamed, t.level, coder).is_ok_and(|again| again == written)
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

// ----- Property 25: sentence accounting conserves -----

/// Drawn sentence lengths, 0 to 10 words, with 0, 1, 2 and 3 over-
/// represented: half the draws land on or below the threshold.
#[derive(Debug)]
struct Lengths(Vec<usize>);

impl Gen for Lengths {
    fn draw(g: &mut Seeded) -> Self {
        let count = g.below(30) as usize;
        Lengths(
            (0..count)
                .map(|_| match g.below(2) {
                    0 => g.below(4) as usize,
                    _ => g.below(11) as usize,
                })
                .collect(),
        )
    }
}

#[test]
fn property_25_sentence_accounting_conserves() {
    // Property 25: Sentence accounting conserves.
    // **Validates: Requirements 5.15**
    let run = for_all::<Lengths>(0x25, CASES, |l| {
        let text: Vec<Sentence> =
            l.0.iter()
                .enumerate()
                .map(|(u, &n)| sentence(&vec!["the"; n], u))
                .collect();
        let account = Account::of(&text);
        // The two counts sum to the text's, and the two lists partition it.
        if account.total() != text.len() {
            return false;
        }
        let mut union = [account.scored.clone(), account.unshaped.clone()].concat();
        union.sort_unstable();
        if union != (0..text.len()).collect::<Vec<_>>() {
            return false;
        }
        // A sentence is unshaped exactly when it holds fewer than three
        // words, and the gate every Kind count and purity number passes
        // its sentences through agrees.
        (0..text.len()).all(|i| {
            let under = l.0[i] < SHAPED;
            account.unshaped.contains(&i) == under
                && account.scored.contains(&i) == !under
                && account.holds(i) == !under
                && shaped(&text[i]) == !under
        })
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
