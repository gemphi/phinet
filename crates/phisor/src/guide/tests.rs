use super::*;

const UNITS: &str = "\
Contents
Unit
6  Past continuous (I was doing)
A girl was wearing a blue suit. Was she wearing a hat?
What a lovely day it is!
the layout broke this sentence across
3 They were waiting in the rain.
6.1    Complete the sentences using the past continuous.
Key to Exercises
";

const KEY: &str = "\
Key to Exercises (contents)
Key to Exercises
UNIT 6
6.1
1 was wearing a white cap / wore a white cap
2 was
3 they were waiting
in the rain
Key to Additional Exercises
";

#[test]
fn every_piece_keeps_its_mark_and_what_it_is_for() {
    let s = read(UNITS, KEY);
    let seen: Vec<(Function, Mark, usize)> = s.iter().map(|x| (x.function, x.mark, x.words.len())).collect();
    assert_eq!(
        seen,
        vec![
            (Function::Explain, Mark::Stop, 7),
            (Function::Explain, Mark::Question, 5),
            (Function::Explain, Mark::Exclamation, 6),
            (Function::Explain, Mark::Line, 6),
            (Function::Explain, Mark::Stop, 6),
            (Function::Instruct, Mark::Stop, 7),
            (Function::Answer, Mark::Line, 5),
            (Function::Answer, Mark::Line, 6),
        ]
    );
    assert_eq!(s[0].words, ["a", "girl", "was", "wearing", "a", "blue", "suit"]);
    assert_eq!(s[1].unit, 6);
    // An item's number is dropped, as the Key's reader drops the Key's.
    assert_eq!(s[4].words, ["they", "were", "waiting", "in", "the", "rain"]);
    // The first alternative only; a wrapped answer is joined.
    assert_eq!(s[6].words, ["was", "wearing", "a", "white", "cap"]);
    assert_eq!(s[7].words, ["they", "were", "waiting", "in", "the", "rain"]);
}

#[test]
fn the_split_holds_out_one_in_ten_and_every_answer() {
    let sentences: Vec<Sentence> = (0..40)
        .map(|i| Sentence { words: vec!["w".into(); 4], function: Function::Explain, mark: Mark::Stop, unit: i / 20 + 1, run: 0 })
        .chain((0..3).map(|_| Sentence { words: vec!["w".into(); 4], function: Function::Answer, mark: Mark::Line, unit: 1, run: 1 }))
        .collect();
    let split = Split::new(sentences);
    assert_eq!(split.of(Part::Test), vec![0, 10, 20, 30, 40, 41, 42]);
    assert_eq!(split.of(Part::Dev), vec![5, 25]);
    // A run never spans a held-out sentence, a new unit, or the Key.
    let runs = split.training_runs();
    assert_eq!(runs[0], vec![1, 2, 3, 4]);
    assert_eq!(runs[1], vec![6, 7, 8, 9]);
    assert!(runs.iter().all(|r| r.windows(2).all(|w| split.before(w[1]) == Some(w[0]))));
    assert_eq!(split.before(20), None);
}
