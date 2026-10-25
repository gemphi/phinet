use phistd::{unit_random, Tape, Tensor};
use phical::{Tie, Fno, FnoConfig};
use std::time::Instant;
fn main() {
    let space = 64; let (bn, wd) = (4096usize, 8usize);
    let fs: Vec<_> = (0..bn).map(|i| unit_random(space, i as u64)).collect();
    let x = phistd::stack(&fs, space);
    let mut t = Tape::new();
    let xv = t.constant(x);
    let lift = t.param(Tensor::randn([1, wd, 1], 1.0, 1));
    let w = t.param(Tensor::randn([wd, wd, 1], 0.3, 2));
    let r = t.param(Tensor::randn([wd, wd, space], 0.3, 3));
    let b = t.param(Tensor::zeros([1, wd, 1]));
    let time = |name: &str, f: &mut dyn FnMut()| { let s = Instant::now(); f(); println!("{name:<12} {:.1} ms", s.elapsed().as_secs_f64() * 1e3); };
    let mut h = xv;
    time("linear lift", &mut || { h = t.linear(xv, lift); });
    let mut hb = h; time("add_bias", &mut || { hb = t.add_bias(h, b); });
    let mut hg = hb; time("gelu", &mut || { hg = t.gelu(hb); });
    let mut hl = hg; time("linear w", &mut || { hl = t.linear(hg, w); });
    let mut hf = hg; time("fft", &mut || { hf = t.fft(hg); });
    let mut hm = hf; time("mode_mix", &mut || { hm = t.mode_mix(hf, r); });
    let mut hi = hm; time("ifft", &mut || { hi = t.ifft(hm); });
    let mut ha = hi; time("add", &mut || { ha = t.add(hi, hl); });
    let cfg = FnoConfig { space, width: wd, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: Tie::Circle };
    let mut fno = Fno::new(cfg, 1);
    time("Fno::apply", &mut || { let _ = fno.apply(&fs); });
    let _ = ha;
}
