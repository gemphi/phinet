---
id: phinet-three-dimensions
title: Phinet in Three Dimensions
description: An interactive 3-D page — the relation torus, the Riemann sphere, the sixty-four gates and one Fourier layer — on real words from the Austen voice lexicon.
sidebar_position: 3
badge: Interactive
---

# Phinet in Three Dimensions

> **The paradigm.** In Phient's world, a word *w* is a seed of what it relates to, under what conditions *c*. The pictures below are that sentence drawn: no point for a word, only its phasors on the relation circle.

**Open the interactive page: [/phinet/3d/](/phinet/3d/index.html)** — self-contained (three.js bundled, no network), drag to orbit, wheel to zoom. Eight words from `phiano/data/austen_voice.chroma` — *emma, love, marriage, said, the, elinor, mother, evening* — with their real phases and evidence counts, exported by `phiano/src/bin/word_json.rs`.

## The four views

**Relation torus.** The major circle is the 64 relation channels (offsets 0–7, then the marker words); the minor circle is phase. A word is a set of points on the torus, one per evidenced channel: blue for its source side, orange for its target side, size by evidence. Select two words and every channel both have evidence on draws a chord coloured by `on(a, b, k) = cos(src_a − tgt_b)` — green agrees, red repels. The panel prints the mean over evidenced channels, which is `Facet::relational_score` on a two-word context. *emma → love* reads +0.283 over 31 channels; the offset channels agree at +0.95 to +1.00 and *offset+2*, *offset+3* repel at −0.87, −0.98 — *love* follows *emma* next to it, not two or three words on.

**Riemann sphere.** One channel at a time. Each word's phasor on that channel, with magnitude `ln(1 + evidence)`, is lifted stereographically (`phi_cal::sphere::lift`): the unit circle is the equator, more evidence climbs toward the north pole, no evidence is the south pole. This is the closed form of one channel — Paper 2 §1.1 and Paper 3.

**Sixty-four gates.** The word's target-side function (`encode_side(p, true)`) as a ring of 64 bars — height Re, colour Im — and, inside it, the same function after the hexagram gate (`phinet::hexagram::apply`). Toggle the six lines; the panel prints the octave energies (`lines`) and the novelty `1 − cos(f, H f)`. ䷀ (all lines) leaves the ring unchanged; ䷁ (no lines) leaves only the mean.

**Operator layer.** One Fourier layer `F⁻¹(R·F h)` with a random `R` restricted to the kept modes; slide the mode count from 0 to 32 and watch the inner ring smooth as fewer modes are kept. This is the regulariser of the FNO seen rather than stated — the trained `R` is what [the operator page](/docs/phinet/fourier-operator) learns.

## How it is built

`public/phinet/3d/index.html` carries the data inline and a copy of three.js r128; the FFT, the gates and the cosine in the page are line-for-line the Rust functions (`fft_inplace`, `hexagram::apply`, `hexagram::lines`, `cosine`) so that what is drawn is what the engine computes. To show other words: `cargo run --release --bin word_json data/austen_voice.chroma word …` in Phiano and paste the JSON over `WORDS`.
