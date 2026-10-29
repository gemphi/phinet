# phiano · engine (phase B) — plan

**Why.** phiano keeps only apps (the ruling of 2026-10-27). After the distribution, its last library code is the generation engine (`generate`, `compose`, `learn`) and `reasoning`: 4,367 lines and 12 tests. It belongs in phient, beside the trainer that trains it and the net it reads. It could not move while it was one cycle with `persona`: compose ↔ generate ↔ learn ↔ persona, 4 of phiano's 19 modules.

## The cycle is three couplings, each narrow

| edge | what crosses it | the cut |
|---|---|---|
| generate → persona | `PersonaPrior::new(facet, &Fingerprint, weight)` reads only `sector_histogram`, and calls two functions that sit on `Fingerprint`: `lexicon_mean` and `relational_sector` (generate/persona.rs:15, :41, :74) | the prior takes the histogram (`&[f64]`). The two functions read only a facet and `Wave::word_sector`, so they go beside it in `phiecs::phasor::wave` (a new `sector.rs`). The four callers pass `&fp.sector_histogram` |
| generate ↔ learn | generate reads `GenWeight` and `CandidateFeature`; learn reads `OperatorGuide` (learn/positions.rs:17) | learn is the generator's scorer weights, and the generator is its only customer: it becomes `generate::learn`. The guide reads only a facet, so generator → learn → guide runs one way |
| persona → compose → generate | `SectorPalette` (4 uses in persona) | none needed: once generate stops reading persona, persona → compose → generate runs one way. persona stays in phiano and reads both from phient |

**Two more ties, outside the cycle:**

- **reasoning → searle:** reasoning reads searle's `Chain<S>` (reasoning/pathfinding.rs:1), a plain generic struct: the problem, the steps, whether it converged, the final answer. It moves down to `phient::reasoning::Chain`, and searle re-exports it beside its `Thought`.
- **instruction → searle:** instruction runs searle's `CognitiveCore` (instruction.rs:33), so it stays in phiano with searle (the ruling: `phiano::core::searle`). This corrects the distribution plan, which listed it with the engine.

## Where each goes

| module | lines | tests | to | why |
|---|---|---|---|---|
| `generate` | 1,562 | 7 | `phient::generate` | the decoders, the beam, the context wave, the operator guide, the persona prior; it reads phient's `net` and `train` already |
| `learn` | 368 | 1 | `phient::generate::learn` | the generator's log-linear scorer and the `Tuner` that fits it |
| `compose` | 1,175 | 0 | `phient::generate::compose` | better/worse composition and the river flow, built on the generator; not `phient::compose`, which would sit beside `phient::net::compose` (words as functions within functions), a different thing |
| `reasoning` | 1,262 | 4 | `phient::reasoning` | pathfinding, analogy, planning, abstraction in phase space; with `Chain<S>` from searle |
| 16 constants they read from config | — | — | `phient::generate::constants` | `DEFAULT_CONTEXT_WINDOW`, `TORUS_DECODE_POOL`, `DEGENERACY_WARN`, `LEARNED_FILE` and compose's twelve weights and alarms; config re-exports them |
| `Fingerprint::lexicon_mean`, `::relational_sector` | ~35 | — | `phiecs::phasor::wave::sector` | a word's sector against the lexicon mean, from the facet alone; `Fingerprint` calls them there |

**Stays in phiano, the apps:** `persona` (the 256 personas, dialog, impersonation, the fingerprint), `instruction`, `core::searle`, `server`, `command`, `drivers`, `model`, `seam`, `sys`, `arena`, `lab`, `frames`, `wiki`; `config` and `tokenizer` (re-exports).

**After:** phiano has 15 modules, and its only cycle is command ↔ drivers, both app code. Tests: phiano 36 → 24, phient 155 → 167; the workspace stays at 462.

## Order

1. **The cut, in phiano, verified alone:** the two sector functions go to `phiecs::phasor::wave::sector`; the prior and `Generator::with_persona*` take a histogram; the four callers pass `&fp.sector_histogram`. phiano's cycle loses persona.
2. **The move:** `generate` with `learn` inside it and the 16 constants, then `compose` into it, then `reasoning` with `Chain<S>`. Inside the movers, `crate::tokenizer::Tokenizer` becomes `phiton::p64::words::Tokenizer` (18 uses) and the aliases `crate::phasor` and `crate::phiton` become their crates (3). The 38 importers are rewritten: server 9, command 7, persona 5, core 3, model 2, instruction 1, bin 1, examples 10.
3. **Verify** the workspace; **record**: originals parked with INVENTORY rows, the four module pages moved to docs/phient, each crate's ARCHITECTURE, README and TODO, phiano's ARCHITECTURE regenerated (the parent session's script), the jobs ticked.

## Risks

- **A read-only importer:** examples/flow.rs reads `phiano::compose::flow::RiverFlow`. `pub use phient::generate::compose;` in phiano's lib keeps it building, as for `facet`, `storage` and `trainer`.
- **The prior's signature** changes for four callers (persona's dialog and impersonation, examples/persona, generate's own `with_persona*`): a histogram instead of a fingerprint, with the same numbers.
- **phient grows** by 4,367 lines (generate becomes a cluster like `train`); no new dependencies, since serde, serde_json, phical and phiton are there already.
- **The machine is busy** (phikit builds): verify at low priority, one build at a time.
