//! Plain declaratives, imperatives and the words a plain clause holds.

use super::*;

#[test]
fn a_simple_sentence_reads_and_realises_back() {
    let act = read("they play tennis.").expect("a simple declarative is covered");
    assert_eq!(act.force, crate::ir::Force::Assert);
    assert_eq!(act.frame.pred, "play");
    assert_eq!(realise(&act), "they play tennis.");
}

#[test]
fn negation_is_read_and_realised() {
    let act = read("he does not smoke.").expect("negation is covered");
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "he does not smoke.");
}

#[test]
fn an_imperative_reads_with_the_direct_force() {
    // A base verb with no subject is a command (Murphy Unit 35), with its
    // object in place and the Direct force recorded.
    let act = read("close the window.").expect("an imperative is covered");
    assert_eq!(act.force, crate::ir::Force::Direct);
    assert_eq!(act.frame.pred, "close");
    assert_eq!(realise(&act), "close the window.");
}

#[test]
fn a_ditransitive_command_names_the_recipient_first() {
    // "tell" takes a recipient; a lone object pronoun after it is the
    // recipient, not the theme.
    let act = read("tell me the truth.").expect("a ditransitive command is covered");
    assert_eq!(act.frame.get(&crate::ir::Role::Recipient), Some(&crate::ir::Filler::pronoun("me")));
}

#[test]
fn a_lone_object_pronoun_after_a_plain_verb_is_the_theme() {
    // "how did you do it?" - "it" is the theme of "do", not a recipient.
    let act = read("how did you do it?").expect("covered");
    assert_eq!(act.frame.get(&crate::ir::Role::Theme), Some(&crate::ir::Filler::pronoun("it")));
}

#[test]
fn never_is_read_as_negation() {
    use crate::ir::Polarity;
    let act = read("i never have lunch.").expect("never is covered");
    assert_eq!(act.frame.pred, "have");
    assert_eq!(act.frame.polarity, Polarity::Negative);
}

#[test]
fn a_blanked_exercise_stem_is_not_read_as_a_plain_sentence() {
    // The blank leaves a stem with no verb. The Reader reads no frame for it,
    // so the -ing rule does not turn a blank into a plain reading (R7.5).
    assert!(read("he two hours every morning.").is_none());
    assert!(read("he watching two hours every morning.").is_none());
}

#[test]
fn a_pronoun_is_never_read_as_a_verb() {
    // The second `she` sits where the verb belongs; no frame, not a verb `she`.
    assert!(read("she she went to now.").is_none());
    assert!(read("they we went home.").is_none());
    // A pronoun subject still reads.
    assert!(read("she went home.").is_some());
}
