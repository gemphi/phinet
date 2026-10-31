//! The word height's example tests — the range gate, the Mode gate, the
//! write in all three arms and the unshaped accounting — and the corpus the
//! properties of `property.rs` draw over.

use super::*;
use crate::code::Coder;
use crate::guide::{Function, Mark};

/// The vocabulary a drawn text is written from. `c3`, `c7` and `c12` are
/// spelled exactly like class tokens, so a leaked word cannot hide behind
/// the token form; the rest are ordinary words of the guide.
pub(super) const VOCAB: [&str; 16] = [
    "c3", "c7", "c12", "the", "cat", "sat", "on", "mat", "were", "you", "waiting", "in", "rain",
    "yes", "i", "was",
];

pub(super) fn sentence(words: &[&str], unit: usize) -> Sentence {
    Sentence {
        words: words.iter().map(|w| w.to_string()).collect(),
        function: Function::Explain,
        mark: Mark::Stop,
        unit,
        run: 0,
    }
}

/// A fixed training corpus: every word told more than twice, so the
/// Learned arm's code has something to grow from.
pub(super) fn corpus() -> Vec<Sentence> {
    let said: [&[&str]; 4] = [
        &["the", "cat", "sat", "on", "the", "mat"],
        &["were", "you", "waiting", "in", "the", "rain"],
        &["yes", "i", "was", "waiting", "in", "the", "rain"],
        &["c7", "c12", "c3", "the", "cat", "sat"],
    ];
    (0..12)
        .flat_map(|u| said.into_iter().map(move |w| sentence(w, u)))
        .collect()
}

/// The first word of the vocabulary sharing `word`'s class: the rename
/// Property 21 writes again through.
pub(super) fn first<'a>(coder: &Coder, word: &'a str, level: Level) -> &'a str {
    let class = coder.class(word, level);
    VOCAB
        .iter()
        .copied()
        .find(|w| coder.class(w, level) == class)
        .unwrap_or(word)
}

#[test]
fn the_rename_property_twenty_one_writes_again_through_really_changes_the_words() {
    // The rename check in Property 21 is only worth running if it moves
    // spellings. The Given arm reads eleven parts of speech, so the
    // vocabulary's nouns all collapse onto the first of them.
    let coder = Coder::Given;
    let moved = VOCAB
        .iter()
        .filter(|w| first(&coder, w, Level::P16) != **w)
        .count();
    assert!(
        moved >= 5,
        "the rename moved {moved} of {} words",
        VOCAB.len()
    );
    // And a word spelled like a class token is written as its own class,
    // which is not the number it spells.
    let written = write(&[sentence(&["c7"], 0)], Level::P64, &Coder::Hash).unwrap();
    assert_ne!(
        show(&written),
        ["c7".to_string()],
        "`c7` was written as itself"
    );
}

#[test]
fn a_text_is_written_as_one_class_token_per_word_in_every_arm_and_every_mode() {
    let split = Split::new(corpus());
    let ladder = Ladder::new(&split, Seeds::DECLARED);
    let text = [
        sentence(&["the", "cat", "sat", "on", "the", "mat"], 0),
        sentence(&["c7", "c12"], 1),
    ];
    for arm in Arm::ALL {
        for mode in crate::code::LEVELS {
            let written = ladder.write_mode(&text, mode, arm).unwrap();
            assert_eq!(written.len(), 2, "{} at {}", arm.name(), named(mode));
            assert_eq!(written.iter().map(Vec::len).collect::<Vec<_>>(), [6, 2]);
            // Structure only: every token is a class, and the classes stay
            // inside the mode's alphabet.
            let alphabet = ladder.coder(arm, Grow::Fixed(3)).alphabet(mode) as u16;
            assert!(
                written.iter().flatten().all(|&c| c < alphabet),
                "{written:?} at {}",
                named(mode)
            );
            // The repeated word is written the same way both times.
            assert_eq!(written[0][0], written[0][4]);
        }
    }
}

