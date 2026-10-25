---
id: phinet-api-adam
title: tensor::Adam
description: The optimiser — Adam on real and imaginary parts as independent reals, bias-corrected, one instance per parameter set.
sidebar_position: 3
badge: API
---

# `phinet::tensor::Adam`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

```rust
pub struct Adam { pub lr: f64, pub beta1: f64, pub beta2: f64, pub eps: f64, pub weight_decay: f64, /* moments, t */ }
pub struct AdamState { /* lr, betas, eps, weight_decay, m, v, t — Serialize/Deserialize */ }
impl Adam {
    pub fn new(lr: f64) -> Adam;                          // β₁ 0.9, β₂ 0.999, ε 1e-8, no decay
    pub fn with_decay(self, weight_decay: f64) -> Adam;   // AdamW: decoupled decay, p ← p·(1 − lr·wd) before the update
    pub fn with_schedule(self, schedule: Schedule) -> Adam; // lr × schedule.factor(step) each step
    pub fn current_lr(&self) -> f64;
    pub fn step(&mut self, tape: &mut Tape, params: &[Var]);
    pub fn steps(&self) -> usize;
    pub fn state(&self) -> AdamState;                     // for Fno::to_bytes_with
    pub fn from_state(s: AdamState) -> Adam;
}
```

`with_decay` matches `torch.optim.AdamW` after one step to 1e-16 (parity check "AdamW step").

## `Schedule`

```rust
pub enum Schedule {
    Constant,
    Cosine { warmup: usize, total: usize, floor: f64 },   // linear warm-up, cosine decay to floor·lr at `total`
    Step { every: usize, gamma: f64 },                    // × gamma every `every` steps
}
impl Schedule { pub fn factor(self, step: usize) -> f64 }   // the multiplier at a 1-based step
```

Saved in `AdamState` with the rest of the optimiser. The counterparts are `torch.optim.lr_scheduler.{LambdaLR, CosineAnnealingLR, StepLR}`; the warm-up-then-cosine shape is the one transformer training uses.

`step` reads each parameter's gradient from the tape (skipping any without one), updates first and second moments separately for the real and imaginary parts, applies bias correction with the step count, and writes the new value back with `set_value`. Moments are allocated on the first call and keyed by position in `params`, so the same `params` slice must be passed every time; a different length resets the state.

Two optimisers can drive one tape — `Joint` uses one for the operator and one, at a lower rate, for the table. Change `adam.lr` between steps for a schedule; there is no scheduler type.

Matches `torch.optim.Adam(params, lr, betas=(0.9, 0.999), eps=1e-8)` on complex parameters after one step to 1e-16 ([parity](/docs/interop/pytorch-parity)).
