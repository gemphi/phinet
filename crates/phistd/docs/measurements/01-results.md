---
id: phinet-results
title: Results
description: Every held-out number Phinet has produced inside Phiano, with the arm that differs from it in exactly one thing.
sidebar_position: 1
badge: Results
---

# Results

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

All from Phiano (`docs/65`–`68`), one seed, held-out sentences, chance-normalised where a pool is ranked. Each number sits beside the arm that differs from it in exactly one thing.

## Next-word ranking (WikiText-2, 20k sentences, pool ≈ 2,200, chance MRR 0.0037)

| function space | identity (superposition) | pointwise control | Fourier operator |
|---|---|---|---|
| shipped v1 manifold (flat) | 2.6× | 2.8× | 11.6× |
| promoted lexicon (evidenced) | 6.7× | 18.9× | 27.9× |

## Perplexity of the phase term alone (Rust Book, 6,221 words; Kneser-Ney 132)

| phase term | alone |
|---|---|
| manifold coherence (existing scorer) | 177 |
| function-space identity | 156 |
| Fourier operator, hinge | 149 |
| operator + lexicon trained jointly | 134 |
| operator on the mixture likelihood | 134 |

The mixture with the n-gram tables stays at 125.5 on this corpus: the term carries little the tables do not, by construction. On a corpus the tables cannot cover the question is open.

## Recursive composition (704 composable words, 4 roles, chance MRR 0.0101)

| level | phase unbind | cleanup trained on 1..=3 |
|---|---|---|
| 1 | 19× | 66× |
| 2 | 10× | 52× |
| 3 | 5× | 45× |
| 4 | 2× | 3× |

## One operator, both objectives

| | LM valid MRR | recover L1 | recover L2 | recover L3 |
|---|---|---|---|---|
| specialists | 0.148 | 0.664 | 0.531 | 0.456 |
| one shared operator | 0.145 | 0.635 | 0.513 | 0.419 |

## Refuted along the way

- Frequency-proportional decoys do not help a mixed language model (149 → 154 → 175).
- A cleanup trained only on level 1 is worse than nothing at level 2 (0.031 vs 0.086): it snaps everything toward a plain word.
- Joint training with the pool as the only target set improves the pool and hurts the vocabulary (149 → 168).

## Since docs/68 (Phiano docs/69–75)

| what | number |
|---|---|
| directed reading, two-sided lexicon, WikiText-2 ranking | 15.5× chance (identity), 31.9× with the operator |
| perplexity harness, directed mixture | 142.8 vs one-sided 134.2 — the two-sided lexicon still trails on the tail |
| Austen (three novels, pool 1,228): identity 7.9×, operator 16.1× | `bin/austen` |
| composition with the trainer's own pair key, 4 fillers | 52/36/13/8× phase-only, 81/66/57/3× cleanup |
| next-hexagram prediction | 0.9% vs chance 1.6% — refuted |
| Phi-4 pieces vs words as units on the circle (Austen) | 8.0× vs 7.6× chance — a wash |
| PyTorch parity | 55 checks, max Δ 9.9e-16 |
