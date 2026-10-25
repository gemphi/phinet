---
id: phinet-api-tape
title: tensor::Tape and Var
description: The reverse-mode tape — parameters, constants, every forward op with its shapes, backward, and MixtureTerms.
sidebar_position: 2
badge: API
---

# `phinet::tensor::Tape`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

```rust
pub struct Var(pub usize);      // a handle to a node on the tape
pub struct Tape { /* nodes */ }
```

## Lifecycle

| item | signature | notes |
|---|---|---|
| `Tape::new` | `fn new() -> Tape` | |
| `param` | `fn param(&mut self, t: Tensor) -> Var` | requires grad |
| `constant` | `fn constant(&mut self, t: Tensor) -> Var` | no grad |
| `value` | `fn value(&self, v: Var) -> &Tensor` | |
| `grad` | `fn grad(&self, v: Var) -> Option<&Tensor>` | `None` until `backward`, or for constants |
| `set_value` | `fn set_value(&mut self, v: Var, t: Tensor)` | used by optimisers and `mask_modes` |
| `truncate` | `fn truncate(&mut self, keep: usize)` | drops nodes after `keep` — how a step's graph is released |
| `len` / `is_empty` | `fn len(&self) -> usize` | number of nodes |
| `backward` | `fn backward(&mut self, out: Var)` | `out` must be `[1,1,1]`; zeroes all grads first |

## Forward ops

All take and return `Var`; shapes are asserted.

| op | signature | in → out |
|---|---|---|
| `add` | `fn add(&mut self, a: Var, b: Var) -> Var` | same shape |
| `add_bias` | `fn add_bias(&mut self, x: Var, bias: Var) -> Var` | `[b,w,s] + [1,w,1]` |
| `mul` | `fn mul(&mut self, a: Var, b: Var) -> Var` | elementwise complex |
| `scale` | `fn scale(&mut self, a: Var, k: f64) -> Var` | real scalar |
| `linear` | `fn linear(&mut self, x: Var, w: Var) -> Var` | `[b,i,s] × [i,o,1] → [b,o,s]` |
| `mode_mix` | `fn mode_mix(&mut self, x: Var, r: Var) -> Var` | `[b,i,m] × [i,o,m] → [b,o,m]` |
| `fft` / `ifft` | `fn fft(&mut self, x: Var) -> Var` | along `s`, unitary |
| `fft2` / `ifft2` | `fn fft2(&mut self, x: Var, s1: usize, s2: usize) -> Var` | the sample axis read as an `s1 × s2` torus; unitary; `fft2_inplace` is the slice form |
| `gelu` | `fn gelu(&mut self, x: Var) -> Var` | on Re and Im |
| `unit` | `fn unit(&mut self, x: Var) -> Var` | each `[·,·,s]` row to unit energy |
| `cosine` | `fn cosine(&mut self, pred: Var, target: Var) -> Var` | `[b,1,s]` × `[b,1,s]` → `[b,1,1]` real |
| `cosine_table` | `fn cosine_table(&mut self, pred: Var, table: Var) -> Var` | `[b,1,s]` × `[V,1,s]` → `[b,V,1]`; rayon over `b` |
| `gather` | `fn gather(&mut self, table: Var, idx: &[usize]) -> Var` | `[V,1,s]` rows → `[b,1,s]`; scatter-add backward |
| `hinge` | `fn hinge(&mut self, pos: Var, neg: Var, margin: f64) -> Var` | `[b,1,1]` × 2 → `[1,1,1]` |
| `mean_real` | `fn mean_real(&mut self, x: Var) -> Var` | → `[1,1,1]` |
| `mobius` | `fn mobius(&mut self, x: Var, abcd: Var) -> Var` | `abcd` is `[1,1,4]` |
| `mixture_nll` | `fn mixture_nll(&mut self, scores: Var, terms: MixtureTerms) -> Var` | `[b,V,1]` → `[1,1,1]` |
| `cross_entropy` | `fn cross_entropy(&mut self, scores: Var, targets: &[usize], beta: f64) -> Var` | `[b,V,1]` → `[1,1,1]`; `mean −log softmax(β s)[t]` |

Mathematics and the torch equivalent of each: [Every operation](/docs/api/every-operation). Backward rules: [Autograd](/docs/guide/autograd).

## `MixtureTerms`

```rust
pub struct MixtureTerms {
    pub targets: Vec<usize>,   // true item per row
    pub k: Vec<f64>,           // additive constant per row
    pub coef: Vec<f64>,        // multiplier on the scored term per row
    pub gamma: f64,            // weight of the operator inside the scored term
    pub other: Vec<f64>,       // the other component's probability per row
    pub beta: f64,             // softmax temperature
}
```

`mixture_nll = mean_b −log( k_b + coef_b · (γ · softmax(β · s_b)[t_b] + (1 − γ) · other_b) )`.

## Example — a bare training step

```rust
let mut tape = Tape::new();
let w = tape.param(Tensor::randn([1, 4, 1], 1.0, 1));
let keep = tape.len();                 // everything after this is per-step
for step in 0..100 {
    tape.truncate(keep);
    let x = tape.constant(batch_tensor(step));
    let h = tape.gelu(tape.linear(x, w));
    let loss = tape.mean_real(h);
    tape.backward(loss);
    adam.step(&mut tape, &[w]);
}
```
