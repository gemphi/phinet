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
- [x] `ground::unfold` / `unground` and `told` under any force (2026-11-08, docs/219): a clause filling a role is rows about its sub-event under the kind `clause`; the told predicate is matched under every force kind the store holds; an answer that is an event is realised. The complement form of a clause answer followed (below).
- [x] A hole inside a clause (2026-11-08): `told::nested` reads the question as levels down to the hole and descends through the `told-under` card (the sub-events under a role that meet the clause's own roles), then `told-answer-of` / `told-whether-of` at the bottom - "where does sarah want to go?" → home; whether she wants to go home → yes, to work → no. Three cards in `core.lisp`, none of the 64. A `where` the reader puts on the top frame beside the clause moves down into the clause when it finds nothing at its level, and reads every role of the sub-event the question left open ("go home" keeps its place under `theme`); tested with the told act's own clause. The reader's side followed the same hour: the control-verb infinitive branch of `read_question` ran only for yes/no questions, so "where does sarah want to go?" was read as `want(to: go, in: ?where)`; now a wh on a control verb puts its hole in the infinitive clause, and a `where` inside a clause reads every role the question left open. `converse --prime`: where does sarah want to go → home, where does tom want to go → work, does tom want to go home → no. "what does sarah want to do?" followed: a clause that is only a pro-verb (`data/lex/pro_verb.txt`: do) and a `what` hole asks for the clause itself, so the hole is the parent's role and the answer is the sub-event said in English ("sarah goes home" at first; then, the same morning, the complement form: a sub-event answer that fills a told event's theme is said as that event's complement through `ladder::realise_complement` - `say_theme` made public under one name - so "what does sarah want to do?" → "to go home", "what does tom want to do?" → "to go to work", and "what does sarah want?" the same; any other event answer stays a statement without its stop).
- [x] Test order: `guide::tests` and `abstraction::proposal::tests` call `lexicon::prime` process-wide, so a reader-dependent test elsewhere failed when it ran after them (seen 2026-11-08, 2 of 294). The engine tests now build their rows and questions from hand-made acts, not the reader. The told tests still read English; a scoped lexicon (prime per test) is the general fix.
- [x] Arrangement at the curriculum's scale (2026-11-08, docs/220): phiano's `examples/learner --record` / `--propose` over `data/curriculum/instalments.tsv`.
- [x] The keyboard's 64 is the right count until a measured operation is missing - measured (2026-11-08, docs/phisor/runs/mill-usage-2026-11-08.md): the guide's 17,385 sentences through `examples/converse --guide --usage`; 467 read as questions, 204 answered, using 11 of 40 primitives, 1 host call, 5 cards and **none of the 64 Operators**. `machine::usage` sorts the machine's tally (phiclm `Machine::tally`); `machine::answer_usage` returns the `Run` (answer, trace, usage). Operations stay cards in files, not keyboard rows.
- [x] The one operation the count found missing, punched the same hour: `told-whether` in `data/ops/core.lisp` - a yes/no question (an ASK with no hole) is whether the events its roles name are more than none; `no` is "not among what was told". The guide again: 467 of 467 questions read now answered (34 yes, 229 no), 13 of 40 primitives, 6 cards, still none of the 64. A card over primitives the mill had, not an Operator.
- [x] `machine::told`: a question answered over what was told — the fixed roles name the events, the hole's role is read off them with `image`; a `where` over every preposition role, told structurally. `examples/converse` is the loop in English. docs/211.
- [ ] A subject wh-question ("who was reading a book?") is not read; "boy" is outside the unprimed words; primed, "wearing" is refused in a statement. The reader's, and its words go in files.
- [ ] `told`: a clause filling a role is passed over; the told force is taken as `assert`.
