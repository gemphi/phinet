---
id: phinet-tut-priors
title: 7 · Spectral Priors
description: The mode mask, the Riemann-zeta weighting, and Möbius maps — three ways to say what a smooth operator is.
sidebar_position: 7
---

# 7 · Spectral Priors

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## The mask

`FnoConfig::modes` keeps that many modes on each side of zero frequency; the rest of each layer's `R` is zero at construction and re-zeroed by `mask_modes()` after every optimiser step (the training loops call it). Fewer modes = a smoother operator over the circle. `modes = 0` is the pointwise control. With `space = 64, width = 8, layers = 2` the operator has 8,360 parameters whatever `modes` is; the mask decides how many are alive.

## The zeta prior

```rust
let w = zeta_weights(cfg.modes, 2.0);   // mode m ≥ 1 weighted m^{−s}/ζ(s), mode 0 → 1
fno.scale_modes(&w);                     // after each step, as a projection
```

A soft version of the mask: high modes are shrunk rather than cut, with the Riemann zeta function as the normaliser (for `s ≤ 1` the un-normalised `m^{−s}` is used, since ζ diverges). It is the zeta function used as a spectral weighting and nothing deeper. Negative frequencies share their positive partner's weight.

## Möbius maps

```rust
let abcd = tape.param(Tensor::from_vec([1,1,4], vec![a, b, c, d]));
let y = tape.mobius(x, abcd);            // (a z + b)/(c z + d) on every sample
```

Rotation (`a = e^{iα}, d = 1`), dilation (`a = ρ`) and inversion (`b = d = 1, a = c = 0`) are special cases, so one op contains bind, unbind and an evidence change exactly, and the group composes by 2×2 matrix product. The gradient with respect to `abcd` is on the tape and checked. Phical (Paper 3) uses the same map on the Riemann sphere, where it is a rigid motion.

## Resolution, now a real thing

`Fno::at_resolution(space)` moves an operator to another power-of-two resolution: `R` is per signed frequency and the kept low frequencies exist at every resolution, so they are copied across and the rest stays zero. Trained at P32 on a rotation of one thirty-second of a turn, it does the same rotation at P64 and P128 with cosine 0.799 against 0.805 for an operator trained there natively — the measurement and the twelve phasor ranges are in [Phasor ranges](/docs/guide/phasor-ranges).
