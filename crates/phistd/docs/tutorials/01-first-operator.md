---
id: phinet-tut-first-operator
title: 1 · Your First Operator
description: Build the operator by hand on the tape, watch the gradient flow, and see why the pointwise control cannot learn a rotation.
sidebar_position: 1
---

# 1 · Your First Operator

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

The quickstart used `Fno`. This tutorial builds the same layer by hand so that nothing is hidden.

## One layer on the tape

```rust
use phinet::{c64, Tape, Tensor, unit_random};

let space = 64;
let x = unit_random(space, 7);
let mut tape = Tape::new();

// input as a constant [1,1,64]; parameters as params
let xv = tape.constant(Tensor::from_vec([1, 1, space], x.clone()));
let lift = tape.param(Tensor::randn([1, 4, 1], 1.0, 1));          // width 1 → 4
let r    = tape.param(Tensor::randn([4, 4, space], 0.25, 2));     // one 4×4 per mode
let w    = tape.param(Tensor::randn([4, 4, 1], 0.5, 3));          // pointwise path
let b    = tape.param(Tensor::zeros([1, 4, 1]));
let proj = tape.param(Tensor::randn([4, 1, 1], 0.5, 4));          // width 4 → 1

let h  = tape.gelu(tape.linear(xv, lift));                        // [1,4,64]
let hf = tape.fft(h);                                             // spectrum
let hm = tape.mode_mix(hf, r);                                    // filter per mode
let hs = tape.ifft(hm);                                           // back to samples
let hp = tape.add_bias(tape.linear(h, w), b);                     // pointwise path
let y  = tape.linear(tape.add(hs, hp), proj);                     // [1,1,64]
```

That is `h ← F⁻¹(R·F h) + W h + b` — the Fourier layer of Li et al. (2020) in its one-dimensional periodic form, with lift and projection around it. `Fno::forward` does exactly this, GELU between layers and none after the last.

## A loss and its gradient

```rust
let target = { let mut g = x.clone(); g.rotate_right(1); g };
let tv  = tape.constant(Tensor::from_vec([1, 1, space], target));
let cos = tape.cosine(y, tv);              // [1,1,1], real
let loss = tape.scale(cos, -1.0);          // maximise cosine
tape.backward(loss);
let g_r = tape.grad(r).unwrap();           // [4,4,64] complex
```

`tape.grad` returns `∂L/∂Re + i ∂L/∂Im` for every parameter. The [Guide](/docs/guide/autograd) explains why that is the right object and how each op's backward is derived; the crate's tests check every one against finite differences.

## Why the pointwise control fails

Set `modes = 0` in `FnoConfig` and the spectral path is removed: what remains is `W h + b` per sample, which cannot move information from sample *k* to sample *k+1*. Train both on the rotate-by-one task from the quickstart:

| operator | MRR after 60 epochs |
|---|---|
| pointwise control (`modes: 0`) | ≈ chance (0.07) |
| spectral (`modes: 32`) | 1.00 |

That gap — a relation between neighbouring samples that no per-sample map can express — is what the spectral parameterisation is for, and it is the same gap Phiano measured on words: 2.8× → 11.6× chance on a flat function space, 18.9× → 27.9× on an evidenced one ([Measurements](/docs/measurements/results)).

## Next

[Tutorial 2](/docs/tutorials/ranking-a-lexicon): the same loop on a table of words.
