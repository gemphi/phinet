---
id: phinet-api-operator
title: operator::Fno and friends
description: FnoConfig, Fno (construction, forward, apply, masks, training epochs, save/load), Example, MixtureExample, FrequencySampler, rank, cosine, zeta_weights, stack, unstack.
sidebar_position: 4
badge: API
---

# `phinet::operator`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## `FnoConfig`

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FnoConfig {
    pub space: usize,    // samples on the circle; power of two
    pub width: usize,    // features after the lift
    pub layers: usize,   // Fourier layers
    pub modes: usize,    // kept modes each side of zero; 0 = pointwise control
    pub margin: f64,     // hinge margin on cosine
    pub lr: f64,
}
impl FnoConfig { pub fn for_space(space: usize) -> FnoConfig }   // width 8, layers 2, modes max(space/5, 1), margin 0.2, lr 2e-3
```

## `Fno`

```rust
pub struct Fno { pub config: FnoConfig, pub tape: Tape, /* lift, lift_b, layers (r, w, b), proj, keep, mask */ }
```

| item | signature | notes |
|---|---|---|
| `new` | `fn new(config: FnoConfig, seed: u64) -> Fno` | parameters: lift `[1,w,1]` (σ 1), lift_b zeros, per layer `R [w,w,s]` (σ 1/w, masked), `W [w,w,1]` (σ 1/√w), `b` zeros, proj `[w,1,1]` (σ 1/√w) |
| `params` | `fn params(&self) -> Vec<Var>` | `[lift, lift_b, (r, w, b)…, proj]` |
| `parameter_count` | `fn parameter_count(&self) -> usize` | complex parameters |
| `attach_param` | `fn attach_param(&mut self, t: Tensor) -> Var` | an extra parameter that survives `reset` (used by `Joint`) |
| `reset` | `fn reset(&mut self)` | truncates the tape after the parameters |
| `forward` | `fn forward(&mut self, x: Var) -> Var` | `[b,1,s] → [b,1,s]` on the tape |
| `apply` | `fn apply(&mut self, inputs: &[Function]) -> Vec<Function>` | off the tape, in chunks of `APPLY_CHUNK` (256) |
| `mask_modes` | `fn mask_modes(&mut self)` | re-zero masked modes of every `R` |
| `scale_modes` | `fn scale_modes(&mut self, weights: &[f64])` | multiply mode `m` (and `s − m`) of every `R` by `weights[m]` |
| `decoy` | `fn decoy(pool: &[usize], target: usize, r: u64) -> usize` | a pool item ≠ target |
| `train_epoch` | `fn train_epoch(&mut self, table: &[Function], examples: &[Example], pool: &[usize], batch: usize, adam: &mut Adam, seed: u64) -> f64` | hinge vs one uniform decoy; returns mean loss |
| `train_epoch_with` | `fn train_epoch_with(&mut self, table, examples, sampler: &dyn Fn(usize, u64) -> usize, batch, adam, seed) -> f64` | decoys from `sampler(target, seed)` |
| `train_epoch_mixture` | `fn train_epoch_mixture(&mut self, table, examples: &[MixtureExample], gamma: f64, beta: f64, batch, adam, seed) -> f64` | full-table likelihood; `exp(loss)` is the phase term's perplexity |
| `train_epoch_softmax` | `fn train_epoch_softmax(&mut self, table, examples: &[Example], beta: f64, batch, adam, seed) -> f64` | full-table cross-entropy at temperature β; `exp(loss)` is the perplexity over the table |
| `at_resolution` | `fn at_resolution(&self, space: usize) -> Fno` | the same operator at another power-of-two resolution; `R` copied by signed frequency ([Phasor ranges](/docs/guide/phasor-ranges)) |
| `to_bytes` | `fn to_bytes(&self) -> Vec<u8>` | `PHINET` + version 2 + bincode; see [Serialization](/docs/guide/serialization) |
| `to_bytes_with` | `fn to_bytes_with(&self, adam: &Adam) -> Vec<u8>` | the same, with the optimiser state |
| `to_bytes_compact` | `fn to_bytes_compact(&self) -> Vec<u8>` | format 3: parameters as `f32` pairs, half the size; loads back to `f64` with < 1e-10 cosine drift on `apply` |
| `from_bytes` | `fn from_bytes(bytes: &[u8]) -> Option<Fno>` | reads v2 and legacy v1 |
| `from_bytes_with` | `fn from_bytes_with(bytes: &[u8]) -> Option<(Fno, Option<Adam>)>` | with the optimiser when the file carried one |

Every training epoch shuffles the examples by `mix64(i ^ seed)`, runs `batch`-sized steps, calls `adam.step` then `mask_modes` after each, and returns the mean batch loss.

## `Module` and `Sequential`

```rust
pub trait Module {
    fn tape(&mut self) -> &mut Tape;
    fn parameters(&self) -> Vec<Var>;
    fn forward(&mut self, x: Var) -> Var;
    fn parameter_count(&self) -> usize;
}
impl Module for Fno { … }
pub struct Sequential { pub first: Fno, /* second's params on first's tape */ }
impl Sequential { pub fn new(first: Fno, second: Fno) -> Sequential }   // one tape, one backward for both
impl Module for Sequential { … }
```

## Examples

```rust
pub struct Example        { pub input: Function, pub target: usize }
pub struct MixtureExample { pub input: Function, pub target: usize, pub k: f64, pub coef: f64, pub other: f64 }
```

## `FrequencySampler`

```rust
impl FrequencySampler {
    pub fn new(items: Vec<usize>, weights: &[f64], alpha: f64) -> FrequencySampler;   // draws ∝ weight^alpha
    pub fn draw(&self, target: usize, r: u64) -> usize;                               // an item ≠ target (16 tries, then `items[0]`)
}
```

## Free functions

| item | signature | notes |
|---|---|---|
| `rank` | `fn rank(table: &[Function], preds: &[Function], examples: &[Example], pool: &[usize]) -> (f64, f64)` | (MRR, hit@10) of each target among `pool` by cosine to the prediction |
| `cosine` | `fn cosine(a: &[c64], b: &[c64]) -> f64` | `Re⟨a,b⟩ / (‖a‖‖b‖)` |
| `zeta_weights` | `fn zeta_weights(modes: usize, s: f64) -> Vec<f64>` | `m^{−s}/ζ(s)`, mode 0 → 1 |
| `stack` | `fn stack(fs: &[Function], space: usize) -> Tensor` | → `[n,1,space]`; asserts lengths |
| `unstack` | `fn unstack(t: &Tensor) -> Vec<Function>` | from `[n,1,space]` |
