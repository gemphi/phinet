//! Every phasor range, measured: what each quantum costs at each resolution,
//! and how an operator trained at one resolution reads at the others.
//! `cargo run --release --example ranges`
use phistd::phasor::{quantisation_loss, resample, sector_of, Quantum, Range, Resolution};
use phistd::{unit_random, Adam, Function};
use phical::{Tie, rank, Example, Fno, FnoConfig};

fn main() {
    // 1. quantisation cost per range, 200 unit-random functions at P64 resampled to each resolution
    println!("| range | sectors | bound (rad) | mean quantisation loss | sector agrees with P64·Q64 |");
    println!("|---|---|---|---|---|");
    let base: Vec<Function> = (0..200).map(|i| unit_random(64, 1000 + i)).collect();
    for r in Range::ALL {
        let (mut loss, mut agree) = (0.0, 0usize);
        for f in &base {
            let g = resample(f, r.resolution);
            loss += quantisation_loss(&g, r.quantum);
            // the same function's sector at the default range, mapped down to this quantum
            let s64 = sector_of(f, Quantum::Q64);
            let mapped = s64 * r.sectors() / 64;
            if sector_of(&g, r.quantum) == mapped { agree += 1; }
        }
        println!("| {} | {} | {:.4} | {:.4} | {:.0}% |", r.name(), r.sectors(), r.quantum.bound(), loss / base.len() as f64, 100.0 * agree as f64 / base.len() as f64);
    }

    // 2. an operator trained at P32 on rotate-by-one, transferred to every resolution
    println!("\n| trained at | applied at | shift there | MRR identity | MRR transferred | cos(pred, target) transferred | MRR trained natively | cos native |");
    println!("|---|---|---|---|---|---|---|---|");
    let n = 60;
    let make = |space: usize, by: usize, seed: u64| {
        let inputs: Vec<Function> = (0..n).map(|i| resample(&unit_random(32, seed + i as u64), Resolution::from_space(space).unwrap())).collect();
        let targets: Vec<Function> = inputs.iter().map(|f| { let mut g = f.clone(); g.rotate_right(by); g }).collect();
        let ex: Vec<Example> = (0..n).map(|i| Example { input: inputs[i].clone(), target: i }).collect();
        (inputs, targets, ex)
    };
    let pool: Vec<usize> = (0..n).collect();
    let cfg = FnoConfig { space: 32, width: 4, layers: 1, modes: 8, margin: 0.5, lr: 1e-2, tie: Tie::Circle };
    let (i32_, t32, e32) = make(32, 1, 500);
    let mut fno = Fno::new(cfg, 1);
    let mut adam = Adam::new(cfg.lr);
    for e in 0..80 { fno.train_epoch(&t32, &e32, &pool, 20, &mut adam, e); }
    for res in Resolution::ALL {
        let space = res.space();
        let by = (space / 32).max(1); // the same rotation of the circle, 1/32 of a turn (P16: half a sample — use 1)
        let (inp, tgt, ex) = make(space, by, 500);
        let (mrr_id, _) = rank(&tgt, &inp, &ex, &pool);
        let mut moved = fno.at_resolution(space);
        let preds = moved.apply(&inp);
        let (mrr_tr, _) = rank(&tgt, &preds, &ex, &pool);
        let cos_tr = preds.iter().zip(&tgt).map(|(p, t)| phistd::cosine(p, t)).sum::<f64>() / n as f64;
        let mut native = Fno::new(FnoConfig { space, modes: cfg.modes.min(space / 2), ..cfg }, 1);
        let mut a2 = Adam::new(cfg.lr);
        for e in 0..80 { native.train_epoch(&tgt, &ex, &pool, 20, &mut a2, e); }
        let npreds = native.apply(&inp);
        let (mrr_nat, _) = rank(&tgt, &npreds, &ex, &pool);
        let cos_nat = npreds.iter().zip(&tgt).map(|(p, t)| phistd::cosine(p, t)).sum::<f64>() / n as f64;
        println!("| P32 | P{space} | {by} sample{} | {mrr_id:.3} | {mrr_tr:.3} | {cos_tr:.3} | {mrr_nat:.3} | {cos_nat:.3} |", if by == 1 { "" } else { "s" });
    }
    let _ = (i32_, t32);
}
