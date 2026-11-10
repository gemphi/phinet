//! Inflection: participles, -ing, irregulars and doubled consonants.

use super::*;

#[test]
fn read_participle_inverts_the_realiser_table() {
    // Irregulars and regulars both invert, confirmed by re-realising.
    assert_eq!(read_participle("worn"), Some("wear".to_string()));
    assert_eq!(read_participle("written"), Some("write".to_string()));
    assert_eq!(read_participle("sold"), Some("sell".to_string()));
    assert_eq!(read_participle("finished"), Some("finish".to_string()));
    assert_eq!(read_participle("arrived"), Some("arrive".to_string()));
    // A plain word that is no participle is not mistaken for one.
    assert_eq!(read_participle("answer"), None);
}

#[test]
fn read_ing_inverts_the_realiser_table() {
    assert_eq!(read_ing("wearing"), Some("wear".to_string()));
    assert_eq!(read_ing("living"), Some("live".to_string()));
    assert_eq!(read_ing("waiting"), Some("wait".to_string()));
    assert_eq!(read_ing("being"), Some("be".to_string()));
    // Not an -ing form.
    assert_eq!(read_ing("answer"), None);
}

#[test]
fn irregular_verbs_come_from_the_guides_own_appendix() {
    // Prime from the real Guide when it is present (the pure crate names no
    // path of its own; the test locates the data beside the workspace).
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data/person/murphy.txt");
    let Ok(text) = std::fs::read_to_string(path) else { return };
    crate::lexicon::prime_irregular(&text);
    let table = crate::lexicon::parse_irregulars(&text);
    // 58 lines of two verbs is 116 rows; `be` is skipped, leaving 115.
    assert_eq!(table.len(), 115, "the appendix table should read whole");
    assert_eq!(crate::lexicon::lemma_of_past("met"), Some("meet".to_string()));
    assert_eq!(crate::lexicon::lemma_of_participle("paid"), Some("pay".to_string()));
    // The reader inverts them and the realiser writes them, so the pair agrees.
    let act = read("i have met her.").expect("an irregular participle is read");
    assert_eq!(act.frame.pred, "meet");
    assert_eq!(realise(&act), "i have met her.");
    let (matched, covered, _) = read_realise_rate(["i've already paid it."]);
    assert_eq!((matched, covered), (1, 1));
}

#[test]
fn short_stressed_verbs_double_the_final_consonant() {
    assert_eq!(inflect("run", Suffix::ING), "running");
    assert_eq!(inflect("sit", Suffix::ING), "sitting");
    assert_eq!(inflect("stop", Suffix::PAST), "stopped");
    assert_eq!(inflect("admit", Suffix::PAST), "admitted");
    assert_eq!(inflect("quit", Suffix::ING), "quitting");
    // No doubling: two vowels, final w/x/y, unstressed, or a consonant pair.
    assert_eq!(inflect("wait", Suffix::PAST), "waited");
    assert_eq!(inflect("rain", Suffix::ING), "raining");
    assert_eq!(inflect("show", Suffix::PAST), "showed");
    assert_eq!(inflect("fix", Suffix::PAST), "fixed");
    assert_eq!(inflect("visit", Suffix::PAST), "visited");
    assert_eq!(inflect("open", Suffix::ING), "opening");
    assert_eq!(inflect("add", Suffix::PAST), "added");
}

#[test]
fn doubled_forms_read_back_to_the_lemma() {
    assert_eq!(read_ing("stopping"), Some("stop".to_string()));
    assert_eq!(read_ing("sitting"), Some("sit".to_string()));
    assert_eq!(read_ing("adding"), Some("add".to_string()));
    assert_eq!(read_ing("kissing"), Some("kiss".to_string()));
    assert_eq!(read_verb("stopped"), ("stop".to_string(), Tense::Past));
    assert_eq!(read_verb("admitted"), ("admit".to_string(), Tense::Past));
    assert_eq!(read_participle("stopped"), Some("stop".to_string()));
    let act = read("he stopped the car.").expect("a doubled past reads");
    assert_eq!(act.frame.pred, "stop");
    assert_eq!(realise(&act), "he stopped the car.");
}

#[test]
fn read_verb_returns_the_lemma_for_a_doubled_consonant_ed() {
    use crate::ir::Tense;
    assert_eq!(super::read_verb("added"), ("add".to_string(), Tense::Past));
    assert_eq!(super::read_verb("missed"), ("miss".to_string(), Tense::Past));
    // A silent-e verb and a -ied verb keep their lemma.
    assert_eq!(super::read_verb("arrived"), ("arrive".to_string(), Tense::Past));
    assert_eq!(super::read_verb("studied"), ("study".to_string(), Tense::Past));
}
