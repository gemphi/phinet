//! The act height, measured honestly on the frozen gold set.

use super::*;
use crate::gold;

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
fn a_copula_now_reads_into_a_be_frame() {
    // The extended reader handles the copula shape `X is Y`.
    let act = read("her name is helen.").expect("a copula is now covered");
    assert_eq!(act.frame.pred, "be");
    assert_eq!(act.force, crate::ir::Force::Assert);
}

#[test]
fn a_participle_copula_reads_as_aspect_not_a_complement() {
    use crate::ir::{Aspect, Voice};
    // `is boiling` is a progressive, not a copula+complement: the reader now
    // reads the aspect rather than treating `boiling` as a predicate.
    let prog = read("the water is boiling.").expect("progressive is covered");
    assert_eq!(prog.frame.pred, "boil");
    assert_eq!(prog.frame.aspect, Aspect::Progressive);
    assert_eq!(prog.frame.voice, Voice::Active);
    // `is repaired` is a simple passive, the theme promoted to subject.
    let pass = read("the car is repaired.").expect("passive is covered");
    assert_eq!(pass.frame.pred, "repair");
    assert_eq!(pass.frame.voice, Voice::Passive);
}

#[test]
fn the_gold_round_trip_is_measured_and_reported() {
    let gold = gold::read().expect("the frozen gold set loads");
    let (matched, total, _failures) = round_trip(gold.pairs());
    assert_eq!(total, gold::COUNT);
    let rate = 100.0 * matched as f64 / total as f64;
    println!("gold realisation round-trip: {matched}/{total} = {rate:.1}%");
    assert!(matched > 0, "the realiser round-trips no gold frame at all");
}

#[test]
fn the_read_realise_rate_is_measured_over_the_gold_sentences() {
    let gold = gold::read().expect("the frozen gold set loads");
    let sentences: Vec<&str> = gold.entries.iter().map(|e| e.sentence.as_str()).collect();
    let (matched, covered, total) = read_realise_rate(sentences);
    let cov = 100.0 * covered as f64 / total as f64;
    let acc = if covered > 0 { 100.0 * matched as f64 / covered as f64 } else { 0.0 };
    println!("read coverage {covered}/{total} = {cov:.1}%; round-trip on covered {matched}/{covered} = {acc:.1}%");
    assert_eq!(total, gold::COUNT);
    assert!(covered > 0, "the reader covers no gold sentence");
}
/// A diagnostic, not a gate: it prints the gold frames the realiser does not
/// reproduce, each the name of a lesson the generator is still missing.
/// `#[ignore]` so it runs only when asked: `cargo test -p phisor
/// gold_realisation_failures -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: prints the gold frames the realiser misses"]
fn gold_realisation_failures() {
    let gold = gold::read().expect("the frozen gold set loads");
    let mut shown = 0;
    for (act, sentence) in gold.pairs() {
        let got = realise(act);
        let want = crate::guide::holdout::normalise(sentence);
        if got != want && shown < 40 {
            shown += 1;
            println!("WANT: {want}");
            println!("GOT:  {got}");
            println!("ACT:  {act:?}");
            println!("---");
        }
    }
    println!("shown {shown} of the realiser's gold misses");
}

