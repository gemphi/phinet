# Guard — what the system must not do, so that it cannot collapse

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle; meaning is a side effect of pairs of words moving under a condition, and they move by the error of what was expected — toward what came, away from what did not.

A joint-embedding system collapses when its encoder learns to ignore the input and emit a constant: the prediction error goes to zero and nothing was learned. Its whole discipline is the prevention of that — information maximisation, a differentiable surrogate for the information content of a matrix whose rows are samples and whose columns are variables, made informative by making the rows different (contrast) or the columns different (variance and decorrelation). This document lists what *our* system must not do. Its matrix is different in kind — the rows are words, the columns are conditions, and every cell is a **count** — so the classic collapse (an encoder emitting a constant) has no place to happen: there is no encoder. But a counting tree has collapses of its own, each of which has been met and measured in a numbered doc, and each has a guard that is a rule of the system, not a knob.

## 1. Prior collapse — every cast falls to the most frequent scope

The trivial solution of a tree is the unigram: answer the most common word, enter the biggest scope. It is the exact analogue of the constant encoder, and it is what a cast does if scopes are ranked by how much they say rather than by how much they say *beyond their parent*.

**Guard (D3, locality; D4, nested).** A scope attracts an asked word only beyond its parent: the child's distribution is smoothed toward the parent's and the attraction is `ln(p_child / p_parent)`, so a word every child says draws nowhere; and a word said by more than half the siblings counts for none of them (docs/129 §2, `ScopeTree::attraction`). Measured in docs/129: without locality a turn that happened to say *is* four times outdrew the turn that said the key; with it, `alpha` said by two turns of two thousand draws exactly, 10/10 · 20/20 · 20/20 at both seeds. In phikit the same rule discounted `hand` at the kind level where most kinds say it (docs/141 §2).

**Must not:** rank by raw count; drop the parent's base; let a root scope say words.

## 2. Depth collapse — a longer path draws more

If asked words are summed along a path, a scope four levels deep that says a word at each level outdraws one that says it once; the cast then prefers depth to fit, and every ask ends at the deepest leaf.

**Guard: uniform depth per kind of thing.** Every thing is one scope under its kind; the tree is domain → module → kind → thing (docs/140 §2, docs/141 §1). Two alternatives were tried and reverted because they broke the update test: counting an asked word once per path (13–15 of 20) and ranking by the leaf's own saying first (stops at the parent). The remaining honest residue is recorded: a longer *name* that says the asked words itself still outdraws a shorter one that inherits them from its kind (`left hand thumb tip` over `thumb tip` for `hand thumb tip`), which is the rule working, not failing.

**Must not:** give one kind of thing scopes of unequal depth; sum a word's saying across levels that are the same thing restated.

## 3. Scope explosion — every occurrence its own condition

The opposite of collapse is dilution: if every token occurrence is its own scope (`w@path#i`), the counts are exact and the store grows at 2.8 KB a word (docs/133 — the first wall, still standing). Exactness is bought with memory, and a tree that remembers everything predicts nothing new (the unheard stratum: 23 % of held-out positions fall back to the prior, docs/137).

**Guard: measure the store with the accuracy.** Every doc that changes the rows reports KB per word beside the accuracy; nothing may lower one without reporting the other. The phasor field (docs/136) is the compression under test — 11–18 of 20 in one scope against the counts' 20/20 — and it is an *addition* over the counts, never a replacement, until it matches them.

**Must not:** replace counts with a summary that has not been shown to find what the counts find; hide the store behind an accuracy.

## 4. Phasor collapse — a hub pulls every angle to itself

phiecs's learning rule moves two phasors toward each other on a channel. A hub word related to everything (`the`, `is`, a root scope) is pulled from all sides to the mean and pulls everything a little toward it; given time all angles on a busy channel meet, and agreement is 1 everywhere — the constant encoder, arrived at by coupling.

**Guard: evidence is kept, silence is not agreement, hubs are conditions.** `agreement` is `None` when either word has no evidence on the channel (docs/141: `smile ~ brow down` is `None` at every repetition); `resonance` weights channels by the smaller evidence; and a word said by every sibling is a condition, not a relation (§1's locality rule applied to the world: it is `declare`d as a kind, not related as a word). Measured so far: six words over 288 frames, related pairs above 0.99, unrelated below −0.99 — no hub was present. The hub case is **not yet measured** on a real corpus in the world; it is the first thing to measure when `Kit::world()` is run over *Emma*.

**Must not:** relate a word to a scope on a word channel; average angles without evidence; read agreement where there is none.

## 5. Schema collapse — telling the system what the source did not say

The quietest collapse: a hierarchy typed in by hand, a parent list we believed, a group membership we assumed. It makes the first test pass and the tenth fail, because the tree then holds our prior rather than the source's conditions.

**Guard: nothing hard-coded; a name is its words; a frame is a sentence.** The port of phikit declares no skeleton parents (the binding does not set them, the headers do not list them; `Bone::parent` is what a device reports); the hand skeleton's parent is read along the finger from the names, and that one inference is tested (docs/141 §3). Groups and regions are read from the words (`Joint::group`, `Shape::region`). What a reader adds beyond the source is listed under "what was not done, honestly."

**Must not:** add a parent, a group, a synonym or a weight the source did not state; strip a word because it looks redundant.

## 6. Knob creep — a heuristic beside the base

Every round offers a shortcut: a threshold tuned to the test, a special case for commas, a second ranking when the first ties. Each is a column added to the matrix by hand, and each is a place the system stops being one data structure.

**Guard: build on the base, measure before asserting, revert what breaks the earlier test.** Rows → counts → operators; a change must be expressible as a row, a count or a function of them (docs/124, docs/137). Two cast changes were reverted in docs/140 for breaking the update test; docs/138 §3 corrected an asserted cause ("a rare word before a comma") that the measurement did not support (20 % top-20, 62 % outside top-100). An assertion without a number in a numbered doc is not made.

**Must not:** tune to a test; add a parameter without a doc that measures it on and off; say the approach "doesn't work" or "works" without the stratum it was measured in.

## 7. The information content, measured rather than maximised

The joint-embedding system cannot measure the information in its representation because it has samples and no distribution. Our matrix is a contingency table: `count(word, condition)` over a finite lexicon and a finite tree, so its information content is **computed**, not estimated — the nested distributions are the columns' entropies, `ln(p_s / p)` the information a scope adds beyond its parent (D4), and the strata of docs/137 the information that was not there. The rows are different because words are discrete and told apart by spelling; the columns are different because conditions nest and a child's column is read only beyond its parent's. Nothing is maximised. What must be guarded is that the counts stay counts — §§1–6 are the ways they would stop being so.

| collapse | the trivial solution | the guard | measured in |
|---|---|---|---|
| prior | the biggest scope wins | locality (D3), nested base (D4) | docs/126, 129, 135, 141 |
| depth | the deepest path wins | one scope per thing, uniform depth | docs/140, 141 |
| explosion | every occurrence a scope | store reported with accuracy; field as addition | docs/133, 136, 137 |
| phasor | every angle meets the hub | evidence kept; silence is not agreement; hubs are conditions | docs/141 (hub case open) |
| schema | our prior in the tree | nothing hard-coded; a name is its words | docs/140, 141 |
| knob | a column added by hand | build on the base; measure; revert | docs/124, 137, 138, 140 |
