---
id: phinet-tut-joint
title: 4 · Joint Training
description: The table of items as a parameter on the operator's tape, and the tether that keeps it from memorising.
sidebar_position: 4
---

# 4 · Joint Training

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

In `operator`, the table is constant: the operator learns to map a fixed input to a fixed target. But `cosine` back-propagates into both of its arguments and `gather` scatter-adds into a table, so nothing stops the table from being a parameter. `joint::Joint` does that.

## How the context is built

An `IndexExample { ctx: Vec<(item, weight)>, target }` names its context by index. `Joint` gathers those rows from the table *on the tape*, weights and sums them, and normalises — so an item receives gradient both as a target (be findable after this context) and as context (make what follows findable).

```rust
let mut joint = Joint::new(fno, &table, window).with_tether(0.02);
let mut adam_op = Adam::new(lr);
let mut adam_table = Adam::new(lr * 0.25);
for e in 0..epochs {
    joint.train_epoch(&examples, &pool, batch, &mut adam_op, &mut adam_table, e);
}
let learned: Vec<Function> = joint.functions();
```

## The tether

With `V × space` complex numbers in the table against a few thousand in the operator, the table memorises the training contexts if left free. `with_tether(λ)` pulls every row back toward its initial value by a fraction `λ` after each step — a proximal term, so the operator can reshape an item only as far as the objective keeps paying for it.

## Two things measured in Phiano

- Free, the table lowered training loss 0.065 → 0.043 and *raised* held-out perplexity 149 → 157. Tethered at 0.02 with the table's Adam at a quarter of the operator's rate, held-out went to 134.
- Every item has to be a target. Training targets from the frequent pool alone improved ranking inside the pool and made perplexity over the whole vocabulary worse (149 → 168), because the items outside the pool moved only as context and drifted into the softmax's way.

## Next

[Tutorial 5](/docs/tutorials/composition-and-cleanup): binding functions into functions, and the operator as cleanup.
