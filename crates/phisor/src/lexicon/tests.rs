use super::*;

#[test]
fn a_primed_lexicon_keeps_the_tag_set_and_answers_by_ambiguity() {
    // Under test the canonical lexicon is primed first, whatever runs first,
    // so this table is not the one primed: the mined tags carry the same
    // three cases (arrive `v`, water `nv`, thing `n`).
    prime("arrive\tv\nwater\tnv\nthing\tn\nblue\ta\n");
    assert_eq!((tag("arrive"), tag("water"), tag("thing")), (Some("v"), Some("nv"), Some("n")));
    // "arrive" is verb-only: an imperative may generalise to it.
    assert!(verb_only("arrive"));
    assert!(verb_capable("arrive"));
    // "water" is noun-and-verb: verb-capable but NOT verb-only, so the
    // imperative gate does not fire on it.
    assert!(!verb_only("water"));
    assert!(verb_capable("water"));
    assert!(noun_capable("water"));
    // "thing" is a noun only.
    assert!(!verb_only("thing"));
    assert!(!verb_capable("thing"));
    assert!(noun_capable("thing"));
}

#[test]
fn every_test_reads_the_canonical_lexicon_whatever_primed_first() {
    // A two-word table primed first used to become every later test's
    // lexicon; under test the canonical files win.
    prime("she\tn\nwater\tv\n");
    prime_function("she");
    assert!(is_primed());
    assert_eq!(tag("water"), Some("nv"), "the mined tags, not the two-word table");
    assert!(function_word("the"), "the function words from function.txt");
    assert_eq!(irregular_past("go").as_deref(), Some("went"), "the Guide's irregular verbs");
}

#[test]
fn the_appendix_table_parses_both_column_blocks_and_alternatives() {
    let text = "List of irregular verbs\n\
        infinitive        past simple   past participle   infinitive   past simple   past participle\n\
        be                was/were      been              light        lit           lit\n\
        get               got           got/gotten        lie          lay           lain\n\
        lay               laid          laid              read         read [red]*   read [red]*\n\
        * pronunciation\n\
        pay               paid          paid\n";
    let t = parse_irregulars(text);
    // be is skipped; the rest of both blocks are read; the footnote ends it.
    assert!(t.forms("be").is_none());
    assert_eq!(t.forms("light").map(|f| f.past.clone()), Some(vec!["lit".to_string()]));
    assert_eq!(t.forms("get").map(|f| f.participle.clone()), Some(vec!["got".to_string(), "gotten".to_string()]));
    assert_eq!(t.forms("read").map(|f| f.past.clone()), Some(vec!["read".to_string()]));
    assert!(t.forms("pay").is_none(), "rows after the footnote are not part of the table");
    // "lay" is the past of "lie" but is also a verb of its own, so it reads
    // as its own base and not as the past of "lie".
    assert_eq!(t.lemma_of_past("lay"), None);
    assert_eq!(t.lemma_of_past("laid"), Some("lay"));
    assert_eq!(t.lemma_of_participle("gotten"), Some("get"));
}
