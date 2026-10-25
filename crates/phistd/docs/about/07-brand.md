---
id: phinet-brand
title: Brand
description: The Phinet mark — what it draws, the files, the colours, and how it sits beside the other phi* marks.
sidebar_position: 7
---

# Brand

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

![Phinet mark](/logo.svg)

![phinet wordmark](/logo-wordmark.svg)

## What it draws

Every phi* mark in the workspace shares one glyph: a vertical stroke and a U — the φ of Phi, drawn the way `phient`, `phidoc` and `phiace` draw it — with a motif inside that says what the project does. Phinet's motif is a **spectrum**: three modes, the low one strongest, in the second colour; the full mark adds the **circle** every function lives on, as a gradient from the first colour to the second.

## Files (`phinet/web/public/`)

| file | use |
|---|---|
| `favicon.svg` | 48-unit icon, browser tab; the same construction as the other phi* favicons |
| `logo.svg` | 128-unit mark with the circle; docs, cards, avatars |
| `logo-wordmark.svg` | mark + "phinet" + tagline on light |
| `logo-wordmark-dark.svg` | the same on dark |

Hand-written SVG, no fonts embedded (the wordmark uses the system sans stack), no raster.

## Colours

The puijs brand `phinet` (`puijs/src/providers/themes.ts`):

| token | value |
|---|---|
| primary | `#f97316` (hover `#ea580c`) |
| secondary | `#0891b2` (hover `#0e7490`) |
| gradient | `linear-gradient(135deg, #f97316 0%, #06b6d4 100%)` |
| gradient, dark | `linear-gradient(135deg, #f97316 0%, #164e63 100%)` |

Orange for the samples, cyan for the spectrum — the two colours the 3-D page already uses for source and target sides. No other phi* brand uses orange, so the mark reads as Phinet's at a glance in the hub.

## Beside the family

![Phient](/phient-favicon.svg) ![PhiDoc](/phidoc-favicon.svg) ![Phinet](/favicon.svg)

Same stroke, same U, same weight; only the motif and colour differ.
