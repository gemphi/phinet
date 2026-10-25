---
id: phinet-tut-ranking
title: 2 · Ranking a Lexicon
description: A table of items, examples that name targets by index, decoys, and the ranking numbers — the way Phiano uses the engine.
sidebar_position: 2
---

# 2 · Ranking a Lexicon

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

In Phiano a word's function is its 64 relation channels, one complex phasor each (`encode_side`); a *context* is the evidence-weighted sum of the functions of the words before a position; the *target* is the next word. The engine never sees words, only a table of functions and indices into it.

## The table and the pool

```rust
// table[i] is item i's function; every function has length space
let table: Vec<Function> = /* one function per word, unit energy */;
// the pool: items frequent enough to be worth ranking against (Phiano: seen ≥ 20 times)
let pool: Vec<usize> = (0..table.len()).filter(|&i| count[i] >= 20).collect();
```

Chance MRR for a pool of size *n* is `(1/n) Σ_{k=1..n} 1/k` — every number in the docs is reported as a multiple of it.

## Examples

```rust
let examples: Vec<Example> = positions.iter()
    .filter(|p| pool_set.contains(&p.target))
    .map(|p| Example { input: context_function(p), target: p.target })
    .collect();
```

Only positions whose target is in the pool are kept, so the ranking task and the training task see the same candidate set.

## Train, early-stop, measure

```rust
let cfg = FnoConfig { space: 64, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3 };
let mut fno = Fno::new(cfg, 7);
let mut adam = Adam::new(cfg.lr);
let (mut best, mut best_bytes) = (0.0, fno.to_bytes());
for epoch in 0..4 {
    fno.train_epoch(&table, &train, &pool, 64, &mut adam, epoch);
    let (mrr, _) = rank(&table, &fno.apply(&valid_inputs), &valid, &pool);
    if mrr > best { best = mrr; best_bytes = fno.to_bytes(); }
}
let fno = Fno::from_bytes(&best_bytes).unwrap();
```

Early stopping is not optional: an 8,360-parameter operator on 100k examples overfits between epochs two and four while its training loss keeps falling.

## Reading the numbers

On Austen (three novels, pool 1,228, chance 0.0063) the identity reading — cosine of the context function against the table with no operator — is 7.9× chance, and the operator is 16.1×. On WikiText-2 (pool ≈ 2,200) the evidenced lexicon reads 6.7× as identity, 18.9× with the pointwise control, 27.9× with the operator. The operator's contribution is the gap between the last two columns, every time.

## Directed sides

Phiano's lexicon has two sides per channel — what a word does as a *source* (before) and as a *target* (after). `Sides::Directed` builds the context from source sides and ranks against target sides; it is the reading that wins (15.5× vs the one-sided reading on WikiText-2). The engine does not know about sides; Phiano chooses which functions go in the table.

## Next

[Tutorial 3](/docs/tutorials/mixture-objective): training on the language model's own likelihood instead of a hinge.
