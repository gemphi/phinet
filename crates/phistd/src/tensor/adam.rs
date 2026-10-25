//! Adam / AdamW on the `(Re, Im)` pairs of each parameter, its schedule, and its saved state.

use super::{Tape, Var};
use crate::c64;

/// Adam on the `(Re, Im)` pairs of each parameter.
pub struct Adam {
    pub lr: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub eps: f64,
    /// Decoupled weight decay (AdamW, Loshchilov & Hutter 2019): each step
    /// first shrinks the parameter by `lr · weight_decay`, then applies the
    /// Adam update. `0` is plain Adam.
    pub weight_decay: f64,
    /// How `lr` changes with the step; the base `lr` is scaled by
    /// `schedule.factor(t)` each step. `Constant` by default.
    pub schedule: Schedule,
    m: Vec<Vec<c64>>,
    v: Vec<Vec<c64>>,
    t: usize,
}

/// A learning-rate schedule: what `lr` is at a given step.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Schedule {
    /// The same rate forever.
    Constant,
    /// Linear warm-up over `warmup` steps, then cosine decay to `floor × base`
    /// at step `total` (and `floor × base` after).
    Cosine { warmup: usize, total: usize, floor: f64 },
    /// Multiply by `gamma` every `every` steps.
    Step { every: usize, gamma: f64 },
}

impl Schedule {
    /// The multiplier on the base rate at `step` (1-based, as `Adam` counts).
    pub fn factor(self, step: usize) -> f64 {
        match self {
            Schedule::Constant => 1.0,
            Schedule::Cosine { warmup, total, floor } => {
                if warmup > 0 && step <= warmup {
                    return step as f64 / warmup as f64;
                }
                if total <= warmup || step >= total {
                    return floor;
                }
                let p = (step - warmup) as f64 / (total - warmup) as f64;
                floor + (1.0 - floor) * 0.5 * (1.0 + (std::f64::consts::PI * p).cos())
            }
            Schedule::Step { every, gamma } => gamma.powi(step.checked_div(every).unwrap_or(0) as i32),
        }
    }
}

/// The optimiser's state, for saving beside an operator.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdamState {
    pub lr: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub eps: f64,
    pub weight_decay: f64,
    pub schedule: Schedule,
    pub m: Vec<Vec<(f64, f64)>>,
    pub v: Vec<Vec<(f64, f64)>>,
    pub t: usize,
}

impl Adam {
    pub fn new(lr: f64) -> Self {
        Self { lr, beta1: 0.9, beta2: 0.999, eps: 1e-8, weight_decay: 0.0, schedule: Schedule::Constant, m: Vec::new(), v: Vec::new(), t: 0 }
    }

    /// Attaches a learning-rate schedule.
    pub fn with_schedule(mut self, schedule: Schedule) -> Self {
        self.schedule = schedule;
        self
    }

    /// The rate the next step will use.
    pub fn current_lr(&self) -> f64 {
        self.lr * self.schedule.factor(self.t + 1)
    }

    /// AdamW: the same optimiser with decoupled weight decay.
    pub fn with_decay(mut self, weight_decay: f64) -> Self {
        self.weight_decay = weight_decay;
        self
    }

    /// Steps taken so far.
    pub fn steps(&self) -> usize {
        self.t
    }

    pub fn state(&self) -> AdamState {
        let cv = |m: &Vec<Vec<c64>>| m.iter().map(|r| r.iter().map(|z| (z.re, z.im)).collect()).collect();
        AdamState { lr: self.lr, beta1: self.beta1, beta2: self.beta2, eps: self.eps, weight_decay: self.weight_decay, schedule: self.schedule, m: cv(&self.m), v: cv(&self.v), t: self.t }
    }

    pub fn from_state(s: AdamState) -> Self {
        let cv = |m: Vec<Vec<(f64, f64)>>| m.into_iter().map(|r| r.into_iter().map(|(re, im)| c64::new(re, im)).collect()).collect();
        Self { lr: s.lr, beta1: s.beta1, beta2: s.beta2, eps: s.eps, weight_decay: s.weight_decay, schedule: s.schedule, m: cv(s.m), v: cv(s.v), t: s.t }
    }

    /// One update over `params`, reading their gradients from the tape.
    pub fn step(&mut self, tape: &mut Tape, params: &[Var]) {
        if self.m.len() != params.len() {
            self.m = params.iter().map(|p| vec![c64::new(0.0, 0.0); tape.value(*p).len()]).collect();
            self.v = self.m.clone();
            self.t = 0;
        }
        self.t += 1;
        let lr = self.lr * self.schedule.factor(self.t);
        let (b1, b2) = (self.beta1, self.beta2);
        let c1 = 1.0 - b1.powi(self.t as i32);
        let c2 = 1.0 - b2.powi(self.t as i32);
        for (pi, p) in params.iter().enumerate() {
            let g = match tape.grad(*p) {
                Some(g) => g.data.clone(),
                None => continue,
            };
            let mut value = tape.value(*p).clone();
            if self.weight_decay > 0.0 {
                let keep = 1.0 - lr * self.weight_decay;
                for z in value.data.iter_mut() {
                    *z *= keep;
                }
            }
            for (k, gz) in g.iter().enumerate() {
                let m = &mut self.m[pi][k];
                let v = &mut self.v[pi][k];
                *m = c64::new(b1 * m.re + (1.0 - b1) * gz.re, b1 * m.im + (1.0 - b1) * gz.im);
                *v = c64::new(
                    b2 * v.re + (1.0 - b2) * gz.re * gz.re,
                    b2 * v.im + (1.0 - b2) * gz.im * gz.im,
                );
                let mhat = c64::new(m.re / c1, m.im / c1);
                let vhat = c64::new(v.re / c2, v.im / c2);
                value.data[k] -= c64::new(
                    lr * mhat.re / (vhat.re.sqrt() + self.eps),
                    lr * mhat.im / (vhat.im.sqrt() + self.eps),
                );
            }
            tape.set_value(*p, value);
        }
    }
}
