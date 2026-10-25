---
id: phinet-colab
title: Colab Notebook
description: Run the parity checks and train the sixty-second operator in PyTorch, cell by cell.
sidebar_position: 2
badge: Interop
---

# Colab Notebook

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

`phinet/torch/phinet_colab.ipynb` has four cells.

**1 · Fetch.** Reads `PHINET_RAW` (the raw URL of a checkout's `torch/` folder) and downloads `phinet_torch.py` and `reference.json` if they are not already beside the notebook; or upload both with the file panel.

**2 · Check.** `pn.check("reference.json")` — the 55 parity checks, printed one per line with the max absolute difference; ends with `ALL PASS`.

**3 · Train.** The README task in torch: sixty unit-random items on a 64-point circle, target = item rotated by one sample, `pn.Fno(space=64, width=4, layers=1, modes=32)`, hinge against a random decoy, Adam 1e-2, sixty epochs of batches of twenty with `mask_modes()` after each step. Output on the run recorded here:

```
untrained MRR 0.073
trained MRR 1.0 (phinet's README asserts > 0.9 on the same task)
```

**4 · Gates.** One function through ䷀, ䷘, ䷒ and ䷁ — octave energies, novelty, and the hexagram of the result.

```
111111  lines=[0.01, 0.03, 0.14, 0.31, 0.48, 0.04]  novelty=-0.000  hexagram_of=23
101010  …                                             novelty=0.374   hexagram_of=23
000111  …                                             novelty=0.559   hexagram_of=23
000000  …                                             novelty=0.847   hexagram_of=23
```

(`hexagram_of` is the centroid sector, and every gate passes mode 0 — the mean — so the sector is unchanged by any gate. That is by construction, not a coincidence.)

The notebook needs only `torch`; complex128 on CPU is enough. A GPU changes nothing but speed.
