---
id: phinet-glossary
title: Glossary
description: The words this site uses, and what they mean here specifically.
sidebar_position: 5
---

# Glossary

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

**Function** — `Vec<Complex64>`, samples of a complex function on a circle at a power-of-two number of points. The unit of everything the engine handles.

**Space** — the number of samples on the circle (`FnoConfig::space`); Phiano uses 64, one per relation channel.

**Width** — the feature dimension after the lift; `[b, w, s]`'s middle axis.

**Mode** — a Fourier coefficient index; modes `1..space/2` are positive frequencies, `space/2+1..space` their negatives.

**Mode mixing** — one learned complex `width × width` matrix per mode (`Tape::mode_mix`); the spectral filter of an FNO.

**Pointwise control** — an operator with `modes = 0`: no spectral path, so no information can move between samples. The baseline every operator number is measured against.

**Neural operator** — a learned map between functions rather than between fixed-size vectors; here the 1-D periodic Fourier neural operator of Li et al. (2020).

**Unitary FFT** — the DFT scaled by `1/√N` in both directions, so it is an isometry and its adjoint is its inverse.

**Tape** — the recorded sequence of operations that `backward` replays; Phinet's autograd.

**Var** — a handle (index) to a node on the tape.

**Conjugate-Wirtinger gradient** — `∂L/∂Re + i ∂L/∂Im` for a real loss of a complex value; what `Tape::grad` and `torch.Tensor.grad` both return.

**Table** — `[V, 1, space]`: the functions of all items, ranked against by cosine.

**Pool** — the subset of items an example may have as target or decoy; chance MRR is computed over it.

**Decoy** — a negative item drawn for the hinge; uniform by default, frequency-proportional with `FrequencySampler`.

**Hinge** — `max(0, margin − cos(pred, target) + cos(pred, decoy))`, the ranking loss.

**Mixture objective** — training on `−log(k + coef·(γ p_op + (1−γ) other))`, the mixed language model's own likelihood of the true word.

**Joint training** — the table as a parameter on the operator's tape, held by a **tether** (a proximal pull toward its initial values after each step).

**Bind / unbind / compose** — pointwise multiply by a role, multiply by its conjugate, normalised sum: holographic reduced representations in the Fourier domain.

**Cleanup** — an operator trained on `(unbind → clean filler)` pairs, removing the crosstalk of the other fillers.

**Hexagram / gate** — a six-bit pattern; line *j* passes octave *j* of the spectrum. Sixty-four exact spectral operations.

**Octave** — modes `[2^j, 2^{j+1})`; six of them on a 64-sample circle.

**Q64** — the 64-sector quantisation of a phase; `hexagram_of` is the sector of a function's centroid.

**Novelty** — `1 − cos(f, H f)`, how much a gate changes a function.

**Möbius map** — `(az + b)/(cz + d)` on every sample; rotations, dilations and inversions are special cases.

**Zeta prior** — per-mode weights `m^{−s}/ζ(s)` applied as a projection after each step.

**MRR / hit@10** — mean reciprocal rank of the true item, and the fraction of examples whose true item is in the top ten.

**× chance** — MRR divided by the MRR a random ranking of the pool would get; the unit every ranking number is reported in.

**Two-sided lexicon / directed reading** — Phiano's words carry a source side and a target side per channel; the directed reading builds contexts from source sides and ranks against target sides. The engine sees only which functions Phiano puts in the table.
