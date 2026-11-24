//! `feynman` — the twins and the combinations (docs/phisor/kinds.md,
//! "Feynman's run" and "Learned phases"; R9.6–R9.10). A held-out sentence is
//! read three ways in one space of kinds: from its opening alone, its closing
//! alone, and both ends. The two single ends are then combined: as
//! probabilities added (the classical sum), as amplitudes added and squared
//! with the phases aligned (the keyboard's `twins`), and as amplitudes added
//! at phases learned on the training sentences, so two readings can cancel.
//! Purity of the guide's labels (explain, instruct, answer) is read for each
//! on the held-out sentences; none of them is told, and the phases are
//! learned without a label.
//!
//! ```text
//! cargo run --release -p phisor --example feynman -- crates/phiano/data/person/murphy.txt --key crates/phiano/data/person/murphy_key.txt [--seed 7]
//! ```

use phical::classes::Grow;
use phisor::code::{Code, Coder, LEVELS};
use phisor::guide::{self, Part, Split};
use phisor::ladder::{Judged, Measured, Unit};
use phisor::measure::phase::{self, Row};
use phisor::measure::{measure_modes, readings, ModeInput, ModeReport};

fn verdict(j: &Judged) -> String {
    match j {
        Judged::Pass => "pass".to_string(),
        Judged::Miss { shortfall, .. } => format!("miss by {shortfall:.1}"),
    }
}

fn twins(arm: &str, r: &ModeReport) -> String {
    let c = &r.combinations;
    format!(
        "| {arm} | {} | {} | {:.1}% | {:.1}% | {:.1}% | {} | {} | {:.1}% | {:.1}% | {:+.1} | {} |",
        r.level.count(),
        r.scored,
        r.purity.opening,
        r.purity.closing,
        r.purity.both,
        verdict(&r.both_ends.verdict),
        r.disagreements.count,
        c.probability,
        c.amplitude.value,
        c.difference_points,
        verdict(&c.amplitude.verdict),
    )
}

fn learned(arm: &str, r: &ModeReport, input: &ModeInput, rows: &[Row]) -> String {
    let kinds = input.sentences.first().map_or(0, |s| s.opening_probabilities.len());
    let phases = phase::learn(rows, kinds);
    let purity = phase::purity_at(input, &phases);
    let judged = Measured::at_least("learn.phases", purity, r.combinations.probability, Unit::Percentage);
    let (off, far) = phases.turned();
    format!(
        "| {arm} | {} | {} → {} of {} | {} | {off} | {far} | {:.1}% | {:.1}% | **{purity:.1}%** | {} |",
        r.level.count(),
        phases.hits[0],
        phases.hits.last().copied().unwrap_or(0),
        rows.len(),
        phases.changed.len(),
        r.combinations.probability,
        r.combinations.amplitude.value,
        verdict(&judged.verdict),
    )
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let flag = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned();
    let (Some(path), Some(keypath)) = (a.first().filter(|p| !p.starts_with("--")).cloned(), flag("--key")) else {
        eprintln!("usage: feynman <murphy.txt> --key <murphy_key.txt> [--seed 7]");
        return;
    };
    let seed: u64 = flag("--seed").and_then(|s| s.parse().ok()).unwrap_or(7);
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let key = std::fs::read_to_string(&keypath).unwrap_or_default();
    let split = Split::new(guide::read(&text, &key));
    let train = split.of(Part::Train);
    let told: Vec<&[String]> = train.iter().map(|&i| split.sentences[i].words.as_slice()).collect();
    let coders = [Coder::Hash, Coder::Given, Coder::Learned(Code::learn(&told, Grow::Paid))];

    println!("# Feynman's run — {path}\n");
    println!("held out: {} sentences · the aligned arrows' seed: {seed} (the opening's; the closing's is seed ^ 1)\n", split.of(Part::Test).len());
    let mut done: Vec<(&str, [ModeReport; 3], Vec<(ModeInput, Vec<Row>)>)> = Vec::new();
    println!("| arm | mode | scored | opening | closing | both ends | both ≥ better end + 2 | disagree | probabilities added | amplitudes aligned | difference | aligned ≥ probabilities |");
    println!("|---|---|---|---|---|---|---|---|---|---|---|---|");
    for coder in &coders {
        let read: Vec<(ModeInput, Vec<Row>)> = LEVELS.iter().map(|&level| readings(&split, coder, level, Grow::Paid, seed)).collect();
        let inputs: [ModeInput; 3] = std::array::from_fn(|i| read[i].0.clone());
        let reports = measure_modes(&inputs);
        for r in &reports {
            println!("{}", twins(coder.name(), r));
        }
        done.push((coder.name(), reports, read));
    }

    println!("\n## learned phases\n");
    println!("Each kind's closing arrow turned by one of sixteen phases, kept when more training sentences peak at their own both-ends kind (no label read).\n");
    println!("| arm | mode | training read as their own kind, phase 0 → learned | passes | kinds turned | a quarter turn or more | probabilities added | amplitudes aligned | learned phases | learned ≥ probabilities |");
    println!("|---|---|---|---|---|---|---|---|---|---|");
    for (arm, reports, read) in &done {
        for (r, (input, rows)) in reports.iter().zip(read) {
            println!("{}", learned(arm, r, input, rows));
        }
    }
}
