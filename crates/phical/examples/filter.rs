//! Circle vs sphere on a relation that is one gain per degree — the
//! synthetic task where the sphere tie is the exact operator.
//!
//! `cargo run --release --example filter`
use phical::{rank, Adam, Example, Fno, FnoConfig, Function};
use phifno::{degree_of, HARMONICS};
use phistd::unit_random;

fn main() {
    let gains = [1.0, 0.8, -0.5, 0.3, 1.2, -1.0, 0.5, 0.1];
    let inputs: Vec<Function> = (0..200).map(|i| unit_random(HARMONICS, i as u64)).collect();
    let targets: Vec<Function> = inputs
        .iter()
        .map(|f| f.iter().enumerate().map(|(k, c)| c * gains[degree_of(k)]).collect())
        .collect();
    let examples: Vec<Example> = (0..200).map(|i| Example { input: inputs[i].clone(), target: i }).collect();
    let pool: Vec<usize> = (0..200).collect();
    let (id, _) = rank(&targets, &inputs, &examples, &pool);
    println!("degree filter · 200 items · chance MRR {:.4} · identity MRR {id:.3}\n", 1.0 / 200.0);
    println!("| operator | params | epochs | MRR |\n|---|---|---|---|");
    for (name, cfg) in [
        ("circle (FFT, 12 modes)", FnoConfig::circle64()),
        ("sphere (one weight per degree)", FnoConfig::sphere()),
        ("anisotropic (one weight per l,|m|)", FnoConfig::anisotropic()),
    ] {
        let mut fno = Fno::new(cfg, 1);
        let mut adam = Adam::new(cfg.lr);
        for epoch in 0..40 {
            fno.train_epoch(&targets, &examples, &pool, 20, &mut adam, epoch);
        }
        let (mrr, _) = rank(&targets, &fno.apply(&inputs), &examples, &pool);
        println!("| {name} | {} | 40 | {mrr:.3} |", fno.parameter_count());
    }
}
