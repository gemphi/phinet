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
