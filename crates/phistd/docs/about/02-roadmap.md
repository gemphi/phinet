---
id: phinet-roadmap
title: Roadmap
description: What is next for the engine and for its use in Phiano, with what the last roadmap promised and delivered.
sidebar_position: 2
---

# Roadmap

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## Delivered since the last roadmap

- **Parallel `cosine_table`** — rayon over batch rows; the mixture epoch that took seven minutes on one core is 2.4 s for 2,000 examples on two.
- **Generation** — `Fno::apply` on the context function is inside Phiano's `generate.rs` (`OperatorGuide`), measured (docs/71–75).
- **Möbius maps, the zeta prior, the sixty-four gates** — on the tape, tested, in the docs.
- **The PyTorch twin** — every op and gradient checked to 1e-16; a Colab.
- **This documentation site.**

## Also delivered (the impediments round)

- `Tape::cross_entropy` and `Fno::train_epoch_softmax`; `Adam::with_decay` (AdamW) with `state()` / `from_state`.
- `to_bytes` v2 (magic + version), `to_bytes_with` / `from_bytes_with` carrying the optimiser; v1 files still load.
- Slice-based `linear` / `mode_mix` forward and backward, chunked `apply`: 916 → 88 µs per function.
- `Module` trait, `Sequential`.
- `Fno::at_resolution` and the `phasor` module — P16 · P32 · P64 · P128 × Q16 · Q32 · Q64, every combination measured.

## Also delivered (the completion round)

- `Tape::fft2` / `ifft2` (the 2-D torus), `Schedule` (constant, warm-up + cosine, step) on `Adam`, `to_bytes_compact` (f32 storage), rustdoc published by the docs site (`npm run rustdoc`). 29 tests; 62 parity checks.

## Engine, next

1. **A real batched complex GEMM** for `linear` / `mode_mix` and tape allocation reuse — the remaining ~10×.
2. **Complex GELU alternatives** — modReLU / cardioid keep phase; measurable in an afternoon.
3. **f32 compute**, if a size ever demands it.

## In Phiano

- **Raise the operator's own ranking** — decoding from it is now implemented (`Decode::Operator`) and measured; its 16×-chance ranking is what limits the text it produces.
- **Persona sectors** balanced by channel instead of the centroid.
- **Phi-4 as a teacher**, once weights exist on a GPU machine: the torch twin makes distillation a plain torch loop.
- **WikiText-2 at the mixture objective, directed** — the run that can move the mixture.
- **Personas as tethered tables** — `Joint` has the mechanism; not yet used for personas.
