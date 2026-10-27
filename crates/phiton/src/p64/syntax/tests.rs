//! Unit tests for syntax and keyed dictionary.

use super::*;

#[test]
fn test_keyed_dictionary_lookups() {
    let dict = PosDictionary::default_lexicon();
    assert_eq!(dict.lookup("i"), Some(PartOfSpeech::Pronoun));
    assert_eq!(dict.lookup("the"), Some(PartOfSpeech::Determiner));
    assert_eq!(dict.lookup("is"), Some(PartOfSpeech::Auxiliary));
    assert_eq!(dict.lookup("to"), Some(PartOfSpeech::Preposition));
    assert_eq!(dict.lookup("and"), Some(PartOfSpeech::Conjunction));
    assert_eq!(dict.lookup("want"), Some(PartOfSpeech::Verb));

    let pronouns = dict.words_for(PartOfSpeech::Pronoun);
    assert!(pronouns.contains(&"i".to_string()));
    assert!(pronouns.contains(&"we".to_string()));
}

#[test]
fn test_dynamic_word_registration() {
    let mut dict = PosDictionary::new();
    dict.register("shall", PartOfSpeech::Auxiliary);
    assert_eq!(dict.lookup("shall"), Some(PartOfSpeech::Auxiliary));
}

#[test]
fn test_syntax_parser_keyed_pipeline() {
    let key = SyntaxParser::parse("i want to hug you");
    assert_eq!(key.key, "PRON+V+PREP+V+PRON");
    assert_eq!(key.parts[0], PartOfSpeech::Pronoun);
    assert_eq!(key.parts[2], PartOfSpeech::Preposition);
}
