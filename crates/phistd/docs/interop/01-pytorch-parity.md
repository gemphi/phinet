---
id: phinet-pytorch-colab
title: PyTorch Parity and Colab
description: Every phinet operation implemented in PyTorch, checked against the Rust engine to floating-point precision, with a Colab notebook that trains the same operator.
sidebar_position: 1
badge: Interop
---

# PyTorch Parity and Colab

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations; this page shows the same computations in PyTorch and proves they are the same.

## What is checked

`phinet/torch/phinet_torch.py` implements every operation of [the catalogue](/docs/phinet/operations) as a torch function and the operator as an `nn.Module` with phinet's exact parameter layout. `phinet/examples/torch_reference.rs` runs the Rust engine on fixed inputs and writes every intermediate, every gradient, and one Adam step to `torch/reference.json`. The Python side replays the same tape with `torch.autograd` and compares.

```
cargo run --release --example torch_reference > torch/reference.json
python torch/phinet_torch.py torch/reference.json
```

Result (torch 2.14, complex128), 62 checks:

| group | checks | max |Δ| |
|---|---|---|
| FFT / iFFT, unitary both ways | 4 | 3.3e-16 |
| linear, add_bias, gelu, mode_mix, unit, proj | 6 | 3.5e-16 |
| cosine, cosine_table, hinge | 4 | 1.7e-16 |
| gradients of the tape (∂/∂lift, ∂/∂R, ∂/∂proj, ∂/∂bias, ∂/∂x) | 5 | 0.0 |
| mixture_nll and its gradient | 2 | 2.2e-16 |
| Möbius map, mean, gradient w.r.t. `abcd` | 3 | 1.1e-16 |
| bind / unbind | 2 | 0.0 |
| hexagram gate, apply, hexagram_of, lines, novelty, changing_lines | 6 | 2.2e-16 |
| zeta_weights, cosine | 2 | 8.3e-17 |
| `Fno::apply` on a 2-layer operator (321 parameters) | 2 | 4.2e-17 |
| one training step: loss, nine parameter gradients | 10 | 9.9e-16 |
| one Adam step on all nine parameters, then `mask_modes` | 9 | 1.1e-16 |
| `cross_entropy` and its gradient; one AdamW step | 3 | 2.8e-17 |
| `resample` P16→P32, `quantise` Q16 | 2 | 1.7e-16 |
| `fft2` on a 4×4 torus and its gradient | 2 | 1.1e-16 |

**ALL PASS.** The two engines are the same arithmetic. The one place the first draft disagreed — `lines()` normalising by total energy instead of octave energy — was a documentation bug on the Python side and is fixed; the Rust definition (mode 0 has no line) stands.

## Conventions that had to match

- The FFT is unitary in both directions (`norm="ortho"`); torch's default (`"backward"`) would put a factor of `N` between the two engines.
- GELU is the tanh approximation applied to Re and Im separately, not `torch.nn.GELU` on the complex tensor (which is undefined).
- The complex gradient is torch's conjugate-Wirtinger derivative; phinet's `Mul` backward is `g · conj(b)`, the same choice.
- Adam treats a complex parameter as two real ones; `torch.optim.Adam` does the same through the real view, so one step agrees to 1e-16.

## Colab

`phinet/torch/phinet_colab.ipynb` runs the 55 checks, then trains the README's sixty-second example in torch — sixty items on a 64-point circle whose target is the item rotated by one sample — from MRR 0.073 untrained to **1.000** in 60 epochs (the Rust README asserts > 0.9 on the same task), and reads a function through four hexagram gates. Open it in Colab, upload `phinet_torch.py` and `reference.json` (or set `PHINET_RAW` to your checkout's raw URL), and run all.

## What is not in torch

The lexicon side — two-sided phasors, the relation vocabulary, the pair key, `relational_score`, personas — is Phiano, not Phinet, and is not ported. What Phiano hands Phinet is already a function on the circle; everything from there is on this page.
