# phisor — the intermediate language between English and the machine

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon.

A sentence is a speech act over a frame. Its function is the force (Searle: assert, ask, direct, commit, express, wish, declare). Its structure is the frame: a predicate, its roles, tense, aspect and modality. With the fillers taken out a frame is a template, as in C++: "a girl was wearing a blue suit" and "a boy was wearing a white cap" are one template with two fillings. A question is a query the machine runs against stored rows.

```text
"a girl was wearing a blue suit"     ASSERT(wear(agent: girl, theme: suit{colour: blue}), past, progressive)
"a boy was wearing a white cap"      ASSERT(wear(agent: boy, theme: cap{colour: white}), past, progressive)
  one template                       ASSERT(wear(agent: $N, theme: $N{$A}), past, progressive)
"who was the first king of France?"  ASK(be(theme: ?who, as: king{order: first, of: France}), past)
```

Below the act, phisor is a code at 16, 32 and 64 levels: a story in mode 16 is each word written as its class, structure only. A sentence's variation is a template in a mode: how it opens and closes, or the longest opening the guide shows twice. One exchange routine (phical's `classes::halve`) codes the words, then, one level up, clusters the variations into kinds of intention by the river (which variation follows which). Both grow only as far as the data pays for. No label is told; the guide's marks and functions are read against the kinds on held-out sentences.

| module | what | state |
|---|---|---|
| `ir` | `Act`, `Force` with Searle's `Point` and `Fit`, `Frame` and its features, `Role`, `Filler`, `Template`, the written form | built, 8 tests |
| `guide` | the English guide's units and its Key, each sentence with its function and mark, docs/203's split | built, 2 tests |
| `code` | three arms at modes 16, 32, 64: the hash (the null), the given dictionary, the learned code | built |
| `kind` | variations (fixed shapes or grown templates with a trie back-off), kinds by the river, the labels read against them, family resemblance | built, 5 tests |
| `units` | names as unit ids, handed to the one routine (`phical::classes::halve`: exchange by halving, a split kept only while it pays) | built, 1 test |
| `english` | the closed words, the chunker, the clause reader, the realiser, morphology | planned |
| `learn` | the one loop: Huberman's stage and measure, better and worse, forcing | planned |
| `ground` | an act → `phiecs::Row`s, the event as an entity | planned |
| `machine` | phiclm's Lisp `Host` over the rows; every operation a `lambda`; `ASK` → expression → answer | planned |
| `measure` | each run's record; the thinkers' table | planned |

Measured 2026-10-29 ([docs/phisor/kinds.md](../../docs/phisor/kinds.md)):
- **Word classes.** The data pays for 53, at depth 7, and they read as parts of speech.
- **Kinds of intention.** The river pays for 6 to 20, and they barely carry what the sentences are for: function purity at most 53.3%, against a 51.7% majority.

The plan, the order and the measures: [docs/phisor/plan.md](../../docs/phisor/plan.md). Diagrams: [ARCHITECTURE.md](ARCHITECTURE.md). What is left to do: [TODO.md](TODO.md). Module pages: [docs/phisor/](../../docs/phisor/).
