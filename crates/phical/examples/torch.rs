//! Dumps every phinet operation on fixed inputs to JSON so that a PyTorch
//! implementation of the same operation can be checked against it to
//! floating-point precision — `torch/phinet_torch.py` reads this file.
//!
//! `cargo run --release --example torch > torch/reference.json`



use phistd::tensor::fft_inplace;
use phistd::{c64, unit_random, Adam, MixtureTerms, Tape, Tensor};
use serde_json::{json, Value};

fn cx(f: &[c64]) -> Value {
    json!(f.iter().map(|z| [z.re, z.im]).collect::<Vec<_>>())
}
fn tensor(t: &Tensor) -> Value {
    json!({ "shape": t.shape, "data": cx(&t.data) })
}


mod torch {
    pub mod extra;
}
use torch::extra;

fn main() {
    let space = 16usize;
    let width = 3usize;
    let batch = 2usize;
    let mut out = serde_json::Map::new();
    out.insert("space".into(), json!(space));

    // ── inputs ──
    let fs: Vec<Vec<c64>> = (0..batch).map(|i| unit_random(space, 100 + i as u64)).collect();
    let x = phistd::stack(&fs, space);
    out.insert("x".into(), tensor(&x));

    // ── fft / ifft ──
    let mut f = fs[0].clone();
    fft_inplace(&mut f, false);
    out.insert("fft_x0".into(), cx(&f));
    let mut g = f.clone();
    fft_inplace(&mut g, true);
    out.insert("ifft_fft_x0".into(), cx(&g));

    // ── tape ops ──
    let mut t = Tape::new();
    let xv = t.param(x.clone());
    let w = Tensor::randn([1, width, 1], 1.0, 7);
    let wv = t.param(w.clone());
    out.insert("lift_w".into(), tensor(&w));
    let h = t.linear(xv, wv);
    out.insert("linear".into(), tensor(t.value(h)));
    let b = Tensor::randn([1, width, 1], 0.5, 8);
    let bv = t.param(b.clone());
    out.insert("bias".into(), tensor(&b));
    let hb = t.add_bias(h, bv);
    out.insert("add_bias".into(), tensor(t.value(hb)));
    let hg = t.gelu(hb);
    out.insert("gelu".into(), tensor(t.value(hg)));
    let hf = t.fft(hg);
    out.insert("fft".into(), tensor(t.value(hf)));
    let r = Tensor::randn([width, width, space], 0.3, 9);
    let rv = t.param(r.clone());
    out.insert("r".into(), tensor(&r));
    let hm = t.mode_mix(hf, rv);
    out.insert("mode_mix".into(), tensor(t.value(hm)));
    let hi = t.ifft(hm);
    out.insert("ifft".into(), tensor(t.value(hi)));
    let proj = Tensor::randn([width, 1, 1], 1.0, 10);
    let pv = t.param(proj.clone());
    out.insert("proj_w".into(), tensor(&proj));
    let y = t.linear(hi, pv);
    out.insert("proj".into(), tensor(t.value(y)));
    let yu = t.unit(y);
    out.insert("unit".into(), tensor(t.value(yu)));
    let table = Tensor::from_vec([4, 1, space], (0..4).flat_map(|i| unit_random(space, 200 + i)).collect());
    let tv = t.constant(table.clone());
    out.insert("table".into(), tensor(&table));
    let ct = t.cosine_table(yu, tv);
    out.insert("cosine_table".into(), tensor(t.value(ct)));
    let pos = t.gather(tv, &[1, 3]);
    let neg = t.gather(tv, &[0, 2]);
    let cp = t.cosine(yu, pos);
    let cn = t.cosine(yu, neg);
    out.insert("cosine_pos".into(), tensor(t.value(cp)));
    out.insert("cosine_neg".into(), tensor(t.value(cn)));
    let loss = t.hinge(cp, cn, 0.2);
    out.insert("hinge".into(), json!(t.value(loss).data[0].re));
    t.backward(loss);
    out.insert("grad_lift_w".into(), tensor(t.grad(wv).unwrap()));
    out.insert("grad_r".into(), tensor(t.grad(rv).unwrap()));
    out.insert("grad_proj_w".into(), tensor(t.grad(pv).unwrap()));
    out.insert("grad_bias".into(), tensor(t.grad(bv).unwrap()));
    out.insert("grad_x".into(), tensor(t.grad(xv).unwrap()));

    // mixture nll on the cosine table
    let mut t2 = Tape::new();
    let sv = t2.param(t.value(ct).clone());
    let terms = MixtureTerms { targets: vec![1, 3], k: vec![0.05, 0.02], coef: vec![0.6, 0.7], gamma: 0.5, other: vec![0.1, 0.3], beta: 8.0 };
    out.insert("mixture_terms".into(), json!({"targets": terms.targets, "k": terms.k, "coef": terms.coef, "gamma": terms.gamma, "other": terms.other, "beta": terms.beta}));
    let nll = t2.mixture_nll(sv, terms);
    out.insert("mixture_nll".into(), json!(t2.value(nll).data[0].re));
    t2.backward(nll);
    out.insert("grad_mixture_scores".into(), tensor(t2.grad(sv).unwrap()));

    // cross entropy over the same cosine table (+ its gradient)
    let mut t4 = Tape::new();
    let sv4 = t4.param(t.value(ct).clone());
    let ce = t4.cross_entropy(sv4, &[1, 3], 6.0);
    out.insert("cross_entropy".into(), json!(t4.value(ce).data[0].re));
    t4.backward(ce);
    out.insert("grad_cross_entropy_scores".into(), tensor(t4.grad(sv4).unwrap()));

    // AdamW: one decayed step on a small parameter with a known gradient
    let mut t5 = Tape::new();
    let pw = t5.param(Tensor::randn([1, 2, 3], 1.0, 77));
    out.insert("adamw_param".into(), tensor(t5.value(pw)));
    let lw = t5.mean_real(pw);
    t5.backward(lw);
    let mut adamw = Adam::new(0.05).with_decay(0.1);
    adamw.step(&mut t5, &[pw]);
    out.insert("adamw_after".into(), tensor(t5.value(pw)));

    // resample between resolutions
    out.insert("resample_x0_to_32".into(), cx(&phistd::phasor::resample(&fs[0], phistd::phasor::Resolution::P32)));
    out.insert("quantise_x0_q16".into(), cx(&phistd::phasor::quantise(&fs[0], phistd::phasor::Quantum::Q16)));

    // fft2 on a 4×4 torus read from the 16-sample function
    let mut t6 = Tape::new();
    let xv6 = t6.param(Tensor::from_vec([1, 1, space], fs[0].clone()));
    let f2 = t6.fft2(xv6, 4, 4);
    out.insert("fft2_x0_4x4".into(), tensor(t6.value(f2)));
    let l6 = t6.mean_real(f2);
    t6.backward(l6);
    out.insert("grad_fft2_x".into(), tensor(t6.grad(xv6).unwrap()));

    // mobius
    let mut t3 = Tape::new();
    let zv = t3.param(x.clone());
    let abcd = Tensor::from_vec([1, 1, 4], vec![c64::new(0.8, 0.6), c64::new(0.1, -0.2), c64::new(0.05, 0.1), c64::new(1.0, 0.0)]);
    let mv = t3.param(abcd.clone());
    out.insert("mobius_abcd".into(), tensor(&abcd));
    let mob = t3.mobius(zv, mv);
    out.insert("mobius".into(), tensor(t3.value(mob)));
    let mr = t3.mean_real(mob);
    t3.backward(mr);
    out.insert("mobius_mean_real".into(), json!(t3.value(mr).data[0].re));
    out.insert("grad_mobius_abcd".into(), tensor(t3.grad(mv).unwrap()));

    extra::spectral_and_misc(&mut out, &fs, space);
    extra::whole_operator(&mut out, &fs, &x, &table, space, width);
    println!("{}", serde_json::to_string(&Value::Object(out)).unwrap());
}
