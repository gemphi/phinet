---
id: phinet-tut-mixture
title: 3 · The Mixture Objective
description: Train the operator on the mixed language model's own likelihood — cosine_table, mixture_nll, and what k, coef and other are.
sidebar_position: 3
---

# 3 · The Mixture Objective

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

A hinge teaches the operator to beat one decoy. A language model is judged on the probability it gives the true word among *all* words, mixed with other components. `train_epoch_mixture` trains on that.

## The reader, as a formula

Phiano's mixed model reads a position as

```
p(w) = k + coef · ( γ · p_op(w) + (1 − γ) · other(w) )
```

where `k` and `coef` come from the n-gram tables at that position, `other` is the unigram, and `p_op = softmax(β · cosine(Fno(ctx), table))[w]`. Everything except `p_op` is a constant of the position, so the operator gets gradient only through the mass the other components fail to place on the true word.

## Examples carry their constants

```rust
let ex = MixtureExample { input: ctx_fn, target: w, k: 0.012, coef: 0.81, other: 0.0004 };
let loss = fno.train_epoch_mixture(&table, &examples, /*gamma*/ 0.5, /*beta*/ 8.0, 64, &mut adam, epoch);
// perplexity of the phase term ≈ loss.exp()
```

On the tape this is `cosine_table` (every prediction against every table row, `[b,V,1]`, rayon over rows) followed by `mixture_nll`.

## What it bought

The same operator that read 149 under the hinge reads 134 under this loss on the Rust Book — what joint training of the whole lexicon had reached, without touching the lexicon. It costs a full-vocabulary softmax per example: on this machine 2,000 examples against a 2,000-item table take 2.4 s an epoch against 3.0 s for 10,000 hinge examples ([Benchmarks](/docs/measurements/benchmarks)).

## What it did not buy

The mixture with the n-gram tables stays at 125.5 on that corpus: the phase term carries little the tables do not, by construction. And frequency-proportional *decoys* for the hinge (`FrequencySampler`) made the LM term worse (149 → 154 → 175 as α rose) — ranking inside the frequent pool is not what a softmax rewards. Both refutations are in [Measurements](/docs/measurements/results).

## Next

[Tutorial 4](/docs/tutorials/joint-training): let the table move too.
