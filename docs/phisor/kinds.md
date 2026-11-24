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
| Feynman | the twins added as amplitudes against probabilities | run 2026-11-23 ("Feynman's run" and "Learned phases" below): aligned arrows ≥ probabilities for the learned code at every mode (+0.9 at mode 16). With phases learned without a label they gain +0.7 to +1.2 points over probabilities, but the hash gains as much, and only 1–2 kinds turn far enough to cancel |
| Lovelace, Whitehead, von Neumann | — | not run yet |

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

## Inside: kinds by what sentences hold

The user chose it ("Yes, inside"). The templates are grouped by what their sentences hold, not by their neighbours. The same routine is handed one pair per class pair of each training sentence: its template, and the pair (`f:c6c8`, with the edges `f:<s>c6` and `f:c3</s>`). The class-pair likelihood of these pairs is their mutual information, so this is information-theoretic co-clustering (Dhillon, Mallela and Modha 2003). The same MDL stop says how many kinds there are, and no label is told (`kind::Reading::Inside`).

```text
the same command; output in runs/kinds-2026-11-23-inside.md (2026-11-23)
```

| arm · reading, mode 16, the opening | kinds | function purity | mark purity | river z |
|---|---|---|---|---|
| hash · river | 6 | 51.7% | 64.9% | 22.2 |
| hash · inside | 51 | 56.3% | 67.3% | 17.9 |
| given · river | 6 | 53.1% | 64.9% | 62.7 |
| given · inside | 32 | 56.9% | **70.6%** | 36.3 |
| learned · river | 7 | 52.8% | 65.0% | 48.3 |
| learned · inside | 45 | **58.0%** | 66.8% | 28.9 |
| learned · inside, mode 64 | 54 | **58.4%** | 68.2% | 26.8 |

The baselines are 51.7% (function) and 64.9% (mark).

**What it changes.**
- **The inside pays for more kinds.** The data pays for 29 to 58 kinds read inside, against 6 to 20 by the river: 45, 50 and 54 for the learned opening at modes 16, 32 and 64.
- **The inside reads more, in 27 of the 28 rows of the grid.** Function purity rises for every arm, mode and template but one: the hash's both-ends template at mode 32 falls from 52.4% to 52.1%. The learned opening reads 58.0% inside against 52.8% by the river.
- **I1 is met with kinds that pay.** 58.0% is 6.3 points over the majority. I1 was missed by every river reading once the cap was gone.
- **I2 is still not met.** The learned code beats the hash by 1.7 points at mode 16 and 3.6 at mode 64, short of the 5-point bar.

**Instructions are sensed for the first time.** Read inside, two kinds answer to `instruct` on dev and sense 28 of the 98 held-out instructions at 33.9% F1. Every earlier reading sensed none.
- Kind 32, `c12 c4 …` ("use the present continuous"), holds 26 instructions in 50 held-out sentences.
- Kind 46, `c12 c7 …` ("did you go out"), holds 35 questions in its 58 marked sentences.

The two openings share their first class: c12 joins `do`/`did`/`have` with `use`/`complete`/`take`. The inside reading parts them by the class after it, a determiner for the imperative and a pronoun for the question. So it already does part of what lever 2 below was for.

**What is unchanged.**
- **Questions.** The hand-made dictionary still senses questions best: F1 53.2% against the learned code's 34.4%.
- **Answers.** `answer` cannot be sensed this way: dev holds no answers, because the Key is never told.
- **Family resemblance.** Read inside, kinds hang together far more than shuffled ones: 51 pieces against 129, z −14.5, and the widest shared feature holds 77% against 40%. The hash's kinds do the same (z −18.3). Grouping by shared class pairs makes kinds that share features; it is the routine's resemblance, not shown to be the language's.

## Feynman's run: the twins added as amplitudes

The user asked for it (2026-11-23). It is Requirement 9.6–9.10 of the ladder spec (`.kiro/specs/phinet-64-operators-learning-ladder`). The measure (`measure::twin`) and phical's operator for it (keyboard Circle index 13, `twins`) had been written, but nothing had ever fed them the guide. `measure::mode_input` is that producer.

**How a sentence is read.**
- **One space of kinds for all three readings:** the both-ends templates' kinds, read inside, so the both-ends reading of a sentence is its own kind.
- **Each single end** reads a held-out sentence as the distribution of those kinds over the training sentences that open (or close) the same way.
- **The two ends are then combined twice:**
  - as probabilities added;
  - as amplitudes added and then squared: each distribution becomes arrows of length √share at a seeded phase, and `twins` adds the two.

