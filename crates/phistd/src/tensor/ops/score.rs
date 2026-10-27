//! Forward ops that read values: cosine, hinge, means, gather, unit, the table, Möbius.

use super::super::tape::Op;
use super::super::{Tape, Tensor, Var};
use crate::c64;
use rayon::prelude::*;

impl Tape {
    pub fn cosine(&mut self, pred: Var, target: Var) -> Var {
        let (p, t) = (&self.nodes[pred.0].value, &self.nodes[target.0].value);
        assert_eq!(p.shape, t.shape, "cosine: shape mismatch");
        assert_eq!(p.shape[1], 1, "cosine: width must be 1");
        let (bn, s) = (p.shape[0], p.shape[2]);
        let mut out = Tensor::zeros([bn, 1, 1]);
        for b in 0..bn {
            let (mut dot, mut np, mut nt) = (0.0, 0.0, 0.0);
            for k in 0..s {
                let (pz, tz) = (p.get(b, 0, k), t.get(b, 0, k));
                dot += (pz * tz.conj()).re;
                np += pz.norm_sqr();
                nt += tz.norm_sqr();
            }
            let d = (np.sqrt() * nt.sqrt()).max(1e-12);
            out.set(b, 0, 0, c64::new(dot / d, 0.0));
        }
        let rg = self.any_grad(&[pred, target]);
        self.push(out, Op::Cosine(pred, target), rg)
    }

    /// `mean_b max(0, margin − pos_b + neg_b)` over `[b,1,1]` real scores.
    pub fn hinge(&mut self, pos: Var, neg: Var, margin: f64) -> Var {
        let (p, n) = (&self.nodes[pos.0].value, &self.nodes[neg.0].value);
        assert_eq!(p.shape, n.shape, "hinge: shape mismatch");
        let bn = p.shape[0] as f64;
        let loss: f64 = p
            .data
            .iter()
            .zip(&n.data)
            .map(|(a, b)| (margin - a.re + b.re).max(0.0))
            .sum::<f64>()
            / bn;
        let t = Tensor::from_vec([1, 1, 1], vec![c64::new(loss, 0.0)]);
        let rg = self.any_grad(&[pos, neg]);
        self.push(t, Op::Hinge(pos, neg, margin), rg)
    }

    /// Mean of the real parts of a `[b,1,1]` tensor, as `[1,1,1]`.
    pub fn mean_real(&mut self, x: Var) -> Var {
        let xv = &self.nodes[x.0].value;
        let m = xv.data.iter().map(|z| z.re).sum::<f64>() / xv.len().max(1) as f64;
        let t = Tensor::from_vec([1, 1, 1], vec![c64::new(m, 0.0)]);
        let rg = self.any_grad(&[x]);
        self.push(t, Op::MeanReal(x), rg)
    }

    /// Expands `r [i,o,g]` to `[i,o,group.len()]` by `y[..,k] = r[..,group[k]]`.
    /// The gradient of each group weight is the sum over the modes tied to it.
    pub fn tie(&mut self, r: Var, group: &[usize]) -> Var {
        let rv = &self.nodes[r.0].value;
        let (wi, wo, g) = (rv.shape[0], rv.shape[1], rv.shape[2]);
        let s = group.len();
        let mut out = Tensor::zeros([wi, wo, s]);
        for io in 0..wi * wo {
            for (k, &gk) in group.iter().enumerate() {
                assert!(gk < g, "tie: group {gk} out of range");
                out.data[io * s + k] = rv.data[io * g + gk];
            }
        }
        let rg = self.any_grad(&[r]);
        self.push(out, Op::Tie(r, group.to_vec()), rg)
    }

