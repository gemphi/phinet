# phinet — what is left to do

Workspace-wide. Each crate keeps its own list: `crates/<crate>/TODO.md`.

## Learning (docs/203)

- [ ] Beat the smoothed Witten–Bell trigram on held-out sentences (learner 21.3% top-1, perplexity 101.2; trigram 24.8%, 84.7).
- [ ] Train on more text than one book: every model here is starved at 142k words.
- [ ] Try a wider operator block and more channels with more silence (the block-8 run was still improving).
- [ ] Full softmax for the last epochs.
- [ ] Put the learner behind `talk` and the workshop, so nothing that generates runs on counts.
- [ ] Decide `phiecs::Phasor::couple` (the running mean): make it error-driven, or keep it as the counted contrast line only.
- [x] Re-word the paradigm sentence "each a complex phasor on a closed circle with a count" in the current-state files (they now say the words move by the error of what was expected; rounds 001–202 and the papers keep the old wording).

## The prose ladder (docs/199–202)

- [ ] The template as TRANSITIONS: each position typed by (its type, the previous word's type), read off the deep channel (docs/202).
- [ ] Re-measure intention purity against the 87.8% baseline (docs/201).
- [ ] The paragraph alphabet: the kept sentence templates as the next level's symbols (docs/201).
- [ ] Ask the crowd question again under the learner: do the closed-stratum words part into types (`Learner::reached`)?
- [ ] Grade the Key at the back of the guide (docs/192).

## Structure

- [ ] Break phiano's dependency cycle: 25 of its 54 modules use each other (crates/phiano/ARCHITECTURE.md, layer 3).
- [ ] Diagram pages and TODO.md for phistd, phifno, phical, phioml, phiton (the phiclm session has these).
- [ ] Keep every source file under 200 lines (250 at most).

## The compiler (crates/phiclm)

- [ ] The CG stage: O-code → Cintcode.
- [ ] Coroutines, `level`/`longjump`, file streams and `sys`, `#(` method application.

## Housekeeping

- [ ] Commit rounds 198–203: nothing is committed past round 197 in phiano, phiecs, phiclm or the superproject.
