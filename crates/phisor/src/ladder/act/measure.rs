//! Moved from `ladder/act.rs` unchanged.
use super::*;

/// The closed word lists the Reader consults, each as a named list of
/// lower-case words, as loaded from their files under data/lex. Exposed so
/// the harness can audit how much held-out text each covers and how much would change if the list were
/// learned instead (R5.2, R5.3). The names match the lists' identifiers in
/// this module. This is a read-only view: it does not change the Reader.
pub fn word_lists() -> Vec<(&'static str, &'static [String])> {
    vec![
        ("control_verbs", lists::control_verbs().as_slice()),
        ("gerund_verbs", lists::gerund_verbs().as_slice()),
        ("complement_verbs", lists::complement_verbs().as_slice()),
        ("performatives", lists::performatives().as_slice()),
        ("set_aside_adverbs", lists::set_aside_adverbs().as_slice()),
        ("clause_conjunctions", lists::clause_conjunctions().as_slice()),
    ]
}

/// The realisation round-trip over a set of `(sentence, act)` pairs: the
/// share whose realised Act equals the sentence under `normalise`, with the
/// mark kept. Returns `(matched, total, first failures up to 20)`.
pub fn round_trip<'a>(pairs: impl IntoIterator<Item = (&'a Act, &'a str)>) -> (usize, usize, Vec<String>) {
    let (mut matched, mut total) = (0usize, 0usize);
    let mut failures = Vec::new();
    for (act, sentence) in pairs {
        total += 1;
        let want = normalise(sentence);
        if realise(act) == want {
            matched += 1;
        } else if failures.len() < 20 {
            failures.push(format!("{want} != {}", realise(act)));
        }
    }
    (matched, total, failures)
}

/// The canonical realisation round-trip: as [`round_trip`], but both sides go
/// through [`canonical`] first (contractions expanded, "going to" folded to
/// "will", set-aside adverbs dropped). The strict [`round_trip`] stays the
/// declared measure; this one is reported beside it. Returns
/// `(matched, total, first failures up to 20)`.
pub fn round_trip_canonical<'a>(
    pairs: impl IntoIterator<Item = (&'a Act, &'a str)>,
) -> (usize, usize, Vec<String>) {
    let (mut matched, mut total) = (0usize, 0usize);
    let mut failures = Vec::new();
    for (act, sentence) in pairs {
        total += 1;
        let want = canonical(&normalise(sentence));
        let got = canonical(&realise(act));
        if got == want {
            matched += 1;
        } else if failures.len() < 20 {
            failures.push(format!("{want} != {got}"));
        }
    }
    (matched, total, failures)
}

/// Force purity over `(sentence, gold act)` pairs: the share whose read Force
/// equals the gold Force, among the sentences the reader covers. Returns
/// `(matched, covered)`. A sentence the reader abstains on is not counted -
/// force purity is a statement about what it reads, not what it skips.
pub fn force_purity<'a>(pairs: impl IntoIterator<Item = (&'a Act, &'a str)>) -> (usize, usize) {
    let (mut matched, mut covered) = (0usize, 0usize);
    for (gold, sentence) in pairs {
        let Some(read_act) = read(sentence) else { continue };
        covered += 1;
        if read_act.force == gold.force {
            matched += 1;
        }
    }
    (matched, covered)
}

/// The read-and-realise round-trip: a sentence read to an Act and realised
/// back equals the original, over the sentences this reader covers. A
/// sentence it cannot read is counted as uncovered, never as a pass.
pub fn read_realise_rate<'a>(sentences: impl IntoIterator<Item = &'a str>) -> (usize, usize, usize) {
    let (mut matched, mut covered, mut total) = (0usize, 0usize, 0usize);
    for sentence in sentences {
        total += 1;
        let Some(act) = read(sentence) else { continue };
        covered += 1;
        if canonical(&realise(&act)) == canonical(&normalise(sentence))
            && names::kept(&act.frame, &realise_cased(&act), sentence)
        {
            matched += 1;
        }
    }
    (matched, covered, total)
}
