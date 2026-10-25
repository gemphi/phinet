---
id: phinet-performance
title: Performance
description: What is fast, what is slow, why, and the numbers on a two-core machine.
sidebar_position: 7
---

# Performance

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

Measured with `cargo run --release --example bench` on two cores ([Benchmarks](/docs/measurements/benchmarks) has both the before and the after):

| path | before | after |
|---|---|---|
| `fft_inplace` + inverse, 64 points | 1.85 µs per pair | 1.2 µs |
| `Fno::apply`, one function, width 8, 2 layers, 12 modes | 916 µs | **88 µs** |
| `train_epoch` (hinge), 10,000 examples, batch 64 | 3.0 s | 1.8 s |
| `train_epoch_mixture`, 2,000 examples, 2,000-item table | 2.4 s | 1.5 s |
| `cosine_table` 256 × 6,000 × 64 | 79 ms | 69 ms |

## What changed

`linear` and `mode_mix` — forward and backward — now walk row slices instead of calling a bounds-checked `get` per element, and the backward passes no longer clone their input tensors. That gave about 2×. The other 5× came from `Fno::apply` running in chunks of `APPLY_CHUNK = 256` functions: every op allocates its output, so one 4,096-function batch kept a dozen 32 MB tensors alive and paid for the memory in page faults; `examples/profile_apply.rs` showed a plain `add` taking 136 ms for that reason.

## What is still slow, and why

The remaining ~10× to a BLAS-backed batched complex GEMM is the loop structure itself (no SIMD, no blocking, one output row at a time) and per-op allocation. `cosine_table` is the one parallel path. In order of payoff: a real batched GEMM for `linear` / `mode_mix`, reusing tape allocations between steps, f32, SIMD butterflies in the FFT, a GPU. For the sizes Phiano uses today the engine is no longer the wall-clock; the lexicon trainer is.