/// A diagnostic, not a gate: for every gold sentence it reports whether the
/// reader abstained (returned None - an uncovered shape) or read a wrong Act
/// (a soundness concern), so the two kinds of gap can be told apart.
/// `#[ignore]` so it runs only when asked: `cargo test -p phisor
/// gold_reading_failures -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: prints the gold sentences the reader abstains on or mis-reads"]
fn gold_reading_failures() {
    let gold = gold::read().expect("the frozen gold set loads");
    let (mut abstained, mut wrong, mut correct) = (0usize, 0usize, 0usize);
    let mut shown_abstain = 0;
    let mut shown_wrong = 0;
    for (want, sentence) in gold.pairs() {
        match read(sentence) {
            None => {
                abstained += 1;
                if shown_abstain < 40 {
                    shown_abstain += 1;
                    println!("ABSTAIN: {sentence}");
                }
            }
            Some(got) if &got == want => correct += 1,
            Some(got) => {
                wrong += 1;
                if shown_wrong < 40 {
                    shown_wrong += 1;
                    println!("WRONG:   {sentence}");
                    println!("  want:  {want:?}");
                    println!("  got:   {got:?}");
                }
            }
        }
    }
    println!(
        "reader over {} gold: {correct} correct, {wrong} wrong, {abstained} abstained",
        gold::COUNT
    );
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
fn a_do_support_question_inverts_and_drops_do() {
    // "do you know the answer?" reads to a present ASK with the operator
    // dropped and the object in place.
    let act = read("do you know the answer?").expect("a yes/no question is covered");
    assert_eq!(act.force, crate::ir::Force::Ask);
    assert_eq!(act.frame.pred, "know");
    assert_eq!(act.frame.modal, None);
    assert_eq!(realise(&act), "do you know the answer?");
}

#[test]
fn a_wh_question_places_the_hole_in_its_role() {
    // "where do you live?" places the wh as a Hole under its preposition role
    // and reads the subject and tense.
    let act = read("where do you live?").expect("a wh-question is covered");
    assert_eq!(act.force, crate::ir::Force::Ask);
    assert_eq!(act.frame.pred, "live");
    assert_eq!(realise(&act), "where do you live?");
}

#[test]
fn a_modal_question_keeps_the_modal() {
    // "can you swim?" keeps the modal and inverts.
    let act = read("can you swim?").expect("a modal question is covered");
    assert_eq!(act.frame.modal, Some(crate::ir::Modal::Can));
    assert_eq!(realise(&act), "can you swim?");
}

#[test]
fn a_lone_object_pronoun_after_a_plain_verb_is_the_theme() {
    // "how did you do it?" - "it" is the theme of "do", not a recipient.
    let act = read("how did you do it?").expect("covered");
    assert_eq!(act.frame.get(&crate::ir::Role::Theme), Some(&crate::ir::Filler::pronoun("it")));
}

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
fn read_verb_group_reads_active_aspects() {
    use crate::ir::{Aspect, Tense, Voice};
    let w = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();

    // Perfect active: "has arrived".
    let (lemma, tense, aspect, voice, neg, consumed) =
        read_verb_group(&w("has arrived"), 0).expect("perfect active reads");
    assert_eq!((lemma.as_str(), tense, aspect, voice, neg, consumed),
               ("arrive", Tense::Present, Aspect::Perfect, Voice::Active, false, 2));

    // Perfect progressive: "had been working".
    let (lemma, tense, aspect, voice, neg, consumed) =
        read_verb_group(&w("had been working"), 0).expect("perfect progressive reads");
    assert_eq!((lemma.as_str(), tense, aspect, voice, neg, consumed),
               ("work", Tense::Past, Aspect::PerfectProgressive, Voice::Active, false, 3));

    // Negated perfect: "have not finished".
    let (lemma, tense, aspect, voice, neg, consumed) =
        read_verb_group(&w("have not finished"), 0).expect("negated perfect reads");
    assert_eq!((lemma.as_str(), tense, aspect, voice, neg, consumed),
               ("finish", Tense::Present, Aspect::Perfect, Voice::Active, true, 3));

    // Progressive active: "is boiling".
    let (lemma, _t, aspect, voice, _n, _c) =
        read_verb_group(&w("is boiling"), 0).expect("progressive active reads");
    assert_eq!((lemma.as_str(), aspect, voice), ("boil", Aspect::Progressive, Voice::Active));

    // A modal head is not this reader's job.
    assert!(read_verb_group(&w("will come"), 0).is_none());
}

#[test]
fn a_perfect_active_reads_to_the_gold_act_and_round_trips() {
    use crate::ir::{Aspect, Role, Tense};
    // "i have lost my keys." - present perfect, agent I, theme my keys.
    let act = read("i have lost my keys.").expect("perfect active is covered");
    assert_eq!(act.frame.pred, "lose");
    assert_eq!(act.frame.tense, Tense::Present);
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Agent), Some(&crate::ir::Filler::pronoun("I")));
    assert_eq!(realise(&act), "i have lost my keys.");
}

#[test]
fn a_perfect_progressive_reads_to_the_gold_act_and_round_trips() {
    use crate::ir::Aspect;
    // "the children have been reading my books." - present perfect progressive.
    let act = read("the children have been reading my books.").expect("perfect progressive is covered");
    assert_eq!(act.frame.pred, "read");
    assert_eq!(act.frame.aspect, Aspect::PerfectProgressive);
    assert_eq!(realise(&act), "the children have been reading my books.");
}

#[test]
fn a_simple_passive_promotes_the_theme_to_subject() {
    use crate::ir::{Aspect, Role, Tense, Voice};
    // "the window was broken." - simple past passive, theme the window.
    let act = read("the window was broken.").expect("simple passive is covered");
    assert_eq!(act.frame.pred, "break");
    assert_eq!(act.frame.tense, Tense::Past);
    assert_eq!(act.frame.aspect, Aspect::Simple);
    assert_eq!(act.frame.voice, Voice::Passive);
    assert!(matches!(act.frame.get(&Role::Theme), Some(crate::ir::Filler::Thing(t)) if t.head == "window"));
    assert_eq!(realise(&act), "the window was broken.");
}

