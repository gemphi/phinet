---
id: phinet-conventions
title: Numerical Conventions and Reproducibility
description: The FFT normalisation, GELU, epsilons, the seeds every constructor takes, and what "bit-for-bit" means here.
sidebar_position: 5
---

# Numerical Conventions and Reproducibility

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## Precision

Everything is `f64` / `Complex64`. There is no f32 path and no mixed precision.

## FFT

Radix-2 Cooley–Tukey, in place, length a power of two, **unitary**: both directions divide by `√N`, so `ifft(fft(x)) = x` and each is the adjoint of the other. The forward twiddle is `e^{−2πi/len}`. Equivalent to `torch.fft.fft(x, norm="ortho")`; NumPy's default and torch's default (`"backward"`) differ by a factor of `√N` on the forward and `1/√N` on the inverse. Parseval's identity and a pure tone landing in one bin are tested.

## GELU

The tanh approximation, `0.5·x·(1 + tanh(√(2/π)(x + 0.044715 x³)))`, applied to real and imaginary parts separately. It is not holomorphic; modReLU / cardioid alternatives are on the roadmap.

## Epsilons

| where | value |
|---|---|
| `cosine`, `cosine_table` denominators | `max(‖p‖‖t‖, 1e-12)` |
| `unit`, `normalise` | rows with energy < 1e-24 are left at zero |
| `mixture_nll` | `log(max(p, 1e-12))` |
| `Adam` | `eps = 1e-8`, `β₁ = 0.9`, `β₂ = 0.999`, bias-corrected |

## Seeds

There is no global RNG. `Fno::new(cfg, seed)`, `Tensor::randn(shape, scale, seed)`, `unit_random(space, seed)`, `unit_phase(space, seed)`, `hrr::role(space, seed)` and every `train_epoch*(…, seed)` take an explicit `u64`; decoys are drawn from `mix64` (splitmix64's finaliser) of the seed and the example index. Two runs with the same seeds produce identical bytes from `to_bytes()` on the same machine.

`rayon` is used only in `cosine_table`, where rows are independent and each row's reduction is sequential, so parallelism does not change results.

## Determinism across machines

`f64` arithmetic is IEEE and the code uses no fused multiply-add intrinsics, so results are expected to match across x86-64 and aarch64; this has been verified only on one Linux x86-64 machine and against torch's CPU kernels (which are not bit-identical but agree to 1e-15).
