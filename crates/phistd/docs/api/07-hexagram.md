---
id: phinet-api-hexagram
title: hexagram
description: The sixty-four operations — octave, gate, apply, changing_lines, hexagram_of, phase_of, novelty, lines.
sidebar_position: 7
badge: API
---

# `phinet::hexagram`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

```rust
pub const HEXAGRAMS: usize = 64;
pub const LINES: usize = 6;
```

| item | signature | notes |
|---|---|---|
| `octave` | `fn octave(mode: usize, space: usize) -> Option<usize>` | `None` for mode 0; `⌊log₂ m⌋` clamped to 5; `m > space/2` mirrors to `space − m` |
| `gate` | `fn gate(id: u8, space: usize) -> Vec<bool>` | mode `m` passes iff mode 0 or line `octave(m)` of `id` is set (bit `j` = line `j`, bottom first) |
| `apply` | `fn apply(f: &[c64], id: u8) -> Function` | `ifft(gate ⊙ fft(f))`; exact, idempotent; `63` is the identity |
| `changing_lines` | `fn changing_lines(id: u8, mask: u8) -> u8` | `id ^ mask` |
| `hexagram_of` | `fn hexagram_of(f: &[c64]) -> u8` | Q64 sector of the centroid phase `arg Σ f` |
| `phase_of` | `fn phase_of(id: u8) -> f64` | sector centre, `(id + 0.5)/64 · τ` |
| `novelty` | `fn novelty(f: &[c64], id: u8) -> f64` | `1 − cos(f, apply(f, id))` |
| `lines` | `fn lines(f: &[c64]) -> [f64; 6]` | energy per octave, normalised over the six (mode 0 excluded) |

Ids are the six-bit line pattern, not King Wen numbers; Phiano's `phinum::iching::KING_WEN_MAP` translates. [Tutorial 6](/docs/tutorials/sixty-four-gates) has the readings and the refuted prediction claim.