#[test]
fn a_perfect_passive_reads_and_round_trips() {
    use crate::ir::Voice;
    // "the house has been sold." - present perfect passive.
    let act = read("the house has been sold.").expect("perfect passive is covered");
    assert_eq!(act.frame.pred, "sell");
    assert_eq!(act.frame.voice, Voice::Passive);
    assert_eq!(realise(&act), "the house has been sold.");
}

#[test]
fn a_passive_by_agent_reads_the_demoted_agent() {
    use crate::ir::Role;
    // "my car has been repaired by a mechanic." - the by-phrase is the agent.
    let act = read("my car has been repaired by a mechanic.").expect("by-agent passive is covered");
    assert_eq!(act.frame.pred, "repair");
    assert!(matches!(act.frame.get(&Role::Agent), Some(crate::ir::Filler::Thing(t)) if t.head == "mechanic"));
    assert!(matches!(act.frame.get(&Role::Theme), Some(crate::ir::Filler::Thing(t)) if t.head == "car"));
    assert_eq!(realise(&act), "my car has been repaired by a mechanic.");
}

#[test]
fn a_perfect_yes_no_question_reads() {
    use crate::ir::{Aspect, Force, Role};
    // "has she arrived?" - present perfect, theme she (unaccusative in gold),
    // Ask force. The verb group is read across the subject.
    let act = read("has she arrived?").expect("perfect question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "arrive");
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Agent), Some(&crate::ir::Filler::pronoun("she")));
}

#[test]
fn a_perfect_wh_question_places_the_hole() {
    use crate::ir::{Aspect, Force, Role, Wh};
    // "what has happened?" - the wh is the theme subject of an agentless
    // perfect; it is placed as a Hole and not duplicated.
    let act = read("what has happened?").expect("perfect wh-question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "happen");
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Theme), Some(&crate::ir::Filler::Hole(Wh::What)));
}

#[test]
fn a_progressive_wh_question_reads() {
    use crate::ir::{Aspect, Force};
    // "what are you doing?" - progressive active, wh theme.
    let act = read("what are you doing?").expect("progressive wh-question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "do");
    assert_eq!(act.frame.aspect, Aspect::Progressive);
}


#[test]
fn a_simple_modal_reads_with_its_sense_and_tense() {
    use crate::ir::{Aspect, Modal, Role, Tense};
    // "i can swim." - present, modal can, simple active.
    let act = read("i can swim.").expect("simple modal is covered");
    assert_eq!(act.frame.pred, "swim");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.tense, Tense::Present);
    assert_eq!(act.frame.aspect, Aspect::Simple);
    assert_eq!(act.frame.get(&Role::Agent), Some(&crate::ir::Filler::pronoun("I")));
    assert_eq!(realise(&act), "i can swim.");
}

#[test]
fn a_past_modal_negative_reads() {
    use crate::ir::{Modal, Tense};
    // "he could not sing." - can in the past, negative.
    let act = read("he could not sing.").expect("past modal negative is covered");
    assert_eq!(act.frame.pred, "sing");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.tense, Tense::Past);
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "he could not sing.");
}

#[test]
fn a_modal_passive_reads() {
    use crate::ir::{Modal, Voice};
    // "the work will be finished tomorrow." - will + be + participle, passive.
    let act = read("the work will be finished tomorrow.").expect("modal passive is covered");
    assert_eq!(act.frame.pred, "finish");
    assert_eq!(act.frame.modal, Some(Modal::Will));
    assert_eq!(act.frame.voice, Voice::Passive);
    assert_eq!(realise(&act), "the work will be finished tomorrow.");
}

#[test]
fn a_cannot_passive_reads_the_negation() {
    use crate::ir::{Modal, Voice};
    // "this door cannot be opened." - cannot carries can + negative.
    let act = read("this door cannot be opened.").expect("cannot passive is covered");
    assert_eq!(act.frame.pred, "open");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.voice, Voice::Passive);
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "this door cannot be opened.");
}

#[test]
fn a_modal_perfect_reads() {
    use crate::ir::{Aspect, Modal, Role, Tense};
    // "i could have told you." - can/past + have + participle, perfect.
    let act = read("i could have told you.").expect("modal perfect is covered");
    assert_eq!(act.frame.pred, "tell");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(act.frame.tense, Tense::Past);
    assert_eq!(act.frame.aspect, Aspect::Perfect);
    assert_eq!(act.frame.get(&Role::Recipient), Some(&crate::ir::Filler::pronoun("you")));
    assert_eq!(realise(&act), "i could have told you.");
}

