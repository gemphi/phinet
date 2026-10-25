---
id: phinet-holographic-binding
title: Holographic Binding
description: Bind, unbind and compose on the circle, and the operator as the learned cleanup that makes nesting recoverable.
sidebar_position: 8
---

# Holographic Binding

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

`phinet::hrr` is holographic reduced representation in the Fourier domain: a role is a unit-magnitude function, binding is pointwise multiplication (phase addition on every sample), unbinding is multiplication by the conjugate, and a composite is the normalised sum of bound fillers.

```rust
use phinet::hrr::{role, bind, unbind, compose};

let r1 = role(64, 1);
let identity = compose(&[bind(&a, &r1), bind(&b, &role(64, 2))]);
let a_again = unbind(&identity, &r1);   // ≈ a, plus b's crosstalk
```

## Why nesting fails on its own

Each unbind leaves every other filler's crosstalk in the result. One level is fine; an identity whose fillers are themselves identities is mostly noise after two unbinds. Phiano measured phase-only recovery decaying 29× → 8× → 5× chance across three levels.

## The operator as cleanup

Holographic memories fix this with a cleanup after each unbind. Here the cleanup is a trained `Fno`: `(unbind(identity, role) → clean filler)` pairs, so it learns to remove crosstalk the way it is actually shaped on the circle.

Measured in Phiano: a cleanup trained on levels 1–3 recovers level 1 at 66× chance, level 2 at 52×, level 3 at 45× — and level 4, which it never saw, at 3×. The rule is exact: the operator recovers every depth it was trained on and is at chance one deeper. Depth is a training decision, not a representational limit.

One operator can carry both this and a language-model objective within a few percent of the two specialists.
