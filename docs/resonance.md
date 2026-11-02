# The whole picture: little pieces that resonate

> The Analytical Engine weaves algebraical patterns just as the Jacquard-loom
> weaves flowers and leaves. - Ada Lovelace, Note A (1843)

> Standing waves cannot exist unless they divide their medium into an integral
> number of half waves. - Itzhak Bentov, Stalking the Wild Pendulum

This is the reading that connects the dots. phinet is not a stack of unrelated
crates; it is one idea seen at several scales - a word, a sentence, a
conversation - and the same two images describe it at every scale: Bentov's
resonance (small oscillators falling into phase) and Lovelace's weaving
(structure carried without the thread). Both are already in the code, by name,
with tests. This document only draws the line between them.

## The smallest piece: a word is an oscillator

In phient a word is not a point but a set of phasors on a circle - 32 to 64 of
them, one per relation channel. `phiton::physics::oscillator` makes this
literal: a word is "a living oscillator on a sphere," spinning at a frequency
set by its band and the fine-structure constant, with hue from longitude and
brightness from latitude. Two words have a `synchronization` strength -
`sync = r · exp(-|w1 - w2| / Omega) · lat_factor` - which is Bentov's
resonance written as code: oscillators close in frequency and phase pull into
step, those far apart do not.

Learning is these oscillators moving. The trainer couples two words by what
they were told with, a Kuramoto-style phase update toward agreement when the
corpus puts them together and away when it does not. Meaning, in the design's
own words, is a side effect of pairs of words moving under a condition. Nothing
is placed; everything settles, the way a room of pendulums on one beam finds a
common swing.

## The next piece: a dictionary is a graph of mutual resonance

A word does not resonate only with its neighbours in a sentence. phient's
`DefinitionGraph` reads a dictionary as a directed graph - an edge a -> b means
b is in the definition of a - and splits the edges the way Dict2vec does: a
two-way edge (car defines vehicle, vehicle defines car) is a strong pair, a
one-way edge (car defines road, not the reverse) a weak one. Strong pairs pull
harder. The `DefinitionGrounder` then lays each word at the centre of mass of
what defines it, so a word's place is what it means and not how it is spelled.

This is the resonance image again, one level up: words that define each other
reinforce each other and converge into a concept region, while a word that is
merely mentioned drifts less. The wish-list item on Wiktionary is exactly this
graph made richer - synonyms and hypernyms are more strong edges, more pairs
that should end up in phase.

## The sentence: Lovelace's loom

Above the word, phisor weaves. A sentence written in a mode is one class token
per word, in the word's own order, carrying no word of the sentence - the loom
weaving the pattern of a design without a thread of the original. Its shape,
how it opens and closes, is its variation; the variations cluster into kinds of
intention. The weave carries structure and lets the words go, which is why the
`lovelace::weaves_no_thread` check is a real property and not a slogan.

The same exchange routine grows the word code and the sentence kinds -
`phical::classes::halve`, one routine, twice. That is the quiet claim at the
heart of the design: the method that finds classes of words also finds kinds of
sentences, with no new setting. If it holds, learning transfers up the scales
the way a harmonic repeats up the octaves.

## The conversation: the River as a standing wave

A sentence does not stand alone; it follows another. The River pairs each
sentence with the one before it, inside a run, and asks whether the kind of one
predicts the kind of the next. Measured on the held-out Guide, the mutual
information is a z-score of 7.58 against a bar of 3.0 - the order of kinds
carries far more than chance, learned from structure alone with no label told.

Bentov's standing wave is the image that fits. A bounded medium of n channels
sustains exactly n modes; `phiton::physics::standing` already carries his rule
verbatim. The River's kind-to-kind transitions are such a medium - a bounded
set of kinds, a fixed grammar of what may follow what - and the mutual
information is the energy standing in its modes. The wish-list item that reads
the two as one thing would let each check the other.

## The mill, and what it may not do

Under all of it is Lovelace's store and mill. phisor grounds an Act as rows -
the event, its roles, its features - and holds them apart from the machine that
operates on them, the 64-operator keyboard lowered onto phiclm's Lisp and
checked byte-for-byte against the Rust reader. And her sharpest line is a
specification the `truth` module meets: the engine has no pretensions whatever
to originate anything. The Falsehood_Detector classifies every claim before a
row is told from it and invents none. The machine plays the score; it does not
compose one it was not given.

## The one picture

Read top to bottom it is a single resonance at four scales:

- a word settles into phase with the words it is told with;
- a dictionary pulls words that define each other into one region;
- a sentence is woven into a kind, structure without thread;
- a conversation is a standing wave of kinds, one following another.

And read as architecture it is Lovelace's engine: a store that holds, a mill
that operates, a loom that weaves, and a guard that keeps the mill from
originating what it was not given. The little pieces already dance. The wish
list is the short set of wires that let the whole thing resonate as one - and
the harness is the instrument that will say, in numbers against bars fixed
before the run, whether it truly does.

## Sources and honest footing

- Ada Lovelace, Notes A and G on Menabrea's sketch of the Analytical Engine
  (1843): the weaving figure (Note A) and "no pretensions whatever to originate
  anything" (Note G). See `docs/lovelace.md` and `phisor::lovelace`.
- Itzhak Bentov, Stalking the Wild Pendulum: the standing-wave rule, quoted in
  `phiton::physics::standing`.
- The resonance, oscillator and Kuramoto coupling are real modules
  (`phiton::physics`, `phient::train::trainer`), not metaphors added for this
  document. The z-score 7.58 and the purity numbers are from run record 208 and
  `docs/status.md`; the function-purity gap is open and named, not hidden.