#[test]
fn a_modal_progressive_negative_reads() {
    use crate::ir::{Aspect, Modal};
    // "he will not be coming." - will + be + -ing, progressive, negative.
    let act = read("he will not be coming.").expect("modal progressive is covered");
    assert_eq!(act.frame.pred, "come");
    assert_eq!(act.frame.modal, Some(Modal::Will));
    assert_eq!(act.frame.aspect, Aspect::Progressive);
    assert_eq!(act.frame.polarity, crate::ir::Polarity::Negative);
    assert_eq!(realise(&act), "he will not be coming.");
}


#[test]
fn contractions_expand_to_their_words() {
    use crate::ir::Polarity;
    // "i don't go" reads as a negated present: the n't expands to "do not".
    let act = read("i don't go away.").expect("a contraction is covered");
    assert_eq!(act.frame.pred, "go");
    assert_eq!(act.frame.polarity, Polarity::Negative);
    // "i'm" expands to "i am", so a progressive copula reads.
    let w = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    assert_eq!(expand_contractions(&w("i don't")), vec!["i", "do", "not"]);
    assert_eq!(expand_contractions(&w("i'm you're we've")), vec!["i", "am", "you", "are", "we", "have"]);
    assert_eq!(expand_contractions(&w("can't won't")), vec!["cannot", "will", "not"]);
    // Ambiguous forms are left as read.
    assert_eq!(expand_contractions(&w("he's tired i'd")), vec!["he's", "tired", "i'd"]);
}


#[test]
fn a_set_aside_adverb_is_understood_around_not_inside_the_frame() {
    use crate::ir::{Aspect, Polarity};
    // "he had already left." reads to the same frame as "he had left.": the
    // IR has no slot for "already", and the gold leaves it out too.
    let with = read("he had already left.").expect("adverb is set aside");
    let without = read("he had left.").expect("plain perfect reads");
    assert_eq!(with.frame, without.frame);
    assert_eq!(with.frame.aspect, Aspect::Perfect);
    // A final or degree adverb is set aside the same way.
    let often = read("i don't go away very often.").expect("degree adverbs are set aside");
    assert_eq!(often.frame.pred, "go");
    assert_eq!(often.frame.polarity, Polarity::Negative);
}

#[test]
fn never_is_read_as_negation() {
    use crate::ir::Polarity;
    let act = read("i never have lunch.").expect("never is covered");
    assert_eq!(act.frame.pred, "have");
    assert_eq!(act.frame.polarity, Polarity::Negative);
}

#[test]
fn round_trip_ignores_only_the_set_aside_adverbs() {
    // The same adverbs are set aside on both sides, so a sentence whose only
    // difference is such an adverb compares equal; a different content word
    // does not.
    assert_eq!(canonical("he had left."), canonical("he had already left."));
    assert_ne!(canonical("he had left."), canonical("she had left."));
}

#[test]
fn a_control_infinitive_is_realised_with_to_and_its_complements() {
    // The reader builds promise[come]; the realiser must say the "to".
    let act = read("i promise to come.").expect("an infinitive is covered");
    assert_eq!(realise(&act), "i promise to come.");
    // The inner clause keeps its own complements.
    let act = read("i want to go to london.").expect("an infinitive with a phrase is covered");
    assert_eq!(realise(&act), "i want to go to london.");
}

#[test]
fn nested_infinitives_compose() {
    let act = read("i promise to try to pay the money.").expect("nested infinitives are covered");
    assert_eq!(realise(&act), "i promise to try to pay the money.");
}

#[test]
fn a_finite_clause_is_not_mistaken_for_an_infinitive() {
    use crate::ir::{Filler, Force, Frame, Modal, Role, Tense};
    // "i wish [i could sing]": the inner clause has its own modal and tense, so
    // it is finite and must not be said as "to sing".
    let inner = Frame::new("sing", Tense::Past).modal(Modal::Can).with(Role::Agent, Filler::pronoun("I"));
    let matrix = Frame::new("wish", Tense::Present)
        .with(Role::Agent, Filler::pronoun("I"))
        .with(Role::Theme, Filler::clause(inner));
    let said = realise(&crate::ir::Act::new(Force::Wish, matrix));
    assert!(!said.contains(" to sing"), "a finite clause took a to: {said}");
}

