//! Per-op forward/backward timing at the operator's working sizes.
use phistd::{Tape, Tensor, c64};
use std::time::Instant;

fn main() {
    let (b, w, s) = (64usize, 16usize, 64usize);
    let mk = |shape: [usize; 3], seed: usize| {
        let n = shape[0] * shape[1] * shape[2];
        Tensor::from_vec(shape, (0..n).map(|k| c64::from_polar(1.0, ((seed * 31 + k * 7) % 97) as f64 * 0.0647)).collect())
    };
    let ops: Vec<(&str, Box<dyn Fn(&mut Tape) -> phistd::Var>)> = vec![
        ("linear", Box::new(move |t: &mut Tape| { let x = t.constant(mk([b, w, s], 1)); let wv = t.param(mk([w, w, 1], 2)); t.linear(x, wv) })),
        ("add_bias", Box::new(move |t: &mut Tape| { let x = t.param(mk([b, w, s], 1)); let bv = t.param(mk([1, w, 1], 2)); t.add_bias(x, bv) })),
        ("gelu", Box::new(move |t: &mut Tape| { let x = t.param(mk([b, w, s], 1)); t.gelu(x) })),
        ("fft", Box::new(move |t: &mut Tape| { let x = t.param(mk([b, w, s], 1)); t.fft(x) })),
        ("mode_mix", Box::new(move |t: &mut Tape| { let x = t.param(mk([b, w, s], 1)); let r = t.param(mk([w, w, s], 2)); t.mode_mix(x, r) })),
        ("add", Box::new(move |t: &mut Tape| { let x = t.param(mk([b, w, s], 1)); let y = t.param(mk([b, w, s], 2)); t.add(x, y) })),
    ];
    for (name, f) in &ops {
        let mut fwd = 0.0; let mut bwd = 0.0;
        for round in 0..4 {
            let mut t = Tape::new();
            let t0 = Instant::now();
            let y = f(&mut t);
            let m = t.mean_real(y);
            let t1 = Instant::now();
            t.backward(m);
            let t2 = Instant::now();
            if round > 0 { fwd += (t1 - t0).as_secs_f64() * 1e3 / 3.0; bwd += (t2 - t1).as_secs_f64() * 1e3 / 3.0; }
        }
        println!("{name:9} forward {fwd:7.2} ms  backward {bwd:7.2} ms   [b={b} w={w} s={s}]");
    }
}
