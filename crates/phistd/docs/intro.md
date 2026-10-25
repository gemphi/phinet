---
id: phinet-intro
title: Phinet
description: A Fourier tensor engine for neural operators on the circle — complex autograd, a differentiable unitary FFT, Fourier neural operators, joint training, holographic binding and sixty-four spectral gates, in one dependency-light Rust crate, with a PyTorch twin checked to 1e-16.
sidebar_position: 0
badge: Engine
---

![Phinet](/logo.svg)

# Phinet

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

Rust, one real dependency (`num-complex`), `rayon` for the one hot loop, no unsafe, every gradient finite-difference checked in the crate **and** matched against `torch.autograd` on a reference dump.

## Where to go

| you want to… | read |
|---|---|
| install and run something in sixty seconds | [Install](/docs/get-started/install), [Quickstart](/docs/get-started/quickstart) |
| come from PyTorch or TensorFlow | [From PyTorch](/docs/get-started/from-pytorch), [Parity](/docs/interop/pytorch-parity) |
| learn by doing | [Tutorials](/docs/tutorials/first-operator) — seven, in order |
| understand a design decision | [Guide](/docs/guide/tensor-engine) — tensors, the operator, objectives, autograd, conventions, serialization, performance |
| look a function up | [API Reference](/docs/api/every-operation) — every public item, by module |
| see what the numbers are | [Measurements](/docs/measurements/results), [Benchmarks](/docs/measurements/benchmarks) |
| know what it does not do | [What is missing](/docs/about/what-is-missing) |

## What it is

A *function* is `Vec<Complex64>`: samples of a complex function on a circle at a power-of-two number of points. Anything that can be read as such a function — a word as 64 relation channels, a signal, a periodic field — can be fed to an operator that maps functions to functions and learns, from examples, what the mapping should be. That is a neural operator: a map between *functions*, parameterised in the Fourier domain so that it is smooth over the circle and applies at any resolution.

| capability | where |
|---|---|
| Complex tensors `[batch, width, space]` with reverse-mode autograd | `tensor::Tensor`, `tensor::Tape` |
| A unitary radix-2 FFT as a differentiable op | `Tape::fft`, `Tape::ifft` |
| Fourier neural operator: per-mode filter + pointwise path + GELU, lift and projection | `operator::Fno` |
| Ranking objective (hinge on cosine vs a decoy), uniform or frequency-proportional decoys | `Fno::train_epoch`, `train_epoch_with`, `FrequencySampler` |
| Full-table likelihood objective for a mixed language model | `Fno::train_epoch_mixture`, `Tape::cosine_table`, `Tape::mixture_nll` |
| The table of items as a parameter, trained with the operator, held by a proximal tether | `joint::Joint` |
| Möbius maps, a Riemann-zeta spectral prior | `Tape::mobius`, `zeta_weights`, `Fno::scale_modes` |
| Plain cross-entropy over the table; AdamW; versioned save with optimiser state | `Tape::cross_entropy`, `Adam::with_decay`, `Fno::to_bytes_with` |
| The phasor ranges P16–P128 × Q16–Q64, and the operator at any resolution | `phasor`, `Fno::at_resolution` |
| A module trait and a two-operator stack | `Module`, `Sequential` |
| The 2-D torus, learning-rate schedules, f32 storage | `Tape::fft2`, `Schedule`, `Fno::to_bytes_compact` |
| Holographic bind / unbind / compose | `hrr` |
| The sixty-four operations: I Ching hexagrams as spectral gates | `hexagram` |
| Save and load an operator | `Fno::to_bytes`, `Fno::from_bytes` |
| The same operations in PyTorch, checked | `torch/phinet_torch.py` |

## Sixty seconds

```rust
use phinet::{Fno, FnoConfig, Adam, Example, rank, unit_random};

// Sixty items on a 64-point circle; the target of each is itself shifted by
// one sample — a relation the identity cannot express and a spectral filter can.
let inputs: Vec<_> = (0..60).map(|i| unit_random(64, i)).collect();
let targets: Vec<_> = inputs.iter().map(|f| { let mut g = f.clone(); g.rotate_right(1); g }).collect();
let examples: Vec<Example> = (0..60).map(|i| Example { input: inputs[i].clone(), target: i }).collect();
let pool: Vec<usize> = (0..60).collect();

let cfg = FnoConfig { space: 64, width: 4, layers: 1, modes: 32, margin: 0.5, lr: 1e-2 };
let mut fno = Fno::new(cfg, 1);
let mut adam = Adam::new(cfg.lr);
for epoch in 0..60 {
    fno.train_epoch(&targets, &examples, &pool, 20, &mut adam, epoch);
}
let (mrr, _) = rank(&targets, &fno.apply(&inputs), &examples, &pool);
assert!(mrr > 0.9);
```

## Where it came from

Built for [Phiano](https://github.com/gemphi/phiano), where a word is 64 complex numbers on a circle of relations. Phiano's `src/phinet/` is the application layer — a lexicon as a function space, the operator inside a perplexity harness, joint training with write-back, recursive composition with the operator as cleanup, the operator inside generation. The engine itself has no notion of words.

```
gemphi/
├── phinet/    ← this crate: tensor, operator, joint, hrr, hexagram; torch/ twin; docs/; web/
└── phiano/    ← depends on it: phinet = { path = "../phinet" }
```
