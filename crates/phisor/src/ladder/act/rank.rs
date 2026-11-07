//! Ranked candidate Reader changes (task 7.1, requirement 6.1).
//!
//! Baseline: clean 1201, read 388, abstained 813; clean coverage 388/1201,
//! soundness 309/388 (79 unsound readings).
//!
//! Abstention by distance bucket (`abstention_kinds`): simple 305,
//! one_join 274, infinitive 122, ing_form 76, multi_clause 26, very_long 10.
//!
//! Gain = held-out abstained sentences containing the trigger word, from
//! `abstention_words` (words in >= 12 abstained sentences and >= 90%
//! abstained). Counts are upper bounds: sentences with several triggers are
//! counted under each, and no fix was prototyped. Known defects are counted
//! as unsound readings addressed; per-defect totals were not measured, so
//! only the exemplar is counted (1 each, a lower bound).
//!
//! | rank | candidate change                              | gain | bucket         |
//! |------|-----------------------------------------------|------|----------------|
//! | 1    | `but` coordination                            | 54   | one_join       |
//! | 2    | `or` coordination                             | 42   | one_join       |
//! | 3    | `not` negation without an auxiliary           | 39   | simple         |
//! | 4    | `if` conditional clause                       | 31   | one_join       |
//! | 5    | existential `there`                           | 30   | simple         |
//! | 6    | `ago` / `last` time adverbials (15 + 12)      | 27   | simple         |
//! | 7    | `more` / `much` / `few` quantifiers (15+15+13)| 43*  | simple         |
//! | 8    | `would` / `i'd` modal contraction (15 + 15)   | 30*  | simple         |
//! | 9    | plural `friend(s)` / `things` / `here`        | 12-37| simple         |
//! | 10   | irregular past `went`                         | 12   | simple         |
//! | -    | `it` dropped in "it was not raining when ..." | 1+   | unsound        |
//! | -    | `that` dropped in "she said that ..."         | 1+   | unsound        |
//! | -    | "learn" becomes "learns" after `suggested`    | 1+   | unsound        |
//! | -    | `very` dropped in "walked very carefully"     | 1+   | unsound        |
//! | -    | "insisted me anyway" read as "did not"        | 1+   | unsound        |
//!
//! `*` summed over words that may co-occur, so the true gain is lower.
//! Ranking by sound-reading gain: items 1-5 first; `but`/`or`/`if` share the
//! one_join bucket (274) and `not`/`there` the simple bucket (305).

/// The ranking above, as data: (trigger, abstained sentences it marks).
const RANKED: [(&str, usize); 10] = [
    ("but", 54),
    ("or", 42),
    ("not", 39),
    ("if", 31),
    ("there", 30),
    ("ago+last", 27),
    ("more+much+few", 43),
    ("would+i'd", 30),
    ("friend(s)+things+here", 37),
    ("went", 12),
];

/// Inputs: `cargo test -p phisor abstention_kinds -- --ignored --nocapture`
/// and `abstention_words`. Prints the table.
#[test]
#[ignore = "diagnostic: prints the ranked candidate changes"]
fn rank_candidates() {
    for (i, (word, gain)) in RANKED.iter().enumerate() {
        println!("{:>2} {:>3} {word}", i + 1, gain);
    }
}
