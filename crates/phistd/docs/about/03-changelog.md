---
id: phinet-changelog
title: Changelog
description: The crate's history, commit by commit, newest first.
sidebar_position: 3
---

# Changelog

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

Phinet is pre-1.0 (`0.1.0`); the format of `to_bytes` and every signature may change between commits. Newest first.

### (next) — files under 200 lines
The crate broken into `tensor/` (13 files), `operator/` (9), `phasor/` (3), `joint/` (2) and `examples/torch_reference/extra.rs`; every source file under 200 lines; behaviour unchanged (29 tests, the reference dump byte-identical, 62 parity checks). `lib.rs` binds modules by `#[path]`.

### (previous) — the completion round
`Tape::fft2` / `ifft2` + `fft2_inplace` (2-D torus, unitary, backward = inverse, FD-checked, torch parity); `Schedule` {Constant, Cosine, Step} on `Adam` (`with_schedule`, `current_lr`, saved in `AdamState`); `Fno::to_bytes_compact` (format 3, f32 parameters; `from_bytes` reads 1/2/3); `web/rustdoc.mjs` and `npm run rustdoc`. 29 tests; 62 parity checks.

### (previous) — the impediments round
`Tape::cross_entropy` (+ FD test, torch parity); `Adam::with_decay` (AdamW), `AdamState`, `state()` / `from_state`; `Fno::to_bytes` format 2 with magic `PHINET` and version, `to_bytes_with` / `from_bytes_with` (optimiser state), legacy v1 files still load; slice-based `linear` / `mode_mix` forward and backward; `Fno::apply` in chunks of `APPLY_CHUNK = 256` (916 → 88 µs per function); `Fno::train_epoch_softmax`; `Module` trait and `Sequential`; `Fno::at_resolution`; the `phasor` module (`Resolution`, `Quantum`, `Range`, `resample`, `quantise`, `sector_of`, `quantisation_loss`, `table`); `examples/ranges.rs`, `examples/profile_apply.rs`. 26 tests; 60 parity checks.

### bea5720 — PyTorch parity
`examples/torch_reference.rs` dumps every op, gradient and one Adam step; `torch/phinet_torch.py` is the PyTorch side (55 checks, all pass, max Δ 9.9e-16); `torch/phinet_colab.ipynb`. `serde_json` as a dev-dependency. README section.

### 08fd280 — the sixty-four operations
`hexagram` module: I Ching hexagrams as spectral gates by octave (`gate`, `apply`), `hexagram_of`, `changing_lines`, `phase_of`, `novelty`, `lines`. Tests for identity/receptive gates, idempotence, octave coverage.

### 4ca1014 — parallel table, Möbius, zeta
`cosine_table` over rayon; `Tape::mobius` with a finite-difference check on both arguments; `zeta_weights` + `Fno::scale_modes` as a spectral prior.

### 90385d5 — the mixture objective
`Tape::cosine_table`, `Tape::mixture_nll`, `MixtureTerms`, `MixtureExample`, `Fno::train_epoch_mixture`; finite-difference test for both new ops.

### a344607 — decoy sampling
`Fno::train_epoch_with(sampler)`, `FrequencySampler` (draws ∝ weight^α).

### b78269d — the engine
`Tensor`, `Tape` (add, add_bias, mul, scale, linear, mode_mix, fft, ifft, gelu, unit, cosine, hinge, mean_real, gather), `Adam`, `Fno` with `train_epoch`, `to_bytes`/`from_bytes`, `rank`, `Joint` with the tether, `hrr`. Finite-difference gradient tests, Parseval and pure-tone FFT tests, the rotate-by-one operator test.

Also in this release of the docs: `examples/bench.rs`, this site (`docs/`, `web/`), the logo.
