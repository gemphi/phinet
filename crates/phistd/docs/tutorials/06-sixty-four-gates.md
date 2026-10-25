---
id: phinet-tut-gates
title: 6 · The Sixty-Four Gates
description: I Ching hexagrams as spectral gates on the circle — apply one, read a function's lines, measure novelty, and see why they predict nothing.
sidebar_position: 6
---

# 6 · The Sixty-Four Gates

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

A function on a 64-sample circle has six octaves of spectrum: modes {1}, {2,3}, {4..7}, {8..15}, {16..31}, {32}. A hexagram is six lines. Line *j* yang passes octave *j*; yin drops it; mode 0 (the mean) always passes. Sixty-four gates, all exact under the unitary FFT.

```rust
use phinet::hexagram::{apply, gate, hexagram_of, lines, novelty, changing_lines};

let f = /* a word's target-side function, 64 samples */;
let h = apply(&f, 0b101010);          // ䷘ King Wen 25: keep octaves 1, 3, 5
let e = lines(&f);                    // energy per octave, sums to 1
let n = novelty(&f, 0b101010);        // 1 − cos(f, h)
let id = hexagram_of(&f);             // the Q64 sector of f's centroid: the hexagram f *is*
let walk = changing_lines(0b111111, 0b101010);   // which lines change: 0b010101
```

䷀ (111111) is the identity; ䷁ (000000) keeps only the mean; `apply` is idempotent.

## Novelty on demand

In Phiano's generator, `OperatorGuide.novelty = Some((id, every))` passes the operator's prediction through a named gate every *n*-th token. On Austen it raised the evaluator slightly (0.689 vs 0.669) and perplexity slightly (12.2 vs 11.6): novelty costs what it should.

## What they are not

Whether the operator's predicted hexagram matches the next word's is measurable (chance 1/64 = 1.6%). Measured on the Rust Book: operator 0.9%, identity 5.8%, previous word 3.6%, King Wen successor 1.1%, and "the most frequent hexagram, always" 12.8% — only the skew of centroid sectors. Sixty-four operations are a keyboard, not an oracle. The [3-D page](/docs/interop/three-dimensions) lets you toggle the six lines on a real word and watch the ring change.

## Next

[Tutorial 7](/docs/tutorials/spectral-priors): masks and the zeta prior.
