# Training — how Phient's world model learns

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle; meaning is a side effect of pairs of words moving under a condition, and they move by the error of what was expected — toward what came, away from what did not.

This document is the approach: what the system is, what "training" means in it, in what order a thing is learned, and by what it is judged. Its companions are `guard.md` (what it must not do, so that it cannot collapse) and `ecs_vs_jepa.md` (how this stands to a joint-embedding predictive architecture). Everything asserted here was measured in a numbered doc of `crates/phiano/docs`, cited in brackets.

## 1. What the system is

The world model is an entity–component system (phiecs) in which the entities are **words** and the components are **relations**. A word is not a vector and not a pixel; it is an entity with a spelling and up to sixty-four channels, one per kind of relation, each channel a phasor (an angle on a closed circle) with a count of the evidence behind it. A relation is a row `(about, kind, other)` — X ⇒ Y under c. Nothing else is stored. The entities and components are already the nodes of a tree, because every row is told **under a condition**, and conditions nest: the sentence in the paragraph in the chapter in the book; the function in the file in the crate; the frame in the scene in the session; the joint in its kind in its module in the body (docs/126, docs/140–141). The tree is the file system of what was said, and it is where a word is looked up and where a prediction is read.

So the model is three things at once, and they are one data structure:

- a **lexicon** of words, each an entity (phiecs `World`, `Lexicon`);
- a **tree of conditions**, each scope a word too, with the counts of what was said under it (phiplm `Counts`, `ScopeTree`);
- a **field of phasors** over the same words and scopes, coupled by the same rows (phiecs `Phasor`, phiplm `Field`).

The three agree because they are written by the same row. That is the whole architecture. There is no encoder, no decoder, no latent variable, and no reconstruction.

## 2. What "training" means

Training is **telling**. A source — a text, a crate, a Phi-4 answer, a tracked face — is turned into rows under the scopes it occurred in, and the rows are counted. One pass, one observation per row, no epochs, no gradient. The verbs are:

1. **Read** the source by its own grammar into rows: prose by tokens and paragraphs (`phiplm::prose::paragraph`, docs/127), Rust by its syntax (`phiplm::code`, docs/108), a framework by its names (`phikit::tell`, docs/141), a frame by what it is sure of (`phikit::body::Face::sentence`, docs/140). The reader adds nothing the source did not say; a name is its words and a frame is a sentence.
2. **Count** every row under every scope on its path (`Counts::learn`): the word's movement (D1), its posts — what came before it (D2) — and the nested distributions, each scope's smoothed toward its parent's (D4; docs/124, docs/126).
3. **Relate** the same rows in the world (`World::relate`): the two words' phasors on that channel move a step of the coupling rate toward each other, and the count rises. This is phiecs's one learning rule — pairs of words moving under a condition — and the `Watch` system runs it one frame at a time (docs/141).

Nothing is fitted in this base. What can be fitted sits **on top of** it and is measured against it: the relation operator (`phical::Fno` over a table of word functions, docs/91–97, docs/137) and the kin reading (docs/138). Both are honest additions: as of docs/137–138 neither beats the frequency prior in the unheard stratum (operator 9.2 %, kin 14.2 % / 0.5 %), and the base alone carries the context tests. The rule, from docs/137 on: build on the base (rows → counts → operators); no side heuristic, no knob, nothing that is not a row or a count or a function of them.

## 3. The order a thing is learned in

The order is the order of telling, and it matters because a condition is set by what came before (D8).

- **One pointing.** A new word is learned the first time it is told: it exists in the lexicon, it has one count under one scope, and it is found again by a cast into that scope (docs/116, docs/125, docs/129 — the context tests: a thing pointed at once is kept over 400 and 2,000 turns of other things, 10/10 · 20/20 · 20/20 at both seeds).
- **Repetition sharpens, it does not create.** The count of a pair grows with each telling, and the phasor pair closes its gap by the coupling rate each time (0.84ⁿ of the gap after n tellings at both ends; docs/141 table: the edge count is right at one showing, the phasor agreement passes 0.94 at twelve).
- **Updating is telling again, later.** A newer scope outranks an older one among equals (the cast's `later` tie-break; phiecs's `Condition`: now over was), so a corrected fact wins without the old one being erased (docs/129, the update test).
- **A teacher cycle** (docs/124 §3, docs/130, docs/137): what the tree does not know is asked (`prose::unknown`), told (`teach`), and the next cast finds it. Learning speed is the number of tellings a thing needs before it is found, and the measure of the system is that number, not a benchmark score (Piaget, docs/140: intelligence is what you do when you don't know).

## 4. Prediction: a cast, not a reconstruction

A prediction is a **cast** into the tree (docs/126, docs/129): from the roots down, enter the children that say most of the asked words beyond their parent, and read the next word where the path ends, by the nested distributions. The prediction is a word under a condition — never a surface, never a pixel. What a cast cannot say it does not pretend to: the held-out Emma run reads 20.6 % next-word accuracy at perplexity 159, split into a heard stratum (77 % of positions, where the condition was seen) and an unheard one (23 %) where the base falls back to the prior (docs/137). The strata are reported, not averaged away.

The same cast is drawn by cosine in the phasor field (docs/136), where it holds a superposition — 11, 14, 16, 18 of 20 things in one scope against the counts' 20 of 20 — which is the trade the field makes: smaller, abstract, slower to learn, less exact.

## 5. What is judged, and how

Before anything is asserted it is measured, in a numbered doc, with the command that reproduces it. The measures are: the context tests (kept over turns; docs/129); held-out next-word accuracy and perplexity by stratum (docs/137); the number of tellings to find a thing (docs/130, docs/141); capacity per scope (docs/136); and the store per word (2.8 KB/word, the first wall, docs/132–133 — still standing). A result that improves one of these and worsens another is reported with both numbers. A change to the cast that breaks the update test is reverted (docs/140 §2, two such).

## 6. The body, as the newest source

phikit (docs/140–141) is the same approach on a different source: the body's shapes and joints, tracking's anchors and sessions, the device's registry, ported into our words and told as rows by one mechanism, so that a persona learns its body from the frames a device reports — each frame a sentence of what the face or hand is sure of — exactly as it learns a story, and a `System` watches those frames into the world as pairs of words under `near` and `before`. No frame is reconstructed; what the face will do next is read as what followed it before.

```
source ──read──▶ rows (about, kind, other) under scopes
rows ──count──▶ Counts: D1 movement, D2 posts, D4 nested distributions
rows ──relate──▶ World: phasors coupled per channel, with evidence
ask ──cast──▶ the scope that says the asked words, the next word read there
```

`cargo run --release --bin context -- --turns 2000 --seed 2` · `cargo run --release --bin story -- --heldout 0.05` · `cargo run -p phikit --example watch -- 12`
