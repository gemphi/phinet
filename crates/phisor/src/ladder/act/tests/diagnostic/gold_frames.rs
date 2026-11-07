//! Ignored diagnostics over the gold set.

use super::*;
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
