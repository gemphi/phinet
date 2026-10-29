# phisor — what is left to do

Paths are under `src/`. The plan and the full checklist: [docs/phisor/plan.md](../../docs/phisor/plan.md), [docs/phisor/jobs.md](../../docs/phisor/jobs.md). Diagrams: [ARCHITECTURE.md](ARCHITECTURE.md).

## Waiting on the user

- [ ] Read the first slice's result (`docs/phisor/kinds.md`) and choose the next lever: the kinds reading the sentence's own shape, or parting the learned class that joins auxiliaries with imperative verbs.

## The first slice: the kinds of intention (measured 2026-10-29)

- [x] `guide`, `code`, `kind`, `units`, `examples/kinds`; `docs/phisor/kinds.md`
- [x] No cap: grown templates with a trie back-off, classes and kinds that split only while they pay (now `phical::classes::halve`)
- [ ] `kind`: kinds from inside the sentence (the templates' own class pairs), not only the river
- [x] The exchange routine moved to `phical::classes::{halve, Grow, Halving}` (the parent session, under the ruling "any calculations must go to phical"); phisor keeps `Units` (`src/units.rs`); the old module parked in `_temp/phinet-root/phisor-exchange-before-phical` with an INVENTORY row; the kinds run is identical before and after (53 word classes at depth 7)
- [ ] `code`: part `do`/`did`/`have` from `use`/`complete`/`take` (c12 at mode 16)
- [ ] Feynman's second run: the twins' kinds as probabilities added against amplitudes added (`phical`, asked first)
- [ ] Bentov's rank-frequency slopes (a least-squares slope, asked of the parent)

## After it

- [ ] `english` and `learn`: the act from English, M1–M5 (150 gold frames written before the learner runs)
- [ ] `family`: the second family, the cross table, the river
- [ ] `ground` and `machine`: rows, the Lisp host, `ASK` answered (M6); the templates as small programs on the judge

## The IR itself

- [ ] Read the written form back (`ASSERT(…)` → `Act`), so the IR round-trips as text as well as through English
- [ ] `Modal` covers the five modals with `could`, `would`, `might`, `should` as their past forms; `have to`, `be going to`, `used to` are not modals yet (Murphy units 20–34)
- [ ] One owner for Searle's five points: phisor's `Point` or phiano's `core::searle::SpeechActType` (the user's ruling)

## Docs

- [ ] One page per module in `phinet/docs/phisor/` as each lands (today: `ir.md`)
