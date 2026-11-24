# Jobs — phisor, the intermediate language

| State | Open | Done |
|---|---|---|
| 🟢 the first slice measured, uncapped, read inside, Feynman's run, learned phases, c12 parted on two channels (docs/phisor/kinds.md) | 17 | 26 |

## Done — the plan and the IR

- [x] The survey: the guide and its Key, docs/86, 153, 200, 201, 203, Huberman's thesis (papers/liskov/01), `phical::liskov` and `joint`, `phiecs::Row` and `World::tell`, phiclm's Lisp (`Host`, the set algebra), phiano's `core::searle`, phiton's `p64` (levels, `classify_str`, `PosDictionary`), `conjugate`, `physics::standing`, the eight thinkers' personas (2026-10-29)
- [x] The finding: `P16::classify_str` is a hash of the spelling, so it is the null, not a class
- [x] The plan written (plan.md, architecture.md, this checklist), then revised for the user's levels, the 64 questions and the thinkers
- [x] `crates/phisor` with README, ARCHITECTURE, TODO; `ir` (act, force, frame, filler, template, the written form), 8 tests and a doc test
- [x] phisor joins the root `Cargo.toml` members; the parent session and the phiclm session told
- [x] The IR builds and its tests pass (low priority)
- [x] Module page: docs/phisor/ir.md

## Done — the guide and the code (steps 2, 3)

- [x] The user's correction taken (2026-10-29): "the questions were just an example … learn to abstract my instructions". No kind is chosen by hand; the kinds are found, and the labels are checks
- [x] The parent session added the calculations: `phical::classes` (`likelihood`, `move_gain`, `apply_move`), `phical::liskov::staged`, `phistd::stats::{precision, recall, purity, mutual_information, z_score}`
- [x] `guide`: the units' sentences with their function and mark (docs/201's reader), the Key's answers (all held out), docs/203's split rule, the training runs, the sentence before; 2 tests
- [x] phisor gains phiecs (tokens), phiton (levels, hash, dictionary), phical, phistd
- [x] `code`: the hash null, the given dictionary, the learned 6-bit code; a sentence written in a mode
- [x] `exchange`: the one routine (pairs, halving, gain from phical, Babbage's stop); 3 tests. Moved to `phical::classes::halve` the same day (the parent session); phisor keeps `units`, and the old module is parked in `_temp/phinet-root/phisor-exchange-before-phical` with an INVENTORY row; the kinds run is identical after the move
- [x] `kind`: variations and the twins, kinds by the river, label tables, the river's pairs, family resemblance; 3 tests
- [x] Module pages: guide.md, code.md, kind.md, units.md (exchange.md parked with its module; the routine's page is docs/phical/classes.md)

## Todo — the kinds of intention and their variations (step 4, the first measured slice)

- [x] The run: every arm, mode and twin; I1–I5 on the held-out sentences. I1 met by 0.2 points, I2 not met, I3 met (the hand beats the learned code on questions), I4 met, I5 not supported
- [x] Two layout confounds found in the first run and fixed: exercise item numbers (dropped, as the Key's reader drops them) and the Key's missing marks (marks read on the units only); both runs kept in `runs/`
- [x] Each label sensed by the kinds that answer to it on dev, scored held out
- [x] Babbage: the passes at both heights; Wittgenstein: resemblance per kind against shuffled kinds; Bentov: the counts per mode
- [x] `docs/phisor/kinds.md`: every number straight, what works and what does not
- [x] No cap (the user: "built dynamically, we can't limit the templates"): grown templates with a back-off along their trie; classes and kinds split only while they pay (`phical::classes::split_cost`, added by the parent). The data asks for 53 word classes, ~4,500 templates and 6–20 river kinds, which carry almost no label (runs/kinds-2026-10-29-paid.md)
- [x] Kinds from inside the sentence (2026-11-23): the templates grouped by their own class pairs (`kind::Reading::Inside`), the same routine and MDL stop. The data pays for 29–58 kinds; function purity 58.0% (learned opening, mode 16) against 52.8% by the river and a 51.7% majority, so I1 is met with paid kinds and I2 is still not; instructions sensed for the first time (F1 33.9%). runs/kinds-2026-11-23-inside.md
- [x] Parting c12 (2026-11-24): `Code::learn_opening` gives a sentence-opening word a sense of its own (`^do`), coded by what follows it (no `<s>` before a sense). Questions F1 56.1% at mode 16 (from 34.4%; the hand dictionary 53.2%); instructions lost at 16, 31.1% at 32 (runs/kinds-opening-2026-11-24.md, runs/class-tree-2026-11-24.md)
- [x] One mode senses both (2026-11-24): the opener on two channels; mode 64: questions 61.4%, instructions 55.7%, function 59.3%, mark 72.2% (runs/kinds-two-channel-2026-11-24.md)
- [x] Feynman's run (2026-11-23): `measure::mode_input` feeds the ladder spec's twins measure (R9.6–R9.10) from the guide; `phistd::stats::share` and `phical::circle::amplitude` added. Amplitudes ≥ probabilities for the learned code at every mode (+0.9 at 16), the hash misses at two of three; both ends never beat the better end + 2; the phases align by construction (runs/feynman-2026-11-23.md)
- [x] Learned phases (2026-11-23): `measure::phase`, a phase per kind for the closing's arrow, sixteen to choose from, kept by `liskov::staged::better` when more training sentences peak at their own both-ends kind (no label); `phical::circle::intensity` added. +0.7 to +1.2 points over probabilities for the learned code, about as much for the hash; only 1–2 kinds turn a quarter turn or more (runs/feynman-phases-2026-11-23.md)
- [ ] Bentov's rank-frequency slopes (a least-squares slope, asked of the parent)

## Todo — the act from English and the second family (step 5)

- [ ] `english`: the closed words, morphology, the chunker, the clause reader, the realiser; M2
- [ ] `learn`: the one loop over the open lexicon; M1, M3 (150 gold frames, frozen first), M4, M5
- [ ] The kinds read into acts: each kind's force and frame from its variations

## Todo — grounding and the machine (step 6)

- [ ] phisor gains phiecs and phiclm; `ground::rows`, the copula's flattening
- [ ] `machine::host` with `(rows kind)`; `machine::query`, `ASK` answered (M6, Whitehead)
- [ ] von Neumann: each template a small program on phiclm's machine, the same senses as the matcher
- [ ] Lovelace: generated mode-16 sequences, never told, attested in held-out

## Todo — the records (step 7)

- [ ] `docs/phisor/thinkers.md`: the table filled, run by run
- [ ] Module pages for every module as it lands; the root README, ARCHITECTURE and TODO name phisor

## Later

- [ ] The joint: the 64 templates as a table trained with an operator (`phical::joint`)
- [ ] Recoding: an act bound into one phasor (`phiton::hrr`) and read back by the conjugate
- [ ] The alignment arm: templates found with no skeleton given (van Zaanen)
- [ ] `DIRECT` → a BCPL program; `WISH` → a goal for `World::plan` and philog's planning
- [ ] The rows told to a phiecs `World`; phient's learner as the critic
- [ ] Family 2, the relative clause; family 3, the conditional
- [ ] One owner for Searle's five points (phisor's `Point` or phiano's `SpeechActType`)
