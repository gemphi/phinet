---
id: phinet-what-is-missing
title: What Is Missing
description: Phinet and Phiano held against PyTorch and TensorFlow — engine by engine, and documentation surface by surface — with what is deliberate and what is simply not done.
sidebar_position: 1
badge: Honest
---

# What Is Missing

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

Two frameworks with thousands of contributors are the measuring stick the question asked for. This page is that comparison without flattery. "—" means not present; "deliberate" means the absence is a design decision of the paradigm, not a gap to fill.

## The engines

### Phinet against `torch` / `tf`

| area | PyTorch / TensorFlow | Phinet | status |
|---|---|---|---|
| dtypes / ranges | f16/bf16/f32/f64, int, bool, complex | complex128 compute; `f32` storage (`to_bytes_compact`, half the file, < 1e-10 drift); the *phasor ranges* P16–P128 × Q16–Q64 are the family every computation runs on | f32 *compute* is not offered — at these sizes f64 is not the wall-clock |
| tensor rank / broadcasting | any rank, NumPy broadcasting | rank 3 `[b,w,s]`, no broadcasting | deliberate for now: every op asserts its shape |
| views, strides, in-place | yes | no — every op allocates; `apply` runs in chunks so the working set stays small | partial |
| devices | CPU, CUDA, MPS, TPU, distributed | CPU | gap |
| kernels | BLAS/cuDNN, fused, vectorised | slice-based loops, `rayon` on the table | `apply` 88 µs per function (was 916); still ~10× off a BLAS matmul |
| autograd | dynamic graph, higher-order, checkpointing, hooks, anomaly mode | first-order tape, replayed per step | gap: no second derivatives, no hooks |
| complex autograd | conjugate Wirtinger | the same convention, checked to 1e-16 | parity |
| FFT | 1-D/2-D/n-D, real FFTs, any length | 1-D and 2-D torus (`fft2`), radix-2 powers of two, unitary | arbitrary lengths and real FFTs are not planned |
| layers | conv, attention, norm, dropout, embedding, RNNs, transformers | `linear` (pointwise), `mode_mix` (spectral), `gelu`, `gather`, `unit`, `mobius` | deliberate: a neural operator is these; attention is what the paradigm replaces with relations |
| losses | dozens | `hinge`, `cross_entropy`, `mixture_nll`, `cosine`, `mean_real` | present for what the paradigm needs |
| optimisers | SGD, Adam(W), RMSprop, LBFGS, schedulers | `Adam`, AdamW (`with_decay`), `Schedule` (constant, warm-up + cosine, step), state saved with the operator | SGD/RMSprop/LBFGS not offered; not needed |
| initialisation | Xavier, Kaiming, … | fixed-σ complex Gaussian per parameter | fine at these sizes |
| data loading | `DataLoader`, datasets, samplers, augmentation | `Vec<Example>`, `batch`, `FrequencySampler` | gap for anything beyond in-memory tables |
| saving | state_dict, checkpoints incl. optimiser, ONNX, TorchScript, SavedModel | `to_bytes` (magic + version 2), `to_bytes_with(adam)` with optimiser state, legacy files still load | no ONNX; not planned |
| module system | `nn.Module`, parameter registration, hooks | `Module` trait (`tape`, `parameters`, `forward`); `Fno` implements it; `Sequential` stacks two on one tape | no hooks |
| resolution freedom (FNO) | n/a | `Fno::at_resolution` copies `R` by signed frequency; P32→P64/P128 transfer reads 0.799 cosine vs 0.805 native | present, measured ([Phasor ranges](/docs/guide/phasor-ranges)) |
| mixed precision / quantisation | yes | — | gap |
| distributed / multi-GPU | yes | — | out of scope |
| compilation (`torch.compile`, XLA) | yes | — | out of scope |
| profiling, tensorboard | yes | `examples/bench.rs` | gap |
| testing | vast suites | 15 tests, every gradient finite-difference checked, plus 55 torch parity checks | small but complete for what exists |
| language bindings | Python-first, C++ | Rust; PyTorch twin in Python | the twin is a port, not a binding |

### Phiano against a language-model stack

