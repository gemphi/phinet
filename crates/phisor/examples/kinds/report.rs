//! The grid's rows and the setting around them: the labels held out, the
//! routine's passes, the word classes, a story in mode 16, the baselines.

use phisor::code::{show, Code, Coder};
use phical::classes::Halving;
use phisor::guide::{Function, Mark, Part, Split};
use phisor::kind::{pairs_table, river, Kinds, Tables};
use phistd::stats::{mutual_information, purity, z_score};
use phiton::p64::variants::Level;

/// How the held-out sentences fall under each label set.
pub fn labels(split: &Split, test: &[usize]) {
    let count = |f: &dyn Fn(usize) -> bool| test.iter().filter(|&&i| f(i)).count();
    let functions: Vec<String> = Function::ALL.iter().map(|&f| format!("{} {}", f.name(), count(&|i| split.sentences[i].function == f))).collect();
    let marks: Vec<String> = Mark::ALL.iter().map(|&m| format!("{} {}", m.name(), count(&|i| split.sentences[i].function != Function::Answer && split.sentences[i].mark == m))).collect();
    println!("held out by function: {} · by mark (the units' sentences; the Key has none): {}", functions.join(", "), marks.join(", "));
}

/// The likelihood the routine gained, pass by pass, and the splits kept, at each depth.
pub fn passes(what: &str, h: &Halving) {
    for (d, gains) in h.passes.iter().enumerate() {
        let g: Vec<String> = gains.iter().map(|x| format!("{x:.0}")).collect();
        println!("  {what}, depth {}: {} splits kept, {} passes, gains {}", d + 1, h.kept.get(d).copied().unwrap_or(0), gains.len(), g.join(" "));
    }
}

/// Every class of the learned code at mode 16, read by its commonest members.
pub fn classes(code: &Code) {
    println!("\nthe learned classes at mode 16, most told members first:\n");
    for c in 0..16u16 {
        let m = code.members(c, 4);
        let shown: Vec<&str> = m.iter().take(14).copied().collect();
        println!("- c{c} ({} words): {}", m.len(), shown.join(" "));
    }
}

/// The first training sentences of the guide, in words and in mode 16 by each arm.
pub fn story(split: &Split, train: &[usize], coders: &[Coder]) {
    println!("\n## a story in mode 16: structure only\n");
    for &i in train.iter().take(6) {
        let w = &split.sentences[i].words;
        println!("- {}", w.join(" "));
        for c in coders {
            println!("  - {}: {}", c.name(), show(&c.write(w, Level::P16)));
        }
    }
}

/// The majority share of a label set: the purity of one row of column sums.
fn majority(t: &[Vec<f64>]) -> f64 {
    let cols = t.first().map_or(0, Vec::len);
    let sums: Vec<f64> = (0..cols).map(|j| t.iter().map(|r| r[j]).sum()).collect();
    purity(&[sums])
}

/// The baselines every purity is read against.
pub fn baselines(split: &Split, test: &[usize]) {
    let f: Vec<Vec<f64>> = vec![Function::ALL.iter().map(|&x| test.iter().filter(|&&i| split.sentences[i].function == x).count() as f64).collect()];
    let m: Vec<Vec<f64>> = vec![Mark::ALL.iter().map(|&x| test.iter().filter(|&&i| split.sentences[i].function != Function::Answer && split.sentences[i].mark == x).count() as f64).collect()];
    println!("\nthe majority baselines on the held-out sentences: function {:.1}%, mark {:.1}%", 100.0 * majority(&f), 100.0 * majority(&m));
}

pub fn header() {
    println!("| arm | mode | template | templates | kinds | depth | function purity | mark purity | river MI (bits) | river z |");
    println!("|---|---|---|---|---|---|---|---|---|---|");
}

/// One arm, mode and template: its kinds, read on the held-out sentences.
pub fn row(arm: &str, kinds: &Kinds, split: &Split, shuffles: usize) -> String {
    let t = Tables::read(kinds, split, Part::Test);
    let pairs = river(kinds, split, Part::Test);
    let k = kinds.count();
    let mi = mutual_information(&pairs_table(&pairs, k));
    let null = shuffled(&pairs, k, shuffles);
    format!(
        "| {arm} | {} | {} | {} | {} | {} | {:.1}% | {:.1}% | {mi:.3} | {:.1} |",
        kinds.level.count(),
        kinds.twin.name(),
        kinds.templates(),
        kinds.occupied(),
        kinds.halving.depth,
        100.0 * purity(&t.function),
        100.0 * purity(&t.mark),
        z_score(mi, &null)
    )
}

/// The river's mutual information with the sentence before shuffled away,
/// `n` times (a fixed seed, so a run repeats).
fn shuffled(pairs: &[(usize, usize)], k: usize, n: usize) -> Vec<f64> {
    let mut state = 0x9e37_79b9_7f4a_7c15u64;
    let mut before: Vec<usize> = pairs.iter().map(|p| p.0).collect();
    (0..n)
        .map(|_| {
            for i in (1..before.len()).rev() {
                state = phistd::random::xorshift64(state);
                before.swap(i, (state % (i as u64 + 1)) as usize);
            }
            let p: Vec<(usize, usize)> = before.iter().zip(pairs).map(|(&a, &(_, b))| (a, b)).collect();
            mutual_information(&pairs_table(&p, k))
        })
        .collect()
}
