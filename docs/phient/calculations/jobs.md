# Jobs — calculations to phical

| State | Open | Done |
|---|---|---|
| 🟡 in progress | 7 | 4 |

## Done — the map

- [x] Survey: about 150 calculations in phient, 26 duplicating phical/phistd (11 exact), the rest phient's own repeats; plan and diagrams written (plan.md, architecture.md), 2026-10-28

## Todo

- [x] 1 · The bit-identical duplicates, 2026-10-28. The nine 'wraps' were the same angle as `phistd::wrap` but not the same bits (its `rem_euclid` makes `(d + τ) − τ` for d < 0), so they went to a new `phical::circle` written as the sites wrote them: `delta` (conception ×5 calls, grounding, roles' purity) and `distance` (synthesis' heuristic, synthetic ×2, metrics' ood). operator's context ×2 → `phical::relate::context` (now pub); the sentence benchmark's chance → `phical::relate::chance`. Tests unchanged: phical 19 (17 + circle's 2), phient 163, philog 4, phiano 24.
    - Kept for later: `train/topdown/votes.rs`' wrap (a third form, `(Δ + π).rem_euclid(2π) − π`; batch 3) and `net/space/encode.rs`' normalisation (energy from the amplitudes and a 1e-12 floor, not `phistd::normalise`'s; batch 2).
    - `eval/verdict.rs:115` is a BUG, not a duplicate: it caps |Δarg| at π instead of folding it, so a wave across the ±π seam from the centroid (−3.0 against 3.0, 0.28 rad apart) reads as maximally novel. Fixed 2026-10-28 at the user's "Yes, fold it": 41 of 1,993 guide sentences on the Austen lexicon moved, 22 changed class (docs/206).
- [x] 2 · phistd numerics, 2026-10-28, each written as its sites wrote it (docs/phistd/numerics.md):
    - `random`: `unit_interval` (four sites, and phistd's own three), `unit_interval_f32` (the learner), `lcg` (scribe's coin), `xorshift64` (the harness's negatives; its comment had said xorshift64*), `xorshift64_star` (the generator's sampler), `draw_by_weight` (scribe's draw and mold's pick).
    - `stats`: `mean` and `population_sd` (the evaluator), `f1` (arc), `log_share` (metrics' baseline and forgetting).
    - `prob`: `perplexity` (7 sites), `perplexity_of_nll` (study's Score), `softmax_z`/`softmax_p` (heldout, levels, the context readout, gamma, sweep), `log_softmax_at` and `softmax_minus_onehot` (the tuner), `sample_softmax` (the generator; its uniform drawn through a closure, so the generator advances as before).
    - `real`: `normalise_f32`, `dot_f32` (synthesis' library), `norm`, `re_dot_interleaved` (net's lm). `optim`: `adam_step` (the tuner's Adam on slices).
    - Tests unchanged: phistd 35 (26 + 9 new), phient 163, philog 4, phiano 24.
    - Left for their batches: the learner's f32 softmax and Adagrad (4), the tuner's feature dot products (8), `encode_side`'s normalisation (it reads `SpectralPhasor`; 3 or later).
- [x] 3 · `phical::circle`, 2026-10-28. circle.rs became the folder `circle/` (`angle`, `resultant`, `agreement`, `kmeans`, `centre`, `field`, `tests`; docs/phical/circle.md), each function written as its sites wrote it. Where the copies differed in their last step they got one name each: `direction` (atan2), `direction_euclid` (then `rem_euclid(τ)`), `direction_lifted` (then `+ τ` if negative), `direction_or_none` (atan2 where `hypot > 1e-12`), `vanishes` (the box test |x|, |y| < 1e-12).
    - `Resultant` (running Σ w·cos θ, Σ w·sin θ; `add`, `add_unit`) and `mean`: the trainer's sentence, predictive (carried forward) and ngrams centroids; conception's `compose_with` (none → NaN); grounding's definition centroid; lifelong's reuse centroid and meta's `circular_mean`; roles' bind identity; synthesis' `mean_phase`; ood's prompt and lexicon centroids; preprocess's wave, `normalize_wave` (`unit`) and `wave_phase`; the discarder's losers; net's reader `dispersion_all` (`dispersion`); kinds' head, filler, grand and class means (were `c64::from_polar(1, θ)` sums: the same bits).
    - `agreement`, `centroid` (were `RoleDiscovery::agreement`/`centroid`, now gone), `nearest` (kinds' `best_match`, `classify`), `mean_agreement` (purity's `measure`, k-means' coherence), `mean_nearest` (three identical held-out loops: relation's harmonic saturation, saturation's held-out agreement, table's harmonic agreement), `shape_similarity` (reuse's `similarity`).
    - `kmeans` and `Cluster`: roles' `cluster`, verbatim (farthest-point seeds, Lloyd rounds, coherence); phient maps the member indices back to pairs.
    - `reduce`, `centred`, `double_centred`: the offset estimators and the per-class centring of kinds' table.
    - `field`: topdown's votes and settle — `add_scaled`, `leave_one_out`, `is_silent`, `add_directions`, `rayleigh_floor` (max(c₀, C/√n)), `scattered`, `vote_target` (the contrast) and `step_toward` over `delta_euclid`, the third wrap form `(Δ + π) rem 2π − π` kept from batch 1.
    - `analogy` (c + b − a): the relation benchmark's query.
    - Tests: phical 26 (19 + 7 new), phient 163, philog 4, phiano 24; no new warnings.
    - Left, and why: predictive's hinge scores (mean cos Δ, but over `channels.len()` with no `.max(1)`; they go with the hinge update, 7); conception's definer pull in `compose.rs` (a weighted resultant, not on the survey list, whose finisher is `hypot < 1e-12` — not `!(> 1e-12)`, so a NaN differs — and a `min(1, |R|/count)` scale that feeds the step, 7); purity's `align` target and step (7); topdown's own share Σ contribution · 1/len (the rotation, 6); conception's golden-angle roll and 1e-6 amplitude floor (the binding, kept with the loop); `RoleDiscovery::offset` (filler − head, read from the facet); version's plain means of dispersions and amplitudes (not circular).
    - Noticed: on atan2's range `rem_euclid(τ)` and `+ τ if negative` give the same bits (`x % τ` is exact for |x| < τ), so `direction_euclid` and `direction_lifted` could become one — kept apart under the rule until the user says otherwise.
- [ ] 4 · `phical::learner`: the lineup learner's math (plane, condition, step, scratch, sampler with ln q); docs/203 numbers re-run
- [ ] 5 · `phical::smooth` + `posterior`: relate, prose strand, Kneser–Ney (baseline, harness)
- [ ] 6 · `phical::kernel`: the damped kernel and the recurrent state (5 sites)
- [ ] 7 · `phical::couple`: the trainer's, conception's, grounding's, roles', lifelong's and the generator's sin-coupling
- [ ] 8 · `phical::score` + `lm`: the generator's scorer, features and tuner; eval's and memory's scores; the harness readouts
- [ ] 9 · philog: survey its phase arithmetic, then into `circle` and `couple`
- [ ] After each batch: tests at the same counts (phistd, phical, phient, philog, phiano); docs/203, docs/113 and a heldout table re-run against their records
