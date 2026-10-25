---
id: phinet-serialization
title: Serialization
description: What Fno::to_bytes writes, how to load it, and how to move an operator to and from PyTorch.
sidebar_position: 6
---

# Serialization

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## Format (version 2)

`Fno::to_bytes()` writes the six bytes `PHINET`, a little-endian `u16` format version (`2`), then `bincode` 1.x of a struct holding the `FnoConfig`, the parameters in order — `lift, lift_b, (r, w, b) × layers, proj` — each as `([usize; 3], Vec<(f64, f64)>)` (shape, then `(re, im)` pairs), and an `Option<AdamState>`. `to_bytes()` leaves the option empty; `to_bytes_with(&adam)` fills it with the optimiser's moments, step count and hyper-parameters, so a run resumes exactly.

`Fno::from_bytes` reads version 2, and — when the bytes carry no header — the pre-version format (plain bincode of config + params), so every `.fno` in Phiano's `data/` still loads. An unknown version returns `None` rather than mis-loading. `from_bytes_with` also returns the optimiser when one was saved.

```rust
std::fs::write("operator.fno", fno.to_bytes_with(&adam))?;
let (again, adam) = Fno::from_bytes_with(&std::fs::read("operator.fno")?).ok_or("bad file")?;
```

Format **3** (`to_bytes_compact`) stores the parameters as `f32` pairs and no optimiser state: half the size, and a round trip moves `apply`'s output by less than 1e-10 in cosine (tested). `from_bytes` reads 1, 2 and 3.

Size: 16 bytes per complex parameter plus headers — 134,113 bytes for the standard 8,360-parameter operator without optimiser state, roughly three times that with it.

## Phiano's files

`data/phinet_generate.fno`, `data/austen_operator.fno` and the others in Phiano's `data/` are exactly this format; `OperatorGuide::load` reads them.

## To and from PyTorch

The torch twin's `Fno` has the same parameter names and shapes. `Fno.load_phinet(params)` takes the JSON layout written by `examples/torch_reference.rs` (`{"shape": […], "data": [[re, im], …]}` per parameter). Going the other way — a torch-trained operator into `.fno` — is a small serialiser that has not been written; the layout above is all it needs.

## Functions and tables

Functions are plain `Vec<Complex64>`; Phiano stores its lexicon in its own `.chroma` format and builds the table with `FunctionSpace`. The engine has no table format of its own.
