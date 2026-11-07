use super::*;

const SENTENCE: &str = "she saw the dog.";

fn proposal(token: &str, role: Role) -> Proposal {
    Proposal { token: token.into(), class: 3, role }
}
fn on() -> Config {
    Config { enabled: true, ..Config::default() }
}

#[test]
fn accepts_a_token_the_reader_places_in_the_role() {
    assert!(read(SENTENCE).is_some());
    assert!(proposal("dog", Role::Theme).check(SENTENCE).is_some());
}

#[test]
fn rejects_a_token_in_the_wrong_role_or_absent() {
    assert!(proposal("dog", Role::Agent).check(SENTENCE).is_none());
    assert!(proposal("cat", Role::Theme).check(SENTENCE).is_none());
}

#[test]
fn rejects_when_the_reader_cannot_read_the_sentence_whole() {
    assert!(read("dog the blorf of").is_none());
    assert!(proposal("dog", Role::Theme).check("dog the blorf of").is_none());
}

#[test]
fn flag_off_checks_nothing_and_leaves_the_reading() {
    let r = review(Config::default(), vec![proposal("dog", Role::Theme)], SENTENCE);
    assert_eq!(r.act, read(SENTENCE));
    assert!(r.accepted.is_empty() && r.rejected.is_empty());
}

#[test]
fn flag_on_splits_proposals_and_the_reading_is_unchanged() {
    let ps = vec![proposal("dog", Role::Theme), proposal("dog", Role::Agent)];
    let r = review(on(), ps, SENTENCE);
    assert_eq!(r.act, read(SENTENCE));
    assert_eq!((r.accepted.len(), r.rejected.len()), (1, 1));
}
