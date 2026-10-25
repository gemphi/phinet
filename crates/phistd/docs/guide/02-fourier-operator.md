---
id: phinet-fourier-operator
title: Fourier Neural Operator
description: The layer, why it is parameterised in the spectral domain, and what makes it resolution-free.
sidebar_position: 2
---

# Fourier Neural Operator

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

`phinet::operator::Fno` is the Fourier neural operator of Li et al. (2020) in its one-dimensional periodic form: a stack of layers

```
h ← σ( F⁻¹( R · F(h) )  +  W h + b )
```

`F` is the unitary FFT along the sample axis. `R` is a learned complex `width × width` matrix **per Fourier mode** — a filter in the spectral domain, equivalently a learned circular convolution with width-mixing. `W` is a pointwise linear map across the width. `σ` is GELU on real and imaginary parts. A lift takes the single input function to `width` features; a projection brings it back.

## Why a filter and not a matrix

A dense `space × space` matrix would also mix sample 3 with sample 40. But a *filter* is a statement about **neighbouring samples**, and keeping only the low `modes` on each side of zero frequency forces the operator to be smooth over the circle — the exceptions have to earn their high-frequency coefficients. That is the regulariser, and it is why the Fourier parameterisation generalises where a dense matrix memorises.

It also makes the operator resolution-free: a filter is defined per signed frequency, not per sample, so `Fno::at_resolution` carries a trained `R` to any power-of-two `space` by copying the kept frequencies. Measured: an operator trained at 32 samples does the same rotation at 64 and 128 as well as one trained there ([Phasor ranges](/docs/guide/phasor-ranges)).

## Configuration

```rust
FnoConfig {
    space: 64,   // samples on the circle (power of two)
    width: 8,    // features after the lift
    layers: 2,   // Fourier layers
    modes: 12,   // kept modes each side of zero; 0 = pointwise control
    margin: 0.2, // hinge margin on cosine
    lr: 2e-3,
}
```

`FnoConfig::for_space(n)` gives a sensible default. With `space = 64, width = 8, layers = 2` the operator has 8,360 parameters.

## The pointwise control

`modes = 0` removes the spectral path entirely. What remains cannot move information between samples, so the gap between it and the full operator is what the spectral filter is worth. In Phiano's measurements that gap was 2.8× → 11.6× chance on a flat function space, and 18.9× → 27.9× on an evidenced one.

## Applying and saving

```rust
let outputs: Vec<Function> = fno.apply(&inputs);   // off the tape
let bytes = fno.to_bytes();
let again = Fno::from_bytes(&bytes).unwrap();
```
