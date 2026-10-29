# phient · calculations to phical — plan

**Why.** The ruling of 2026-10-28: "any calculations must go to phical, phient is not supposed to do any real calculations". phient is the brain: it holds the arena, the study protocol, the readers and the drivers, and runs them. The math they run — gradients, softmax, smoothing, circular means, phase coupling, kernels — belongs in phical, or in phistd when it is generic numerics.

**What a read-only survey found** (2026-10-28, every file of `phient/src` but tests; philog's reasoning is surveyed separately): about **150 places** where phient computes. Only **26** repeat a function phical or phistd already has; **11** of them exactly. The rest is phient writing the same formula over and over, with no shared version anywhere yet:

| pattern | copies in phient | home |
|---|---|---|
| circular mean / resultant, agreement as mean cos Δ | ~16 + 5 | `phical::circle` (new) |
| softmax / log-sum-exp | ~10 | `phistd` |
| perplexity | ~6 | `phistd` |
| hash → uniform in [0, 1) | 5 | `phistd::unit_interval` |
| sin-coupling steps (nudge toward, repel away) | ~15 | `phical::couple` (new) |
| damped rotating kernel and the recurrent phase state | 3 + 2 | `phical::kernel` (new) |
| roulette draw from weights | 3 | `phistd::sample_categorical` |
| KL gate, posterior as ln prior + Σ ln(d/p) | 3 + 3 | `phical::posterior` (new) |
| add-k, Dirichlet, Witten–Bell, Kneser–Ney, EM weights, MDL | ~12 | `phical::smooth` (new) |
| the learner's condition, hand-written gradient, Adagrad, sampled softmax | 1 block, ~250 lines | `phical::learner` (new) |
| the generator's scorer, features, tuner (with a hand-written Adam) | ~12 | `phical::score` (new), plain-slice Adam in `phistd` |
| the LM harness readouts (β-softmax, γ-mixture, grids) | ~8 | `phical::lm` (new) |

## The one constraint

phiecs depends on phical, so **phical cannot read `Facet`, `SpectralPhasor` or `Wave`**. A function that moves takes plain values: angles, amplitudes, counts, `&[Function]`. phient keeps the loop that reads the facet and hands the numbers over; phical computes. (phical may depend on phiton, which depends only on phistd.)

## The rule for numbers

The tests and the recorded reports must not move.

- A computation moves **verbatim**, in its own operation order. Two copies are merged into one function **only where they are the same arithmetic**. Where they differ, even in order, each keeps its own form or the difference is measured and recorded.
- After each batch: every crate's tests at the same counts, and the reports that read the moved math re-run against their records. Those are the learner (docs/203), builders (docs/113) and a story `heldout` table.

## Batches, smallest risk first

1. **Exact duplicates** — no design.
   - Nine inline angle wraps → `phistd::wrap`.
   - `operator/{model,wave}.rs` → `phical::relate::inherit::context` (make it `pub`).
   - `metrics/sentence/state.rs:93` → `phical::relate::score::chance`.
   - `net/space/encode.rs:23-26` → `phistd::normalise`.
   - `eval/verdict.rs:115` caps an angle with `.min(PI)` where every sibling wraps. Check whether this is a bug before changing it: it changes numbers.
2. **phistd numerics.** `softmax`, `log_sum_exp`, `perplexity`, `sample_categorical`, `unit_interval`, plain-slice `Adam` and `adagrad_step`, the basic statistics (mean, population sd, F1), `log_interp`, and real-valued `cosine` and `normalise`.
3. **`phical::circle`.** Circular mean and resultant, agreement, circular k-means, the Rayleigh floor, double-centering, analogy (θc + θb − θa).
4. **`phical::learner`.** The lineup learner's `Plane`, condition, step, gradient gather and its count^¾ sampler with ln q. This is beside, or an extension of, phifno's `FrequencySampler`.
5. **`phical::smooth` and `phical::posterior`.** relate (counts, levels, path, dist, weights, kin, tree), prose's strand (code, path, reading, prior), and Kneser–Ney (baseline, the harness's affine).
6. **`phical::kernel`.** The damped kernel with prime frequencies, and the recurrent state (generate's context, the harness's advance, the sentence benchmark, topdown's rotation).
7. **`phical::couple`.** The sin-coupling updates of the trainer, conception, grounding, roles, lifelong, and the generator's steer and decode.
8. **`phical::score` and `phical::lm`.** The generation scorer and features, the tuner, the evaluator's and memory's scores, the LM harness's readouts.
9. **philog's phase arithmetic.** Surveyed next, then into `phical::circle` and `couple`.

**After:** phient holds no formula of its own. It reads the facet and the counts, calls phical and phistd, and keeps the protocol, the readers and the reports. Each batch is one change with its tests and records. The phiclm session keeps off `phient/src` and `phical/src` while a batch is in flight.

Diagrams: [architecture.md](architecture.md). Checklist: [jobs.md](jobs.md).
