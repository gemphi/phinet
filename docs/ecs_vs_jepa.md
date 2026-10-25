# ECS vs JEPA — a tree of relations against a joint embedding

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

The argument for the joint-embedding predictive architecture (JEPA; LeCun's position, as the user relayed it) runs: a world model must not reconstruct — a system trained to predict Y in its minute details is killed by the details it cannot know; predict instead in a representation space, encode X and Y and predict the encoding of Y from that of X and the action A, so the representation can *drop* what is not predictable; train it self-supervised, two views or a corrupted view; and since such a system can collapse to a constant, prevent collapse by maximising the information content of the representation — for which there is no proper measure, because the distribution is unknown and only samples are in hand, so a differentiable surrogate is used (variance, covariance, contrast: the rows of the sample-by-variable matrix made different, or its columns). Every piece of that argument has a counterpart in our system, and the comparison is worth making precisely, because on three of the five points the entity–component tree is in a *better* position by construction, on one it is the same, and on one JEPA does something we do not do at all. Each claim below is tied to a measurement in `crates/phiano/docs`.

## 1. Side by side

| | JEPA | Phient's ECS tree |
|---|---|---|
| **What is predicted** | the *representation* of Y, `s_y = Enc(Y)`, from `s_x = Enc(X)` and the action | the next **word** under a **condition**: a cast into the tree, the word read where the path ends (docs/126, 129) |
| **Where prediction happens** | a learned latent space | a given symbolic space: the lexicon, and the tree of conditions it was told in |
| **The representation of Y** | learned; the encoder may drop the unpredictable | not learned: a word *is* its relations under conditions (an entity with 32–64 components), and what was not told is not there to drop |
| **Training signal** | prediction error in latent space + an anti-collapse term | none — one row per observation, counted; nothing is minimised (training.md §2) |
| **Learns from one example** | no: an encoder needs many | yes: one pointing, kept over 2,000 turns (docs/129, 10/10 · 20/20 · 20/20) |
| **Collapse** | the encoder emits a constant; prediction becomes trivial | no encoder, so not that; the tree's own collapses are prior, depth, explosion, hub, schema, knob (guard.md) |
| **How collapse is prevented** | information maximisation, with a differentiable surrogate (VICReg, Barlow Twins, SIGReg, …) | rules of the structure: locality (D3), nested base (D4), uniform depth, evidence kept — none differentiable, none a loss |
| **Information content** | not measurable: samples, no distribution; estimated | **computed**: the matrix is `count(word, condition)`, a contingency table over finite sets; its entropies and `ln(p_s/p)` are exact (docs/124, 126) |
| **Rows and columns of the matrix** | rows = samples, columns = variables; made different by a loss | rows = words, different by spelling; columns = conditions, different because they nest and a child is read beyond its parent |
| **What it cannot do** | say what it dropped; learn a fact once; show its distribution | perceive: it has no encoder from pixels; it starts where a device (or a teacher, or a text) already speaks in words |
| **A constraint or an objective** | a small head trained per constraint or task, from few samples | a word to avoid, a word to reach: `World::plan_avoiding`; nothing trained (paper 26 §11) |
| **Reward** | minimised; used on top of the representation | none anywhere; a teacher is asked, and the measure is tellings-to-find |
| **Abstraction** | in the latent space, by what the encoder keeps | in the tree, by the level a word is read at (D4); in the field, by superposition (docs/136: 11–18 of 20 vs the counts' 20/20) |
| **Measured on** | image/video representations, downstream heads | held-out text (20.6 %, ppl 159, strata 77/23; docs/137), context tests (docs/129), the body (docs/141) |

## 2. Where the tree is better, by construction

**It never reconstructs, and never had to be told not to.** JEPA's first move is a renunciation: do not predict Y in detail. The tree cannot predict Y in detail, because it holds no detail — a word is what it relates to and under what, and a prediction is a word under a condition. The thing JEPA achieves by designing an encoder that drops the unpredictable, the tree has because its unit is already the abstract one: `mouth smile left` is a word, not fifty-two floats, and a frame is the sentence of what it is sure of (phikit, docs/140–141). The cost of this is stated in §4: someone must have done the perceiving.

**Its information content is a number, not a surrogate.** The question LeCun leaves open — how to measure the information in a representation from finite samples — does not arise for a contingency table. `count(w, c)` over a finite lexicon and a finite tree *is* the distribution; D4's `ln(p_child(w) / p_parent(w))` is the information a condition adds about a word, in nats, and the strata of docs/137 (77 % heard, 23 % unheard) are the measured information that was and was not there. Nothing is maximised; what is guarded is that the counts stay counts (guard.md §7). This is the point on which the architectures differ most: JEPA needs a loss to make its matrix informative; the tree's matrix is informative or it is not, and one can read which.

**It learns from one telling.** JEPA, like every encoder, learns a representation from many samples; a fact seen once is noise to it. The tree holds a thing from the first row (docs/116, 125, 129), updates it by telling it again later (now over was), and asks for what it does not know (docs/124 §3, 130). The measure of the system is the number of tellings a thing needs — Piaget's measure, speed of learning, not a benchmark (docs/140). On the body: the edge count under `before` is right at one showing, where the phasor's agreement needs twelve (docs/141 table). That table is also the honest limit of the phasor half of our system: where we use a continuous summary, we pay what JEPA pays.

## 3. Where it is the same

**Prediction in a representation space, and abstraction by dropping.** JEPA predicts `s_y` not `Y`; the tree predicts a word read at a *level* — the paragraph's distribution smoothed toward the chapter's toward the book's (D4) — and reads higher when the lower level has nothing, which drops exactly the detail the lower level could not predict. The phasor field (docs/136) is the closer analogue: scopes as phasors, a cast by cosine, several things held in one scope at once — a learned-looking latent that is in fact the sum of seeded word functions, with capacity measured against the counts and found short (11, 14, 16, 18 of 20). JEPA's latent is trained; ours is composed. Both are abstract and both are inexact, and ours is measured against an exact baseline it is an addition to.

## 4. Where JEPA does what we do not

**Perception.** JEPA's encoder turns pixels into a representation; it is a theory of how to get from the world to something that can be predicted. The tree has no such theory. It starts where something already speaks in words: a text, a teacher (Phi-4 in the quests), a device (ARKit's face tracker giving fifty-two named coefficients; phikit reads them into our words at the edge and nowhere else, docs/141). The honest statement of the relation is therefore not rivalry but position: **a JEPA encoder is a candidate front end to the tree**, and the tree is a candidate world model *behind* a JEPA — the place its representations could be told as words under conditions, counted, and have their information read exactly. Apple's tracker is the encoder phikit uses today; a learned one would be another source of sentences, and the tree would not change.

## 5. The one architecture, drawn

```
JEPA                                  Phient's ECS tree
────                                  ─────────────────
X ──Enc──▶ s_x ─┐                     source ──read──▶ rows (about, kind, other)
                ├─Pred──▶ ŝ_y          rows   ──count─▶ Counts under nested scopes      ─┐
Y ──Enc──▶ s_y ─┘   ↕ latent loss      rows   ──relate▶ World: phasors per channel     ─┤ one data structure
      + anti-collapse term             ask    ──cast──▶ the scope that says the words   ─┘
                                                        the next word, read there
information: estimated from samples    information: computed from counts (D4, strata)
collapse: constant encoder             collapse: prior / depth / explosion / hub / schema / knob (guard.md)
learns from: many samples              learns from: one telling; sharpened by repetition
front end: learned encoder             front end: whatever already speaks in words (a text, a teacher, a device)
```

## 6. What this comparison does not claim

It does not claim the tree perceives, generalises past the unheard stratum (23 % of held-out positions fall to the prior, docs/137), or that its continuous half (the field, the operator, the coupled phasors) does better than frequency where the counts are silent — docs/137–138 measured that it does not yet. It claims what is measured: that a world model whose entities are words and whose components are relations under nested conditions predicts without reconstructing, learns from one telling, has no encoder to collapse, and has an information content one can compute rather than maximise — and that where it uses a continuous summary of itself, it inherits exactly the costs JEPA describes, and reports them.

Read with `training.md` (the approach) and `guard.md` (the collapses and their guards). Measurements: docs/116, 124–126, 129, 133, 135–138, 140–141.
