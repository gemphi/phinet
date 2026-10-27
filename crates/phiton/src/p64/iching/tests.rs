//! P64's own I Ching: a syntax key's hexagram and its speech act. The
//! hexagram and the trigrams themselves are tested in `phistd::hexagram`.

use super::*;

#[test]
fn a_syntax_key_reads_as_a_hexagram_and_a_speech_act() {
    let key = crate::p64::SyntaxParser::parse("i want to hug you");
    let hex = of_syntax_key(&key);
    assert_eq!(hex, of_syntax_key(&key), "the same key, the same hexagram");
    assert!(hex.id < 64 && !hex.archetype_name().is_empty());
    assert_eq!(speech_act(Hexagram::from_id(63)), SpeechAct::Declaration, "the Creative declares");
    assert_eq!(speech_act(Hexagram::from_id(0)), SpeechAct::Assertive, "the Receptive asserts");
}
