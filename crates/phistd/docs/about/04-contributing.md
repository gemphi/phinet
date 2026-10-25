---
id: phinet-contributing
title: Contributing
description: How a change gets in — the two checks every op must pass, how docs are written, and the rules the project holds itself to.
sidebar_position: 4
---

# Contributing

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## The two checks

Every tape op ships with **both**:

1. a finite-difference test in the crate (`cargo test --release`) — add the op to `test_gradients_match_finite_differences` or give it its own, checking real and imaginary partials;
2. a line in `torch/phinet_torch.py` and a field in `examples/torch_reference.rs`, so the PyTorch twin covers it (`python torch/phinet_torch.py torch/reference.json` must print `ALL PASS`).

A change to a convention (FFT scaling, GELU, epsilons) must update [Numerical conventions](/docs/guide/numerical-conventions) and the twin in the same commit.

## The rules

- **Every source file under 200 lines, one concern per file.** `tensor/` is the tensor, the FFTs, the tape, three files of forward ops, three of backward, the optimiser, and one test file per topic; `operator/` is the operator, its masks, its training epochs, its save format, the module trait, the samplers; `phasor/` is the ranges and their functions. A new op goes in the file of its kind, with its backward in the matching backward file and its test in the matching test file. Modules are bound by `#[path]` in `lib.rs`, so a stale flat file from an older checkout is never compiled.

- **Honest measurements only.** A number in the docs is one that was run, with the arm that differs from it in exactly one thing beside it. A refuted idea stays in the docs as refuted.
- **State the paradigm first.** Every page opens with it.
- **No hidden constants.** A tunable is a field of a config or an argument; if it has to be a constant, it is named and its value is justified where it is defined.
- **No dependencies without a reason.** The crate has four; each carries its weight.

## Docs

The site is [PhiDoc](/docs/phidoc/quickstart): Markdown under `phinet/docs/`, categories from folders with `_category_.json`, order from number prefixes, frontmatter for `id`, `title`, `description`, `sidebar_position`, `badge`. The same files are mirrored into the workspace hub at `phidoc/docs/phinet/`. Links are absolute (`/docs/…`). Run `npm run dev` in `phinet/web/`.

## Commit messages

One line of what, a body of why and what was measured. Co-authorship lines are welcome when a machine helped.

## Licence

MIT.