| area | Hugging Face / a modern LM | Phiano | status |
|---|---|---|---|
| tokenizer | byte-level BPE, fast, trained | whitespace words; Phi-4's BPE ported and checked (`src/bpe.rs`) | present; measured to be a wash on the circle |
| representation | dense vectors | 64 relation channels, two sides, evidence counts | deliberate — the paradigm |
| sequence model | transformer | trigram tables + operator + relational score + learned log-linear scorer | the sampler is the ceiling (docs/75) |
| decoding | beam, nucleus, speculative | trigram sampler, beam with per-hypothesis guide, softmax over learned logits | decode from the operator directly: not written |
| training objective | next-token cross-entropy | ranking hinge, mixture likelihood, log-linear NLL for the scorer | deliberate, measured |
| evaluation | perplexity, benchmarks | perplexity vs Kneser-Ney, ranking × chance, attribution, KN judge for compositions | present, honest |
| scale | billions of parameters | 8,360-parameter operator, 17k-word lexicon | deliberate |
| serving | APIs, batching | `server` module (axum) | present, unmeasured |

## The documentation surface

What `pytorch.org/docs` and `tensorflow.org` have, and where this site now stands:

| surface | PyTorch | TensorFlow | Phinet (this site) | Phiano |
|---|---|---|---|---|
| Install / get started | yes | yes | [Install](/docs/get-started/install), [Quickstart](/docs/get-started/quickstart) | `docs/62_getting_started.md` |
| Migration / cheat sheet | "PyTorch for NumPy users" | "TF1→TF2" | [From PyTorch](/docs/get-started/from-pytorch) | — |
| Tutorials (graded) | yes | yes | [seven](/docs/tutorials/first-operator) | scattered across docs/01–75 |
| Concept notes (autograd mechanics, broadcasting, numerical precision, reproducibility, serialization, CUDA semantics) | yes | "Guide" | [Guide](/docs/guide/tensor-engine) — autograd, conventions, serialization, performance | partial (paper 1 §2–3) |
| API reference, every symbol | generated from docstrings | generated | [hand-written, every public item](/docs/api/every-operation) | `cargo doc` only |
| Examples gallery | yes | yes | two crate examples + the Colab | 20 `bin/` measurement programs |
| Benchmarks | separate site | yes | [one machine](/docs/measurements/benchmarks) | run times in each doc |
| Results / papers | model zoo | model garden | [Results](/docs/measurements/results) | papers 11–15 (PDF) |
| Interop | ONNX, TorchScript, JAX bridges | Keras, TFLite, TF.js | [PyTorch twin](/docs/interop/pytorch-parity), [Colab](/docs/interop/colab) | — |
| Visual explanations | some | playground | [3-D page](/docs/interop/three-dimensions) | LinkedIn figures |
| Changelog / release notes | yes | yes | [Changelog](/docs/about/changelog) | `CHANGELOG.md` |
| Contributing | yes | yes | [Contributing](/docs/about/contributing) | — |
| Glossary | — | yes | [Glossary](/docs/about/glossary) | — |
| FAQ | yes | yes | [FAQ](/docs/about/faq) | — |
| Roadmap | governance RFCs | RFCs | [Roadmap](/docs/about/roadmap) | `ROADMAP.md` |
| Brand assets | yes | yes | [Brand](/docs/about/brand) — the logo | favicon only |
| Versioned docs, search, i18n | yes | yes | — (PhiDoc has neither versioning nor search yet) | — |
| Generated API docs (rustdoc) published | n/a | n/a | `npm run rustdoc` → served at `/rustdoc/phinet/` | `cargo doc` only |

## What was done about it

Closed since this page was first written, each with a test and a torch parity check: `cross_entropy` and `train_epoch_softmax`; AdamW and saved optimiser state; a versioned `to_bytes`; slice-based `linear` / `mode_mix` and chunked `apply` (916 → 88 µs per function, hinge epoch 3.0 → 1.8 s); the `Module` trait and `Sequential`; `at_resolution` with the transfer measured; the phasor-range family. Sixty parity checks pass.

## Still open, in order

1. **A real batched complex GEMM** (the remaining ~10× on `apply`) and tape allocation reuse.
2. **f32 compute** — storage is f32-capable; the tape is not, and at these sizes it does not need to be.
3. **PhiDoc search and versioning** — a property of the docs engine, not of this crate.

## And in Phiano

Decoding from the operator is implemented (`Decode::{Operator, Union}`, `bin/decode_eval`) and measured: it lifts the composer off the trigram tables (perplexity 97 against 13.6 for stitched trigrams, with Austen's own sentences at 187) but the text is not yet hers — the ceiling has moved from the sampler to the operator's own ranking quality (16× chance), which is the next thing to raise. Persona sectors are now centred on the lexicon mean (Phiano docs/76).
