---
id: phinet-benchmarks
title: Benchmarks
description: Wall-clock of the hot paths from examples/bench.rs, on the machine that ran it.
sidebar_position: 2
badge: Results
---

# Benchmarks

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

`cargo run --release --example bench` — Linux x86-64, **2 cores**, `opt-level = 3`, f64 throughout. One run; treat ±10% as noise.

| path | size | before | after |
|---|---|---|---|
| `fft_inplace` forward + inverse | 64 points × 100,000 pairs | 1.85 µs per pair | 1.2 µs |
| `Fno::apply` | 4,096 functions; space 64, width 8, 2 layers, 12 modes (8,360 params) | 916 µs per function | **88 µs** |
| `Fno::train_epoch` (hinge) | 10,000 examples, pool 2,000, batch 64 | 2.98 s | 1.76 s |
| `Fno::train_epoch_mixture` | 2,000 examples, table 2,000, batch 64 | 2.41 s | 1.45 s |
| `Tape::cosine_table` | 256 × 6,000 × 64 | 79 ms | 69 ms |
| `Fno::to_bytes` | 8,360 params | 134,104 bytes | 134,113 bytes (v2 header) |

"before" is the first draft of this page; "after" is the slice-based loops and chunked `apply` ([Performance](/docs/guide/performance)).

## Reading it

- The FFT is not the cost. A 64-point pair is about a microsecond; what `apply` had been paying for was one enormous batch's memory, and after that the loop structure ([Performance](/docs/guide/performance)).
- The mixture objective is a full-table softmax per example: 2,000 examples cost as much as 8,000 hinge examples at this table size, and it scales with the table.
- `cosine_table` is the one parallel path; on a many-core machine it scales with the batch dimension.

## Where a softmax batch goes

`examples/profile_softmax.rs` times one training batch of the wide operator (width 16, 3 layers, 16 modes; 50,016 params) against a 1,228-row table, and `examples/profile_ops.rs` times each op on a `[64,16,64]` tensor, forward and backward. On the same 2-core machine as above:

| stage | ms |
|---|---|
| stack the batch | 0.3 |
| `forward` (10 ops) | 30 |
| `cosine_table` 64 × 1,228 | 4–7 |
| `cross_entropy` | 0.7 |
| `backward` | 54–75 |
| `Adam::step` + `mask_modes` | 0.8 |

Every op on that 1 MB tensor costs 2–5 ms forward and 1–5 ms backward — `add` included, which does one pass over the data. That is not arithmetic: `Tensor::zeros` of the same 1 MB takes 0.7 ms and a clone 0.66 ms here, so each op is paying for a fresh allocation and its first touch, at this machine's ~1.5 GB/s. The engine is allocation- and bandwidth-bound at these sizes, not compute-bound; the FFT and the mode mix are a small share. Two changes follow from it: the `cosine_table` backward now skips the gradient pass for a side that needs none (the table is a constant while the operator trains — that pass was the larger of the two by v/b, and a full 204k-example softmax epoch was over 14 min before it), and a buffer pool or an f32 path is the next engine item, not a faster FFT.

## How Phiano's runs relate

`bin/austen` (train a 17k-word lexicon three passes, personas, the operator four epochs, the learned scorer, 48 compositions) takes ≈ 400 s; `bin/liskov` ≈ 11 min, almost all of it `relational_fit` over a 622-word pool, which is Phiano's loop, not this engine's. The operator's share of a Phiano run is seconds.

To reproduce: the example is deterministic (seeds fixed); only the times vary by machine.
