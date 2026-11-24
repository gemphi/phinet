//! `class_tree` — where two words part in the learned code, and the tree of
//! a class beneath its mode-16 cut (docs/phisor/kinds.md, "Parting c12").
//! The code is grown as far as it pays on the guide's training sentences,
//! as the `kinds` example grows it. For each pair named, the depth at which
//! the two codes first differ, or that they never do; then the mode-16
//! class of the first word, read by its members at every depth below it.
//!
//! ```text
//! cargo run --release -p phisor --example class_tree -- crates/phiano/data/person/murphy.txt --key crates/phiano/data/person/murphy_key.txt --pair do,use --pair did,complete [--opening]
//! ```
//!
//! With `--opening` the code is learned with opening senses, and a pair may
//! name them: `--pair ^do,^use`.

use phical::classes::Grow;
use phisor::code::Code;
use phisor::guide::{self, Part, Split};

/// The first depth at which two words' classes differ, if they ever do.
fn parting(code: &Code, a: &str, b: &str) -> Option<u32> {
    (1..=code.halving.depth).find(|&bits| code.class(a, bits) != code.class(b, bits))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |k: &str| args.iter().position(|x| x == k).and_then(|i| args.get(i + 1)).cloned();
    let pairs: Vec<(String, String)> = args
        .windows(2)
        .filter(|w| w[0] == "--pair")
        .filter_map(|w| w[1].split_once(',').map(|(a, b)| (a.to_string(), b.to_string())))
        .collect();
    let (Some(path), Some(keypath)) = (args.first().filter(|p| !p.starts_with("--")).cloned(), flag("--key")) else {
        eprintln!("usage: class_tree <murphy.txt> --key <murphy_key.txt> --pair do,use [--pair …]");
        return;
    };
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let key = std::fs::read_to_string(&keypath).unwrap_or_default();
    let split = Split::new(guide::read(&text, &key));
    let told: Vec<&[String]> = split.of(Part::Train).iter().map(|&i| split.sentences[i].words.as_slice()).collect();
    // `--opening`: the words that open sentences get senses of their own there (`^do`).
    let code = match args.iter().any(|x| x == "--opening") {
        true => Code::learn_opening(&told, Grow::Paid),
        false => Code::learn(&told, Grow::Paid),
    };
    println!("# where words part — the code at depth {}, {} classes\n", code.halving.depth, code.halving.classes);
    println!("| pair | mode-16 classes | parts at depth |");
    println!("|---|---|---|");
    for (a, b) in &pairs {
        let at = parting(&code, a, b).map_or("never".to_string(), |d| d.to_string());
        println!("| {a} · {b} | c{} · c{} | {at} |", code.class(a, 4), code.class(b, 4));
    }
    let Some((first, _)) = pairs.first() else { return };
    let top = code.class(first, 4);
    println!("\n## c{top} at mode 16, and beneath it\n");
    for bits in 4..=code.halving.depth {
        let shift = bits - 4;
        for class in (top << shift)..((top + 1) << shift) {
            let m = code.members(class, bits);
            match m.is_empty() {
                true => {}
                false => println!("- depth {bits}, class {class} ({} words): {}", m.len(), m.iter().take(16).copied().collect::<Vec<_>>().join(" ")),
            }
        }
    }
}