#[test]
fn a_text_outside_one_to_ten_thousand_sentences_names_its_count_and_writes_nothing() {
    let one = [sentence(&["the", "cat", "sat"], 0)];
    assert_eq!(
        write(&one, Level::P16, &Coder::Hash).map(|w| w.len()),
        Ok(1)
    );
    let most: Vec<Sentence> = (0..MOST)
        .map(|u| sentence(&["the", "cat", "sat"], u))
        .collect();
    assert_eq!(
        write(&most, Level::P16, &Coder::Hash).map(|w| w.len()),
        Ok(MOST)
    );
    // Nothing written at either end: the error carries the count given.
    assert_eq!(
        write(&[], Level::P16, &Coder::Hash),
        Err(LadderError::TextOutOfRange { sentences: 0 })
    );
    let over: Vec<Sentence> = (0..MOST + 1)
        .map(|u| sentence(&["the", "cat", "sat"], u))
        .collect();
    let flaw = write(&over, Level::P64, &Coder::Hash).unwrap_err();
    assert_eq!(
        flaw,
        LadderError::TextOutOfRange {
            sentences: MOST + 1
        }
    );
    assert_eq!(
        flaw.to_string(),
        "a text of 10001 sentences is outside 1 to 10000"
    );
}

#[test]
fn a_mode_that_is_not_sixteen_thirty_two_or_sixty_four_names_what_it_accepts() {
    assert_eq!(mode("16"), Ok(Level::P16));
    assert_eq!(mode(" P32 "), Ok(Level::P32));
    assert_eq!(mode("p64"), Ok(Level::P64));
    assert_eq!(crate::code::LEVELS.map(named), MODES);
    assert_eq!(MODES.map(|m| mode(m).unwrap()), crate::code::LEVELS);
    let flaw = mode("8").unwrap_err();
    assert_eq!(
        flaw,
        LadderError::UnknownMode {
            given: "8".to_string(),
            accepted: MODES
        }
    );
    assert_eq!(flaw.to_string(), "mode \"8\" is not one of 16, 32 or 64");
    // The gate holds on the way in, so an unknown Mode writes nothing.
    let split = Split::new(corpus());
    let ladder = Ladder::new(&split, Seeds::DECLARED);
    let text = [sentence(&["the", "cat", "sat"], 0)];
    // The error names the Mode actually given, whichever unknown one it is.
    let given = ladder.write_named(&text, "128", Arm::Learned);
    assert_eq!(
        given,
        Err(LadderError::UnknownMode {
            given: "128".to_string(),
            accepted: MODES
        })
    );
    assert_eq!(
        ladder.write_named(&text, "64", Arm::Given).map(|w| w.len()),
        Ok(1)
    );
    assert_eq!(
        ladder.write_named(&[], "64", Arm::Given),
        Err(LadderError::TextOutOfRange { sentences: 0 })
    );
}

#[test]
fn a_sentence_under_three_words_is_unshaped_counted_and_never_scored() {
    let text = [
        sentence(&[], 0),
        sentence(&["yes"], 1),
        sentence(&["yes", "i"], 2),
        sentence(&["yes", "i", "was"], 3),
        sentence(&["the", "cat", "sat", "on", "the", "mat"], 4),
    ];
    let split = Split::new(corpus());
    let account = Ladder::new(&split, Seeds::DECLARED).account(&text);
    assert_eq!(account.unshaped, [0, 1, 2]);
    assert_eq!(account.scored, [3, 4]);
    assert_eq!(account.total(), text.len());
    assert!(!account.holds(2) && account.holds(3));
    // An unshaped sentence is still written: it is left out of the counts,
    // not out of the text.
    assert_eq!(
        write(&text, Level::P16, &Coder::Hash).unwrap().len(),
        text.len()
    );
}

#[test]
fn the_six_heights_and_the_three_arms_are_named() {
    assert_eq!(
        Height::ALL.map(Height::name),
        ["words", "mode", "variation", "kind", "act", "rows"]
    );
    assert_eq!(
        Height::ALL.map(|h| Height::of(h.name())),
        Height::ALL.map(Some)
    );
    assert_eq!(Height::of("river"), None);
    assert_eq!(Arm::ALL.map(Arm::name), ["hash_null", "given", "learned"]);
    // The seeds a run repeats from are the declared ones by default, and
    // the move order is the routine's own.
    assert_eq!(Seeds::default(), Seeds::DECLARED);
    // The move order is the exchange routine's own default: the fixed
    // most-told-first order, which its settings name `0`.
    assert_eq!(Seeds::DECLARED.move_order, 0);
    let split = Split::new(corpus());
    let ladder = Ladder::new(&split, Seeds::DECLARED);
    assert_eq!(ladder.seeds(), Seeds::DECLARED);
    assert_eq!(ladder.split().sentences.len(), corpus().len());
}