Purity of the guide's functions (explain, instruct, answer) is read on the 3,176 held-out sentences, none told. Two small calculations were added in their homes for this: `phistd::stats::share` (counts as a distribution) and `phical::circle::amplitude` (√share at a phase).

```text
cargo run --release -p phisor --example feynman -- crates/phiano/data/person/murphy.txt --key crates/phiano/data/person/murphy_key.txt --seed 7
output in runs/feynman-2026-11-23.md
```

| arm | mode | opening | closing | both ends | probabilities added | amplitudes added | difference |
|---|---|---|---|---|---|---|---|
| hash | 16 | 56.1% | 54.3% | 53.6% | 55.4% | 54.4% | −1.0 |
| hash | 32 | 53.7% | 52.5% | 52.1% | 51.9% | 52.0% | +0.1 |
| hash | 64 | 52.7% | 52.3% | 52.6% | 52.0% | 51.9% | −0.1 |
| given | 16 | 56.5% | 54.4% | 56.5% | 56.6% | 56.8% | +0.2 |
| learned | 16 | 56.1% | 55.0% | 56.6% | 55.7% | **56.6%** | **+0.9** |
| learned | 32 | **57.1%** | 52.9% | 56.4% | 54.9% | 54.9% | 0.0 |
| learned | 64 | 54.8% | 53.1% | 53.7% | 53.4% | 53.4% | 0.0 |

The given dictionary has one alphabet, so its three modes read the same. The majority baseline is 51.7%.

| bar | result |
|---|---|
| R9.10: amplitudes ≥ probabilities | **passes for the learned code at all three modes and for the given dictionary; misses for the hash at two of three** (by 1.0 and 0.1) |
| R9.7: both ends ≥ the better single end + 2 | **missed at every arm and mode**, by 1.5 to 4.5 points; the opening alone is often the best reading (learned, mode 32: 57.1%) |
| R9.8: where the ends disagree | 3,006 of 3,176 sentences at learned mode 16, down to 2,195 at mode 64. Two partial views of a sentence rarely pick the same kind |

**What the amplitudes did, read straight.** The phases do nothing in this run. `twins` keeps the second arrow only when it already fits the first, and otherwise turns it onto the first arrow's phase, so every pair adds in phase. The same run at another seed differs in one cell, by 0.1 point, a rounding tie. So the comparison is p + q against (√p + √q)² = p + q + 2√(pq). The amplitude form adds an agreement term that rewards the kinds both ends support. That term is worth +0.9 points for the learned code at mode 16, nothing at modes 32 and 64, and −1.0 to +0.1 for the hash. Interference that cancels, where an opening that looks like a question meets a closing that says it is not one, needs phases that are learned, not aligned.

## Learned phases

The user asked for them (2026-11-23), so that the arrows can cancel as well as reinforce. Each kind's closing arrow is turned by a phase of its own before the two ends are added, with no phase moved afterwards: `phical::circle::intensity`, |a + b|², added for this. Only the relative phase matters, so the opening's phases stay at 0.

**How the phases are learned** (`measure::phase`).
- **Sixteen phases**, a sixteenth of a turn apart: q64's symbols 0, 4 … 60, read with `q64::lift`.
- **Coordinate ascent.** One kind at a time, every phase is tried on the training rows that kind can move.
- **Kept only when better.** A phase is kept only when `liskov::staged::better` reports more training sentences peaking at their own both-ends kind.
- **No label is read.** The target is the kind the clustering already gave each sentence. The learner is deterministic, and every run stopped within four passes.

```text
the same command as Feynman's run; output in runs/feynman-phases-2026-11-23.md
```

| arm | mode | training read as their own kind, phase 0 → learned (of 14,777) | kinds turned | a quarter turn or more | probabilities added | amplitudes aligned | **learned phases** |
|---|---|---|---|---|---|---|---|
| hash | 16 | 10,401 → 10,516 | 14 | 0 | 55.4% | 54.4% | 54.5% |
| hash | 32 | 8,105 → 11,095 | 5 | 1 | 51.9% | 52.0% | 53.1% |
| hash | 64 | 10,684 → 12,722 | 5 | 1 | 52.0% | 51.9% | 52.6% |
| given | 16 | 12,725 → 12,726 | 1 | 0 | 56.6% | 56.8% | 56.8% |
| learned | 16 | 12,803 → 12,874 | 17 | 1 | 55.7% | 56.6% | **56.7%** |
| learned | 32 | 10,779 → 12,043 | 15 | 1 | 54.9% | 54.9% | **56.1%** |
| learned | 64 | 9,005 → 11,952 | 15 | 2 | 53.4% | 53.4% | **54.1%** |

