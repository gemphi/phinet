---
id: phinet-api-lib
title: Crate root
description: Re-exports, the Function type, mix64, normalise, unit_random, unit_phase.
sidebar_position: 8
badge: API
---

# `phinet` (crate root)

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

```rust
pub mod hexagram; pub mod hrr; pub mod joint; pub mod operator; pub mod tensor;
pub use num_complex::Complex64 as c64;
pub use joint::{IndexExample, Joint};
pub use operator::{cosine, rank, stack, unstack, zeta_weights, Example, Fno, FnoConfig, FrequencySampler, MixtureExample};
pub use tensor::{fft_inplace, Adam, MixtureTerms, Tape, Tensor, Var};
pub type Function = Vec<c64>;
```

| item | signature | notes |
|---|---|---|
| `mix64` | `fn mix64(h: u64) -> u64` | splitmix64's finaliser; the crate's hash for shuffles and decoys |
| `normalise` | `fn normalise(f: &mut [c64]) -> bool` | to unit energy in place; `false` (and unchanged) if energy < 1e-24 |
| `unit_random` | `fn unit_random(space: usize, seed: u64) -> Function` | complex Gaussian samples, normalised |
| `unit_phase` | `fn unit_phase(space: usize, seed: u64) -> Function` | magnitude 1 at every sample, random phase — the shape of a role |

## Layout

| path | what |
|---|---|
| `tensor/mod.rs`, `fft.rs`, `tape.rs` | `Tensor`; the FFTs and GELU; `Var`, the op list, `Tape` lifecycle |
| `tensor/ops_basic.rs`, `ops_score.rs`, `ops_loss.rs` | forward ops: value-moving; value-reading; the losses |
| `tensor/backward.rs`, `backward_score.rs`, `backward_loss.rs` | the reverse pass, split the same way |
| `tensor/adam.rs` | `Adam`, `AdamState`, `Schedule` |
| `operator/mod.rs`, `fno.rs`, `modes.rs`, `train.rs`, `save.rs`, `module.rs`, `sampler.rs` | config and examples; the operator; masks and priors; epochs; the file format; `Module`/`Sequential`; decoys, `rank`, `stack` |
| `phasor/mod.rs`, `functions.rs` | the ranges; `resample`, `quantise`, `sector_of`, `table` |
| `joint/mod.rs`, `hrr.rs`, `hexagram.rs` | joint training; binding; the sixty-four gates |
| `*/tests*.rs` | one test file per topic |

No file is over 200 lines.

## Examples in the crate

| file | run | what |
|---|---|---|
| `examples/torch_reference.rs` | `cargo run --release --example torch_reference > torch/reference.json` | the reference dump for the PyTorch twin |
| `examples/bench.rs` | `cargo run --release --example bench` | wall-clock of the hot paths |

## Generated reference (rustdoc)

`npm run rustdoc` in `phinet/web/` runs `cargo doc --no-deps` and places the output under `public/rustdoc/`, served by this site at [/rustdoc/phinet/](/rustdoc/phinet/index.html) — every item with its doc-comment, source links included. `npm run build` does it automatically. The hand-written pages here are the reference for *what* and *why*; rustdoc is the reference for *exactly*.
