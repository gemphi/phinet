# phisor · the kinds of intention and their variations — the first measured slice

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon.

The user's correction set the experiment: "the questions were just an example … I can't give every example of types of intentions … you must learn to abstract my instructions". So no kind is chosen by hand.

**The routine.** One exchange routine (`phical::classes::halve`, phisor calling it) runs twice:
- it codes the guide's words into a 6-bit code, read at 16, 32 and 64 classes;
- one level up, it codes the sentences' variations into up to 64 kinds by the river, which variation follows which.

**The variation.** A sentence's variation is its shape in a mode. There are three twins: the opening (first two classes), the closing (last two), and both ends (first two and the last).

**Never told, then checked.** No label is told. On the held-out sentences the guide's own labels are read against the kinds:
- the function: explain, instruct, answer; the answers are the Key's, all held out;
- the mark: stop, question, exclamation, line, on the units' sentences only.

**Three arms.** The hash (phiton's `classify_str`) is the null, the given dictionary (`PosDictionary`) is the hand, and the learned code is the third arm.

```text
cargo run --release -p phisor --example kinds -- crates/phiano/data/person/murphy.txt --key crates/phiano/data/person/murphy_key.txt --shuffles 1000
```

18,822 sentences: 14,777 train, 869 dev, 3,176 held out. The held-out sentences break down as:
- **by function:** explain 1,641, instruct 98, answer 1,437;
- **by mark, units only:** stop 1,128, question 187, exclamation 3, line 421.

The majority baselines are 51.7% for function and 64.9% for mark. The full output is in [runs/kinds-2026-10-29.md](runs/kinds-2026-10-29.md).

## The word code: parts of speech nobody named

2,849 words were coded in 0.3 s, with 4 to 9 passes per depth. At mode 16 the classes read, most-told members first:

| class | its commonest members | reads as |
|---|---|---|
| c4 | the this my her your these two his each our three their another those | determiners |
| c6 | i we he she they i've i'd you've we've you'll | subject pronouns |
| c8 | can don't has didn't will would can't could should doesn't couldn't must never always | modals, negated auxiliaries |
| c14 | to in of for at on with about by from after into without | prepositions |
| c11 | sentences car job house same party other friend hotel phone sentence room | nouns |
| c10 | long good much lot past present new few nice tired back hard old right | adjectives |
| c1 | be go me get work want them home him come used us live eat | base verbs (and object pronouns) |
| c9 | was like say had been were know think got need went said | past forms, verbs of saying and knowing |
| c12 | have do use if when see did complete take tell find leave does | auxiliaries **mixed with** imperative verbs and `if`/`when` |
| c13 | is or are that out up unit no all any where last doing before | a mixed class |

No word class was given, and there is no gold to score these against; they are read by eye. The hash's classes are random buckets. A story in mode 16, structure only:

```text
she is on her way to work
  hash:     c0 c10 c15 c5 c15 c11 c1
  given:    c1 c3  c6  c1 c0  c6  c0
  learned:  c6 c13 c14 c4 c11 c14 c1     pronoun · is · preposition · determiner · noun · preposition · verb
```

## Every arm, mode and twin, held out

| arm | mode | twin | kinds | variations | function purity | mark purity | river MI (bits) | river z |
|---|---|---|---|---|---|---|---|---|
| hash | 16 | open | 37 | 256 | 55.0% | 65.9% | 0.425 | 13.9 |
| hash | 16 | close | 33 | 256 | 53.8% | 64.9% | 0.297 | 3.0 |
| hash | 16 | both | 64 | 2,717 | 55.5% | 65.4% | 0.953 | 1.6 |
| hash | 32 | open | 64 | 929 | 55.4% | 65.7% | 0.907 | 12.4 |
| hash | 64 | open | 64 | 1,694 | 55.6% | 65.9% | 0.938 | 11.3 |
| given | 16 | open | 37 | 89 | 56.7% | **70.4%** | 0.533 | **34.2** |
| given | 16 | close | 41 | 86 | 53.8% | 66.0% | 0.300 | 9.8 |
| given | 16 | both | 63 | 452 | 56.6% | 68.1% | 0.565 | 17.0 |
| learned | 16 | open | 55 | 234 | **56.9%** | 66.9% | 0.544 | 25.9 |
| learned | 16 | close | 53 | 231 | 55.4% | 65.6% | 0.401 | 7.2 |
| learned | 16 | both | 64 | 1,239 | 55.3% | 66.0% | 0.757 | 6.6 |
| learned | 32 | open | 64 | 597 | 56.4% | 66.0% | 0.769 | 23.6 |
| learned | 64 | open | 64 | 1,007 | 56.0% | 66.1% | 1.058 | 21.4 |

The rest of the rows (closing and both ends at 32 and 64) are in the run's file; none beats the opening.

## The bars, fixed before the run

| | bar | result | |
|---|---|---|---|
| I1 | function purity ≥ majority + 5 (56.7%) | learned 56.9%, given 56.7%, hash at best 56.0% | **met by 0.2 points**: weak |
| I2 | learned ≥ hash + 5 points, same mode | 56.9% against 55.0% at mode 16 (+1.9) | **not met** |
| I3 | mark purity above its majority; the best question F1 | learned 66.9% (+2.0), given 70.4% (+5.5), hash 65.9% (+1.0). Question F1 on held-out, with the kinds chosen on dev: given **58.5%**, learned 43.8%, hash 23.8% | met, but the hand beats the learned code |
| I4 | the river at z ≥ 3, and above the hash | learned 25.9, hash 13.9; given 34.2 | **met**; the given dictionary is higher |
| I5 | the counts agree across modes, the hash's do not | all 64 kinds fill at modes 32 and 64 for the hash as for the learned code; at mode 16 learned 55, hash 37, given 37 | **not supported**: the count is the routine's capacity, not the language's |

**Sensing each label.** A kind answers to its dev majority; F1 is then read on the held-out sentences.
- **Explain** is sensed at 67.8% F1 by every arm, which is close to marking everything as explain.
- **Answer** cannot be sensed this way: dev holds no answers, because the Key is never told.
- **Instruct**, 98 held out, is answered to by no kind.

**Two kinds the routine found without a name** (learned, mode 16, the opening):
- **Kind 31, `c12 c7 …`:** do, did, have + you, it. "don't you have anything else to do". It holds 51 of the 187 held-out questions, beside 52 statements.
- **Kind 26, `c12 c4 …`:** use, complete, put + the. "use the present continuous". It holds 31 of the 98 held-out instructions, in 117 sentences.

The question kind and the imperative kind are there. They are diluted because c12 holds the auxiliaries `do`, `did`, `have` together with the imperative verbs `use`, `complete`, `take`.

## The first run, and why it was wrong

The first run used docs/201's reader exactly. It gave function purity 64.5% and mark purity 71.1% (learned, mode 16, opening), against baselines of 52.7% and 58.7%. Both were confounds of the layout:

- **The item numbers.** The units' reader kept an exercise line's item number (`3 i have a car …`), and the Key's reader dropped the Key's. The code gave the numbers a class of their own (`1 2 3 4 5 6 7 8 9 complete 10 11 12`), so "opens with a number" told exercise items from answers.
- **The Key's marks.** The Key carries no marks, so every answer read as `line`.

The fix: item numbers are dropped from the units, as the Key's reader drops them, and marks are read on the units' sentences only. With the confounds gone, function purity falls from 64.5% to 56.9%. Most of the first run's gain was the numbering. That run's output is kept in [runs/kinds-2026-10-29-item-numbers-kept.md](runs/kinds-2026-10-29-item-numbers-kept.md).

## The thinkers' first runs

| thinker | run | result |
|---|---|---|
| Searle | function from structure | weak: +5.2 points of function purity. The question and imperative kinds exist unnamed, but blurred |
| Wittgenstein | family resemblance within a kind | no feature is shared by every variation of a kind (the widest holds 44% on average). Kinds hang together more than shuffled ones: 119 pieces against 137, z −3.1. **But the hash's kinds show the same, more strongly** (117 against 152, z −5.3). This is the routine grouping variations that share a class, and it is not shown to be the language's |
| Bentov | the same count at every level | not supported (I5) |
| the mirror | the closing twin against the opening | the closing reads less in every arm and mode: function 55.4% against 56.9%, river z 7.2 against 25.9 (learned, mode 16). What an English sentence is for shows at its start, not at its end |
| Babbage | the differences, the stop | the stop held at both heights: the gain mostly falls by an order of magnitude after the first pass (once only by half), and no depth needed more than 11 passes |
| Feynman, Lovelace, Whitehead, von Neumann | — | not run yet |

## What works, what does not

**Works.**
- The word code finds parts of speech from the guide alone, in 0.3 s: determiners, subject pronouns, modals, prepositions, nouns, adjectives. A story in mode 16 is structure.
- The river carries signal: z 26 against 1,000 shuffles.
- The opening twin is the one that reads.
- A question kind and an imperative kind were found with no name given.

**Does not work.**
- Kinds of intention from the river alone barely beat the majority (I1 by 0.2 points) and do not beat the hash null by the bar (I2).
- The hand-made dictionary senses questions better than the learned code (58.5% against 43.8% F1), because the learned code puts auxiliaries in one class with imperative verbs.
- Answers and instructions are not sensed.
- "64 at every level" is the routine's capacity, not something measured in the language.

## No cap: how many the data asks for

The user, on the templates: "must be built dynamically, we can't limit the templates, but you can start by the ones you can figure out". The second run removes every cap:

- **Templates grow.** A sentence's grown template is its longest opening that the training sentences show at least twice, of any length. A held-out sentence whose opening is no template backs off along the trie of the templates told, to the most told one that shares its longest opening.
- **Classes and kinds grow only while they pay.** At each depth, a class's split is kept only if the likelihood it gains is more than one more class costs to describe (`phical::classes::split_cost`: BIC, (2k − 1)/2 · ln N). A class whose split does not pay stops. The code stops when nothing pays (bound: 12 bits, never reached).

```text
cargo run --release -p phisor --example kinds -- … (the same command; output in runs/kinds-2026-10-29-paid.md)
```

| height | the data asks for |
|---|---|
| **word classes** | **53**, at depth 7, from 2,849 words. Every split pays through depth 5 (32 classes); 18 of the 32 pay at depth 6, 3 at depth 7, and none at depth 8 |
| **templates**, grown | about 4,500 openings told at least twice (4,498 in the learned code's mode 16, 4,664 in the hash's) |
| **kinds**, by the river | **6 to 20**, depending on the template: 7 for the learned opening at mode 16, 20 for its grown templates |

The kinds the river pays for carry almost nothing of what the sentences are for:

| arm, mode 16 | template | kinds | function purity | mark purity | river z |
|---|---|---|---|---|---|
| hash | open | 6 | 51.7% | 64.9% | 22.2 |
| given | open | 6 | 53.1% | 64.9% | 62.7 |
| learned | open | 7 | 52.8% | 65.0% | 48.3 |
| learned | grown | 20 | 52.1% | 64.9% | 5.3 |
| learned, mode 64 | open | 11 | 53.3% | 64.9% | 41.0 |

The baselines are 51.7% (function) and 64.9% (mark). With 64 kinds fixed instead, the grown templates read *less* than the plain opening: learned 55.1% against 56.9% function purity. About 4,500 templates leave too few sightings of each for the river to group.

Read straight:
- **At the words, the user's number is nearly the data's.** MDL pays for 53 word classes, between 32 and 64. It is the first count here that is the data's rather than the routine's (I5 of the first slice).
- **At the sentences, the river is not where intention lives in this book.** It pays for only a handful of kinds, and the guide's labels spread evenly across them. The flow is real (z up to 63 against the shuffles) but thin, at 0.17 bits at most. What a sentence is for shows inside it, in its opening: the question kind and the imperative kind above.
- **Family resemblance is chance under MDL.** Under the paid kinds the pieces and the widest shared feature equal the shuffled kinds' (z −0.6 and −0.7).

## Next, in the order worth doing

1. **Kinds from inside the sentence, not from the river.** The no-cap run says the neighbours pay for only a handful of kinds. Group the templates instead by their own class pairs, with the same routine and the same MDL stop, so the data says how many kinds of sentence there are from what sentences hold.
2. **Part what c12 joins.** `do`, `did`, `have` against `use`, `complete`, `take`. One more bit, or the `after <word>` channels: subject–auxiliary inversion is the question's mark in English, and the code must see it.
3. **Feynman's second run**, Lovelace's test, and the act read from English (step 5 of the plan).

2026-10-29. `crates/phisor/` (`guide`, `code`, `kind` with grown templates and their back-off, `units`; 16 tests and a doc test; the routine is `phical::classes::halve`, moved there from phisor the same day with its 3 tests), `examples/kinds/`. Diagrams: [architecture.md](architecture.md), module pages [guide](guide.md), [code](code.md), [exchange](exchange.md), [kind](kind.md).
