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


#[test]
fn held_out_strings_reconstruct_the_mark_and_skip_the_unmeasured() {
    // A stop and a question reconstruct with their mark; a line-broken or
    // exclamation sentence is skipped (the reader measures neither mark).
    let s = vec![
        Sentence { words: vec!["she".into(), "has".into(), "arrived".into(), "home".into()], function: Function::Explain, mark: Mark::Stop, unit: 1, run: 0 },
        Sentence { words: vec!["w".into(); 4], function: Function::Explain, mark: Mark::Line, unit: 1, run: 0 },
    ];
    let split = Split::new(s);
    // Index 0 is Test by the one-in-ten rule; it reconstructs with its stop.
    let strings = split.strings(Part::Test);
    assert_eq!(strings, vec!["she has arrived home.".to_string()]);
}

#[test]
fn clean_strings_drop_exercise_scaffolding_and_keep_prose() {
    let mk = |w: &str| Sentence { words: w.split_whitespace().map(str::to_string).collect(), function: Function::Explain, mark: Mark::Stop, unit: 1, run: 0 };
    // Index 0 is Test; build ten so only index 0 and 10 are Test-part.
    let mut s = vec![mk("or negative i'm not doing etc")];
    s.extend((0..9).map(|_| mk("filler words go in here now")));
    s.push(mk("anna started evening classes recently"));
    s.push(mk("shut shut it was cold so"));
    let split = Split::new(s);
    let kept = split.clean_strings(Part::Test);
    assert_eq!(kept, vec!["anna started evening classes recently.".to_string()]);
}

// Feature: phinet-english-reader-coverage, Property 1: Filter stability.
// **Validates: Requirements 7.1, 7.2**
#[test]
fn the_operator_table_agrees_with_the_old_well_formed_on_every_guide_sentence() {
    use super::split::{well_formed, well_formed_old};
    // Hand-built sentences covering each operator's accept and reject sides.
    let mut all: Vec<String> = [
        "anna started evening classes recently.",
        "she left etc.",
        "pick b or c.",
        "shut shut it was cold.",
        "or she stayed home.",
        "a she was here.",
        "a they're late.",
        "a dog barked loudly.",
        "i think a lot.",
        "was she wearing a hat?",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    // The real Guide, every part, when it is present.
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data/person");
    let read = |f: &str| std::fs::read_to_string(format!("{base}/{f}"));
    if let (Ok(text), Ok(key)) = (read("murphy.txt"), read("murphy_key.txt")) {
        let split = Split::new(super::read(&text, &key));
        for part in [Part::Train, Part::Dev, Part::Test] {
            all.extend(split.strings(part));
        }
    } else {
        println!("no guide");
    }
    for s in &all {
        assert_eq!(well_formed(s), well_formed_old(s), "verdicts differ on {s:?}");
    }
}
