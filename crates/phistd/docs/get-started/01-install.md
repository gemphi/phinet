---
id: phinet-install
title: Install
description: Add the crate, build, run the tests, and run the PyTorch twin.
sidebar_position: 1
---

# Install

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## The crate

Phinet is a path dependency inside the `gemphi` workspace; it is not on crates.io yet.

```toml
# Cargo.toml
[dependencies]
phinet = { path = "../phinet" }
```

Rust 2021, stable toolchain. Dependencies: `num-complex` (the numbers), `rayon` (one parallel loop), `serde` + `bincode` (saving an operator). No unsafe code, no C, no GPU.

```
git clone https://github.com/gemphi/phinet
cd phinet
cargo build --release
cargo test --release          # 15 tests, every gradient finite-difference checked
cargo run --release --example bench
```

## The PyTorch twin

`torch/phinet_torch.py` needs only `torch` (2.x, CPU is fine; complex128 is used throughout).

```
pip install torch
cargo run --release --example torch_reference > torch/reference.json
python torch/phinet_torch.py torch/reference.json      # 55 checks, ALL PASS
```

## The documentation site

This site is a [PhiDoc](/docs/phidoc/quickstart) site: the Markdown lives in `phinet/docs/`, the site in `phinet/web/`.

```
cd phinet/web
npm install
npm run dev          # http://127.0.0.1:5183
npm run rustdoc      # cargo doc → public/rustdoc, served at /rustdoc/phinet/
```

## Requirements that are really requirements

- `space` must be a power of two: the FFT is radix-2.
- Functions passed to `Fno::apply` must have length `space` exactly; `stack` asserts it.
- A `Tape` grows with every op until `truncate` or `Fno::reset`; the training loops do this for you, a hand-written loop must too.
