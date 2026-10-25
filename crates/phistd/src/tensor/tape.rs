//! The tape: values, the ops that made them, and the handles into it.

use super::ops::MixtureTerms;
use super::Tensor;

/// A handle to a value on the tape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Var(pub usize);

#[derive(Debug, Clone)]
pub(super) enum Op {
    Leaf,
    Add(Var, Var),
    /// `x [b,w,s] + bias [1,w,1]`.
    AddBias(Var, Var),
    Mul(Var, Var),
    Scale(Var, f64),
    /// `y[b,o,s] = Σ_i x[b,i,s]·w[i,o,0]`.
    Linear(Var, Var),
    /// `y[b,o,m] = Σ_i x[b,i,m]·r[i,o,m]` — one complex matrix per mode.
    ModeMix(Var, Var),
    Fft(Var),
    Ifft(Var),
    /// 2-D FFT of the sample axis read as an `s1 × s2` torus.
    Fft2(Var, usize, usize),
    Ifft2(Var, usize, usize),
    Gelu(Var),
    /// `z ↦ z · tanh|z| / |z|`: the phase-preserving activation (squash.rs).
    Squash(Var),
    /// Real cosine similarity per batch row between `[b,1,s]` and a constant
    /// target `[b,1,s]`: output `[b,1,1]` with zero imaginary part.
    Cosine(Var, Var),
    /// Hinge over `[b,1,1]` scores: `mean(max(0, margin − pos + neg))`.
    Hinge(Var, Var, f64),
    /// Mean of real parts of `[b,1,1]`.
    MeanReal(Var),
    /// Rows of a `[V,1,s]` table selected by index: output `[b,1,s]`.
    Gather(Var, Vec<usize>),
    /// Each `[·,·,s]` row divided by its own energy, so it has unit norm.
    Unit(Var),
    /// Real cosine of every row of `pred [b,1,s]` against every row of
    /// `table [V,1,s]`: output `[b,V,1]`.
    CosineTable(Var, Var),
    /// Mixture negative log-likelihood over `[b,V,1]` scores — see
    /// [`Tape::mixture_nll`]. Carries the per-row constants.
    MixtureNll(Var, MixtureTerms),
    /// Möbius map `w = (a z + b)/(c z + d)` applied to every sample of `x`,
    /// with the four coefficients a `[1,1,4]` parameter.
    Mobius(Var, Var),
    /// Plain softmax cross-entropy over `[b,V,1]` real scores at temperature β.
    CrossEntropy(Var, Vec<usize>, f64),
    /// `y[i,o,k] = r[i,o,group[k]]` — one weight per group, tied across the
    /// modes of that group (a spherical operator ties one weight per degree).
    Tie(Var, Vec<usize>),
}

pub(super) struct Node {
    pub(super) value: Tensor,
    pub(super) grad: Option<Tensor>,
    pub(super) op: Op,
    pub(super) requires_grad: bool,
}

/// The tape. Records every operation so that `backward` can replay it.
#[derive(Default)]
pub struct Tape {
    pub(super) nodes: Vec<Node>,
}

impl Tape {
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn push(&mut self, value: Tensor, op: Op, requires_grad: bool) -> Var {
        self.nodes.push(Node { value, grad: None, op, requires_grad });
        Var(self.nodes.len() - 1)
    }

    /// A value that gradients are accumulated for.
    pub fn param(&mut self, t: Tensor) -> Var {
        self.push(t, Op::Leaf, true)
    }

    /// A value that gradients stop at.
    pub fn constant(&mut self, t: Tensor) -> Var {
        self.push(t, Op::Leaf, false)
    }

    pub fn value(&self, v: Var) -> &Tensor {
        &self.nodes[v.0].value
    }

    pub fn grad(&self, v: Var) -> Option<&Tensor> {
        self.nodes[v.0].grad.as_ref()
    }

    pub fn set_value(&mut self, v: Var, t: Tensor) {
        self.nodes[v.0].value = t;
    }

    /// Clears everything recorded after the first `keep` nodes (the
    /// parameters), so one tape can be reused across steps without
    /// re-creating them.
    pub fn truncate(&mut self, keep: usize) {
        self.nodes.truncate(keep);
        for n in self.nodes.iter_mut() {
            n.grad = None;
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub(super) fn any_grad(&self, vs: &[Var]) -> bool {
        vs.iter().any(|v| self.nodes[v.0].requires_grad)
    }

    // ── forward ops ──
}
