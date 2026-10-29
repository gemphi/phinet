# phisor — the intermediate language between English and the machine — plan

**Why.** On 2026-10-29 the system does not know English. The learner predicts the next held-out word at 21.3% top-1, below the smoothed trigram's 24.8% (docs/203). The templates of docs/200 cover 76% of the guide, but their types are the crowd, not parts of speech. No component parses English into structure, and nothing runs from English to the machine. phisor is that middle: English → **a code at 16, 32 and 64 levels** → **templates** → **an act over a frame** → rows → the machine.

The user's words, from three messages:
- "the most important thing is to have an intermediate state between languages … the CLM based model is going to rely mostly on templates, like C++"
- "phisor language (intermediate code at 64 different levels) … give me this story in mode 16, which [has] linguistic structures only, not the actual words"
- "you could safely assume that there's at least 64 ways of asking a question, then you create a clever template that captures and senses a question … then 64 ways to dance in the rain … see how these choices of variations relate … and see that language is the real thing, not you and me"
- "the main idea is to learn how to learn, not to assume we know … but we need the backward mirror … compare the twin, and the difference engine wins … small machines"

## The ladder

phisor is one language at several heights. Each height's alphabet is the kept shapes of the height below (docs/201's ladder):

| height | a sentence written as | example |
|---|---|---|
| words | its words | `were you waiting in the rain` |
| **mode 64, 32, 16** | each word's class at 64, 32 or 16 levels: structure only | `c9 c2 c14 c5 c1 c11` (mode 16) |
| **variation** | the shape a sentence opens and closes with | `c9 c2 … c11` |
| **kind** | the variations that play one part in the flow, found by the river | kind 17: its variations, one of them `c9 c2 … c11` |
| **act** | a force over a frame | `ASK(wait(agent: you, in: rain), past, progressive)` |
| rows | `(about, kind, other)` | `e17 is-a wait · e17 in rain@17 · e17 force ask` |

**Mode 16 is learned, not hashed.** phiton's `P16::classify_str` hashes a word's spelling (multiply and add over its bytes) modulo 16. Its sixteen classes are random buckets of spellings: `the` and `a` fall apart by chance, and `cat` and `when` can fall together. A story written in that mode is noise. It becomes the **control**: whatever a run finds must beat the hash's classes on the same sentences.

The classes are learned as one binary code per word, six bits deep. Cut at 4, 5 or 6 bits it gives 16, 32 or 64 nested classes, as `Level::{P16, P32, P64}` and their `bits()` already say. Mode 16 is mode 64 coarsened. The code is grown top-down by halving. Each half is found by the exchange algorithm (Brown et al. 1992; Martin, Liermann and Ney 1998), which is trial and error: move one word to the other half, and keep the move if the training text's class-bigram likelihood is better. Three arms are written in the same modes:

- **hash** — the null: phiton's `classify_str`.
- **given** — phiton's closed-class dictionary (`p64::syntax::PosDictionary`, 11 parts of speech with a suffix fallback), the book's grammar read by hand.
- **learned** — the exchange code at 16, 32 and 64.

## The first experiment: the kinds of intention and their variations

The user: "you could safely assume that there's at least 64 ways of asking a question … then 64 ways to dance in the rain … see how these choices of variations relate". And then: "the questions were just an example … I can't give every example of types of intentions … you must learn to abstract my instructions". So no kind is chosen by hand. The rule the example stands for: **every kind of intention comes in variations, at any level, and the kinds are to be found, not named.** A question is one kind the run is checked against, never one it is fitted to.

