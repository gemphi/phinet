---
id: phinet-tensor-engine
title: Tensor Engine
description: Complex tensors, the tape, and the gradient convention that makes a differentiable FFT a one-line adjoint.
sidebar_position: 1
---

# Tensor Engine

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

`phinet::tensor` is the minimum machinery that lets something be *learned* across the samples of a function on a circle.

## Shape

Every `Tensor` is `[batch, width, space]`. `space` is the circle, sampled at a power of two. `width` is how many such functions are stacked as features. `batch` is how many are in flight.

## The tape

`Tape` records every operation so `backward` can replay it. Parameters are `tape.param(t)`; constants are `tape.constant(t)`; both return a `Var` handle.

| Op | Shape | Notes |
|---|---|---|
| `add`, `mul`, `scale` | elementwise | complex |
| `add_bias` | `[b,w,s] + [1,w,1]` | |
| `linear` | `[b,i,s] × [i,o,1] → [b,o,s]` | pointwise across width |
| `mode_mix` | `[b,i,m] × [i,o,m] → [b,o,m]` | one complex matrix per Fourier mode |
| `fft`, `ifft` | along `s` | unitary; adjoint = inverse |
| `gelu` | elementwise | on `Re` and `Im` separately |
| `gather` | `[V,1,s]` rows by index → `[b,1,s]` | scatter-add backward |
| `unit` | rows to unit norm | |
| `cosine` | `[b,1,s]` vs `[b,1,s]` → `[b,1,1]` | real, both sides differentiable |
| `cosine_table` | `[b,1,s]` vs `[V,1,s]` → `[b,V,1]` | every item at once |
| `hinge` | `[b,1,1]` pos/neg → scalar | ranking loss |
| `mixture_nll` | `[b,V,1]` scores → scalar | mixed-LM likelihood |
| `mean_real` | → scalar | |

## Gradients of a real loss with respect to complex values

The convention throughout is `g = ∂L/∂Re(z) + i·∂L/∂Im(z)` — the direction of steepest ascent in the plane. Under it:

- `y = x · w` gives `g_x = g_y · conj(w)`
- a unitary transform `y = U x` gives `g_x = U⁻¹ g_y`, so the FFT's backward is the inverse FFT
- a real non-linearity applied to `Re` and `Im` separately back-propagates through each part on its own

No Wirtinger calculus is needed because every parameter update is a real-vector update on `(Re, Im)`. `Adam` runs on those pairs.

## Correctness

`test_gradients_match_finite_differences` pushes a small network through every op — lift, GELU, FFT, mode mixing, IFFT, pointwise path, bias, add, scale, projection, cosine, hinge, gather, unit — and compares each analytic partial to a finite difference. `test_mixture_gradients_match_finite_differences` does the same for `cosine_table` and `mixture_nll`. Parseval's identity and a pure tone landing in one bin check the FFT.
