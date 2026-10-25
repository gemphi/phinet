---
id: phinet-api-hrr
title: hrr
description: Holographic binding on the circle — role, bind, unbind, compose.
sidebar_position: 6
badge: API
---

# `phinet::hrr`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

| item | signature | mathematics |
|---|---|---|
| `role` | `fn role(space: usize, seed: u64) -> Function` | unit-magnitude random phases (`unit_phase`) |
| `bind` | `fn bind(f: &[c64], r: &[c64]) -> Function` | `f · r` pointwise — phase addition |
| `unbind` | `fn unbind(f: &[c64], r: &[c64]) -> Function` | `f · conj(r)` |
| `compose` | `fn compose(parts: &[Function]) -> Function` | normalised sum; a zero sum stays zero |

Because a role has unit magnitude everywhere, `unbind(bind(f, r), r) == f` exactly; crosstalk comes only from the *other* fillers in a composite. Cleanup and the measurements: [Tutorial 5](/docs/tutorials/composition-and-cleanup), [Guide](/docs/guide/holographic-binding).
