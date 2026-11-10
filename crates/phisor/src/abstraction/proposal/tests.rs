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

// Property 6 (proposal discipline): over seeded generated sentences and
// proposals, a proposal never changes what the Reader returns.
const WORDS: [&str; 14] =
    ["the", "a", "girl", "boy", "dog", "cat", "saw", "liked", "fed", "red", "ball", "tree", "she", "in"];

fn next(s: &mut u64) -> usize {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*s >> 33) as usize
}

fn pick<'a>(s: &mut u64, from: &[&'a str]) -> &'a str {
    from[next(s) % from.len()]
}

/// A readable-looking sentence about half the time, word soup otherwise.
fn sentence(s: &mut u64) -> String {
    if next(s) % 2 == 0 {
        let (n1, v, n2) = (pick(s, &WORDS[2..6]), pick(s, &WORDS[6..9]), pick(s, &WORDS[2..6]));
        format!("the {n1} {v} a {n2}.")
    } else {
        let words: Vec<&str> = (0..1 + next(s) % 6).map(|_| pick(s, &WORDS)).collect();
        format!("{}.", words.join(" "))
    }
}

fn proposals(s: &mut u64) -> Vec<Proposal> {
    let roles = [Role::Agent, Role::Theme, Role::Recipient, Role::As, Role::Via("in".into())];
    (0..next(s) % 6)
        .map(|_| Proposal {
            token: pick(s, &WORDS).into(),
            class: (next(s) % 8) as u16,
            role: roles[next(s) % roles.len()].clone(),
        })
        .collect()
}

/// Prime the process-global lexicon from the same data the other tests use
/// (first write wins), so a concurrent prime cannot change `read` between
/// the two readings compared below.
fn prime() {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) {
        crate::lexicon::prime(&p);
    }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) {
        crate::lexicon::prime_function(&f);
    }
    if let Ok(g) = std::fs::read_to_string(format!("{base}/person/murphy.txt")) {
        crate::lexicon::prime_irregular(&g);
    }
}

#[test]
fn proposal_discipline_holds_over_generated_sentences() {
    prime();
    let mut s = 0x5eed_u64;
    let mut accepted_seen = 0;
    for _ in 0..400 {
        let text = sentence(&mut s);
        let ps = proposals(&mut s);
        let expected = read(&text);

        let off = review(Config::default(), ps.clone(), &text);
        assert_eq!(off.act, expected, "{text}");
        assert!(off.accepted.is_empty() && off.rejected.is_empty(), "{text}");

        let on_review = review(on(), ps.clone(), &text);
        assert_eq!(on_review.act, expected, "{text}");
        assert_eq!(on_review.accepted.len() + on_review.rejected.len(), ps.len(), "{text}");

        for p in &on_review.accepted {
            accepted_seen += 1;
            let act = expected.as_ref().expect("accepted implies the Reader read it");
            assert!(fills(&act.frame, &p.token, Some(&p.role)), "{text}: {p:?}");
            let checked = p.check(&text).expect("accepted proposals check again");
            assert_eq!((&checked.force, &checked.frame), (&act.force, &act.frame), "{text}");
        }
        for p in &on_review.rejected {
            assert!(p.check(&text).is_none(), "{text}: {p:?}");
        }
    }
    assert!(accepted_seen > 0, "the generator never produced an accepted proposal");
}
