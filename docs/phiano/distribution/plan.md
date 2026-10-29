# phiano · distribution — plan

**Why.** phiano keeps only apps (the ruling of 2026-10-27). After the facet, the training cluster, metrics, net and the Lisp, fourteen of its modules are still library code. Each goes to the crate its dependencies and its meaning place it in, at the lowest level it can stand on.

**The rule that splits two of them.** The user placed the oscillator beside Bentov (phiton) and the helix in phical. phiton and phical sit below phiecs, where the facet lives, so a module moves whole only when it never reads a facet. The oscillator's sphere model and the helix's code read none, and go where the user placed them; the parts that read a facet go up to phiecs, and the part that reads the Evaluator to phient.

## Where each goes

| module | lines | to | why |
|---|---|---|---|
| `bpe` | 250 | `phiton::p64::bpe` | Phi-4's byte-level tokenizer, beside `p64::words::Tokenizer`; phiton gains serde_json |
| `oscillator` mod · view | ~200 | `phiton::physics::oscillator` | the sphere model of language (α, the bands, `c64`), beside `physics::standing` (Bentov) |
| `oscillator` field · train | ~250 | `phiecs::phasor::oscillator` | the oscillator field over a facet's words, and its trainer |
| `oscillator` eval | ~130 | `phient::eval::oscillator` | compares against the `Evaluator` |
| `helix` code | ~35 | `phical::helix` | `Codon`: the codon, the hexagram, the place on the helix, the sphere's degree and order (phifno) |
| `helix` read · write | ~270 | `phiecs::phasor::facet::helix` | a facet as a directory tree — a file format, beside `storage` |
| `layers` | 282 | `phiecs::phasor::layers` | the phase hierarchy over spectral phasors |
| `flow` | 566 | `phiecs::phasor::flow` | the phase-flow graph between words; `LEARNING_RATE` and `PHASE_REPULSION` move down to the facet's constants (phient's `train::constants` re-exports them) |
| `attention` | 272 | `phiecs::phasor::attention` | phase-space attention over spectral phasors, nothing else |
| `kinds` | 1,146 | `phient::train::kinds` | relation kinds read as standing-wave modes, from the role offsets beside it (`RoleDiscovery` can be crate-private again) |
| `synthetic` | 149 | `phient::train::synthetic` | self-made training data |
| `curriculum` | 146 | `phient::train::curriculum` | the progressive curriculum |
| `synthesis` | 405 | `phient::train::synthesis` | program search — phiclm's by meaning, pinned to phient by the `Trainer` it trains |
| `lifelong` | 663 | `phient::train::lifelong` | the lifelong coordinator: the Evaluator, the benchmarks, the detector, synthesis |
| `correction` | 137 | `phient::train::correction` | the correction journal the trainer applies |
| `envision` | 352 | `phient::train::envision` | suggestions for unknown words, from the chunk store and the trainer |

**Stays in phiano, the apps:** `server`, `command`, `drivers`, `model`, `seam`; `core` (searle), `persona`, `reasoning`, `instruction`; `generate`, `compose`, `learn`; `arena`, `lab`, `frames`, `wiki` (it downloads and trains: an app job); `sys`; `config` and `tokenizer` (re-exports).

**Phase B, a decision for later:** the generation engine (`generate`, `compose`, `learn`, with `reasoning` and `instruction`) belongs in phient, but it is one cycle with `persona`; it moves once that coupling is cut. Planned in [../engine/plan.md](../engine/plan.md): `instruction` stays, since it runs searle's `CognitiveCore`.

## Order

Lowest level first, so each move stands on what has already landed:

1. **phiton:** `bpe`, the oscillator's sphere model. **phical:** the helix's code.
2. **phiecs:** `layers`, `attention`, `flow` (with its two constants), the helix's files, the oscillator's field and trainer.
3. **phient:** `kinds`, then the training six (`synthetic`, `curriculum`, `correction`, `envision`, `synthesis`, then `lifelong`), the oscillator's eval.
4. **Verify** the workspace after each level; **record**: originals parked with INVENTORY rows, the module pages moved, each crate's ARCHITECTURE, README and TODO, the jobs ticked.

## Risks

- **Read-only files** among the movers or their importers: none found so far; a re-export in phiano keeps any that turns up building, as for `facet`, `storage` and `trainer`.
- **Constants** the movers read from phiano's config go to their new owner and are re-exported by config, as before.
- **phient keeps growing** (`train` gains seven): every one of them is learning, and every one needs the trainer.
