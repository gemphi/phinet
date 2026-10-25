//! Reverse-mode replay: the driver, accumulation, and the backward of the value-moving ops.

use super::fft::{fft2_tensor, fft_tensor, gelu_prime};
use super::tape::Op;
mod loss;
mod score;
#[cfg(test)]
use super::tape::Node;
use super::{Tape, Tensor, Var};
use crate::c64;

impl Tape {
    pub(super) fn accumulate(&mut self, v: Var, g: Tensor) {
        if !self.nodes[v.0].requires_grad {
            return;
        }
        match &mut self.nodes[v.0].grad {
            Some(acc) => {
                for (a, b) in acc.data.iter_mut().zip(&g.data) {
                    *a += b;
                }
            }
            None => self.nodes[v.0].grad = Some(g),
        }
    }

    #[cfg(test)]
    pub(crate) fn clone_for_test(&self) -> Tape {
        Tape { nodes: self.nodes.iter().map(|n| Node { value: n.value.clone(), grad: n.grad.clone(), op: n.op.clone(), requires_grad: n.requires_grad }).collect() }
    }


    pub fn backward(&mut self, out: Var) {
        for n in self.nodes.iter_mut() {
            n.grad = None;
        }
        assert_eq!(self.nodes[out.0].value.shape, [1, 1, 1], "backward: output must be scalar");
        self.nodes[out.0].grad = Some(Tensor::from_vec([1, 1, 1], vec![c64::new(1.0, 0.0)]));

        for i in (0..=out.0).rev() {
            let g = match self.nodes[i].grad.clone() {
                Some(g) => g,
                None => continue,
            };
            if !self.nodes[i].requires_grad {
                continue;
            }
            let op = self.nodes[i].op.clone();
            if let Some((op, g)) = self.back_basic(op, g) {
                self.back_score(op, g);
            }
        }
    }

    /// Backward of the value-moving ops; returns the op and gradient untouched
    /// when it is not one of them.
    pub(super) fn back_basic(&mut self, op: Op, g: Tensor) -> Option<(Op, Tensor)> {
        {
            match op {
                Op::Leaf => {}
                Op::Add(a, b) => {
                    self.accumulate(a, g.clone());
                    self.accumulate(b, g);
                }
                Op::AddBias(x, bias) => {
                    let xs = self.nodes[x.0].value.shape;
                    let mut gb = Tensor::zeros([1, xs[1], 1]);
                    for b in 0..xs[0] {
                        for w in 0..xs[1] {
                            let mut acc = c64::new(0.0, 0.0);
                            for s in 0..xs[2] {
                                acc += g.get(b, w, s);
                            }
                            let i = gb.idx(0, w, 0);
                            gb.data[i] += acc;
                        }
                    }
                    self.accumulate(x, g);
                    self.accumulate(bias, gb);
                }
                Op::Mul(a, b) => {
                    let (av, bv) = (self.nodes[a.0].value.clone(), self.nodes[b.0].value.clone());
                    let ga = Tensor::from_vec(g.shape, g.data.iter().zip(&bv.data).map(|(p, q)| p * q.conj()).collect());
                    let gb = Tensor::from_vec(g.shape, g.data.iter().zip(&av.data).map(|(p, q)| p * q.conj()).collect());
                    self.accumulate(a, ga);
                    self.accumulate(b, gb);
                }
                Op::Scale(a, k) => {
                    let ga = Tensor::from_vec(g.shape, g.data.iter().map(|p| p * k).collect());
                    self.accumulate(a, ga);
                }
                Op::Linear(x, w) => {
                    let (gx, gw) = {
                        let (xv, wv) = (&self.nodes[x.0].value, &self.nodes[w.0].value);
                        let (bn, wi, s) = (xv.shape[0], xv.shape[1], xv.shape[2]);
                        let wo = wv.shape[1];
                        let mut gx = Tensor::zeros(xv.shape);
                        let mut gw = Tensor::zeros(wv.shape);
                        for b in 0..bn {
                            for o in 0..wo {
                                let grow = &g.data[(b * wo + o) * s..(b * wo + o + 1) * s];
                                for i in 0..wi {
                                    let wc = wv.data[i * wo + o].conj();
                                    let xrow = &xv.data[(b * wi + i) * s..(b * wi + i + 1) * s];
                                    let gxrow = &mut gx.data[(b * wi + i) * s..(b * wi + i + 1) * s];
                                    let mut acc = c64::new(0.0, 0.0);
                                    for ((gxz, gy), xz) in gxrow.iter_mut().zip(grow).zip(xrow) {
                                        *gxz += gy * wc;
                                        acc += gy * xz.conj();
                                    }
                                    gw.data[i * wo + o] += acc;
                                }
                            }
                        }
                        (gx, gw)
                    };
                    self.accumulate(x, gx);
                    self.accumulate(w, gw);
                }
                Op::ModeMix(x, r) => {
                    let (gx, gr) = {
                        let (xv, rv) = (&self.nodes[x.0].value, &self.nodes[r.0].value);
                        let (bn, wi, s) = (xv.shape[0], xv.shape[1], xv.shape[2]);
                        let wo = rv.shape[1];
                        let mut gx = Tensor::zeros(xv.shape);
                        let mut gr = Tensor::zeros(rv.shape);
                        for b in 0..bn {
                            for o in 0..wo {
                                let grow = &g.data[(b * wo + o) * s..(b * wo + o + 1) * s];
                                for i in 0..wi {
                                    let xrow = &xv.data[(b * wi + i) * s..(b * wi + i + 1) * s];
                                    let rrow = &rv.data[(i * wo + o) * s..(i * wo + o + 1) * s];
                                    let gxrow = &mut gx.data[(b * wi + i) * s..(b * wi + i + 1) * s];
                                    let grrow = &mut gr.data[(i * wo + o) * s..(i * wo + o + 1) * s];
                                    for m in 0..s {
                                        let gy = grow[m];
                                        gxrow[m] += gy * rrow[m].conj();
                                        grrow[m] += gy * xrow[m].conj();
                                    }
                                }
                            }
                        }
                        (gx, gr)
                    };
                    self.accumulate(x, gx);
                    self.accumulate(r, gr);
                }
                Op::Fft(x) => {
                    // Adjoint of a unitary transform is its inverse.
                    let gx = fft_tensor(&g, true);
                    self.accumulate(x, gx);
                }
                Op::Ifft(x) => {
                    let gx = fft_tensor(&g, false);
                    self.accumulate(x, gx);
                }
                Op::Fft2(x, s1, s2) => {
                    let gx = fft2_tensor(&g, s1, s2, true);
                    self.accumulate(x, gx);
                }
                Op::Ifft2(x, s1, s2) => {
                    let gx = fft2_tensor(&g, s1, s2, false);
                    self.accumulate(x, gx);
                }
                Op::Gelu(x) => {
                    let xv = self.nodes[x.0].value.clone();
                    let gx = Tensor::from_vec(
                        g.shape,
                        g.data
                            .iter()
                            .zip(&xv.data)
                            .map(|(gy, z)| c64::new(gy.re * gelu_prime(z.re), gy.im * gelu_prime(z.im)))
                            .collect(),
                    );
                    self.accumulate(x, gx);
                }
                Op::Squash(x) => {
                    let gx = self.squash_grad(x, &g);
                    self.accumulate(x, gx);
                }
                other => return Some((other, g)),
            }
        }
        None
    }
}
