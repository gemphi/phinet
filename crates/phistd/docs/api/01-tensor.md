---
id: phinet-api-tensor
title: tensor::Tensor
description: The dense complex tensor — construction, indexing, and the deterministic Gaussian initialiser.
sidebar_position: 1
badge: API
---

# `phinet::tensor::Tensor`

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    pub shape: [usize; 3],   // [batch, width, space]
    pub data: Vec<c64>,      // row-major: index = (b * width + w) * space + s
}
```

| item | signature | notes |
|---|---|---|
| `Tensor::zeros` | `fn zeros(shape: [usize; 3]) -> Tensor` | |
| `Tensor::from_vec` | `fn from_vec(shape: [usize; 3], data: Vec<c64>) -> Tensor` | asserts `data.len() == b·w·s` |
| `Tensor::randn` | `fn randn(shape: [usize; 3], scale: f64, seed: u64) -> Tensor` | complex Gaussian (Box–Muller on a xorshift stream seeded by `seed`), times `scale`; deterministic |
| `len` / `is_empty` | `fn len(&self) -> usize` | number of complex elements |
| `get` | `fn get(&self, b: usize, w: usize, s: usize) -> c64` | bounds-checked |
| `set` | `fn set(&mut self, b: usize, w: usize, s: usize, v: c64)` | |
| `norm_sq` | `fn norm_sq(&self) -> f64` | squared Frobenius norm |

Free function in the same module:

| item | signature | notes |
|---|---|---|
| `fft_inplace` | `fn fft_inplace(x: &mut [c64], inverse: bool)` | radix-2, length a power of two (asserted), unitary both ways |

`c64` is `num_complex::Complex64`, re-exported at the crate root. `Function` is `Vec<c64>`.

## Example

```rust
use phinet::{c64, Tensor};
let mut t = Tensor::zeros([2, 1, 8]);
t.set(1, 0, 3, c64::new(0.5, -0.5));
assert_eq!(t.get(1, 0, 3).im, -0.5);
let g = Tensor::randn([4, 4, 64], 0.25, 42);   // same numbers every run
```
