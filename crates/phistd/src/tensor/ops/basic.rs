//! Forward ops that move values: arithmetic, the pointwise and spectral maps, the FFTs, GELU.

use super::super::fft::{fft2_tensor, fft_tensor, gelu};
use super::super::tape::Op;
use super::super::{Tape, Tensor, Var};
use crate::c64;

impl Tape {
    pub fn add(&mut self, a: Var, b: Var) -> Var {
        let (x, y) = (&self.nodes[a.0].value, &self.nodes[b.0].value);
        assert_eq!(x.shape, y.shape, "add: shape mismatch");
        let data = x.data.iter().zip(&y.data).map(|(p, q)| p + q).collect();
        let t = Tensor::from_vec(x.shape, data);
        let rg = self.any_grad(&[a, b]);
        self.push(t, Op::Add(a, b), rg)
    }

    pub fn add_bias(&mut self, x: Var, bias: Var) -> Var {
        let (xv, bv) = (&self.nodes[x.0].value, &self.nodes[bias.0].value);
        assert_eq!(bv.shape, [1, xv.shape[1], 1], "add_bias: bias must be [1,w,1]");
        let mut out = xv.clone();
        for b in 0..xv.shape[0] {
            for w in 0..xv.shape[1] {
                let bb = bv.get(0, w, 0);
                for s in 0..xv.shape[2] {
                    let i = out.idx(b, w, s);
                    out.data[i] += bb;
                }
            }
        }
        let rg = self.any_grad(&[x, bias]);
        self.push(out, Op::AddBias(x, bias), rg)
    }

    pub fn mul(&mut self, a: Var, b: Var) -> Var {
        let (x, y) = (&self.nodes[a.0].value, &self.nodes[b.0].value);
        assert_eq!(x.shape, y.shape, "mul: shape mismatch");
        let data = x.data.iter().zip(&y.data).map(|(p, q)| p * q).collect();
        let t = Tensor::from_vec(x.shape, data);
        let rg = self.any_grad(&[a, b]);
        self.push(t, Op::Mul(a, b), rg)
    }

    pub fn scale(&mut self, a: Var, k: f64) -> Var {
        let x = &self.nodes[a.0].value;
        let data = x.data.iter().map(|p| p * k).collect();
        let t = Tensor::from_vec(x.shape, data);
        let rg = self.any_grad(&[a]);
        self.push(t, Op::Scale(a, k), rg)
    }

    /// Pointwise linear map over the width axis: `[b,i,s] × [i,o,1] → [b,o,s]`.
    pub fn linear(&mut self, x: Var, w: Var) -> Var {
        let (xv, wv) = (&self.nodes[x.0].value, &self.nodes[w.0].value);
        let (bn, wi, s) = (xv.shape[0], xv.shape[1], xv.shape[2]);
        assert_eq!(wv.shape[0], wi, "linear: width mismatch");
        assert_eq!(wv.shape[2], 1, "linear: weight must be [i,o,1]");
        let wo = wv.shape[1];
        let mut out = Tensor::zeros([bn, wo, s]);
        // Row slices, no per-element index arithmetic: out[b,o,:] += x[b,i,:] · w[i,o].
        for b in 0..bn {
            for o in 0..wo {
                let orow = &mut out.data[(b * wo + o) * s..(b * wo + o + 1) * s];
                for i in 0..wi {
                    let wgt = wv.data[i * wo + o];
                    let xrow = &xv.data[(b * wi + i) * s..(b * wi + i + 1) * s];
                    for (oz, xz) in orow.iter_mut().zip(xrow) {
                        *oz += xz * wgt;
                    }
                }
            }
        }
        let rg = self.any_grad(&[x, w]);
        self.push(out, Op::Linear(x, w), rg)
    }

    /// Spectral mixing: one complex `[i,o]` matrix per mode `m`.
    /// `[b,i,m] × [i,o,m] → [b,o,m]`.
    pub fn mode_mix(&mut self, x: Var, r: Var) -> Var {
        let (xv, rv) = (&self.nodes[x.0].value, &self.nodes[r.0].value);
        let (bn, wi, s) = (xv.shape[0], xv.shape[1], xv.shape[2]);
        assert_eq!(rv.shape[0], wi, "mode_mix: width mismatch");
        assert_eq!(rv.shape[2], s, "mode_mix: mode count mismatch");
        let wo = rv.shape[1];
        let mut out = Tensor::zeros([bn, wo, s]);
        for b in 0..bn {
            for o in 0..wo {
                let orow = &mut out.data[(b * wo + o) * s..(b * wo + o + 1) * s];
                for i in 0..wi {
                    let xrow = &xv.data[(b * wi + i) * s..(b * wi + i + 1) * s];
                    let rrow = &rv.data[(i * wo + o) * s..(i * wo + o + 1) * s];
                    for ((oz, xz), rz) in orow.iter_mut().zip(xrow).zip(rrow) {
                        *oz += xz * rz;
                    }
                }
            }
        }
        let rg = self.any_grad(&[x, r]);
        self.push(out, Op::ModeMix(x, r), rg)
    }

    pub fn fft(&mut self, x: Var) -> Var {
        let t = fft_tensor(&self.nodes[x.0].value, false);
        let rg = self.any_grad(&[x]);
        self.push(t, Op::Fft(x), rg)
    }

    pub fn ifft(&mut self, x: Var) -> Var {
        let t = fft_tensor(&self.nodes[x.0].value, true);
        let rg = self.any_grad(&[x]);
        self.push(t, Op::Ifft(x), rg)
    }

    /// Unitary 2-D FFT: the sample axis read as an `s1 × s2` torus.
    pub fn fft2(&mut self, x: Var, s1: usize, s2: usize) -> Var {
        let t = fft2_tensor(&self.nodes[x.0].value, s1, s2, false);
        let rg = self.any_grad(&[x]);
        self.push(t, Op::Fft2(x, s1, s2), rg)
    }

    pub fn ifft2(&mut self, x: Var, s1: usize, s2: usize) -> Var {
        let t = fft2_tensor(&self.nodes[x.0].value, s1, s2, true);
        let rg = self.any_grad(&[x]);
        self.push(t, Op::Ifft2(x, s1, s2), rg)
    }

    /// GELU on the real and imaginary parts separately.
    pub fn gelu(&mut self, x: Var) -> Var {
        let xv = &self.nodes[x.0].value;
        let data = xv.data.iter().map(|z| c64::new(gelu(z.re), gelu(z.im))).collect();
        let t = Tensor::from_vec(xv.shape, data);
        let rg = self.any_grad(&[x]);
        self.push(t, Op::Gelu(x), rg)
    }

}
