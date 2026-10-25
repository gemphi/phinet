---
id: phinet-faq
title: FAQ
description: The questions that come up, with the short answers and where the long ones are.
sidebar_position: 6
---

# FAQ

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

**Is this a PyTorch replacement?** No. It is a small, exact engine for one kind of model — neural operators on a closed circle — with a PyTorch twin so that anything it does can be reproduced in torch. [What is missing](/docs/about/what-is-missing) is the full comparison.

**Why complex numbers?** Because a word in Phient's world is a set of phasors — phase *is* the relation — and the FFT of a real function is redundant. Everything is `Complex64`; there is no real path.

**Why a unitary FFT?** So that the forward transform's adjoint is the inverse, which makes the backward pass one line and keeps every spectral weight on the same scale as the samples. `torch.fft.fft(norm="ortho")` is the same choice.

**Can I use a length that is not a power of two?** No; the FFT is radix-2 and `stack` asserts. Pad or resample outside the engine. Two-dimensional functions are fine: `fft2` reads the sample axis as an `s1 × s2` torus.

**Does the same operator apply at 32 and 128 samples?** Yes, through `Fno::at_resolution` — `R` is per signed frequency and the kept ones exist at every resolution. Trained at 32, it reads at 64 and 128 as well as a native operator (cosine 0.799 vs 0.805). An earlier page claimed this before it was implemented; now it is measured.

**How do I know the gradients are right?** Two ways, both run: finite differences in `cargo test`, and `torch.autograd` on a reference dump (`python torch/phinet_torch.py`).

**Why was `apply` slow?** One enormous batch's memory and bounds-checked loops, not the FFT; it is 88 µs per function now, and the remaining ~10× is the lack of a batched GEMM. [Performance](/docs/guide/performance).

**Is there a plain cross-entropy?** Yes: `Tape::cross_entropy` over `cosine_table`, and `Fno::train_epoch_softmax` around it; `exp` of the loss is the operator's perplexity over the table.

**What does Phiano hand the engine?** A table of functions, examples that name targets by index, and a pool. Never words. [Tutorial 2](/docs/tutorials/ranking-a-lexicon).

**Is the I Ching module serious?** The sixty-four gates are exact spectral operations and are used for novelty in composition. The claim that they *predict* anything was measured and refuted (0.9% vs chance 1.6%). Both facts are on the same page.

**Where are the numbers from?** Phiano's `docs/64`–`75` and papers 11–15; each is one run, held-out, chance-normalised, with the arm that differs in exactly one thing beside it. [Results](/docs/measurements/results).

**GPU?** No. CPU, f64, two cores were enough for every number here.

**What are P16/P32/P64/P128 and Q16/Q32/Q64?** The phasor ranges: resolutions (samples on the circle) and quanta (phase sectors), twelve combinations, every function in `phinet::phasor` takes them. Phiano is P64·Q64; Phical's junctures are Q32 and Q64. [Phasor ranges](/docs/guide/phasor-ranges).