Purity is of the guide's functions on the 3,176 held-out sentences; the majority is 51.7%. The given dictionary reads the same at every mode.

**Read straight.**
- **Learned phases beat probabilities added for the learned code at every mode:** +1.0, +1.2 and +0.7 points. They beat the aligned arrows at modes 32 and 64 (+1.2, +0.7) and tie at mode 16 (+0.1).
- **The gain is not the learned code's own.** The hash's phases buy about as much: +1.1 and +0.7 over the aligned arrows at modes 32 and 64. At mode 16 they still fall short of probabilities added, by 0.9. What the phases buy is a free parameter per kind, fitted to the kinds the run already has.
- **Cancellation proper is rare.** Each learned code turns 15 to 17 kinds, but only one or two of them a quarter turn or more, where the two ends start to cancel. Most turns are small, so they reweight the two readings rather than oppose them.
- **The training gain is far larger than the held-out gain.** The phases read up to 2,947 more training sentences as their own kind (learned, mode 64). That becomes 0.7 points of held-out purity of the guide's labels, because the target the phases chase is the kinds, not the labels.
- **The opening alone is still the best single reading at mode 32.** It reads 57.1% there, against 56.1% for the learned phases.

## Parting c12: the opening sense

The user asked for it (2026-11-24). At mode 16 the learned code's c12 joins the auxiliaries `do`, `did`, `have` with the imperative verbs `use`, `complete`, `take`, and with `if` and `when`. Subject–auxiliary inversion is English's mark of a question, and one class cannot show it.

**Where they part without help** (`examples/class_tree.rs`; runs/class-tree-2026-11-24.md).
- **One bit down, c12 splits by where a word stands, not by what it does.** At depth 5, `did`, `does`, `complete`, `if` and `when` go together as sentence openers; `have`, `do`, `use`, `see` and `take` go together after `to` and `don't`.
- **`did`/`does` part from `complete` at depth 6.** `do` parts from `use` only at depth 7, and `if` from `when` never.
- **The root of it is one class per word.** `do` is coded by its commonest use, the base verb, so the `do` that opens "do you…?" reads the same as the `do` of "I don't do it".

**The lever: an opening sense** (`Code::learn_opening`, the channel after `<s>`).
- **A sense of its own.** A word that opens at least two training sentences is a second unit there (`^do` beside `do`), and a sentence's first word is written in it.
- **No `<s>` before a sense.** The first try put `<s>` before every sense, and every sense then shared one left neighbour: all openers fell into one mode-16 class, and the first class of every sentence said nothing.
- **With `<s>` left out, a sense is coded by what follows it.** At depth 4, `^do`, `^did`, `^have` and `^can` part from `^use`, `^complete`, `^put` and `^choose`. The question openers join medial `did` (`^do ^did ^can ^have ^are ^would ^will`), and `^the` stays with `the`. `^does` does not join them.

**What it does on the held-out sentences** (runs/kinds-opening-2026-11-24.md). Kinds are read inside, from the opening; each label is sensed by the kinds that answer to it on dev:

| arm | mode | function purity | mark purity | questions F1 | instructions F1 |
|---|---|---|---|---|---|
| learned | 16 | **58.0%** | 66.8% | 34.4% | **33.9%** |
| learned · opening | 16 | 57.1% | **69.8%** | **56.1%** | 0% |
| learned | 32 | 57.2% | 68.3% | 50.6% | 4.7% |
| learned · opening | 32 | 57.3% | 68.8% | 46.5% | 31.1% |
| given (by hand) | 16 | 56.9% | 70.6% | 53.2% | 0% |

**Read straight.**
- **Questions.** With the opening sense the learned code senses questions at 56.1% F1 at mode 16. That is up from 34.4%, and for the first time above the hand-made dictionary's 53.2%. Mark purity rises from 66.8% to 69.8%.
- **Instructions.** At mode 16 they are lost: from 33.9% to none. The imperative openers now share their mode-16 class with `if`, `when`, `is` and `are`, and they part from them one bit down. At mode 32 the opening code senses both, at 46.5% and 31.1%. No single setting senses both well.
- **Function purity moves less than a point.** It goes −0.9 at mode 16 and +0.1 at 32.
- **Small numbers.** The sensing is chosen on dev's 869 sentences, which by the held-out shares hold about 50 instructions and 90 questions. A few kinds more or fewer move these F1 figures by many points.

