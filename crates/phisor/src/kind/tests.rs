use super::*;
use crate::guide::{Function, Mark, Part, Sentence};

fn s(words: &str, mark: Mark, unit: usize) -> Sentence {
    Sentence { words: words.split(' ').map(String::from).collect(), function: Function::Explain, mark, unit, run: 0 }
}

#[test]
fn a_variation_is_how_a_sentence_opens_and_closes() {
    let c = [9u16, 2, 14, 5, 1, 11];
    assert_eq!(shape(&c, Twin::Open), "c9 c2 …");
    assert_eq!(shape(&c, Twin::Close), "… c1 c11");
    assert_eq!(shape(&c, Twin::Both), "c9 c2 … c11");
}

#[test]
fn a_grown_template_is_the_longest_opening_told_twice() {
    let said: [&[u16]; 4] = [&[1, 2, 3, 4], &[1, 2, 3, 5], &[1, 2, 9, 9], &[7, 7, 7, 7]];
    let told = openings(said.into_iter());
    assert_eq!(grown(&[1, 2, 3, 4], &told, 2), "c1 c2 c3 …");
    assert_eq!(grown(&[1, 2, 9, 9], &told, 2), "c1 c2 …");
    // Nothing told twice: the first class alone.
    assert_eq!(grown(&[7, 7, 7, 7], &told, 2), "c7 …");
    // A whole sentence told twice is its own template, with no `…`.
    assert_eq!(grown(&[1, 2], &told, 2), "c1 c2");
    assert_eq!((opening(&[1, 2, 3], 2), opening(&[1, 2, 3], 3)), ("c1 c2 …".to_string(), "c1 c2 c3".to_string()));
}

#[test]
fn a_held_out_template_backs_off_along_the_trie() {
    // Held out (sentences 0 and 10): one shares four words with what was
    // told, one only its first. Neither opening is anybody's template, and
    // both back off to one that is.
    let said = |i: usize| match i {
        0 => "the dog was here later",
        10 => "the cat sat down there",
        1..=4 => "the dog was here now",
        6..=9 => "the dog ran off now",
        11..=14 => "the dog was here again",
        _ => "the dog ran away then",
    };
    let sentences: Vec<Sentence> = (0..20).map(|i| s(said(i), Mark::Stop, i)).collect();
    let split = Split::new(sentences);
    let kinds = Kinds::learn(&split, &Coder::Hash, Level::P64, Twin::Grown, Reading::River, Grow::Fixed(2));
    let unknown = kinds.units.get(UNKNOWN).unwrap();
    assert_eq!(split.of(Part::Test), vec![0, 10]);
    assert!(kinds.units.get(&kinds.variation[10]).is_none(), "{} is a template", kinds.variation[10]);
    for i in [0, 10] {
        assert_ne!(kinds.unit(i), unknown, "{} fell to <unk>", kinds.variation[i]);
    }
}

#[test]
fn kinds_are_learned_without_a_label_and_read_with_one() {
    // A question is always answered: the river alone should part them.
    let mut sentences = Vec::new();
    for u in 0..60 {
        sentences.push(s("were you waiting in the rain", Mark::Question, u));
        sentences.push(s("yes i was waiting there", Mark::Stop, u));
        sentences.push(s("did she see the film", Mark::Question, u));
        sentences.push(s("no she did not", Mark::Stop, u));
    }
    let split = Split::new(sentences);
    for grow in [Grow::Fixed(6), Grow::Paid] {
        let kinds = Kinds::learn(&split, &Coder::Given, Level::P16, Twin::Open, Reading::River, grow);
        let t = Tables::read(&kinds, &split, Part::Test);
        // Every held-out kind holds one mark only.
        assert!(t.mark.iter().all(|row| row.iter().filter(|&&n| n > 0.0).count() <= 1), "{grow:?} {:?}", t.mark);
        assert!(kinds.occupied() >= 2);
        // The river is read on held-out sentences with a sentence before them.
        assert!(!river(&kinds, &split, Part::Test).is_empty());
    }
}
