//! The likelihood losses over a `[b,V,1]` score table: the mixed-LM likelihood and plain cross-entropy.

use super::super::tape::Op;
use super::super::{Tape, Tensor, Var};
use crate::c64;

/// The per-position constants of a mixture likelihood: the target index, the
/// affine `k + coef · base`, the mixing weight on the scored term, and the
/// probability the other component gives the target.
#[derive(Debug, Clone)]
pub struct MixtureTerms {
    pub targets: Vec<usize>,
    pub k: Vec<f64>,
    pub coef: Vec<f64>,
    pub gamma: f64,
    pub other: Vec<f64>,
    pub beta: f64,
}

impl Tape {
    pub fn mixture_nll(&mut self, scores: Var, terms: MixtureTerms) -> Var {
        let sv = &self.nodes[scores.0].value;
        let (b, v) = (sv.shape[0], sv.shape[1]);
        assert_eq!(terms.targets.len(), b);
        let mut total = 0.0;
        for r in 0..b {
            let row: Vec<f64> = (0..v).map(|i| sv.data[r * v + i].re * terms.beta).collect();
            let max = row.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let z: f64 = row.iter().map(|x| (x - max).exp()).sum();
            let p_t = (row[terms.targets[r]] - max).exp() / z;
            let p = terms.k[r] + terms.coef[r] * (terms.gamma * p_t + (1.0 - terms.gamma) * terms.other[r]);
            total -= p.max(1e-12).ln();
        }
        let t = Tensor::from_vec([1, 1, 1], vec![c64::new(total / b.max(1) as f64, 0.0)]);
        let rg = self.any_grad(&[scores]);
        self.push(t, Op::MixtureNll(scores, terms), rg)
    }

    /// `mean_b −log softmax(β · s_b)[t_b]` over `[b,V,1]` real scores — the
    /// ordinary cross-entropy of a full-table softmax, for ranking against
    /// every item at once rather than one decoy (`hinge`) or inside a mixture
    /// (`mixture_nll`). `exp` of it is a perplexity over the table.
    pub fn cross_entropy(&mut self, scores: Var, targets: &[usize], beta: f64) -> Var {
        let sv = &self.nodes[scores.0].value;
        let (b, v) = (sv.shape[0], sv.shape[1]);
        assert_eq!(targets.len(), b, "cross_entropy: one target per row");
        let mut total = 0.0;
        for r in 0..b {
            let row = &sv.data[r * v..(r + 1) * v];
            let max = row.iter().map(|z| z.re * beta).fold(f64::NEG_INFINITY, f64::max);
            let z: f64 = row.iter().map(|x| (x.re * beta - max).exp()).sum();
            total -= row[targets[r]].re * beta - max - z.ln();
        }
        let t = Tensor::from_vec([1, 1, 1], vec![c64::new(total / b.max(1) as f64, 0.0)]);
        let rg = self.any_grad(&[scores]);
        self.push(t, Op::CrossEntropy(scores, targets.to_vec(), beta), rg)
    }

    // ── backward ──
}
