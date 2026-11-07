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

## The loop (2026-11-07)

- [x] `machine::host::apply` (2026-11-08): `(phisor-apply index a0 …)` reaches every keyboard Operator from the Lisp — operands read by the row's declared types (a phasor `(re im)`, a handle its index), applied through `phical::keyboard::apply` against the host's Arena, a Fault returned as the error text. The lowering's own lambdas (`phisor-op-11-centroid`) now evaluate. Before this the arm answered "not built yet", and the mill ran only inside the agreement check.
- [x] `machine::ops` (2026-11-08): operation cards. The mill is fixed (the 64 Operators, the 39 primitives); every other operator is a card - a `define` of a lambda in `data/ops/*.lisp`, read into every machine at birth, punched at run time by `ops::add` (appended to `learned.lisp`, defined in the running machine, known to the next). `told` answers through the `told-answer` card. `examples/converse --define`.
- [x] `machine::engine` (2026-11-08, paper 28): every host call is a numbered `Step` - a column fetched (`phisor-rows`) or an operation performed (`phisor-apply`); `answer_traced` returns Note G's table beside the answer; `arrange` runs several cards for one operation and chooses the shortest agreeing trace (Note D). Her Bernoulli program runs as `data/ops/bernoulli.lisp` (B1-B7 to 1e-9); phiclm gained `/`.
- [ ] Arrangement at the curriculum's scale: each learner instalment a card, its measured gain the cost, the next proposed from the record.
- [ ] The keyboard's 64 is the right count until a measured operation is missing: ask/told answer with six of phiclm's 39 primitives composed and call none of the 64. Before adding an Operator, have the harness count which primitives and Operators the guide's questions invoke; add operations as named lambdas in files, not keyboard rows.
- [x] `machine::told`: a question answered over what was told — the fixed roles name the events, the hole's role is read off them with `image`; a `where` over every preposition role, told structurally. `examples/converse` is the loop in English. docs/211.
- [ ] A subject wh-question ("who was reading a book?") is not read; "boy" is outside the unprimed words; primed, "wearing" is refused in a statement. The reader's, and its words go in files.
- [ ] `told`: a clause filling a role is passed over; the told force is taken as `assert`.