/// Diagnostic: which spans stop the reader reading a clean held-out sentence?
/// For every abstained sentence, find the shortest leading, trailing or single
/// word span whose removal makes it readable, and rank those spans. Run alone:
/// `cargo test -p phisor blocker_spans -- --ignored --nocapture`. It primes the
/// real lexicon, so it must not share a process with other priming tests.
#[test]
#[ignore = "diagnostic: ranks the spans that block the reader on clean held-out text"]
fn blocker_spans() {
    use std::collections::HashMap;
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    let (text, key) = (
        std::fs::read_to_string(format!("{base}/person/murphy.txt")),
        std::fs::read_to_string(format!("{base}/person/murphy_key.txt")),
    );
    let (Ok(text), Ok(key)) = (text, key) else { println!("no guide"); return; };
    let split = crate::guide::Split::new(crate::guide::read(&text, &key));
    let clean = split.clean_strings(crate::guide::Part::Test);

    let (mut abstained, mut unlocked) = (0usize, 0usize);
    let mut spans: HashMap<String, usize> = HashMap::new();
    for s in &clean {
        if read(s).is_some() { continue; }
        abstained += 1;
        let mark = s.chars().last().unwrap();
        let words: Vec<&str> = s.trim_end_matches(['.', '?']).split_whitespace().collect();
        let mut found: Option<String> = None;
        // Shortest removal first: 1 word anywhere, then 2-word leading/trailing, then 3.
        'search: for width in 1..=3usize {
            for start in 0..words.len() {
                let end = start + width;
                if end > words.len() { break; }
                let edge = start == 0 || end == words.len();
                if width > 1 && !edge { continue; }
                let kept: Vec<&str> = words[..start].iter().chain(words[end..].iter()).copied().collect();
                if kept.len() < 3 { continue; }
                let cand = format!("{}{}", kept.join(" "), mark);
                if read(&cand).is_some() {
                    let place = if start == 0 { "LEAD" } else if end == words.len() { "TAIL" } else { "MID " };
                    found = Some(format!("{place} {}", words[start..end].join(" ")));
                    break 'search;
                }
            }
        }
        if let Some(f) = found { unlocked += 1; *spans.entry(f).or_default() += 1; }
    }
    println!("clean {} ; abstained {abstained} ; unlocked by removing one span {unlocked}", clean.len());
    let mut ranked: Vec<_> = spans.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (span, n) in ranked.iter().take(45) { println!("{n:4}  {span}"); }
}

#[test]
fn an_s_contraction_is_resolved_by_the_word_that_follows() {
    let w = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    // been and got mean "has"; an -ing form and a non-participle mean "is".
    assert_eq!(expand_contractions(&w("it's been raining")), vec!["it", "has", "been", "raining"]);
    assert_eq!(expand_contractions(&w("he's got a boat")), vec!["he", "has", "got", "a", "boat"]);
    assert_eq!(expand_contractions(&w("it's raining")), vec!["it", "is", "raining"]);
    assert_eq!(expand_contractions(&w("he's a nice person")), vec!["he", "is", "a", "nice", "person"]);
    // A set-aside adverb does not hide the word that settles it.
    assert_eq!(expand_contractions(&w("it's never been")), vec!["it", "has", "never", "been"]);
    // An -ed participle could be either, so it is left for the reader to refuse.
    assert_eq!(expand_contractions(&w("he's tired")), vec!["he's", "tired"]);
}

#[test]
fn a_yes_no_copula_question_reads_and_round_trips() {
    use crate::ir::{Force, Role};
    let act = read("was the weather good?").expect("a yes/no copula question is covered");
    assert_eq!(act.force, Force::Ask);
    assert_eq!(act.frame.pred, "be");
    assert!(act.frame.get(&Role::As).is_some());
    assert_eq!(realise(&act), "was the weather good?");
    // A prepositional complement reads under its preposition.
    let act = read("is he at home?").expect("a located copula question is covered");
    assert_eq!(realise(&act), "is he at home?");
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
fn a_complement_clause_becomes_the_theme_and_round_trips() {
    use crate::ir::{Filler, Role};
    let act = read("i think she works in a shop.").expect("a complement clause is covered");
    assert_eq!(act.frame.pred, "think");
    let Some(Filler::Clause(inner)) = act.frame.get(&Role::Theme) else { panic!("theme is not a clause") };
    assert_eq!(inner.pred, "work");
    assert_eq!(realise(&act), "i think she works in a shop.");
    // A same-subject clause under a non-control verb stays finite.
    let act = read("i think i am late.").expect("a same-subject clause is covered");
    assert_eq!(realise(&act), "i think i am late.");
}

#[test]
fn a_negated_matrix_verb_keeps_its_clause() {
    // "don't" expands to "do not"; the round trip compares at the canonical level.
    let (matched, covered, _) = read_realise_rate(["i don't think she works in a shop."]);
    assert_eq!((matched, covered), (1, 1));
}

#[test]
fn a_past_copula_and_a_past_matrix_read() {
    let act = read("i knew sarah was busy.").expect("a past clause is covered");
    assert_eq!(act.frame.pred, "know");
    assert_eq!(realise(&act), "i knew sarah was busy.");
    let act = read("they were students.").expect("a past copula is covered");
    assert_eq!(act.frame.pred, "be");
    assert_eq!(realise(&act), "they were students.");
}

#[test]
fn a_plain_object_is_not_taken_for_a_clause() {
    use crate::ir::{Filler, Role};
    // "the answer" has no verb, so "know" keeps it as an ordinary theme.
    let act = read("i know the answer.").expect("a plain object is covered");
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Thing(_))));
    // "you in your car" is an object and a phrase, not a clause.
    let act = read("i saw you in your car.").expect("object and phrase are covered");
    assert!(!matches!(act.frame.get(&Role::Theme), Some(Filler::Clause(_))));
}

