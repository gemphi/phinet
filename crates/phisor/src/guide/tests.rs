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

#[test]
fn removed_by_counts_each_operators_rejections_and_they_sum_with_the_kept() {
    let mk = |w: &str| Sentence { words: w.split_whitespace().map(str::to_string).collect(), function: Function::Explain, mark: Mark::Stop, unit: 1, run: 0 };
    // Index 0 and 10 are Test; the nine between are Train.
    let mut s = vec![mk("or negative i'm not doing etc")];
    s.extend((0..9).map(|_| mk("filler words go in here now")));
    s.push(mk("shut shut it was cold so"));
    let split = Split::new(s);
    let counts = split.removed_by(Part::Test);
    let names: Vec<&str> = counts.iter().map(|c| c.0).collect();
    assert_eq!(names, ["no_etc", "no_stray_letter", "no_doubling", "no_leading_or", "no_label_a"]);
    let by = |n: &str| counts.iter().find(|c| c.0 == n).map(|c| c.1);
    assert_eq!(by("no_etc"), Some(1));
    assert_eq!(by("no_leading_or"), Some(1));
    assert_eq!(by("no_doubling"), Some(1));
    assert_eq!(by("no_stray_letter"), Some(0));
    assert!(split.clean_strings(Part::Test).is_empty());
}
/// One (accepted, rejected) example per operator, by name.
const OPERATOR_EXAMPLES: [(&str, &str, &str); 5] = [
    ("no_etc", "anna started evening classes.", "she left etc."),
    ("no_stray_letter", "anna started evening classes.", "pick b now."),
    ("no_doubling", "anna started evening classes.", "shut shut it was cold."),
    ("no_leading_or", "anna started evening classes.", "or she stayed home."),
    ("no_label_a", "a dog barked loudly.", "a she was here."),
];

#[test]
fn each_operator_accepts_and_rejects_its_example() {
    use super::split::OPERATORS;
    assert_eq!(OPERATORS.len(), OPERATOR_EXAMPLES.len());
    for (op, (name, ok, bad)) in OPERATORS.iter().zip(OPERATOR_EXAMPLES) {
        assert_eq!(op.name, name);
        assert!(!(op.rejects)(ok), "{name} should accept {ok:?}");
        assert!((op.rejects)(bad), "{name} should reject {bad:?}");
    }
}

// Feature: phinet-english-reader-coverage, Property 2: Text-only.
// **Validates: Requirements 7.2, 7.6**
#[test]
fn operator_verdicts_do_not_depend_on_the_lexicon() {
    use super::split::OPERATORS;
    let probes: Vec<&str> = OPERATOR_EXAMPLES.iter().flat_map(|(_, a, b)| [*a, *b]).collect();
    let verdicts = || -> Vec<Vec<bool>> {
        OPERATORS.iter().map(|o| probes.iter().map(|s| (o.rejects)(s)).collect()).collect()
    };
    let before = verdicts();
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) {
        crate::lexicon::prime(&p);
    } else {
        crate::lexicon::prime("she\tn\nleft\tv\n");
    }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) {
        crate::lexicon::prime_function(&f);
    }
    assert_eq!(before, verdicts(), "a primed Lexicon changed an operator verdict");
}

