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
- [ ] phikit's side is NOT pushed, and is the owning sessions' to push, not this one's. Reviewed 2026-10-29; nothing staged, nothing committed. What a reviewer should know:
  - **Commit order.** phikit's uncommitted `Cargo.toml` adds `crates/{philog,phifx,simkit}` to `members` while all three are untracked, and a missing member path is a hard cargo error — so the Cargo.toml and all three crates must land in ONE commit. `crates/phiber/Cargo.toml` also path-depends on `../philog`, so philog must be committed in phikit before phiber is pushed, and the uncommitted `crates/phigpu/gpui/.../test_context.rs` change must land with phiber's new graph-editor test or that test cannot compile.
  - **Two certain test failures in phiber**, both verified: dropping `BPartType::Spacebar` from the top bar leaves `tests/golden/layout_fingerprint.txt` stale (55 rows still name workspace tabs) and makes `tests/chrome_geometry.rs:65`'s `tab-entity-Layout` lookup panic.
  - **Two decisions for the author**: `crates/phifx` carries 17 doc references to `crates/apple/...`, which is untracked and excluded from the workspace, and is labelled Apache-2.0 beside GPL-3.0 siblings with no root LICENSE; and phikit's `philog` (a log sink) collides by name with phinet's `philog` (this logic crate) — 27 references across 6 files, neither published to crates.io, so the clash is human only.
  - **Every piece has an owner** (settled 2026-10-29, after two wrong guesses of mine). One session owns `philog`, `phifx` and phiray's triangle test; a second owns phiber's top bar, status bar and the runtime-string em-dash sweep; `simkit` is the author's own. phifx came out of a survey of `crates/apple` for wgpu code worth porting, and answers two gaps phiray's ROADMAP §3.1 names.
  - Clean on the mechanical checks: no secrets, nothing over 500 KB, no `phinet` paths, no `todo!`/`dbg!`, no `else if` chains, every `mod` has its file. Nothing in phikit was compiled or tested (a phiber test build is ~95 minutes on this machine).
  - **Corrections to the above, 2026-10-29 04:1x.** phiray's physics module was committed (`18ceb37`, 03:54) and its ledger fixed (`e501bb7`, 04:04) while the review was in flight, and both are pushed, so it is no longer pending — what remains uncommitted in phiray is the watertight Woop/Benthin triangle test (`bvh.wgsl`, `trace/tri.rs`, `tests/preview.rs`), which belongs to the same session as phiber's changes and philog — so one session owns everything pending except phifx. Its lines still apply cleanly on the new phiray HEAD (checked: the `jobs.md` entry, `src/lib.rs:22`, `src/trace/mod.rs:13`). The line-length finding against phiray was wrong: that is this workspace's rule, and phiray's own HEAD already has 14 files over 250 lines (largest 933), so it was never held to it. The same correction applies to simkit. It does stand for phiber, which enforces its own `CAP = 240` with a ratchet list in `tests/level_structure.rs`.
  - **The review found one real bug.** A held orbit arrow in phiber's viewport gizmo never stopped: the press armed a `Cell` that lived one frame, and the repaint the spinning itself caused meant the release cleared a different one, so the view orbited until the window lost focus. Its session has fixed it with a slot that outlives the repaint. It noted that it had tested the arrow by holding it and watching it spin, and never checked that it stops — clicking away to end the test is what hid it. Worth remembering: running it found what reading could not, and reading found what running hid.
  - **Four things wait on the author, not on a session** (2026-10-29):
    1. **`tests/golden/layout_fingerprint.txt`.** Every phiber session's brief says not to regenerate it and to list changed selectors instead, so none will. It is a 4.6 MB file already thousands of lines stale from many sessions' UI work and was failing before any of this round's changes, so re-recording it is one decision for the whole repo, with the written account the file's own module doc demands. (My review told a session to run `UPDATE_LAYOUT_GOLDEN=1`; that was wrong of me, it cuts against the standing rule.)
    2. **`crates/phiber/src/main.rs:80-82`** prints `Read: N objects, M triangles in X.Xs` on every non-`--quiet` headless render, changing `-b` output that scripts may parse. No session claims the line. Keep it, make it opt-in, or drop it.
    3. **The runtime-string em-dash sweep** (about twenty files, `—` → `-`) is half-done and both candidate sessions disclaim it; the files are stamped the same minute as another session's committed reader-page sweep. One session has offered to finish it, including the two places the same diff left em-dashes in. It needs an owner or a revert, since a reader currently sees both conventions with no rule stated.
    4. **`crates/phigpu/gpui/src/app/test_context.rs`** (the `painted_glyphs` / `build_with_text_system` addition) and the phiber test that needs it, `tests/graph_editors_paint_their_text.rs`, are claimed by nobody. The test cannot compile without the gpui change, so the two must land together once an owner is found.
  - Passed on by the phiray session, not yet acted on: phiray's README mode table is stale — it credits Solid to "phiber canvas (today)" and Preview to "wgpu raster", while phiber now maps every shading mode to phiray and Preview is a compute kernel. `docs/ROADMAP.md` §7 lists that drift with eight other cases.