#[test]
fn to_is_an_infinitive_only_after_a_control_verb() {
    use crate::ir::{Filler, Role};
    // "want" controls an infinitive; "came to help" is a purpose phrase and is
    // not read as one.
    let act = read("i want to go.").expect("a control infinitive is covered");
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Clause(_))));
    let came = read("i came to help.");
    assert!(came.map_or(true, |a| !matches!(a.frame.get(&Role::Theme), Some(Filler::Clause(_)))));
}

#[test]
fn a_finite_clause_that_shares_the_subject_is_not_said_as_an_infinitive() {
    let act = read("i promise i will not tell anyone.").expect("a finite promise is covered");
    assert_eq!(realise(&act), "i promise i will not tell anyone.");
}

#[test]
fn i_wish_is_a_wish_force_over_the_wished_for_content() {
    use crate::ir::{Force, Modal};
    let act = read("i wish i could sing.").expect("a wish is covered");
    assert_eq!(act.force, Force::Wish);
    // The frame is the content, not a nested "wish": the gold's convention.
    assert_eq!(act.frame.pred, "sing");
    assert_eq!(act.frame.modal, Some(Modal::Can));
    assert_eq!(realise(&act), "i wish i could sing.");
    let act = read("i wish i had known.").expect("a past-perfect wish is covered");
    assert_eq!(act.force, Force::Wish);
    assert_eq!(realise(&act), "i wish i had known.");
}

#[test]
fn i_promise_is_an_explicit_commissive() {
    use crate::ir::Force;
    let act = read("i promise to come.").expect("a promise is covered");
    assert_eq!(act.force, Force::Commit);
    assert_eq!(realise(&act), "i promise to come.");
}

#[test]
fn a_verb_that_announces_no_force_leaves_the_clause_an_assertion() {
    use crate::ir::Force;
    // Only the explicit performatives set a force; "think" does not, and a bare
    // "will" is never read as a promise.
    let act = read("i think i could sing.").expect("a complement clause is covered");
    assert_eq!(act.force, Force::Assert);
    let act = read("i will explain the rule.").expect("a bare will is covered");
    assert_eq!(act.force, Force::Assert);
}

/// Diagnostic: which words are over-represented in the sentences the reader
/// abstains on, compared with those it reads? A word that appears in many
/// abstained sentences and in few read ones is a likely blocker. Run alone:
/// `cargo test -p phisor abstention_words -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: ranks words by how much they mark an abstained sentence"]
fn abstention_words() {
    use std::collections::{HashMap, HashSet};
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    let (text, key) = (
        std::fs::read_to_string(format!("{base}/person/murphy.txt")),
        std::fs::read_to_string(format!("{base}/person/murphy_key.txt")),
    );
    let (Ok(text), Ok(key)) = (text, key) else { println!("no guide"); return; };
    crate::lexicon::prime_irregular(&text);
    let split = crate::guide::Split::new(crate::guide::read(&text, &key));
    let clean = split.clean_strings(crate::guide::Part::Test);
    let (mut abst, mut read_n) = (0usize, 0usize);
    let mut in_abst: HashMap<String, usize> = HashMap::new();
    let mut in_read: HashMap<String, usize> = HashMap::new();
    for s in &clean {
        let words: HashSet<String> = s.trim_end_matches(['.', '?']).split_whitespace().map(str::to_string).collect();
        if read(s).is_some() { read_n += 1; for w in words { *in_read.entry(w).or_default() += 1; } }
        else { abst += 1; for w in words { *in_abst.entry(w).or_default() += 1; } }
    }
    println!("clean {} ; read {read_n} ; abstained {abst}", clean.len());
    // Rank by abstained count, showing the share of sentences with the word that were abstained.
    let mut rows: Vec<(String, usize, usize)> = in_abst.iter()
        .map(|(w, a)| (w.clone(), *a, *in_read.get(w).unwrap_or(&0)))
        .filter(|(_, a, r)| *a >= 12 && (*a as f64) / ((*a + *r) as f64) >= 0.9)
        .collect();
    rows.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
    println!("words in >= 12 abstained sentences and >= 90% abstained:");
    for (w, a, r) in rows.iter().take(60) { println!("{a:4} abst  {r:3} read  {w}"); }
}

