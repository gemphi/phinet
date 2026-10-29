# phinet — what is left to do

Workspace-wide. Each crate keeps its own list: `crates/<crate>/TODO.md`.
Last read over: 2026-10-29.

## Learning (docs/203, 207)

- [ ] Beat the smoothed Witten–Bell trigram on the guide's held-out sentences. Where it stands: the learner 21.3% top-1 and perplexity 101.2, the trigram 24.8% and 84.7 (the guide alone, 142k words).
- [x] Train on more text than one book. Done, and it did not work: the guide plus 173 Simple Wikipedia articles and phi4 prose made **every** model worse on the guide (the learner 21.3% → 19.8% top-1). An encyclopedia is not the guide's register. docs/207.
- [ ] **The next run, one of two, the user's call:** English in the guide's own register (Tatoeba's English sentences, 24.9 MB, short everyday sentences — closest open source), or a curriculum with no download: tell the wide text first and the guide's own sentences in the last epochs.
- [ ] More channels when the vocabulary grows: 64 channels over 15,609 words is thinner than over 4,705.
- [ ] A dev set that mixes registers, to tell a model that learned English from one that learned the guide.
- [ ] Try a wider operator block and more channels with more silence (the block-8 run was still improving when it stopped).
- [ ] Full softmax for the last epochs.
- [x] Put the learner behind `talk` and the workshop, so nothing that generates runs on counts (docs/205).
- [ ] Decide `phiecs::Phasor::couple` (the running mean): make it error-driven, or keep it as the counted contrast line only.

## The intermediate language (crates/phisor, docs/phisor)

The crate is new and its first two runs are recorded. What it does **not** do yet is the honest part.

- [ ] The kinds of intention carry almost no label: function purity 53.3% against a 51.7% baseline, with 6 to 20 kinds paid for. Knowing which kind follows which is not enough to tell an explanation from an instruction. Next, per `docs/phisor/plan.md`: what a sentence is *made of*, not only what follows it.
- [x] No cap, by the user's ruling ("built dynamically, we can't limit the templates"): the data pays for 53 word classes at depth 7 and grows about 4,500 templates.
- [ ] "Give me this story in mode 16": render a text at the 16, 32 and 64 levels, structure only, no words.
- [ ] One family at a time, 64 assumed and the real number reported: the ways of asking a question, then an activity in a circumstance, then how one family's variations line up with the other's.
- [ ] A measure for each thinker's claim, run on the guide and recorded straight: Bentov's standing waves and his backward mirror, Searle's force, Babbage's difference, Wittgenstein's family resemblance, Feynman's amplitudes added and recoded.
- [ ] The lowering: every phisor operation becomes a `lambda` applied to its arguments, named with `define`, over phiclm's Lisp (`lisp::Host`, a `(rows kind)` primitive). Nothing is built for it yet.

## The calculations (docs/phient/calculations)

phient orchestrates; phical and phistd compute. Batches 1–3 are done and verified.

- [ ] Batch 4, paused: the learner's own math into `phical::learner`. The golden test `the_numbers_on_the_guide_do_not_move` pins its exact losses and score fingerprint, so a move that changes what it learns will fail.
- [ ] Batches 5–9, per `docs/phient/calculations/plan.md`.

## The prose ladder (docs/199–202)

- [ ] The template as TRANSITIONS: each position typed by (its type, the previous word's type), read off the deep channel (docs/202).
- [ ] Re-measure intention purity against the 87.8% baseline (docs/201). phisor's own measure is the new way in.
- [ ] The paragraph alphabet: the kept sentence templates as the next level's symbols (docs/201).
- [ ] Ask the crowd question again under the learner: do the closed-stratum words part into types (`Learner::reached`)?
- [ ] Grade the Key at the back of the guide (docs/192).

## Structure

- [ ] phiano's one remaining dependency cycle: command ↔ drivers (15 modules left, down from 54).
- [ ] Keep every source file under 200 lines (250 at most).
- [x] Diagram pages and a TODO.md for every crate; every ARCHITECTURE.md diagram-only. 190 Mermaid diagrams parse.

## The compiler (crates/phiclm)

- [ ] The CG stage: O-code → Cintcode.
- [ ] Coroutines, `level`/`longjump`, file streams and `sys`, `#(` method application.

## Housekeeping

- [x] Rounds 198–207 committed and pushed (2026-10-29): phinet and the phiano, phient, phiecs, phiclm and phifno submodules.
- [ ] phikit's side is NOT pushed: `Cargo.toml`, a phigpu file, and the untracked `crates/{phifx,philog,simkit}`, with 55 uncommitted files in the phiber submodule and 8 in phiray. That is another session's work in flight; it needs its own read-over before a push.
