//! Wall-clock of the hot paths on this machine. `cargo run --release --example bench`
use phistd::tensor::fft_inplace;
use phistd::{unit_random, Adam, Tape, Tensor};
use phical::{Tie, Example, Fno, FnoConfig, MixtureExample};
use std::time::Instant;

fn main() {
    let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    println!("cores: {cores}");
    let space = 64;
    // fft
    let mut f = unit_random(space, 1);
    let t = Instant::now();
    for _ in 0..100_000 { fft_inplace(&mut f, false); fft_inplace(&mut f, true); }
    println!("fft+ifft 64-point ×100k: {:.3}s ({:.2} µs per pair)", t.elapsed().as_secs_f64(), t.elapsed().as_secs_f64() * 10.0);
    // apply
    let cfg = FnoConfig { space, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: Tie::Circle };
    let mut fno = Fno::new(cfg, 1);
    let inputs: Vec<_> = (0..4096).map(|i| unit_random(space, i)).collect();
    let t = Instant::now();
    let _ = fno.apply(&inputs);
    println!("Fno::apply 4096 × (64, w8, L2, m12; {} params): {:.3}s ({:.1} µs each)", fno.parameter_count(), t.elapsed().as_secs_f64(), t.elapsed().as_secs_f64() * 1e6 / 4096.0);
    // hinge epoch
    let table: Vec<_> = (0..2000).map(|i| unit_random(space, 10_000 + i)).collect();
    let examples: Vec<Example> = (0..10_000).map(|i| Example { input: unit_random(space, 20_000 + i as u64), target: i % 2000 }).collect();
    let pool: Vec<usize> = (0..2000).collect();
    let mut adam = Adam::new(cfg.lr);
    let t = Instant::now();
    fno.train_epoch(&table, &examples, &pool, 64, &mut adam, 0);
    println!("train_epoch hinge, 10k examples, batch 64: {:.3}s", t.elapsed().as_secs_f64());
    // mixture epoch (full table softmax)
    let mex: Vec<MixtureExample> = examples.iter().take(2000).map(|e| MixtureExample { input: e.input.clone(), target: e.target, k: 0.01, coef: 0.8, other: 0.05 }).collect();
    let t = Instant::now();
    fno.train_epoch_mixture(&table, &mex, 0.5, 8.0, 64, &mut adam, 0);
    println!("train_epoch_mixture, 2k examples, table 2000, batch 64: {:.3}s", t.elapsed().as_secs_f64());
    // cosine_table alone
    let mut tape = Tape::new();
    let p = tape.constant(Tensor::from_vec([256, 1, space], (0..256).flat_map(|i| unit_random(space, 30_000 + i)).collect()));
    let tb = tape.constant(Tensor::from_vec([6000, 1, space], (0..6000).flat_map(|i| unit_random(space, 40_000 + i)).collect()));
    let t = Instant::now();
    let _ = tape.cosine_table(p, tb);
    println!("cosine_table 256 × 6000 × 64: {:.3}s", t.elapsed().as_secs_f64());
    let bytes = fno.to_bytes();
    println!("to_bytes: {} bytes for {} params", bytes.len(), fno.parameter_count());
}