#[test]
fn two_clauses_joined_by_a_conjunction_read_and_round_trip() {
    use crate::ir::{Filler, Role};
    let act = read("he works in a shop and she stays at home.").expect("coordinated clauses are covered");
    assert!(matches!(act.frame.get(&Role::named("and")), Some(Filler::Clause(_))));
    assert_eq!(realise(&act), "he works in a shop and she stays at home.");
    let act = read("i was tired so i went to bed.").expect("a past coordination is covered");
    assert_eq!(realise(&act), "i was tired so i went to bed.");
}

#[test]
fn a_subordinate_clause_is_an_adjunct_under_its_conjunction() {
    use crate::ir::{Filler, Role};
    let act = read("i will call you if i have time.").expect("a conditional is covered");
    assert!(matches!(act.frame.get(&Role::named("if")), Some(Filler::Clause(_))));
    assert_eq!(realise(&act), "i will call you if i have time.");
    // Contractions expand inside the clause, and the round trip compares canonically.
    let (matched, covered, _) = read_realise_rate(["i'll call you when i've finished the work."]);
    assert_eq!((matched, covered), (1, 1));
}

#[test]
fn a_conjunction_between_nouns_is_not_a_clause_split() {
    use crate::ir::Role;
    // "karen" is not a clause, so there is nothing to split at "and".
    let act = read("karen and joe played tennis.");
    assert!(act.map_or(true, |a| a.frame.get(&Role::named("and")).is_none()));
    // A noun pair as an object stays an object.
    let act = read("i like tea and coffee.");
    assert!(act.map_or(true, |a| a.frame.get(&Role::named("and")).is_none()));
}

#[test]
fn a_sentence_that_reads_whole_is_not_split() {
    use crate::ir::Role;
    let act = read("i know the answer.").expect("a plain sentence is covered");
    assert!(act.frame.get(&Role::named("and")).is_none());
    assert!(act.frame.get(&Role::named("so")).is_none());
}

#[test]
fn an_existential_reads_and_agrees_on_the_way_back() {
    use crate::ir::{Filler, Role};
    let act = read("there is a problem.").expect("an existential is covered");
    assert_eq!(act.frame.pred, "be");
    assert!(act.frame.get(&Role::Agent).is_none());
    assert!(matches!(act.frame.get(&Role::Theme), Some(Filler::Thing(_))));
    assert_eq!(realise(&act), "there is a problem.");
    // Plural theme: the copula agrees.
    let act = read("there are some tables.").expect("a plural existential is covered");
    assert_eq!(realise(&act), "there are some tables.");
    // Past and negative.
    let act = read("there was no milk.").expect("a negative past existential is covered");
    assert_eq!(realise(&act), "there was no milk.");
}

#[test]
fn there_as_a_place_is_not_mistaken_for_an_existential() {
    // "there" without a following copula is the ordinary adverb, not the dummy;
    // a sentence that does not fit the existential shape is read normally or
    // abstained, never forced into a be-frame.
    let act = read("i live there.");
    assert!(act.map_or(true, |a| a.frame.pred != "be"));
}

