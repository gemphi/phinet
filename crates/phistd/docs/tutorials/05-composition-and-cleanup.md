---
id: phinet-tut-composition
title: 5 · Composition and Cleanup
description: Bind fillers to roles, superpose them, unbind — and train an operator to remove the crosstalk that nesting leaves behind.
sidebar_position: 5
---

# 5 · Composition and Cleanup

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## Bind, unbind, compose

```rust
use phinet::hrr::{role, bind, unbind, compose};

let r_agent = role(64, 1);        // unit-magnitude random phases
let r_theme = role(64, 2);
let sentence = compose(&[bind(&dog, &r_agent), bind(&ball, &r_theme)]);
let dog_again = unbind(&sentence, &r_agent);   // ≈ dog, plus ball's crosstalk
```

Binding is pointwise multiplication — phase addition on every sample — which is circular convolution in the Fourier domain, i.e. Plate's holographic reduced representations with the transform already taken. `compose` sums and normalises.

## Why depth fails

Every unbind leaves every *other* filler's crosstalk in the result. One level is fine; an identity whose fillers are themselves identities is mostly noise after two unbinds. Phiano measured phase-only recovery at 59× / 19× / 8× / 3× chance across four levels with the offset key.

## The operator as cleanup

Holographic memories fix this with a cleanup after each unbind. Here the cleanup is a trained `Fno` on `(unbind(identity, role) → clean filler)` pairs:

```rust
let examples: Vec<Example> = identities.iter().flat_map(|(id, fillers)| {
    fillers.iter().map(|(role_k, filler_idx)| Example { input: unbind(id, &role(64, *role_k as u64)), target: *filler_idx })
}).collect();
fno.train_epoch(&table, &examples, &pool, 64, &mut adam, epoch);
```

Measured in Phiano with the trainer's own pair key (rarest marker between two words, else their offset; four best-evidenced fillers per word): 81× / 66× / 57× / 3× after cleanup trained on levels 1–3. The rule is exact — the operator recovers every depth it was trained on and is at chance one deeper. Depth is a training decision, not a representational limit.

One caution measured too: a cleanup trained only on level 1 is *worse than nothing* at level 2 (0.031 vs 0.086) — it snaps everything toward a plain word.

## Next

[Tutorial 6](/docs/tutorials/sixty-four-gates): the hexagrams as exact spectral operations.