### The opener on two channels

The user: "fix all". A sense for the second word would not part the orders. The trouble was the first word itself: at mode 16 `^use` shared a class with `^in`, `^if` and `^is`, and "use the…" and "in the…" both go on with a determiner. What parts them is what the word is anywhere else: `use` a verb, `in` a preposition, `if` a conjunction.

So a first word with an opening sense is written twice (`Code::write`): first in its opening sense, which says where it stands and parts `^do` from `^use`, then as itself, which says what it is and parts `use` from `in`. The opening template is those two readings. Only codes learned with opening senses write the extra class, so every other caller reads as before (runs/kinds-two-channel-2026-11-24.md):

| arm, read inside from the opening | mode | function purity | mark purity | questions F1 | instructions F1 |
|---|---|---|---|---|---|
| learned | 16 | 58.0% | 66.8% | 34.4% | 33.9% |
| learned · opening, two channels | 16 | 56.6% | 69.2% | 50.0% | 48.6% |
| learned · opening, two channels | 32 | 58.1% | 68.9% | 48.9% | 38.8% |
| **learned · opening, two channels** | **64** | **59.3%** | **72.2%** | **61.4%** | **55.7%** |
| learned | 64 | 58.4% | 68.2% | 46.1% | 39.5% |
| given (by hand) | 16 | 56.9% | 70.6% | 53.2% | 0% |

**Read straight.**
- **One mode now senses both.** At mode 64 the two-channel code senses questions at 61.4% F1 and instructions at 55.7%. Its function purity, 59.3%, is the best of any reading in this report, and its mark purity, 72.2%, the best too.
- **It beats the hand-made dictionary on every column.** That dictionary senses no instructions at all.
- **Mode 16 senses both as well,** at 50.0% and 48.6%. The single-channel sense read questions better there (56.1%) but lost the instructions.
- **The same caution as above.** The sensing is chosen on dev's handful of questions and instructions, so single F1 figures move by points with a kind more or fewer. The direction holds at every mode.

### The order-dependent tests, fixed

`machine::told`'s English test failed in the full run and passed alone. The reader's lexicon is three process-wide tables (the mined tags, the function words, the Guide's irregular verbs), and the first prime wins. Five tests primed it, each in its own way: the full files, a two-word stub, or nothing. So the reader a test saw depended on which test ran first.

- **One owner primes the lexicon.** `lexicon::prime` (a module of its own) holds `prime_canonical`, which primes all three from the harness's files. Under test, every query and every prime first primes those canonical files, once, so every test reads the reader the harness runs, in any order.
- **The copied priming blocks are gone.** All five tests now get their lexicon from that one place.
- **A second test was exposed.** Reading the primed lexicon everywhere showed that `a_yes_no_copula_question_reads_and_round_trips` had only passed unprimed. Primed, the subject reader took "the weather good" as one noun phrase, because a statement's subject keeps a word back for its verb.
- **So a copula question reads its subject differently.** It now reads the longest subject that still leaves a complement (`spanning_subject` in `ladder/act/read/copula.rs`). "Where were the keys?", the reader's own example, reads for the first time.
- **The suite is stable.** phisor's 308 tests pass in two parallel runs and one serial run.

## Next, in the order worth doing

1. **Done: kinds from inside the sentence** (above, 2026-11-23). Reading inside is the better reading at every arm and mode; the river stays as the measure of the flow between kinds.
2. **Done: c12** (above, 2026-11-24). The opener read on two channels, its opening sense and itself, senses questions and instructions both at mode 64 (61.4% and 55.7% F1). That is above the hand-made dictionary on every measure.
3. **Done: learned phases** (above, 2026-11-23). They add about a point for the learned code, and as much for the hash. Interference that pays would need a target closer to what the sentences are for than the kinds themselves.
4. Lovelace's test, and the act read from English (step 5 of the plan).

2026-10-29, the inside reading, Feynman's run and the learned phases 2026-11-23. `crates/phisor/` (`guide`, `code`, `kind` with grown templates, their back-off and the two readings, `unit`, `measure::{mode_input, readings, phase}`; the routine is `phical::classes::halve`, moved there from phisor on 2026-10-29 with its 3 tests), `examples/kinds/`, `examples/feynman.rs`. Diagrams: [architecture.md](architecture.md), module pages [guide](guide.md), [code](code.md), [kind](kind.md).