/// Diagnostic: how many words must be removed to make each abstained clean
/// sentence read? Greedily remove the single word whose removal most helps
/// (makes it read, or failing that keeps trying), up to four, and report the
/// distribution of distances. Also rank the words that, removed, carry a
/// sentence the last step into reading. Run alone:
/// `cargo test -p phisor distance_to_readable -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: counts how many fixes each abstained sentence is from reading"]
fn distance_to_readable() {
    use std::collections::HashMap;
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    let (text, key) = (
        std::fs::read_to_string(format!("{base}/person/murphy.txt")),
        std::fs::read_to_string(format!("{base}/person/murphy_key.txt")),
    );
    let (Ok(text), Ok(key)) = (text, key) else { println!("no guide"); return; };
    crate::lexicon::prime_irregular(&text);
    let split = crate::guide::Split::new(crate::guide::read(&text, &key));
    let clean = split.clean_strings(crate::guide::Part::Test);

    let reads = |ws: &[String], mark: char| -> bool {
        if ws.len() < 3 { return false; }
        read(&format!("{}{}", ws.join(" "), mark)).is_some()
    };
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut last_word: HashMap<String, usize> = HashMap::new();
    let mut abst = 0usize;
    for s in &clean {
        if read(s).is_some() { continue; }
        abst += 1;
        let mark = s.chars().last().unwrap();
        let mut ws: Vec<String> = s.trim_end_matches(['.', '?']).split_whitespace().map(str::to_string).collect();
        let mut removed = 0usize;
        let mut done = false;
        let mut final_word = String::new();
        while removed < 4 {
            // Is any single removal enough to read now?
            let mut hit = None;
            for i in 0..ws.len() {
                let mut t = ws.clone();
                let w = t.remove(i);
                if reads(&t, mark) { hit = Some((i, w)); break; }
            }
            if let Some((i, w)) = hit {
                final_word = w;
                ws.remove(i);
                removed += 1;
                done = true;
                break;
            }
            // Otherwise remove the word whose removal shrinks the sentence
            // toward a known blocker: just drop the last content word and retry.
            if ws.len() <= 3 { break; }
            ws.pop();
            removed += 1;
        }
        let d = if done { removed } else { 9 };
        *dist.entry(d).or_default() += 1;
        if done && d <= 2 && !final_word.is_empty() {
            *last_word.entry(final_word).or_default() += 1;
        }
    }
    println!("clean {} ; abstained {abst}", clean.len());
    let mut keys: Vec<_> = dist.keys().copied().collect();
    keys.sort();
    for k in keys {
        let label = if k == 9 { "4+ (or never)".to_string() } else { format!("{k} removal(s)") };
        println!("  {:>13}: {}", label, dist[&k]);
    }
    println!("final unlocking word (sentences 1-2 removals from reading):");
    let mut rows: Vec<_> = last_word.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (w, n) in rows.iter().take(40) { println!("  {n:4}  {w}"); }
}

#[test]
#[ignore = "diagnostic: copula sentences one removal from reading"]
fn copula_blockers() {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    let (text, key) = (
        std::fs::read_to_string(format!("{base}/person/murphy.txt")),
        std::fs::read_to_string(format!("{base}/person/murphy_key.txt")),
    );
    let (Ok(text), Ok(key)) = (text, key) else { println!("no guide"); return; };
    crate::lexicon::prime_irregular(&text);
    let split = crate::guide::Split::new(crate::guide::read(&text, &key));
    let clean = split.clean_strings(crate::guide::Part::Test);
    let mut shown = 0;
    for s in &clean {
        if read(s).is_some() { continue; }
        // A copula somewhere, and removing exactly one word makes it read.
        let has_cop = s.split_whitespace().any(|w| matches!(w, "is" | "are" | "was" | "were" | "be"));
        if !has_cop { continue; }
        let mark = s.chars().last().unwrap();
        let ws: Vec<&str> = s.trim_end_matches(['.', '?']).split_whitespace().collect();
        for i in 0..ws.len() {
            let kept: Vec<&str> = ws.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, w)| *w).collect();
            if kept.len() >= 3 && read(&format!("{}{}", kept.join(" "), mark)).is_some() {
                if shown < 40 { println!("drop [{}] <- {s}", ws[i]); shown += 1; }
                break;
            }
        }
    }
}

#[test]
#[ignore = "diagnostic: probe specific copula shapes"]
fn copula_probe() {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    for s in [
        "the office is empty.",
        "both restaurants are good.",
        "this notebook is important.",
        "rachel is from australia.",
        "this brush is for washing.",
        "it was a hot day.",
        "the house is dirty.",
    ] {
        println!("{:7} {s}", if read(s).is_some() { "READ" } else { "abstain" });
    }
}

#[test]
fn a_copula_takes_an_adjective_prepositional_or_modified_complement() {
    use crate::ir::{Filler, Role};
    // A prepositional complement reads under its preposition.
    let act = read("rachel is from australia.").expect("a prepositional complement is covered");
    assert_eq!(act.frame.pred, "be");
    assert_eq!(realise(&act), "rachel is from australia.");
    // An adjective before the head is kept as a Mod and round-trips.
    let act = read("it was a hot day.").expect("an adjective-modified complement is covered");
    let Some(Filler::Thing(t)) = act.frame.get(&Role::As) else { panic!("as is not a thing") };
    assert_eq!(t.head, "day");
    assert_eq!(realise(&act), "it was a hot day.");
    // "both" is a quantifier determiner on the subject.
    let act = read("both restaurants are good.").expect("a both-subject is covered");
    assert_eq!(realise(&act), "both restaurants are good.");
}
