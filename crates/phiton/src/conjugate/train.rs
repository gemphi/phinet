//! Training the mirror as the Joint FNO is trained (docs/113): hinge on the
//! matched filter, the told word against a decoy drawn uniformly from the
//! words ever told as an answer, Adam on what the row touched. Every
//! gradient is closed-form. With `u = H a_w`, `n = |u|` and `s = Re⟨u, a_v⟩/n`
//! (gradients written as `∂/∂re + i ∂/∂im`):
//!
//! - `∂s/∂u = (a_v − s·u/n)/n`, which is `a_v` for the unit mirror (`n = 1`;
//!   the radial part is left to renormalising, as docs/153 trained it);
//! - `∂s/∂a_w = H̄ ∂s/∂u` and `∂s/∂a_v = u/n`;
//! - `∂s/∂θ_k = −Im(conj(∂s/∂u_k) u_k)` and, pumped, `∂s/∂g_k = Re(conj(∂s/∂u_k) u_k)`.
//!
//! After each step the words touched are brought back to unit norm.

use super::{Mirror, Told};
use phistd::{c64, mix64};

const B1: f64 = 0.9;
const B2: f64 = 0.999;
const EPS: f64 = 1e-8;

/// Adam's moments for every parameter: a spectrum's `(re, im)` moments are
/// kept as the two parts of one complex number.
pub struct Moment {
    spectra: Vec<Vec<(c64, c64)>>,
    phases: Vec<Vec<(f64, f64)>>,
    gains: Vec<Vec<(f64, f64)>>,
    steps: Vec<u32>,
    kind_steps: Vec<u32>,
}

impl Moment {
    pub fn new(m: &Mirror) -> Moment {
        let z = (c64::new(0.0, 0.0), c64::new(0.0, 0.0));
        Moment {
            spectra: vec![vec![z; m.modes]; m.names.len()],
            phases: vec![vec![(0.0, 0.0); m.modes]; m.kinds.len()],
            gains: vec![vec![(0.0, 0.0); m.modes]; m.kinds.len()],
            steps: vec![0; m.names.len()],
            kind_steps: vec![0; m.kinds.len()],
        }
    }
}

fn adam(p: &mut f64, g: f64, mo: &mut (f64, f64), t: u32, lr: f64) {
    mo.0 = B1 * mo.0 + (1.0 - B1) * g;
    mo.1 = B2 * mo.1 + (1.0 - B2) * g * g;
    let (mh, vh) = (mo.0 / (1.0 - B1.powi(t as i32)), mo.1 / (1.0 - B2.powi(t as i32)));
    *p -= lr * mh / (vh.sqrt() + EPS);
}

impl Mirror {
    /// The cosine `Re ⟨H a_w, a_v⟩ / |H a_w|` for one relation (`|H a_w| = 1`
    /// in an unpumped mirror).
    pub fn score(&self, w: usize, kind: usize, v: usize) -> f64 {
        let u = self.wave(w, kind, 1);
        let n = u.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt().max(1e-12);
        u.iter().zip(&self.spectra[v]).map(|(x, b)| (x * b.conj()).re).sum::<f64>() / n
    }

    /// Descends the loss's gradient `g` (complex, per mode) on one word.
    fn step_word(&mut self, mo: &mut Moment, w: usize, g: &[c64], lr: f64) {
        mo.steps[w] += 1;
        let t = mo.steps[w];
        for ((a, g), m) in self.spectra[w].iter_mut().zip(g).zip(mo.spectra[w].iter_mut()) {
            let (mut re, mut im) = (a.re, a.im);
            let (mut mr, mut mi) = ((m.0.re, m.1.re), (m.0.im, m.1.im));
            adam(&mut re, g.re, &mut mr, t, lr);
            adam(&mut im, g.im, &mut mi, t, lr);
            *a = c64::new(re, im);
            *m = (c64::new(mr.0, mi.0), c64::new(mr.1, mi.1));
        }
        phistd::normalise(&mut self.spectra[w]);
    }

    /// One epoch over `rows` in an order drawn from `seed`; decoys from
    /// `pool`. Returns the mean hinge loss.
    pub fn train_epoch(&mut self, rows: &[Told], pool: &[usize], margin: f64, lr: f64, mo: &mut Moment, seed: u64) -> f64 {
        let mut order: Vec<usize> = (0..rows.len()).collect();
        order.sort_by_key(|i| mix64(*i as u64 ^ seed));
        let mut total = 0.0;
        for (j, &i) in order.iter().enumerate() {
            let r = rows[i];
            let d = phistd::decoy(pool, r.other, mix64(seed ^ (i as u64) << 20 ^ j as u64));
            let (h, aw) = (self.filter(r.kind, 1), &self.spectra[r.about]);
            let u: Vec<c64> = h.iter().zip(aw).map(|(h, a)| h * a).collect();
            let n = u.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt().max(1e-12);
            let (ap, ad) = (self.spectra[r.other].clone(), self.spectra[d].clone());
            let cos = |b: &[c64]| u.iter().zip(b).map(|(x, b)| (x * b.conj()).re).sum::<f64>() / n;
            let (sp, sd) = (cos(&ap), cos(&ad));
            let loss = margin - sp + sd;
            if loss <= 0.0 {
                continue;
            }
            total += loss;
            // ∂s/∂u for a target b: b, or (b − s·u/n)/n once gains move |u|
            // off 1; the unit mirror leaves the radial part to renormalising.
            let du = |b: &c64, x: &c64, s: f64| if self.pumped { (b - x * (s / n)) / n } else { *b };
            // Loss = margin − s(w, p) + s(w, d): its gradient through u.
            let g: Vec<c64> = u.iter().zip(ap.iter().zip(&ad)).map(|(x, (p, q))| du(q, x, sd) - du(p, x, sp)).collect();
            let gw: Vec<c64> = h.iter().zip(&g).map(|(h, g)| h.conj() * g).collect();
            let (gp, gd): (Vec<c64>, Vec<c64>) = u.iter().map(|x| (-x / n, x / n)).unzip();
            let gt: Vec<f64> = g.iter().zip(&u).map(|(g, x)| -(g.conj() * x).im).collect();
            let gg: Vec<f64> = g.iter().zip(&u).map(|(g, x)| (g.conj() * x).re).collect();
            self.step_word(mo, r.about, &gw, lr);
            self.step_word(mo, r.other, &gp, lr);
            self.step_word(mo, d, &gd, lr);
            mo.kind_steps[r.kind] += 1;
            let t = mo.kind_steps[r.kind];
            for ((p, g), m) in self.phases[r.kind].iter_mut().zip(&gt).zip(mo.phases[r.kind].iter_mut()) {
                adam(p, *g, m, t, lr);
            }
            if self.pumped {
                for ((p, g), m) in self.gains[r.kind].iter_mut().zip(&gg).zip(mo.gains[r.kind].iter_mut()) {
                    adam(p, *g, m, t, lr);
                }
            }
        }
        total / rows.len().max(1) as f64
    }
}