1. **Words → the code.** The 6-bit word code is learned by exchange on the training sentences (above).
2. **A sentence → its variation.** Each sentence is written in a mode. Its shape is how it opens and how it closes: its first two classes and its last. At mode 16 that is one of 16³ = 4,096 possible variations, structure only.
3. **Variations → kinds.** The variations are clustered into up to 64 kinds by the same exchange routine one level up. The units are now sentence shapes, and the pairs are which shape follows which in the guide's order (the river). It is one routine at two heights. A kind is the variations that play the same part in the flow: structure (the shape) grouped by function (what comes around it).
4. **Never told, then checked.** On held-out sentences, every label the guide carries is read against the kinds, and no label is told to anything:
   - the terminator: statement, question or exclamation; the mark is gone from the words;
   - docs/201's function: explain, instruct, answer. The answers are the Key's, and the Key is never told, so every answer is held out.
5. **Report.** Each kind with its variations and examples; how many kinds are occupied and how many variations each holds, at modes 16, 32 and 64.

**How the variations relate.** Kind × kind over consecutive held-out sentences, each paired with the sentence before it, is read as mutual information against 1,000 shuffles, as a z-score. The commonest variation pairs are listed: which way of doing one thing is followed by which way of doing another.

Bars, fixed before the run (every arm in the same modes):

