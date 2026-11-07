//! The gold round trip and the pinned gold misses.

use super::*;

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

/// Why a gold frame the realiser misses is missed (task 1.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Miss {
    /// The frame holds the information; the realiser picks the wrong form.
    RealiserForm,
    /// The frame has no field for a word of the sentence.
    NoSlot,
}

/// The 14 gold frames the strict round trip misses, by gold index (0-based
/// file order), with the class of each. Gold is frozen, so this is the
/// yardstick for tasks 2.1 to 2.5: a `RealiserForm` frame is fixed in the
/// realiser, a `NoSlot` frame can only become an Unrealisable_Frame.
///
/// `NoSlot`, sentence beside frame (the frame is the gold `Act`):
///
/// - 0   `water boils at one hundred degrees.` / `Via(at)` Thing `degree`
///   plural, `Count = hundred`: the `one` of `one hundred` is not held.
/// - 27  `we did not go out.` / `go`, Agent `we`, Past, Negative: no particle
///   field for `out`.
/// - 38  `she has worn that suit before.` / `wear`, Perfect: no adverb field
///   for `before`.
/// - 42  `he had already left.` / `leave`, Past Perfect: no adverb field for
///   `already`.
/// - 43  `we had never seen such a storm.` / `see`, Negative, `never: false`,
///   Theme `storm` with `Indefinite`: the gold sets neither `never` nor a
///   `such` determiner, so `never` and `such` have no value to realise.
/// - 61  `they may come later.` / `come`, modal May: no adverb field for
///   `later`.
/// - 102 `please close the window.` / `close`, Direct: no politeness field
///   for `please`.
/// - 104 `read the text again.` / `read`, Direct: no adverb field for `again`.
/// - 121 `i am going to study french.` / Commit, modal Will: `going to` and
///   `will` are one frame, so the realiser says `will`.
/// - 146 `the meeting is declared closed.` / `close`, Passive: the result
///   complement `declared` has no field.
/// - 147 `i pronounce you husband and wife.` / `As` holds `husband` alone: the
///   coordinated `and wife` has no field.
/// - 148 `the meeting is hereby adjourned.` / `adjourn`, Passive: no adverb
///   field for `hereby`.
/// - 149 `i hereby appoint you chairman.` / `appoint`: no adverb field for
///   `hereby`.
///
/// `RealiserForm`: none left. Frame 89 (`Via(for)` with a `How` hole, said
/// "how long") is fixed in task 2.3 from `wh_phrase.tsv`; the Express frames
/// 127 to 131 (`thank`, `apologise`, `congratulate`, `welcome`) are fixed
/// phrases, fixed in task 2.1 from `expressive_phrase.tsv`; the wish frames
/// 137, 138 and 143 are fixed in task 2.2 (`wish_form.tsv`).
const GOLD_MISSES: [(usize, Miss); 13] = [
    (0, Miss::NoSlot),
    (27, Miss::NoSlot),
    (38, Miss::NoSlot),
    (42, Miss::NoSlot),
    (43, Miss::NoSlot),
    (61, Miss::NoSlot),
    (102, Miss::NoSlot),
    (104, Miss::NoSlot),
    (121, Miss::NoSlot),
    (146, Miss::NoSlot),
    (147, Miss::NoSlot),
    (148, Miss::NoSlot),
    (149, Miss::NoSlot),
];

/// Pins the failing gold frames by index and class. It fails when a frame
/// starts to round-trip (shrink the table and move the count) or a new one
/// breaks. Task 2.x changes it as each `RealiserForm` frame is fixed.
#[test]
fn the_failing_gold_frames_are_pinned_by_index_and_class() {
    let gold = gold::read().expect("the frozen gold set loads");
    let failing: Vec<usize> = gold
        .pairs()
        .enumerate()
        .filter(|(_, (act, sentence))| realise(act) != crate::guide::holdout::normalise(sentence))
        .map(|(i, _)| i)
        .collect();
    let pinned: Vec<usize> = GOLD_MISSES.iter().map(|(i, _)| *i).collect();
    assert_eq!(failing, pinned, "the set of gold frames the strict round trip misses moved");
    let count = |m: Miss| GOLD_MISSES.iter().filter(|(_, c)| *c == m).count();
    assert_eq!((count(Miss::RealiserForm), count(Miss::NoSlot)), (0, 13));
    assert!(pinned.iter().all(|i| *i < gold::COUNT));
    // The Unrealisable list is exactly the frames still failing (task 2.4).
    let listed: Vec<usize> = gold::UNREALISABLE.iter().map(|u| u.index).collect();
    assert_eq!(listed, pinned, "the Unrealisable list and the failing frames differ");
}
