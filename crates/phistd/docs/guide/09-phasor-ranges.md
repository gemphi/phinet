---
id: phinet-phasor-ranges
title: Phasor Ranges
description: P16 · P32 · P64 · P128 resolutions and Q16 · Q32 · Q64 quanta as one grouping — every computation at every combination, measured.
sidebar_position: 9
---

# Phasor Ranges

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

A framework does not offer one dtype; it offers the family and lets every operation run on any member. Phinet's family has two axes, both powers of two, and `phinet::phasor` is the grouping:

| axis | members | what it sets | where it came from |
|---|---|---|---|
| **Resolution** `P16 P32 P64 P128` | samples on the circle | `space`; octaves = log₂ (4, 5, 6, 7) | Phiano's Phinum16 / 32 / 64 engines |
| **Quantum** `Q16 Q32 Q64` | phase sectors | the bound a snap can move a phase: π/16, π/32, π/64 | Phical's Q32 and Q64 junctures |

A `Range` is one pair; `Range::ALL` is the twelve combinations and `Range::DEFAULT` is `P64·Q64`, Phiano's. Names print as `P64·Q32`.

## What takes a range

| function | what it does |
|---|---|
| `Quantum::sector(z)`, `sector_of_phase`, `phase(k)`, `snap(z)`, `bound()` | the sector a value is in, the centre of a sector, the snap, the worst-case error |
| `quantise(f, q)` | every sample's phase snapped to its sector centre, magnitude kept |
| `sector_of(f, q)` | the sector of the centroid — `hexagram_of` at any quantum (identical to it at Q64) |
| `quantisation_loss(f, q)` | `1 − cos(f, quantise(f, q))` |
| `resample(f, resolution)` | the function at another resolution by spectral zero-pad / truncation; values interpolated, a band-limited function exact, cosines unchanged |
| `table(f)` | all twelve (range, sector, loss) rows for one function |
| `Fno::at_resolution(space)` | the same operator at another resolution — `R` is per signed frequency and the kept low frequencies exist at every resolution, so they are copied and the rest stays zero |

## Every combination, measured

`cargo run --release --example ranges` — 200 unit-random functions at P64, resampled to each resolution:

| range | sectors | bound (rad) | mean quantisation loss | sector agrees with P64·Q64 |
|---|---|---|---|---|
| P16·Q16 / P32·Q16 / P64·Q16 / P128·Q16 | 16 | 0.196 | 0.0066 / 0.0064 / 0.0064 / 0.0064 | 100% |
| P16·Q32 / P32·Q32 / P64·Q32 / P128·Q32 | 32 | 0.098 | 0.0016 | 100% |
| P16·Q64 / P32·Q64 / P64·Q64 / P128·Q64 | 64 | 0.049 | 0.0004 | 100% |

Two facts to read off it. The cost of a quantum is a property of the quantum alone — a quarter of the loss for each doubling of the sectors — and does not depend on the resolution, because resampling is unitary on the kept modes. And the quanta nest: a Q64 sector maps to its Q32 and Q16 parent by integer division 100% of the time, so a lexicon stored at Q64 can be read at Q32 without re-quantising.

## The operator across resolutions

An operator trained at P32 on "rotate by one sample" (one thirty-second of a turn), moved with `at_resolution` and asked to do the *same rotation of the circle* at each resolution — two samples at P64, four at P128 — against an operator trained natively there:

| trained at | applied at | shift there | MRR identity | MRR transferred | cos(pred, target) transferred | MRR native | cos native |
|---|---|---|---|---|---|---|---|
| P32 | P16 | 1 sample | 0.035 | 1.000 | 0.709 | 1.000 | 0.942 |
| P32 | P32 | 1 sample | 0.083 | 1.000 | 0.798 | 1.000 | 0.798 |
| P32 | P64 | 2 samples | 0.083 | 1.000 | **0.799** | 1.000 | 0.805 |
| P32 | P128 | 4 samples | 0.083 | 1.000 | **0.799** | 1.000 | 0.804 |

Upward the transfer is exact to the third decimal: an operator learned at 32 samples reads at 64 and 128 as well as one trained there. Downward to P16 the requested map is a *different* rotation (one sixteenth of a turn is the smallest a 16-sample circle can express), and the cosine says so — 0.709 against 0.942 native — while the ranking still comes out 1.000, which is a reminder that MRR is lenient and cosine is the honest column. This is the resolution-freedom the earlier draft of these docs claimed without having; it is now implemented and this table is its measurement.

## In Phiano

Phiano's lexicon is P64·Q64 (`PHASE_CHANNELS = 64`, `Q64`). A Phinum32 engine is P32·Q32: `resample` the table, `at_resolution` the operator, read sectors at `Quantum::Q32` — every measurement in the docs can be re-run at any of the twelve ranges by changing one `Range`.
