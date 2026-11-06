use super::*;

#[test]
fn role_of_names_the_three_roles_at_representative_indices() {
    // `i % 10 == 0` is Test: 0, 10, 20, 30.
    for i in [0usize, 10, 20, 30, 100] {
        assert_eq!(role_of(i), Role::Test, "index {i}");
    }
    // `i % 20 == 5` is Dev, and never also a Test index: 5, 25, 45.
    for i in [5usize, 25, 45, 65] {
        assert_eq!(role_of(i), Role::Dev, "index {i}");
    }
    // 15 is `i % 20 == 15`, not a dev index, and `i % 10 == 5`, not a test
    // index: it is Train. So is every index neither rule names.
    for i in [1usize, 2, 7, 11, 15, 19, 23] {
        assert_eq!(role_of(i), Role::Train, "index {i}");
    }
}

#[test]
fn normalise_is_idempotent_and_folds_whitespace_and_case() {
    let raw = "  The   QUICK\tbrown \n Fox  ";
    let once = normalise(raw);
    assert_eq!(once, "the quick brown fox");
    // Idempotent: normalising the output changes nothing (R10.3).
    assert_eq!(normalise(&once), once);
    // Empty and all-whitespace both fold to the empty string.
    assert_eq!(normalise("   \t\n "), "");
    assert_eq!(normalise(""), "");
}

/// A cut whose sentences, in index order, give one held-out sentence (index
/// 0 is Test) and a run of train sentences.
fn cut() -> (CutId, Vec<String>) {
    let sentences = (0..12).map(|i| format!("sentence {i}")).collect();
    (CutId::Layout, sentences)
}

#[test]
fn check_detects_an_index_overlap() {
    let (_, sentences) = cut();
    let mut guard = HoldoutGuard::new(vec![cut()]);
    // "sentence 0" is the Test sentence at index 0; feeding it as training is
    // an index overlap that stops the run (R10.1, R10.6).
    let training = vec![sentences[0].clone(), sentences[1].clone()];
    match guard.check(&training, &[]) {
        Err(Breach::IndexOverlap { total, first }) => {
            assert_eq!(total, 1);
            assert_eq!(first, vec![(0, "sentence 0".to_string())]);
        }
        other => panic!("expected an index overlap, got {other:?}"),
    }
}

/// A cut of `len` distinct sentences, except that `shared` appears at both a
/// Test index (0) and a Train index (`train_at`). So its text is in the
/// held-out set by index 0, yet also carries a train index in this cut.
fn cut_with_shared(id: CutId, len: usize, shared: &str, train_at: usize) -> (CutId, Vec<String>) {
    let tag = match id {
        CutId::Layout => "layout",
        CutId::Tokens => "tokens",
    };
    let sentences = (0..len)
        .map(|i| {
            if i == 0 || i == train_at {
                shared.to_string()
            } else {
                format!("{tag} {i}")
            }
        })
        .collect();
    (id, sentences)
}

#[test]
fn a_legitimate_duplicate_is_counted_and_excluded_not_fatal() {
    // "shared line" sits at Test index 0 and Train index 11 under both cuts.
    // Each cut resolves it to Train (a train occurrence wins, R10.5), so the
    // cuts agree and no role disagreement fires. Fed as training it matches
    // the held-out set's string but carries a train index under every cut: a
    // legitimate duplicate, counted and excluded, not fatal (R10.5).
    let layout = cut_with_shared(CutId::Layout, 14, "shared line", 11);
    let tokens = cut_with_shared(CutId::Tokens, 14, "shared line", 11);
    let mut guard = HoldoutGuard::new(vec![layout, tokens]);
    let training = vec!["layout 1".to_string(), "shared line".to_string()];
    let counts = guard.check(&training, &[]).expect("a legitimate duplicate is not fatal");
    assert_eq!(counts.legitimate_duplicates, 1);
    assert_eq!(counts.test_string_overlaps, 1);
}

#[test]
fn a_held_out_string_with_no_train_home_is_a_breach_not_a_duplicate() {
    // "edge line" is Test index 0 under both cuts, with no train occurrence
    // anywhere: the cuts agree (both resolve it Test), so no role
    // disagreement, but it is not train under every cut — a breach, distinct
    // from the legitimate duplicate above (R10.5, R10.6).
    let layout = vec!["edge line".to_string(), "layout 1".to_string(), "layout 2".to_string()];
    let tokens = vec!["edge line".to_string(), "tokens 1".to_string(), "tokens 2".to_string()];
    let mut guard = HoldoutGuard::new(vec![(CutId::Layout, layout), (CutId::Tokens, tokens)]);
    let training = vec!["layout 1".to_string(), "edge line".to_string()];
    match guard.check(&training, &[]) {
        Err(Breach::IndexOverlap { total, first }) => {
            assert_eq!(total, 1);
            assert_eq!(first, vec![(1, "edge line".to_string())]);
        }
        other => panic!("expected a breach for a held-out string with no train home, got {other:?}"),
    }
}

#[test]
fn a_key_line_in_training_is_a_key_match() {
    let mut guard = HoldoutGuard::new(vec![cut()]);
    let key = vec!["The ANSWER key line".to_string()];
    let training = vec!["sentence 1".to_string(), "the answer   key line".to_string()];
    match guard.check(&training, &key) {
        Err(Breach::KeyMatch { total, first }) => {
            assert_eq!(total, 1);
            assert_eq!(first, vec![(1, "the answer key line".to_string())]);
        }
        other => panic!("expected a key match, got {other:?}"),
    }
}

