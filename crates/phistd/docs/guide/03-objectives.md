---
id: phinet-objectives
title: Training Objectives
description: Ranking by hinge, choosing the decoys, and training on a mixed language model's own likelihood.
sidebar_position: 3
---

# Training Objectives

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

Three ways to train an operator, in the order they were needed.

## Hinge against a decoy

`Fno::train_epoch(table, examples, pool, batch, adam, seed)`: for each example, the operator's output is compared by cosine to its target and to one decoy drawn uniformly from `pool`, and `max(0, margin − cos_target + cos_decoy)` is minimised. It is a ranking objective — make the right item the nearest of the table — and it is cheap: one decoy, no full softmax.

## Choosing the decoys

`Fno::train_epoch_with(table, examples, &sampler, …)` takes any `Fn(target, seed) -> usize`. `FrequencySampler::new(items, weights, alpha)` draws in proportion to `weight^alpha`.

Measured in Phiano: frequency-proportional decoys made the operator *worse* as a language-model term (149 → 154 → 175 perplexity as `alpha` rose), because ranking inside the frequent pool is not what a full-vocabulary softmax rewards. The decoy is not where the mixture problem lived; see the next objective.

## The mixture likelihood

`Fno::train_epoch_mixture(table, examples, gamma, beta, batch, adam, seed)` trains the operator on the reader itself. For each position:

```
scores  = cosine(Fno(input), every item)          Tape::cosine_table
p_op    = softmax(beta · scores)[target]
p       = k + coef · (gamma · p_op + (1 − gamma) · other)
loss    = −log p                                   Tape::mixture_nll
```

`k`, `coef` and `other` are that position's constants — in Phiano, the trigram affine and the unigram — held fixed, so gradient reaches the operator only through the probability mass the other components fail to place on the true word. `MixtureExample { input, target, k, coef, other }` carries them.

Measured: the same operator that read 149 under the hinge reads 134 under this loss — what joint training of the whole lexicon had reached, now without touching the lexicon. Lowering `gamma` makes the operator specialise in what the unigram misses, and the mixture improves in the right direction by a small amount. It costs a full-vocabulary softmax per example: about twenty times the hinge.

## Early stopping

Every held-out number in Phiano's docs is early-stopped on a validation ranking or perplexity. Without it a 8,360-parameter operator on 100k examples overfits between epochs two and four while its training loss keeps falling.
