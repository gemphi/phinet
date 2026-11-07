//! Property 4: the reader returns whole frames or none.

use super::*;

// Property 4: no partial frames (Requirement 3.4). Bounded, seeded generation
// with no new dependency: a hand-rolled LCG as in abstraction/tests.rs.
fn lcg(s: &mut u64) -> usize {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*s >> 33) as usize
}

/// A word is filled when it is non-empty and a single token.
fn filled_word(w: &str) -> bool {
    !w.is_empty() && !w.chars().any(char::is_whitespace)
}

/// A filler is whole when it names something, and every clause it holds is a
/// whole frame. A hole is whole only in a question.
fn whole_filler(f: &crate::ir::Filler, asks: bool) -> bool {
    use crate::ir::Filler;
    match f {
        Filler::Thing(t) => {
            filled_word(&t.head) && t.attrs.iter().all(|(_, v)| whole_filler(v, asks))
        }
        Filler::Name(w) | Filler::Pronoun(w) | Filler::Word(w) => filled_word(w),
        Filler::Hole(_) => asks,
        Filler::Clause(inner) => whole_frame(inner, asks, false),
    }
}

/// A frame is whole when it has a predicate, every slot is filled, and every
/// clause slot holds a whole frame. Only a command (the subject is the hearer)
/// or a weather verb may have no slot at all.
fn whole_frame(frame: &crate::ir::Frame, asks: bool, top_may_be_bare: bool) -> bool {
    filled_word(&frame.pred)
        && (top_may_be_bare || needs_expletive(frame) || !frame.slots.is_empty())
        && frame.slots.iter().all(|(_, f)| whole_filler(f, asks))
}

/// What the Reader returned for `sentence` must be nothing or a whole Act that
/// can be written back: it realises to non-empty text with a terminating mark.
/// Whether that text equals the input is the measured soundness rate, not this
/// property (design, Property 4).
fn assert_whole_or_none(sentence: &str) {
    use crate::ir::Force;
    let Some(act) = read(sentence) else { return };
    let asks = act.force == Force::Ask;
    let bare_ok = act.force == Force::Direct;
    assert!(whole_frame(&act.frame, asks, bare_ok), "partial frame for {sentence:?}: {:?}", act.frame);
    let text = realise(&act);
    assert!(text.ends_with(|c| MARKS.contains(&c)), "{sentence:?} realised without a mark: {text:?}");
}

/// Sentences the tasks 6.1-6.4 added (trailing clauses, multi-word adverbs,
/// purpose infinitives, -ing forms), plus plain ones they must not disturb.
const READER_SEEDS: [&str; 32] = [
    // Tasks 3.1-3.4: a pronoun as a verb, `and` as a head, host + `to` + verb,
    // and regular `-ed` forms.
    "she she went.",
    "he she works here.",
    "and is here.",
    "the and went home.",
    "karen and.",
    "she has a book to read.",
    "he has nothing to read.",
    "he went to read.",
    "he added the sugar.",
    "she missed the bus.",
    "he works in a shop and she stays at home and they are happy.",
    "i was tired so i went to bed and i slept well.",
    "he works in a shop and she stays at home and.",
    "she is here right now.",
    "he works in a shop right now.",
    "i enjoyed reading the book.",
    "they avoided stealing the money.",
    "i walk reading the book.",
    "i enjoy reading the.",
    "he went to the library to read a book.",
    "she came to open the door.",
    "he went somewhere to read a book.",
    "he went to the library to read a book at.",
    "i want to open the door.",
    "she went to london.",
    "he watching two hours every morning.",
    "karen and joe played tennis.",
    "who was the first king of france?",
    "did you read the book?",
    "do not open the door.",
    "it rains.",
    "i wish i could sing.",
];

/// Words spliced into variants: conjunctions, `to`, `-ing` forms, adverbs,
/// the two known weak heads (`and` as a noun, `to read` as `to` + noun).
const SPLICE: [&str; 16] = [
    "and", "to", "but", "so", "reading", "right", "now", "the", "a", "read", "book", "he", "enjoy", "went", "at", "not",
];

/// One variant of a seed: truncate, drop, repeat, swap or insert a word.
fn variant(seed: &str, s: &mut u64) -> String {
    let body = seed.trim_end_matches(|c| MARKS.contains(&c));
    let mut words: Vec<String> = body.split_whitespace().map(str::to_string).collect();
    for _ in 0..1 + lcg(s) % 3 {
        if words.is_empty() {
            break;
        }
        let i = lcg(s) % words.len();
        match lcg(s) % 5 {
            0 => words.truncate(i),
            1 => {
                words.remove(i);
            }
            2 => words.insert(i, words[i].clone()),
            3 => words[i] = SPLICE[lcg(s) % SPLICE.len()].to_string(),
            _ => words.insert(i, SPLICE[lcg(s) % SPLICE.len()].to_string()),
        }
    }
    let mark = ['.', '.', '?'][lcg(s) % 3];
    format!("{}{mark}", words.join(" "))
}

#[test]
fn the_reader_returns_whole_frames_or_none() {
    // Every seed, whole.
    for seed in READER_SEEDS {
        assert_whole_or_none(seed);
    }
    // And bounded seeded variants of them: truncations, drops, repeats, swaps.
    let mut s = 0x5eed_u64;
    for _ in 0..4000 {
        let seed = READER_SEEDS[lcg(&mut s) % READER_SEEDS.len()];
        assert_whole_or_none(&variant(seed, &mut s));
    }
    // And every prefix of every seed: a truncated sentence is never half read.
    for seed in READER_SEEDS {
        let words: Vec<&str> = seed.trim_end_matches(|c| MARKS.contains(&c)).split_whitespace().collect();
        for n in 0..=words.len() {
            assert_whole_or_none(&format!("{}.", words[..n].join(" ")));
        }
    }
    // And every clean Guide sentence (Property 4), when the Guide is present.
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data/person");
    let (text, key) = (
        std::fs::read_to_string(format!("{base}/murphy.txt")),
        std::fs::read_to_string(format!("{base}/murphy_key.txt")),
    );
    let (Ok(text), Ok(key)) = (text, key) else { return };
    let split = crate::guide::Split::new(crate::guide::read(&text, &key));
    for part in [crate::guide::Part::Train, crate::guide::Part::Dev, crate::guide::Part::Test] {
        for sentence in split.clean_strings(part) {
            assert_whole_or_none(&sentence);
        }
    }
}