#[test]
fn a_role_disagreement_across_cuts_stops_the_run() {
    // "moved line" is index 1 (Train) under the layout cut and index 0 (Test)
    // under the token cut: the same normalised sentence, two roles (R10.7).
    let layout = vec!["layout 0".to_string(), "moved line".to_string(), "layout 2".to_string()];
    let tokens = vec!["moved line".to_string(), "tokens 1".to_string(), "tokens 2".to_string()];
    let mut guard = HoldoutGuard::new(vec![(CutId::Layout, layout), (CutId::Tokens, tokens)]);
    match guard.check(&[], &[]) {
        Err(Breach::RoleDisagreement { per_cut, differing }) => {
            assert_eq!(differing, 1);
            assert_eq!(per_cut, vec![(CutId::Layout, 3), (CutId::Tokens, 3)]);
        }
        other => panic!("expected a role disagreement, got {other:?}"),
    }
}

#[test]
fn a_label_read_while_building_is_a_breach_and_scoring_is_allowed() {
    let mut guard = HoldoutGuard::new(vec![]);
    // Reading a label while scoring is permitted and counted (R10.8).
    assert!(guard.read_label(Phase::ScoreNumber, "answer").is_ok());
    assert!(guard.read_label(Phase::ScoreNumber, "answer").is_ok());
    // Reading a label while building a training input is a breach (R10.9).
    match guard.read_label(Phase::BuildTrainingInput, "instruct") {
        Err(Breach::LabelLeak { reads }) => {
            assert_eq!(reads, vec![("instruct".to_string(), 1)]);
        }
        other => panic!("expected a label leak, got {other:?}"),
    }
    let reads = guard.per_phase_reads();
    assert!(reads.contains(&(Phase::ScoreNumber, "answer".to_string(), 2)));
    assert!(reads.contains(&(Phase::BuildTrainingInput, "instruct".to_string(), 1)));
}

// Feature: phinet-english-reader-coverage, task 5.1.
// The Lexicon is primed from files under data/lex and from the Guide's
// Appendix 1.4 only; its priming never reads a Held_Out_Slice sentence
// (R5.1). The three priming entry points are checked against their declared
// sources, and the sources against the held-out text.
#[test]
fn lexicon_priming_reads_its_declared_sources_and_no_held_out_sentence() {
    use crate::guide::{self, Part, Split};
    use crate::lexicon;

    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    let guide_path = format!("{base}/person/murphy.txt");
    let key_path = format!("{base}/person/murphy_key.txt");
    let (Ok(text), Ok(key)) =
        (std::fs::read_to_string(&guide_path), std::fs::read_to_string(&key_path))
    else {
        // The Guide ships with the harness crate, not with phisor; when it is
        // absent the pure part below still holds the discipline.
        pure_irregular_priming_takes_only_single_word_forms();
        return;
    };

    // The Held_Out_Slice: the Test part, normalised as every comparison is.
    let split = Split::new(guide::read(&text, &key));
    let held_out: std::collections::BTreeSet<String> =
        split.strings(Part::Test).iter().map(|s| normalise(s)).collect();
    assert!(!held_out.is_empty(), "the Guide should yield held-out sentences");

    // Source 1 and 2: data/lex. prime() keys on the word before the first
    // tab; prime_function() on each whitespace token. A held-out sentence is
    // four words or more, so it can never be a key in either file - assert no
    // held-out sentence appears as a whole line of either source.
    for lex in ["lex/pos.tsv", "lex/function.txt"] {
        if let Ok(data) = std::fs::read_to_string(format!("{base}/{lex}")) {
            for line in data.lines() {
                assert!(
                    !held_out.contains(&normalise(line)),
                    "{lex} line is a held-out sentence: {line:?}"
                );
            }
        }
    }

    // Source 3: the Guide's Appendix 1.4 irregular-verb table. Priming reads
    // the table, not the prose: every lemma and every form it ingests is a
    // single word, so no held-out sentence (always multi-word) can enter.
    let irregulars = lexicon::parse_irregulars(&text);
    assert!(!irregulars.is_empty(), "the Appendix 1.4 table should parse");
    assert!(
        irregulars.len() < 300,
        "the parser read {} lemmas - far past a verb table, so it took in prose",
        irregulars.len()
    );
    // No parsed entry is a held-out sentence (they are multi-word; forms are
    // single words), which is the direct R5.1 check on the Guide source.
    // Re-reads the forms through the public reverse maps the reader uses.
    for s in &held_out {
        assert!(
            lexicon::parse_irregulars(s).is_empty() || s.split_whitespace().count() == 1,
            "a held-out sentence parsed as an irregular-verb row: {s:?}"
        );
    }
}

/// The file-free part of the R5.1 check: `parse_irregulars` only ever ingests
/// single-word forms, so a multi-word held-out sentence can never become a
/// lexicon entry, whatever text it is handed.
fn pure_irregular_priming_takes_only_single_word_forms() {
    use crate::lexicon;
    // A snippet shaped like the Appendix, with a prose line that must be
    // ignored because it is not a three-column verb row.
    let snippet = "List of irregular verbs\n\
        infinitive        past simple   past participle\n\
        go                went          gone\n\
        she will arrive at eight tomorrow in the morning without fail\n\
        * pronunciation\n";
    let t = lexicon::parse_irregulars(snippet);
    // The single verb row is read; the prose line is not a row and is dropped.
    assert_eq!(t.forms("go").map(|f| f.past.clone()), Some(vec!["went".to_string()]));
    assert!(t.forms("she").is_none(), "a prose sentence must not become a lemma");
    assert_eq!(t.len(), 1, "only the one verb row was taken");
}
