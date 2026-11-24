use super::*;
use phical::classes::Grow;

fn told(lines: &[(&str, usize)]) -> Vec<Vec<String>> {
    lines.iter().flat_map(|&(s, n)| std::iter::repeat_n(s.split(' ').map(String::from).collect::<Vec<_>>(), n)).collect()
}

/// `do` opens questions and `use` opens orders; both also stand inside
/// sentences, after `i`.
fn guide() -> Vec<Vec<String>> {
    told(&[
        ("do you like it", 20),
        ("do you see it", 20),
        ("use the pen now", 20),
        ("use the book now", 20),
        ("i do not know", 20),
        ("i use the pen", 20),
        ("we once said so", 1),
    ])
}

#[test]
fn an_opening_sense_is_coined_for_a_word_that_opens_two_sentences() {
    let lines = guide();
    let s: Vec<&[String]> = lines.iter().map(Vec::as_slice).collect();
    let code = Code::learn_opening(&s, Grow::Fixed(4));
    assert!(code.opening);
    let unknown = code.of("<unk>");
    assert_ne!(code.of("^do"), unknown);
    assert_ne!(code.of("^use"), unknown);
    // `we` opens one sentence only: no sense of its own.
    assert_eq!(code.of("^we"), unknown);
    // The question's opening and the order's part: what follows them differs.
    assert_ne!(code.of("^do"), code.of("^use"));
    // A sentence's first word is read on two channels, its opening sense and
    // itself; the rest as themselves.
    let words: Vec<String> = ["do", "you", "do", "it"].map(String::from).to_vec();
    let w = code.write(&words, 4);
    assert_eq!(w.len(), 5);
    assert_eq!((w[0], w[1], w[3]), (code.class("^do", 4), code.class("do", 4), code.class("do", 4)));
    // A first word with no opening sense is written once.
    let plain: Vec<String> = ["we", "once", "said", "so"].map(String::from).to_vec();
    assert_eq!(code.write(&plain, 4).len(), 4);
}

#[test]
fn a_code_without_senses_writes_each_word_as_itself() {
    let lines = guide();
    let s: Vec<&[String]> = lines.iter().map(Vec::as_slice).collect();
    let code = Code::learn(&s, Grow::Fixed(4));
    assert!(!code.opening);
    assert_eq!(code.of("^do"), code.of("<unk>"));
    let words: Vec<String> = ["do", "you", "like", "it"].map(String::from).to_vec();
    let each: Vec<u16> = words.iter().map(|w| code.class(w, 4)).collect();
    assert_eq!(code.write(&words, 4), each);
    assert_eq!(Coder::Learned(code).name(), "learned");
}
