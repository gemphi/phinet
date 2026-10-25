//! Backward of the value-reading ops: cosine, hinge, means, gather, unit, the table, Möbius.

use super::super::tape::Op;
use super::super::{Tape, Tensor};
use crate::c64;
use rayon::prelude::*;

impl Tape {
    pub(super) fn back_score(&mut self, op: Op, g: Tensor) {
        match op {
                Op::Cosine(pred, target) => {
                    let (p, t) = (self.nodes[pred.0].value.clone(), self.nodes[target.0].value.clone());
                    let (bn, s) = (p.shape[0], p.shape[2]);
                    let mut gp = Tensor::zeros(p.shape);
                    let mut gt = Tensor::zeros(t.shape);
                    for b in 0..bn {
                        let gy = g.get(b, 0, 0).re;
                        let (mut dot, mut np2, mut nt2) = (0.0, 0.0, 0.0);
                        for k in 0..s {
                            let (pz, tz) = (p.get(b, 0, k), t.get(b, 0, k));
                            dot += (pz * tz.conj()).re;
                            np2 += pz.norm_sqr();
                            nt2 += tz.norm_sqr();
                        }
                        let (np, nt) = (np2.sqrt().max(1e-12), nt2.sqrt().max(1e-12));
                        // f = dot/(|p||t|); ∂f/∂p = t/(|p||t|) − dot·p/(|p|³|t|).
                        for k in 0..s {
                            let (pz, tz) = (p.get(b, 0, k), t.get(b, 0, k));
                            let dp = tz / (np * nt) - pz * (dot / (np * np * np * nt));
                            let dt = pz / (np * nt) - tz * (dot / (nt * nt * nt * np));
                            gp.set(b, 0, k, dp * gy);
                            gt.set(b, 0, k, dt * gy);
                        }
                    }
                    self.accumulate(pred, gp);
                    self.accumulate(target, gt);
                }
                Op::Hinge(pos, neg, margin) => {
                    let (p, n) = (self.nodes[pos.0].value.clone(), self.nodes[neg.0].value.clone());
                    let bn = p.shape[0] as f64;
                    let gy = g.get(0, 0, 0).re;
                    let mut gp = Tensor::zeros(p.shape);
                    let mut gn = Tensor::zeros(n.shape);
                    for b in 0..p.shape[0] {
                        let active = margin - p.get(b, 0, 0).re + n.get(b, 0, 0).re > 0.0;
                        if active {
                            gp.set(b, 0, 0, c64::new(-gy / bn, 0.0));
                            gn.set(b, 0, 0, c64::new(gy / bn, 0.0));
                        }
                    }
                    self.accumulate(pos, gp);
                    self.accumulate(neg, gn);
                }
                Op::MeanReal(x) => {
                    let xv = &self.nodes[x.0].value;
                    let gy = g.get(0, 0, 0).re / xv.len().max(1) as f64;
                    let gx = Tensor::from_vec(xv.shape, vec![c64::new(gy, 0.0); xv.len()]);
                    self.accumulate(x, gx);
                }
                Op::Tie(r, group) => {
                    let rs = self.nodes[r.0].value.shape;
                    let (ng, s) = (rs[2], group.len());
                    let mut gr = Tensor::zeros(rs);
                    for io in 0..rs[0] * rs[1] {
                        for (k, &gk) in group.iter().enumerate() {
                            gr.data[io * ng + gk] += g.data[io * s + k];
                        }
                    }
                    self.accumulate(r, gr);
                }
                Op::Gather(table, idx) => {
                    let ts = self.nodes[table.0].value.shape;
                    let s = ts[2];
                    let mut gt = Tensor::zeros(ts);
                    for (b, &i) in idx.iter().enumerate() {
                        for k in 0..s {
                            gt.data[i * s + k] += g.data[b * s + k];
                        }
                    }
                    self.accumulate(table, gt);
                }
                Op::CosineTable(pred, table) => {
                    let (p, t) = (self.nodes[pred.0].value.clone(), self.nodes[table.0].value.clone());
                    let (b, v, s) = (p.shape[0], t.shape[0], p.shape[2]);
                    let tn: Vec<f64> = (0..v)
                        .map(|i| t.data[i * s..(i + 1) * s].iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt().max(1e-12))
                        .collect();
                    let mut gp = Tensor::zeros(p.shape);
                    let mut gt = Tensor::zeros(t.shape);
                    let (need_p, need_t) = (self.nodes[pred.0].requires_grad, self.nodes[table.0].requires_grad);
                    let pn: Vec<f64> = (0..b)
                        .map(|r| p.data[r * s..(r + 1) * s].iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt().max(1e-12))
                        .collect();
                    // f = dot/(|p||t|); ∂f/∂p = t/(|p||t|) − dot·p/(|p|³|t|), and
                    // symmetrically for t. Two passes so each output row is
                    // owned by one thread: prediction rows, then table rows.
                    // A constant table (the usual case: the lexicon is fixed while
                    // the operator trains) gets no gradient pass — it is the
                    // larger of the two passes by v/b.
                    if need_p { gp.data.par_chunks_mut(s).enumerate().for_each(|(r, grow)| {
                        let prow = &p.data[r * s..(r + 1) * s];
                        for (i, &tni) in tn.iter().enumerate() {
                            let gy = g.data[r * v + i].re;
                            if gy == 0.0 {
                                continue;
                            }
                            let trow = &t.data[i * s..(i + 1) * s];
                            let dot: f64 = prow.iter().zip(trow).map(|(a, c)| (a * c.conj()).re).sum();
                            for k in 0..s {
                                grow[k] += (trow[k] / (pn[r] * tni) - prow[k] * (dot / (pn[r] * pn[r] * pn[r] * tni))) * gy;
                            }
                        }
                    }); }
                    if need_t { gt.data.par_chunks_mut(s).enumerate().for_each(|(i, grow)| {
                        let trow = &t.data[i * s..(i + 1) * s];
                        for (r, &pnr) in pn.iter().enumerate() {
                            let gy = g.data[r * v + i].re;
                            if gy == 0.0 {
                                continue;
                            }
                            let prow = &p.data[r * s..(r + 1) * s];
                            let dot: f64 = prow.iter().zip(trow).map(|(a, c)| (a * c.conj()).re).sum();
                            for k in 0..s {
                                grow[k] += (prow[k] / (pnr * tn[i]) - trow[k] * (dot / (tn[i] * tn[i] * tn[i] * pnr))) * gy;
                            }
                        }
                    }); }
                    self.accumulate(pred, gp);
                    self.accumulate(table, gt);
                }
                Op::Mobius(x, abcd) => {
                    let (xv, m) = (self.nodes[x.0].value.clone(), self.nodes[abcd.0].value.clone());
                    let (a, b, c, d) = (m.data[0], m.data[1], m.data[2], m.data[3]);
                    // w is holomorphic in z and in each coefficient, so for a
                    // real loss with g = ∂L/∂Re w + i ∂L/∂Im w the chain rule
                    // is g_z = g · conj(∂w/∂z), and likewise per coefficient.
                    let mut gx = Tensor::zeros(xv.shape);
                    let mut gm = Tensor::zeros([1, 1, 4]);
                    for (k, z) in xv.data.iter().enumerate() {
                        let den = c * z + d;
                        let gy = g.data[k];
                        let dz = (a * d - b * c) / (den * den);
                        gx.data[k] = gy * dz.conj();
                        gm.data[0] += gy * (z / den).conj();
                        gm.data[1] += gy * (c64::new(1.0, 0.0) / den).conj();
                        gm.data[2] += gy * (-(a * z + b) * z / (den * den)).conj();
                        gm.data[3] += gy * (-(a * z + b) / (den * den)).conj();
                    }
                    self.accumulate(x, gx);
                    self.accumulate(abcd, gm);
                }
                Op::Unit(x) => {
                    // y = x/|x|;  g_x = g/|x| − x · Re⟨x, g⟩ / |x|³
                    let xv = self.nodes[x.0].value.clone();
                    let s = xv.shape[2];
                    let mut gx = Tensor::zeros(xv.shape);
                    for (r, row) in xv.data.chunks(s).enumerate() {
                        let e: f64 = row.iter().map(|z| z.norm_sqr()).sum();
                        if e <= 1e-24 {
                            continue;
                        }
                        let n = e.sqrt();
                        let grow = &g.data[r * s..(r + 1) * s];
                        let dot: f64 = row.iter().zip(grow).map(|(z, gz)| (z.conj() * gz).re).sum();
                        for k in 0..s {
                            gx.data[r * s + k] = grow[k] / n - row[k] * (dot / (n * n * n));
                        }
                    }
                    self.accumulate(x, gx);
                }
            other => self.back_loss(other, g),
        }
    }
}
