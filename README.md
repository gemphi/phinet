# phinet — Phient's Phasor Language Models

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Phinet is the workspace that holds the model family. It is a new paradigm, not a transformer: nothing here needs to know everything before it starts learning. A model is told sentences one at a time, on the fly, and is always tested on what it has not yet been told (predict first, learn after). Counts are kept only as the contrast line to beat.

## The crates

| crate | what it is |
|---|---|
| [phistd](crates/phistd) | the standard floor: shared primitives |
| [phifno](https://github.com/gemphi/phifno) | Phi Fourier Neural Operators — learning in the frequency domain |
| [phical](crates/phical) | the Phical paradigm, X ⇒ Y: relations as phasors on the closed circle |
| [phiecs](https://github.com/gemphi/phiecs) | the Entity Component System and the phasor engine: `Row`, `World::tell`, reads from the phasors under conditions-as-channels with answer slots; home of the kit (the body, the track, the device as rows) — its repo also lives under [phikit](https://github.com/gemphi/phikit) |
| [phioml](crates/phioml) | operator machine learning over phiecs |
| [phient](https://github.com/gemphi/phient) | the BRAIN (formerly phiplm): the arena (every model, one table), the study protocol, prose as a tree of knowing — the ollama of Phient systems |
| [phiclm](https://github.com/gemphi/phiclm) | the Compiler Language Model: BCPL's architecture as one of ours — lexer, parser, O-code, the word machine, the judge |
| [phiton](crates/phiton) | tensors and training floor |
| [phiano](https://github.com/gemphi/phiano) | the instrument: arena readers, the scribe, the workshop, the examples (`person`, `talk`, `teacher`, `workshop`, `arena`, `writer`); the seam — the phasor student coupled with the phiclm judge |

The linked crates are their own repositories, taken here as submodules; clone with them:

```
git clone --recurse-submodules https://github.com/gemphi/phinet.git
cargo test --workspace --release
```

## Measured, straight

The round-by-round record is `crates/phiano/docs/001–195`, every number verbatim, including the ones against us. Where it stands: on the glosses corpus the phasor engine leads the arena (32.9% next-word vs counts 29.9%); on prose it still trails counts (13.9% vs 21.4% on the English guide); graded against the guide's Key it was never told, 15.7% vs counts 20.1%. A whole book (22,197 sentences) is told in 3.6 s with no retraining. Continuation in a learned voice works for a few words and then decays — that decay, and the prose gap, are the open problems, named in `docs/194`.

MIT OR Apache-2.0.
