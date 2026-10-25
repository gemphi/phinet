---
id: phinet-api-joint
title: joint::Joint
description: The operator plus a trainable table on one tape — IndexExample, construction, the tether, an epoch, and reading the learned functions back.
sidebar_position: 5
badge: API
---

# `phinet::joint`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

```rust
pub struct IndexExample { pub ctx: Vec<(usize, f64)>, pub target: usize }   // (item, weight) pairs, oldest first
pub struct Joint { pub fno: Fno, pub table: Var, pub tether: f64, /* pad, window, init */ }
```

| item | signature | notes |
|---|---|---|
| `new` | `fn new(fno: Fno, table: &[Function], window: usize) -> Joint` | attaches `[V+1, 1, s]` to the operator's tape; row `V` is the zero padding row |
| `with_tether` | `fn with_tether(self, tether: f64) -> Joint` | proximal pull toward the initial table after every step (0 = free) |
| `train_epoch` | `fn train_epoch(&mut self, examples: &[IndexExample], pool: &[usize], batch: usize, adam_op: &mut Adam, adam_table: &mut Adam, seed: u64) -> f64` | hinge vs a uniform decoy; two optimisers; returns mean loss |
| `functions` | `fn functions(&self) -> Vec<Function>` | the learned table, without the padding row |

## How a context is built

For `j` in `0..window`, the `j`-th most recent context item of every example in the batch is gathered from the table (padding row when the context is shorter), scaled by that position's stored weight, summed, and normalised — all on the tape, so gradient reaches the table both through targets and through contexts.

After each step: `adam_op` on the operator, `adam_table` on the table, `mask_modes`, then `settle`: the padding row is re-zeroed and every row moves a fraction `tether` back toward its initial value.

## Example

```rust
let mut joint = Joint::new(Fno::new(cfg, 1), &table, 6).with_tether(0.02);
let (mut a_op, mut a_tab) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.25));
for e in 0..4 { joint.train_epoch(&examples, &pool, 64, &mut a_op, &mut a_tab, e); }
let learned = joint.functions();
```

Measured behaviour (why the tether and the quarter rate): [Tutorial 4](/docs/tutorials/joint-training).
