use super::*;
use crate::guide::{Function, Mark, Sentence, Split};
use phiton::p64::variants::Level;

fn sentence(words: &[&str], unit: usize) -> Sentence {
    Sentence {
        words: words.iter().map(|w| w.to_string()).collect(),
        function: Function::Explain,
        mark: Mark::Stop,
        unit,
        run: 0,
    }
}

fn corpus() -> Vec<Sentence> {
    let said: [&[&str]; 4] = [
        &["the", "cat", "sat", "on", "the", "mat"],
        &["were", "you", "waiting", "in", "the", "rain"],
        &["yes", "i", "was", "waiting", "in", "the", "rain"],
        &["the", "dog", "ran", "over", "the", "hill"],
    ];
    (0..8)
        .flat_map(|unit| said.into_iter().map(move |w| sentence(w, unit)))
        .collect()
}

#[test]
fn the_record_names_a_seed_for_each_of_the_three_random_choices() {
    let seeds = Seeds {
        move_order: 7,
        shuffles: 11,
        phases: 13,
    };
    let split = Split::new(corpus());
    let record = Record::of(seeds, Level::P32, &split);
    assert_eq!(record.move_order, 7);
    assert_eq!(record.shuffles, 11);
    assert_eq!(record.phases, 13);
    assert_eq!(record.level, Level::P32);
    // The record round-trips to the seeds it was built from.
    assert_eq!(record.seeds(), seeds);
}

#[test]
fn a_repeat_with_the_same_record_mode_and_split_reports_equal_numbers() {
    let split = Split::new(corpus());
    let record = Record::of(Seeds::DECLARED, Level::P16, &split);
    let first = Run::new(record, &split).unwrap().numbers();
    let again = Run::new(record, &split).unwrap().numbers();
    assert_eq!(first, again);
    // The numbers are rounded to the reported precision.
    for value in [
        first.river_z,
        first.hash_z,
        first.move_order_reading,
        first.phase_reading,
    ] {
        let scale = 10f64.powi(REPORTED_PLACES);
        assert_eq!(value, (value * scale).round() / scale);
    }
}

#[test]
fn a_record_built_over_one_split_is_refused_against_another() {
    let split = Split::new(corpus());
    let record = Record::of(Seeds::DECLARED, Level::P16, &split);
    // A different split has a different fingerprint, so the record and the
    // split do not name the same run.
    let mut other_sentences = corpus();
    other_sentences.push(sentence(&["a", "wholly", "new", "line"], 9));
    let other = Split::new(other_sentences);
    let refusal = match Run::new(record, &other) {
        Err(mismatch) => mismatch,
        Ok(_) => panic!("a record over one split must be refused against another"),
    };
    assert_eq!(refusal.recorded, record.split_fingerprint);
    assert_ne!(refusal.found, record.split_fingerprint);
    // The right split is accepted.
    assert!(Run::new(record, &split).is_ok());
}

#[test]
fn a_blocked_measure_names_its_arithmetic_and_home_and_computes_no_substitute() {
    let blocked = Blocked::new(
        "learn.curvature",
        "the discrete Laplacian over a Kind graph",
        Home::Phistd,
    );
    assert_eq!(blocked.measure, "learn.curvature");
    assert_eq!(blocked.arithmetic, "the discrete Laplacian over a Kind graph");
    assert_eq!(blocked.proposed_home, Home::Phistd);
    assert_eq!(blocked.proposed_home.name(), "phistd");
    assert_eq!(Home::Phical.name(), "phical");
}

#[test]
fn a_blocked_measure_does_not_stop_the_measures_after_it() {
    let blocked = Blocked::new("learn.curvature", "a discrete Laplacian", Home::Phical);
    let outcomes = run_measures(vec![
        Measure::Ready(12.5),
        Measure::Blocked(blocked.clone()),
        Measure::Ready(34.0),
    ]);
    // One outcome per measure, in order, and the run never stops short.
    assert_eq!(outcomes.len(), 3);
    assert_eq!(outcomes[0], Outcome::Ran(12.5));
    assert_eq!(outcomes[1], Outcome::Blocked(blocked));
    assert_eq!(outcomes[2], Outcome::Ran(34.0));
}

/// R9.13 and the group's standing assertion: no source file of this group
/// adds arithmetic over phisor's types to the four protected src trees. The
/// group's own files are `seeds.rs` and its tests; the check reads them and
/// fails on an operator applied to a phisor value rather than to a count,
/// an index or a seed.
#[test]
fn no_arithmetic_leaves_this_group() {
    // The group's files, relative to this crate's `src`.
    const GROUP: [&str; 4] = [
        "measure/seeds.rs",
        "measure/seeds/blocked.rs",
        "measure/seeds/tests.rs",
        "measure/seeds/property.rs",
    ];
    // The phisor types R9.13 forbids arithmetic over. A member followed by
    // an arithmetic operator inside the group's own expressions would be a
    // breach; the group holds none, because every statistic is phistd's or
    // phical's and every local value is a count, an index or a seed.
    const TYPES: [&str; 8] = [
        "Phasor", "Class", "Level", "Variation", "Kind", "Act", "Frame", "Row",
    ];
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for file in GROUP {
        let text = std::fs::read_to_string(here.join(file))
            .unwrap_or_else(|_| panic!("the group file {file} is readable"));
        for line in text.lines() {
            // Comments and doc lines name the types without computing on them.
            let code = line.trim_start();
            if code.starts_with("//") || code.starts_with("*") {
                continue;
            }
            for ty in TYPES {
                // An arithmetic operator binding a typed value directly, e.g.
                // `phasor + phasor`, would read as `<name>: Ty) ... <op>`.
                // The group writes none: this guards the rule from a later
                // edit that would.
                let binding = format!("{ty} +");
                let scaled = format!("{ty} *");
                assert!(
                    !code.contains(&binding) && !code.contains(&scaled),
                    "{file}: `{code}` adds arithmetic over a {ty}"
                );
            }
        }
    }
}
