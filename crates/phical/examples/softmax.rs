//! Where a softmax epoch spends its time: one batch of 64 against a 1228-row table.
use phistd::{Adam, Function, c64};
use phical::{Tie, Fno, FnoConfig};
use std::time::Instant;

fn main() {
    let s: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(64);
    let (b, v) = (64usize, 1228usize);
    let f = |seed: usize| -> Function { (0..s).map(|k| c64::from_polar(1.0, ((seed * 31 + k * 7) % 97) as f64 * 0.0647)).collect() };
    let table: Vec<Function> = (0..v).map(f).collect();
    let inputs: Vec<Function> = (0..b).map(|i| f(i + 5000)).collect();
    let targets: Vec<usize> = (0..b).map(|i| i * 19 % v).collect();
    let cfg = FnoConfig { space: s, width: 16, layers: 3, modes: 16, margin: 0.2, lr: 2e-3, tie: Tie::Circle };
    let mut fno = Fno::new(cfg, 7);
    let params = fno.params();
    let mut adam = Adam::new(cfg.lr);
    for round in 0..3 {
        let t0 = Instant::now();
        fno.reset();
        let x = fno.tape.constant(phistd::stack(&inputs, s));
        let table_t = phistd::stack(&table, s);
        let t1 = Instant::now();
        let t = fno.tape.constant(table_t);
        let y = fno.forward(x);
        let t2 = Instant::now();
        let scores = fno.tape.cosine_table(y, t);
        let t3 = Instant::now();
        let loss = fno.tape.cross_entropy(scores, &targets, 8.0);
        let t4 = Instant::now();
        fno.tape.backward(loss);
        let t5 = Instant::now();
        adam.step(&mut fno.tape, &params);
        fno.mask_modes();
        let t6 = Instant::now();
        if round == 2 {
            println!("s={s} b={b} v={v}: stack {:.1} ms · forward {:.1} · cosine_table {:.1} · xent {:.1} · backward {:.1} · adam {:.1} · total {:.1} ms",
                ms(t0, t1), ms(t1, t2), ms(t2, t3), ms(t3, t4), ms(t4, t5), ms(t5, t6), ms(t0, t6));
        }
    }
}

fn ms(a: Instant, b: Instant) -> f64 { (b - a).as_secs_f64() * 1e3 }