| | bar |
|---|---|
| I1 | function purity of the learned kinds on held-out sentences ≥ the majority baseline + 5 points (docs/201's condition: 87.8% with the crowded sector types, no effect) |
| I2 | the learned code beats the hash null's purity by ≥ 5 points, at the same number of kinds |
| I3 | the terminator purity beats its majority baseline: the kinds sense questions and exclamations they were never told about. The best kind's question F1 is reported |
| I4 | the river: kind-to-kind mutual information on held-out pairs at z ≥ 3 against the shuffles, and above the hash's |
| I5 | the counts: kinds occupied and variations per kind at modes 16, 32 and 64. "64 at any level" is supported if the counts agree across modes and the hash's do not |

**Learning how to learn.** One routine learns everything here: propose a move, keep it if better and not worse. At the words it learns the code, and at the sentences the kinds. Later it learns the phases (below) and the open lexicon (below). Its settings are chosen once at the word level and run unchanged at the sentence level. If the second height learns with no new setting, the routine has learned how to learn.

## The thinkers, measured

Each claim is read at practical value and tested by a small run on the guide, recorded straight. Every run is a small machine.

| thinker | the claim, read for phisor | the run | recorded |
|---|---|---|---|
| Searle | function lives in structure | the kinds, found from shapes and the flow, read against the guide's function labels and its marks (I1, I3); later force against the labels (M4) | purity; F1 |
| Wittgenstein (papers 19–23) | family resemblance (§66–67): no one feature common to every member of a kind, a network of overlapping ones | within each kind: the largest share of its variations that hold one class at one place in common, and whether their overlap graph is connected | the share; connected or not |
| Bentov (standing waves) | a bounded medium sustains only whole modes: 64 channels, 64 ways, "at any level" | kinds occupied and variations per kind at modes 16, 32 and 64 (I5), and the rank-frequency slope at each | the counts; the slopes |
| Bentov (the mirror), docs/153 | the wave read backwards reveals what was hidden | the twins: a variation read from its opening alone, from its closing alone, and from both | purity for each |
| Babbage (the difference engine) | compute by differences | the likelihood gained pass by pass; the routine stops where the first difference falls below a thousandth of the first pass's; the twins' disagreements, listed | the passes; the stop |
| Lovelace (Note G) | the engine "has no pretensions to originate anything" | chains of variations walked along the learned river: the share never told in training that occurs in held-out text | novel and attested |
| Feynman (amplitudes) | add the amplitudes, then square | a sentence's kind read from both twins, combined as probabilities added and as amplitudes added then squared, with phases learned by trial and error; the second run | purity for each |
| Whitehead (Principia, the event) | relations are primitive; the event is an occasion | the act's rows, and a question answered by the relation algebra alone (phiclm's Lisp is Russell and Whitehead's) | M6 |
| von Neumann (the stored program) | one description both reads and writes | each kind compiled to a small program on phiclm's machine, which must assign exactly what the Rust reader assigns; the same kind generating | agreement, 100% |

**The joint, later.** The kinds and their variations become a table of functions trained together with an operator (`phical::joint`). The table is a parameter there, so the templates move under the objective. This comes after the counted version has shown what there is to learn. Recoding in Feynman's sense comes later too: an act bound into one phasor, role ⊛ filler (`phiton::hrr`), and read back by the conjugate, the mirror.

## The act: a force over a frame

The top of the ladder is a **speech act** (Searle: "the most important thing is structure and function") over a **frame**. It is built and tested (`ir`):

| piece | what it holds |
|---|---|
| `Act` | a `Force` over a `Frame` |
| `Force` | `ASSERT`, `ASK`, `DIRECT`, `COMMIT`, `EXPRESS`, `WISH`, `DECLARE`; each maps to one of Searle's five points and its direction of fit |
| `Frame` | the predicate, its roles, tense, aspect, modality, voice, polarity |
| `Role` | `agent`, `theme`, `recipient`, `as`, and a preposition's own name until roles are learned |
| `Filler` | a thing, a name, a pronoun, a bare word, a hole (`?who`), or a clause: a frame as an argument, so operators take operators |
| `Template` | the act with its fillers blinded (`$N{$A}`), and the construction with the predicate blinded too |

```text
"a girl was wearing a blue suit"     ASSERT(wear(agent: girl, theme: suit{colour: blue}), past, progressive)
"a boy was wearing a white cap"      ASSERT(wear(agent: boy, theme: cap{colour: white}), past, progressive)
  one template                       ASSERT(wear(agent: $N, theme: $N{$A}), past, progressive)
"who was the first king of France?"  ASK(be(theme: ?who, as: king{order: first, of: France}), past)
"I wish I could sing"                WISH(sing(agent: I), past, can)
"I want to go to London"             ASSERT(want(agent: I, theme: [go(agent: I, to: London), present]), present)
```

The sentence gives the structure; the world gives the meaning. The label `colour` is read from the row `blue is-a colour`. Reading English into acts is step 5. It uses the given closed words and the clause skeleton, translated from the book as Huberman translated her chess books into `better` and `worse`. The open words' classes are learned by the same loop.
- **A position** is the open lexicon.
- **Its stage** is the number of training sentences with a full parse.
- **Its measure** is the lexicon's size.
- **A move** (one word gains or drops a class) is played only if it is better and not worse against every sentence the word occurs in.

Its measures are carried over from the first plan:

| | measure | bar |
|---|---|---|
| M1 | held-out coverage | learned ≥ baseline + 10 points |
| M2 | round trip: realise(act) = the sentence | 100% |
| M3 | 150 gold frames by hand, frozen before the learner | learned > baseline on every field |
| M4 | the guide's labels' purity inside the forces | ≥ 92.8% |
| M5 | constructions covering the family | reported |
| M6 | the loop, a question answered | a test |

## Grounding and execution

An act lands as rows, with the event as an entity:
```text
e17 is-a wear · e17 agent girl@17 · girl@17 is-a girl · e17 theme suit@17 · suit@17 mod blue · e17 tense past
```
`ASK` compiles to phiclm's Lisp. The user's ruling, given in the phiclm session: "every language operation is a lambda expression in lisp underneath". So each phisor operation lowers to a `lambda` applied to its arguments, bound by `define`. phisor is the `Host`, and its primitives, such as `(rows kind)`, are the leaves those lambdas call. A host's names may not repeat the 39 core names (`Machine::with_host` asserts it; docs/phiclm/lisp/spec.md). The query below was checked against the set algebra's definitions:

```lisp
(domain (meet (identity (referents (rows is-a) king))
              (meet (identity (referents (rows of) france))
                    (identity (referents (rows order) first)))))   ; → #class[1]{clovis}
```

Later: `DIRECT` becomes a program for the judge, and `WISH` a goal for `World::plan` (top-down, "I wish I could do this").

## The data and the split

- **The guide.** `crates/phiano/data/person/murphy.txt`, split by docs/203's rule exactly, on the same sentence cut: `i % 10 == 0` is test, `i % 20 == 5` is dev, the rest is train. So phisor's held-out sentences are the learner's.
- **The Key.** `murphy_key.txt` is never told.
- **The labels.** docs/201's layout reader.

## Calculations, asked of the parent session

phisor counts and matches. The arithmetic goes to phical, or phistd for generic numerics:

| what | proposed home |
|---|---|
| better and worse on a stage and a measure (Huberman 3.6, 3.8) | `phical::liskov::staged` |
| the class-bigram likelihood from counts, and the gain of moving one word | `phical` (a `classes` module, or `smooth`) |
| mutual information of a count table, and its shuffle z-score | `phistd::stats` |
| amplitudes added and squared over a set; phases moved by better and worse | `phical` |
| F1 | `phistd::stats::f1`, already there |

## Where it sits

phisor → phistd, phiton (levels, the hash null, the given dictionary), phical, phiecs, phiclm. Each dependency is added when its module lands. It does not depend on phient until the critic does, and never on phiano: phiano reads phisor.

| module | what |
|---|---|
| `ir` | the act: built |
| `guide` | the sentences, the split, the labels, the Key |
| `code` | the three arms at modes 16, 32 and 64; a story written in a mode |
| `units` | names as unit ids for the one routine; the routine itself is `phical::classes::halve` (word codes and sentence kinds alike) |
| `kind` | a sentence's variation (its shape), the kinds found by the river, the twins, reading the labels |
| `english` | the closed words, the chunker, the clause reader, the realiser (step 5) |
| `learn` | the one loop, better and worse, the open lexicon |
| `ground`, `machine` | rows; the Lisp host and `ASK` |
| `measure` | each run's record; the thinkers' table |

## Order

1. **The plan and the IR.** *Done; the user corrects this plan before step 2.*
2. **The guide reader and the split.**
3. **The code.** The three arms at 16, 32 and 64, with the guide's first unit printed in mode 16. The exchange likelihood is asked of the parent first.
4. **The kinds of intention and their variations:** I1–I5, the twins, Babbage's passes, Wittgenstein's overlap, Bentov's counts. Report: [kinds.md](kinds.md). **This is the first measured slice**, done 2026-10-29. The word code finds parts of speech, but the kinds barely beat the majority, and not the hash by the bar. Feynman's amplitudes follow as its second run.
5. **The act from English**; M1–M5.
6. **Grounding and the machine** (M6, Whitehead); the kinds as small programs on the judge (von Neumann); Lovelace's test.
7. **The thinkers' table filled**, as `docs/phisor/thinkers.md`.

## Risks

- **The hash.** If `classify_str` is meant to be a class and not a hash, the user should say. Written as it stands, mode 16 carries no structure.
- **Given against learned.** The given dictionary and the clause skeleton are the book's, translated by hand; the code and the kinds are learned. Each run reports the arms side by side.
- **The labels are checks, not targets.** The `?` mark, which the PDF layout keeps, and docs/201's layout labels are what the guide happens to carry. The kinds are found without them, so a kind that matches no label is reported, not discarded.
- **Sixty-four is assumed, then tested.** The kinds may fill far fewer than 64. That is reported straight, and it is what I5 is for.
- **Two cuts.** The labels need docs/201's layout reader, a different sentence cut from docs/203's. The kinds use docs/201's cut with docs/203's split rule (`i % 10 == 0` held out), and the Key's answers are all held out.
- **Coordination.** phisor does not edit `phient/src`, `phical/src` or `phistd/src`. The machine is busy, so builds run at low priority, one at a time.

Diagrams: [architecture.md](architecture.md). Checklist: [jobs.md](jobs.md).
