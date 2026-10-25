//! Backward of the losses over a score table.

use super::super::tape::Op;
use super::super::{Tape, Tensor};
use crate::c64;

impl Tape {
    pub(super) fn back_loss(&mut self, op: Op, g: Tensor) {
        match op {
                Op::CrossEntropy(scores, targets, beta) => {
                    let gs = {
                        let sv = &self.nodes[scores.0].value;
                        let (b, v) = (sv.shape[0], sv.shape[1]);
                        let gy = g.data[0].re / b.max(1) as f64;
                        let mut gs = Tensor::zeros(sv.shape);
                        for (r, &target) in targets.iter().enumerate().take(b) {
                            let row = &sv.data[r * v..(r + 1) * v];
                            let max = row.iter().map(|z| z.re * beta).fold(f64::NEG_INFINITY, f64::max);
                            let e: Vec<f64> = row.iter().map(|x| (x.re * beta - max).exp()).collect();
                            let z: f64 = e.iter().sum();
                            for (i, &ei) in e.iter().enumerate() {
                                let p = ei / z;
                                let d = if i == target { p - 1.0 } else { p };
                                gs.data[r * v + i] = c64::new(gy * beta * d, 0.0);
                            }
                        }
                        gs
                    };
                    self.accumulate(scores, gs);
                }
                Op::MixtureNll(scores, terms) => {
                    let sv = self.nodes[scores.0].value.clone();
                    let (b, v) = (sv.shape[0], sv.shape[1]);
                    let gy = g.get(0, 0, 0).re / b.max(1) as f64;
                    let mut gs = Tensor::zeros(sv.shape);
                    for r in 0..b {
                        let row: Vec<f64> = (0..v).map(|i| sv.data[r * v + i].re * terms.beta).collect();
                        let max = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                        let ex: Vec<f64> = row.iter().map(|x| (x - max).exp()).collect();
                        let z: f64 = ex.iter().sum();
                        let t = terms.targets[r];
                        let p_t = ex[t] / z;
                        let p = (terms.k[r] + terms.coef[r] * (terms.gamma * p_t + (1.0 - terms.gamma) * terms.other[r])).max(1e-12);
                        // dL/dp_t = −coef·γ/p ; ∂p_t/∂s_i = β p_t (δ_ti − p_i)
                        let dl_dpt = -terms.coef[r] * terms.gamma / p;
                        for (i, &exi) in ex.iter().enumerate() {
                            let p_i = exi / z;
                            let d = if i == t { 1.0 } else { 0.0 };
                            gs.data[r * v + i] = c64::new(gy * dl_dpt * terms.beta * p_t * (d - p_i), 0.0);
                        }
                    }
                    self.accumulate(scores, gs);
                }
            _ => unreachable!("every op has a backward"),
        }
    }
}
