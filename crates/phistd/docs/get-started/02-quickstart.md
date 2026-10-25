---
id: phinet-quickstart
title: Quickstart
description: The sixty-second example line by line — what a function is, what the operator learns, how it is measured.
sidebar_position: 2
---

# Quickstart

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

The README example, read slowly.

```rust
use phinet::{Fno, FnoConfig, Adam, Example, rank, unit_random};
```

**A function.** `unit_random(64, seed)` is a `Vec<Complex64>` of 64 samples with unit energy — a point on the unit sphere of ℂ⁶⁴, deterministic in its seed. Sixty of them are the *items*.

```rust
let inputs: Vec<_> = (0..60).map(|i| unit_random(64, i)).collect();
let targets: Vec<_> = inputs.iter().map(|f| { let mut g = f.clone(); g.rotate_right(1); g }).collect();
```

**A relation the identity cannot express.** Each item's target is itself rotated by one sample. In the Fourier domain that is multiplication of mode *m* by `e^{−2πi m/64}` — a diagonal, mode-dependent filter. A pointwise map (the same complex number at every sample) cannot do it; a spectral filter can, exactly.

```rust
let examples: Vec<Example> = (0..60).map(|i| Example { input: inputs[i].clone(), target: i }).collect();
let pool: Vec<usize> = (0..60).collect();
```

**Examples and a pool.** An `Example` is an input function and the *index* of its target in a table; the operator is trained to rank that target above decoys drawn from `pool`.

```rust
let cfg = FnoConfig { space: 64, width: 4, layers: 1, modes: 32, margin: 0.5, lr: 1e-2 };
let mut fno = Fno::new(cfg, 1);
```

**The operator.** Lift to width 4, one Fourier layer keeping all 32 modes each side of zero (so any circular convolution is representable), project back to width 1. Seed 1 fixes the initialisation.

```rust
let mut adam = Adam::new(cfg.lr);
for epoch in 0..60 {
    fno.train_epoch(&targets, &examples, &pool, 20, &mut adam, epoch);
}
```

**Training.** Each epoch: batches of 20, for each example one uniform decoy from the pool, the hinge `max(0, margin − cos(pred, target) + cos(pred, decoy))`, one Adam step per batch, the masked modes re-zeroed. The epoch number is the seed for decoy draws, so the run is reproducible bit for bit.

```rust
let (mrr, _) = rank(&targets, &fno.apply(&inputs), &examples, &pool);
assert!(mrr > 0.9);
```

**Measuring.** `rank` sorts the whole pool by cosine to each prediction and reports the mean reciprocal rank of the true target (and hit@10). Untrained the MRR is about 0.07; trained it is 1.0 — the [Colab](/docs/interop/colab) does the same run in torch and gets the same.

That is the whole shape of every use of Phinet: functions in a table, examples that name targets by index, an operator, an objective, a ranking. [Tutorial 1](/docs/tutorials/first-operator) takes it apart further; [Tutorial 2](/docs/tutorials/ranking-a-lexicon) does it on words.
