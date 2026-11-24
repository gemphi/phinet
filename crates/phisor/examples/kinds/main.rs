//! `kinds` — the kinds of intention and their variations, found and not
//! named (docs/phisor/kinds.md). One exchange routine codes the words, as
//! deep as the data pays for, read at 16, 32 and 64. Then, one level up, it
//! codes the sentences' templates into kinds by the river: which template
//! follows which. Templates are fixed shapes (the opening, the closing, both
//! ends) or grown (the longest opening the guide shows twice). No label is
//! told. On the held-out sentences the guide's own labels are read against
//! the kinds: the mark, and docs/201's function (explain, instruct, and
//! answer from the never-told Key). The hash arm is the null and the given
//! dictionary the hand.
//!
//! ```text
//! cargo run --release -p phisor --example kinds -- crates/phiano/data/person/murphy.txt --key crates/phiano/data/person/murphy_key.txt [--shuffles 1000]
//! ```

mod detail;
mod report;

use phisor::code::{Code, Coder, LEVELS};
use phical::classes::Grow;
use phisor::guide::{self, Part, Split};
use phisor::kind::{Kinds, Reading, Twin};
use phiton::p64::variants::Level;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let flag = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned();
    let (Some(path), Some(keypath)) = (a.first().filter(|p| !p.starts_with("--")).cloned(), flag("--key")) else {
        eprintln!("usage: kinds <murphy.txt> --key <murphy_key.txt> [--shuffles 1000]");
        return;
    };
    let shuffles: usize = flag("--shuffles").and_then(|s| s.parse().ok()).unwrap_or(1000);
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let key = std::fs::read_to_string(&keypath).unwrap_or_default();
    let split = Split::new(guide::read(&text, &key));
    let (train, dev, test) = (split.of(Part::Train), split.of(Part::Dev), split.of(Part::Test));
    println!("# the kinds of intention — {path}\n");
    println!("{} sentences: train {} · dev {} · held out {} (every answer from the Key among them)\n", split.sentences.len(), train.len(), dev.len(), test.len());
    report::labels(&split, &test);

    let told: Vec<&[String]> = train.iter().map(|&i| split.sentences[i].words.as_slice()).collect();
    let started = std::time::Instant::now();
    let code = Code::learn(&told, Grow::Paid);
    println!("\nthe word code, grown as far as it pays: {} words coded, {} classes at depth {}, learned in {:.1}s", code.len(), code.halving.classes, code.halving.depth, started.elapsed().as_secs_f64());
    report::passes("words", &code.halving);
    report::classes(&code);
    // The same routine with opening senses: a word that opens sentences is a
    // unit of its own there (`^do`), the channel after `<s>`.
    let opening = Code::learn_opening(&told, Grow::Paid);
    println!("\nwith opening senses: {} units coded, {} classes at depth {}", opening.len(), opening.halving.classes, opening.halving.depth);
    let coders = [Coder::Hash, Coder::Given, Coder::Learned(code), Coder::Learned(opening)];
    report::story(&split, &train, &coders);

    println!("\n## every arm, mode and template, kinds grown as far as they pay, on the held-out sentences\n");
    report::header();
    let mut kept: Vec<(&'static str, Kinds)> = Vec::new();
    for coder in &coders {
        let levels: &[Level] = match coder {
            Coder::Given => &LEVELS[..1],
            _ => &LEVELS,
        };
        for &level in levels {
            for twin in Twin::ALL {
                for reading in Reading::ALL {
                    let kinds = Kinds::learn(&split, coder, level, twin, reading, Grow::Paid);
                    let arm = format!("{} · {}", coder.name(), reading.name());
                    println!("{}", report::row(&arm, &kinds, &split, shuffles));
                    // The inside kinds are read closely: the river's are on record
                    // (docs/phisor/runs/kinds-2026-10-29-paid.md).
                    // Every arm at mode 16; the learned codes at modes 32 and 64 too.
                    match (level, twin, reading, coder) {
                        (Level::P16, Twin::Open, Reading::Inside, _) => kept.push((coder.name(), kinds)),
                        (Level::P32 | Level::P64, Twin::Open, Reading::Inside, Coder::Learned(_)) => kept.push((coder.name(), kinds)),
                        _ => {}
                    }
                }
            }
        }
    }
    report::baselines(&split, &test);

    println!("\n## the bridge to the first slice: 64 kinds fixed against kinds that pay, mode 16\n");
    report::header();
    for coder in &coders {
        for twin in [Twin::Open, Twin::Grown] {
            // Fixed kinds against kinds that pay: a like-for-like comparison, so hold
            // the reading constant at River, the reading the first slice was read by.
            let kinds = Kinds::learn(&split, coder, Level::P16, twin, Reading::River, Grow::Fixed(6));
            println!("{}", report::row(coder.name(), &kinds, &split, shuffles));
        }
    }

    for (name, kinds) in &kept {
        println!("\n## {name}, mode {}, the opening, read inside: what each label is sensed by\n", kinds.level.count());
        detail::sensing(kinds, &split);
        println!();
        detail::resemblance_null(kinds, 200);
        detail::passes(kinds);
    }
    match kept.iter().find(|(n, k)| *n == "learned" && k.level == Level::P16) {
        Some((_, kinds)) => {
            println!("\n## learned, mode 16, the opening, read inside: the kinds\n");
            detail::kinds(kinds, &split);
        }
        None => {}
    }
}
