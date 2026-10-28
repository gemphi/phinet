# phinet — notes

The prose that was on the architecture page, kept whole. The architecture itself is diagrams: [ARCHITECTURE.md](../ARCHITECTURE.md).

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle; meaning is a side effect of pairs of words moving under a condition, and they move by the error of what was expected — toward what came, away from what did not.

## The crates, as dependencies run (downward only)

An arrow means "is depended on by". Nothing points upward; the brain (phient) sits between the world and the instrument because it must not know the applications — it knows the MODELS (the arena: counts, concepts, phasors, mirror, operator, learner), the protocol (predict first, learn after) and the trees of knowing, and phiano plugs its readers into that arena from above. phiecs additionally dev-depends on phient for tests only (cargo permits the cycle there).

## Where learning lives — counted, and corrected (docs/203)

Both sides hold the same kind of thing — a word as unit phasors on closed circles, a reaching role and a reached role. What differs is the rule that moves them. A running mean has no way to be wrong, so a word reached from everywhere sits at the mean of everything; an error has a sign, so the angles are pushed apart where the text says they differ.

## Two compilers, one ladder

The compiler half (`crates/phiclm`) and the prose half (`crates/phiano`) are the same architecture, BCPL's: LEX → SYN → TRN (compression to a single-type intermediate) → CG (expansion) → the machine, with one numbering shared by every phase (`phiclm::manifest`). For a program the rules are written and the judge is exact; for a sentence the rules must be learned, and the TRN step is the template — the sentence's sequence of types. The diagrams, the BCPL-to-ours map, what each half knows and from which corpus: [crates/phiclm/ARCHITECTURE.md](crates/phiclm/ARCHITECTURE.md).

## The repositories, as submodules run

phiecs appears twice on purpose: one repo, two superprojects — phinet consumes it as the world under the models, phikit owns it as the kit under the realities. In-tree in phinet (no repo of their own): phistd, phical, phiton.

## The single layer (docs/197)

One phasor world is the base everything else is a cheap condition on: a feature is a channel (a construction family opened lazily), an action is a kit row (typed, told, never scraped), and **phient is the orchestrator** — it holds the arena, knows its inventory (a phiano in phinet, a phiora database), routes an ask to the model that answers it, and reaches an outside oracle only as overflow. The same shape Apple runs Siri and its utilities on — one base model, LoRA adapters per feature, a declared App-Intents vocabulary, a router — arrived here by the paradigm's own sentence.

## Where the honest numbers live

Every crate's README states what is measured; the round-by-round record is `crates/phiano/docs/001–203`, conditions of satisfaction fixed before each run, the numbers against us included: phasors lead the glosses arena 32.9% vs counts 29.9% and trail counts on prose 13.9% vs 21.4%; Key completion 15.7% vs 20.1%; a whole book told in 3.6 s, no retraining.

Round 203, held-out sentences of the English guide (14,831 test positions, told to no model): the counted phasors 13.6% top-1, the count of the word before 17.1%, **the error-driven learner 21.3% (top-5 40.9%, perplexity 101.2)**, a smoothed Witten–Bell trigram 24.8% (perplexity 84.7). The learner beats both counted readers and does not yet beat the best of counting.