    /// Selects rows `idx` of a `[V,1,s]` table into `[b,1,s]`. The backward
    /// pass scatter-adds into the table, which is what lets the table's rows —
    /// the words themselves — be parameters.
    pub fn gather(&mut self, table: Var, idx: &[usize]) -> Var {
        let tv = &self.nodes[table.0].value;
        assert_eq!(tv.shape[1], 1, "gather: table must be [V,1,s]");
        let s = tv.shape[2];
        let mut out = Tensor::zeros([idx.len(), 1, s]);
        for (b, &i) in idx.iter().enumerate() {
            assert!(i < tv.shape[0], "gather: index {i} out of range");
            let src = &tv.data[i * s..(i + 1) * s];
            out.data[b * s..(b + 1) * s].copy_from_slice(src);
        }
        let rg = self.any_grad(&[table]);
        self.push(out, Op::Gather(table, idx.to_vec()), rg)
    }

    /// Normalises every `[·,·,s]` row to unit energy. Rows of zero energy are
    /// left at zero.
    pub fn unit(&mut self, x: Var) -> Var {
        let xv = &self.nodes[x.0].value;
        let s = xv.shape[2];
        let mut out = xv.clone();
        for row in out.data.chunks_mut(s) {
            let e: f64 = row.iter().map(|z| z.norm_sqr()).sum();
            if e > 1e-24 {
                let k = 1.0 / e.sqrt();
                for z in row.iter_mut() {
                    *z *= k;
                }
            }
        }
        let rg = self.any_grad(&[x]);
        self.push(out, Op::Unit(x), rg)
    }

    /// Cosine of every prediction row against every table row: `[b,V,1]`.
    pub fn cosine_table(&mut self, pred: Var, table: Var) -> Var {
        let (p, t) = (&self.nodes[pred.0].value, &self.nodes[table.0].value);
        assert_eq!(p.shape[1], 1, "cosine_table: pred width must be 1");
        assert_eq!(t.shape[1], 1, "cosine_table: table width must be 1");
        assert_eq!(p.shape[2], t.shape[2], "cosine_table: space mismatch");
        let (b, v, s) = (p.shape[0], t.shape[0], p.shape[2]);
        let tnorm: Vec<f64> = (0..v)
            .map(|i| t.data[i * s..(i + 1) * s].iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt().max(1e-12))
            .collect();
        let mut out = Tensor::zeros([b, v, 1]);
        // Rows are independent: one thread per batch row. This is the full
        // vocabulary softmax's input and was the single hottest loop.
        out.data.par_chunks_mut(v).enumerate().for_each(|(r, orow)| {
            let prow = &p.data[r * s..(r + 1) * s];
            let pn = prow.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt().max(1e-12);
            for i in 0..v {
                let trow = &t.data[i * s..(i + 1) * s];
                let dot: f64 = prow.iter().zip(trow).map(|(a, c)| (a * c.conj()).re).sum();
                orow[i] = c64::new(dot / (pn * tnorm[i]), 0.0);
            }
        });
        let rg = self.any_grad(&[pred, table]);
        self.push(out, Op::CosineTable(pred, table), rg)
    }

    /// Möbius map on every sample: `w = (a·z + b)/(c·z + d)` with `abcd` a
    /// `[1,1,4]` parameter. Rotation (`a = e^{iα}, d = 1`), dilation
    /// (`a = ρ`), and inversion (`b = d = 1, a = c = 0` gives `1/z`) are
    /// special cases, so this one op contains bind, unbind, and an evidence
    /// change exactly; the group composes by 2×2 matrix product.
    pub fn mobius(&mut self, x: Var, abcd: Var) -> Var {
        let (xv, m) = (&self.nodes[x.0].value, &self.nodes[abcd.0].value);
        assert_eq!(m.shape, [1, 1, 4], "mobius: coefficients must be [1,1,4]");
        let (a, b, c, d) = (m.data[0], m.data[1], m.data[2], m.data[3]);
        let mut out = Tensor::zeros(xv.shape);
        for (o, z) in out.data.iter_mut().zip(&xv.data) {
            *o = (a * z + b) / (c * z + d);
        }
        let rg = self.any_grad(&[x, abcd]);
        self.push(out, Op::Mobius(x, abcd), rg)
    }

}
