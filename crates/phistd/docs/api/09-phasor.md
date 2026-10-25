---
id: phinet-api-phasor
title: phasor
description: Resolution, Quantum, Range — the phasor ranges — and resample, quantise, sector_of, quantisation_loss, table.
sidebar_position: 9
badge: API
---

# `phinet::phasor`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

```rust
pub enum Resolution { P16 = 16, P32 = 32, P64 = 64, P128 = 128 }
pub enum Quantum    { Q16 = 16, Q32 = 32, Q64 = 64 }
pub struct Range { pub resolution: Resolution, pub quantum: Quantum }
```

| item | signature | notes |
|---|---|---|
| `Resolution::ALL` / `Quantum::ALL` / `Range::ALL` | consts | 4, 3, and their 12 products |
| `Range::DEFAULT` | `P64·Q64` | Phiano's |
| `Resolution::space`, `from_space`, `octaves` | | samples; parse; `log₂ space` |
| `Quantum::sectors`, `bound` | | sectors; `π / sectors` |
| `Quantum::sector(z)`, `sector_of_phase(φ)`, `phase(k)`, `snap(z)` | | the sector of a value / phase; a sector's centre; the snap (magnitude kept) |
| `Range::new`, `space`, `sectors`, `name` | | `name()` prints `P64·Q32` |
| `quantise` | `fn quantise(f: &[c64], q: Quantum) -> Function` | every sample snapped |
| `sector_of` | `fn sector_of(f: &[c64], q: Quantum) -> usize` | sector of the centroid; equals `hexagram_of` at Q64 |
| `quantisation_loss` | `fn quantisation_loss(f: &[c64], q: Quantum) -> f64` | `1 − cos(f, quantise(f))` |
| `resample` | `fn resample(f: &[c64], to: Resolution) -> Function` | spectral zero-pad / truncate; values interpolated; cosines unchanged |
| `table` | `fn table(f: &[c64]) -> Vec<(Range, usize, f64)>` | the twelve rows for one function |

Measured on every combination, and the operator moved between resolutions: [Phasor ranges](/docs/guide/phasor-ranges). Torch equivalents (`resample`, `quantise`) are in `phinet_torch.py` and checked.
