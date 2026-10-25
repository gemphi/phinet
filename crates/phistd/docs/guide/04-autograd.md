---
id: phinet-autograd
title: Autograd Mechanics
description: How the tape records and replays, what a complex gradient is here, the backward rule of every op, and how correctness is checked.
sidebar_position: 4
---

# Autograd Mechanics

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## The tape

`Tape` is a vector of nodes. Each node holds a value, an optional gradient, the op that produced it and its input `Var`s, and a `requires_grad` flag that is true for `param`s and for anything computed from one. Forward ops push a node and return its index as a `Var`. `backward(out)` zeroes every gradient, seeds `out` with 1, and walks the nodes in reverse, accumulating into inputs that require grad. There is no graph pruning, no retain/release: the tape is replayed as recorded, and `truncate(keep)` throws away everything after the parameters when a step is done (`Fno::reset`).

## The complex gradient

For a real loss `L` of a complex value `z = x + iy`, the gradient stored is

```
g = ∂L/∂x + i · ∂L/∂y
```

the direction of steepest ascent in the plane, which is `2 · ∂L/∂z̄` in Wirtinger terms — and exactly what `torch.autograd` returns as `.grad` for a complex leaf. Because every parameter update is a real-vector update on `(x, y)`, no Wirtinger calculus is needed anywhere: Adam runs on the pairs.

## Backward rules

With `g` the incoming gradient of the output:

| op | backward |
|---|---|
| `add(a, b)` | `g_a = g`, `g_b = g` |
| `add_bias(x, bias)` | `g_x = g`; `g_bias = Σ_{b,s} g` |
| `mul(a, b)` | `g_a = g · conj(b)`, `g_b = g · conj(a)` |
| `scale(a, k)` | `g_a = k · g` (real `k`) |
| `linear(x, w)` | `g_x = Σ_o g_o · conj(w_io)`; `g_w[i,o] = Σ_{b,s} g[b,o,s] · conj(x[b,i,s])` |
| `mode_mix(x, r)` | same as linear, per mode |
| `fft(x)` | `g_x = ifft(g)` — a unitary map's adjoint is its inverse |
| `ifft(x)` | `g_x = fft(g)` |
| `gelu(x)` | `g_x = gelu'(Re x) · Re g + i · gelu'(Im x) · Im g` |
| `unit(x)` | projection of `g` orthogonal to `x`, divided by `‖x‖` |
| `cosine(p, t)` | `g_p = Re(g) · (t/(‖p‖‖t‖) − cos · p/‖p‖²)`, symmetric for `t` |
| `cosine_table(p, T)` | the same, for every row pair; both sides receive gradient |
| `gather(T, idx)` | scatter-add `g` rows into `T` — the table can be a parameter |
| `hinge(pos, neg, m)` | `−1/b` into `pos`, `+1/b` into `neg` where the margin is violated |
| `mean_real(x)` | `1/n` into the real part of each element |
| `mobius(z, abcd)` | `∂w/∂z = (ad − bc)/(cz + d)²`; `∂w/∂a = z/(cz+d)`, `∂w/∂b = 1/(cz+d)`, `∂w/∂c = −z w/(cz+d)`, `∂w/∂d = −w/(cz+d)`, each conjugated into the convention |
| `mixture_nll(S, terms)` | `−coef·γ/p · ∂p_t/∂S` with the softmax Jacobian, times `β`, per row |

## How it is checked, twice

1. **Finite differences** in the crate: `test_gradients_match_finite_differences` pushes a small network through lift, GELU, FFT, mode mixing, IFFT, pointwise path, bias, add, scale, projection, cosine, hinge, gather, unit; `test_mixture_gradients_match_finite_differences` does `cosine_table` + `mixture_nll`; the Möbius test does `mobius` with respect to both arguments. Each analytic partial is compared with `(L(θ+ε) − L(θ−ε))/2ε` on real and imaginary parts.
2. **Against torch** on the reference dump: every gradient above, plus the nine parameter gradients of a two-layer operator and one Adam step, agree with `torch.autograd` to ≤ 1e-15 ([PyTorch parity](/docs/interop/pytorch-parity)).

## What the tape does not do

No higher-order gradients, no in-place ops, no checkpointing, no graph reuse across steps (the tape is rebuilt every batch), no gradient clipping. See [What is missing](/docs/about/what-is-missing).
