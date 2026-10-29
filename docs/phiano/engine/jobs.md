# Jobs — phiano's engine (phase B)

| State | Open | Done |
|---|---|---|
| 🟢 done | 0 | 13 |

## Done — the map

- [x] The cycle mapped to three couplings, the importers counted, the plan written (plan.md, architecture.md), 2026-10-28

## Todo — the cut (in phiano)

- [x] `Fingerprint::lexicon_mean` and `::relational_sector` → `phient::net::space::{lexicon_mean, relational_sector}` (not phiecs: they call `encode_side`, and phiecs cannot depend on phient); `Fingerprint` calls them there
- [x] `PersonaPrior::new` and `Generator::with_persona`, `with_persona_examples` take the sector histogram (`&[f64]`); the four callers pass `&fp.sector_histogram`
- [x] Verify: generate no longer reads persona (no `crate::persona` in it); the tests are counted below

## Todo — the move (to phient)

- [x] `generate` → `phient::generate`, with `learn` inside it as `generate::learn`
- [x] The 16 constants → `phient::generate::constants`; phiano's config re-exports them
- [x] `compose` → `phient::generate::compose`; `pub use phient::generate::compose;` in phiano's lib for the read-only examples/flow.rs
- [x] `reasoning` → `phient::reasoning`, with `Chain<S>` from `core::searle` (searle re-exports it); since moved on, by the user's ruling, to its own logic crate, `philog`
- [x] The 38 importers rewritten (server, command, persona, core, model, instruction, bin, examples); `crate::tokenizer` → `phiton::p64::words::Tokenizer` inside the movers

## Todo — the checks and the records

- [x] The workspace builds and every test count is conserved: phiano 36 → 24, phient 155 → 167 (both measured 2026-10-28, low priority; phient and phiano --all-targets type-check clean; the other crates were not touched, so the workspace stays 462)
- [x] Originals parked with INVENTORY rows (`_temp/phinet-root/phiano-engine-before-move`, `phiano-engine-docs`)
- [x] The module pages (generate, learn, compose, reasoning) moved to docs/phient; phient's and phiano's ARCHITECTURE and TODO updated; phient's README gained `generate` and `reasoning` (phiano's README lists no modules)
- [x] phiano's ARCHITECTURE regenerated: 15 modules, one cycle (command ↔ drivers); 103 diagrams parse, no dead links
