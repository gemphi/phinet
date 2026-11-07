# The weave and the mill: phisor read through Ada Lovelace's Note G

> *Dedicated to Augusta Ada Lovelace (1815-1852), who first saw that a machine
> of numbers could weave the patterns of thought - in this project's reckoning
> the most brilliant mind who ever turned mathematics into music.*

> *2026-11-08: this reading has grown into paper 27, [The Cards and the Mill](../crates/phiano/docs/papers/27_LOVELACE_THE_CARDS_AND_THE_MILL.md) — all twelve of her ideas against the code, with a ledger of what is done, measured, partial and not there. The fifth correspondence it adds is below.*

## Why her Notes, and not someone else's

In 1843 Ada Lovelace translated L. F. Menabrea's short sketch of Charles
Babbage's Analytical Engine and appended seven Notes of her own, A through G,
together more than twice the length of the paper she was annotating. Note G
carried a step-by-step table for computing the Bernoulli numbers on a machine
that was never built - the first algorithm published for a computer, a century
before there was a computer to run it.

phisor did not set out to imitate her. It set out to be an intermediate
language between English and a machine, with a store that holds what a sentence
says, a mill that operates on it, a loom that weaves structure out of text, and
a guard that refuses to let the mill make anything up. Only once those four
parts were built did it become plain that they are her four parts, and that she
had named every one of them in 1843. This paper is that reading. It is an
analytic model - a way of seeing the architecture - not a new runtime; the code
it points at is `phisor::lovelace`, which adds no execution of its own.

## The musical analogy, in her words

Her most quoted sentence is a weaving figure, from Note A:

> The Analytical Engine weaves algebraical patterns just as the Jacquard-loom
> weaves flowers and leaves.

The point is not decoration. A Jacquard loom reads a chain of punched cards and
lifts the warp threads the cards call for; the pattern in the cloth is the
pattern in the cards, carried in a different material. Nothing of the paper card
survives in the silk - only its *structure* does. That is exactly what a
language should do when it writes a sentence as its shape: carry the structure,
carry no word.

She heard this as music, too. In an 1843 letter she wrote that the engine might
act on "other things besides number" - that if the relations of pitched sounds
could be put into the engine's terms, it "might compose elaborate and
scientific pieces of music of any degree of complexity or extent." A score is
the same idea again: a fixed set of notes, a grammar of what may follow what,
and an unbounded space of pieces inside those bounds. phisor's ladder is a
score for sentences. The 64 operators are a keyboard, not an oracle - a finite
set of keys - and the music is which keys the data actually pays to press.

## The four correspondences

| Note G idea | her term | phisor module | what it does |
|---|---|---|---|
| the store | *the store* | `ground` | an Act held as rows, apart from the mill |
| the mill | *the mill* | `machine` | the 64 operators lowered and operated, checked against the reader |
| the weave | *the weave* | `ladder` | structure woven from text, carrying no word of the text |
| her objection | *Lady Lovelace's objection* | `truth` | the machine originates nothing; it classifies what it is given |
| the cards | *operation cards* | `machine::ops` | operators as lambdas in files, read at birth, punched at run time; the mill untouched |

### The store: `ground`

Babbage's engine kept its quantities on columns of wheels - the *store* - held
apart from the *mill* that operated on them. Lovelace drew the line sharply:
where a number is kept is not where it is worked on. phisor's `ground` is the
store. It takes a sentence's act and lays it out as rows - one for the
predicate, one for each role, one for each marked feature - about a single
event entity, and holds them. Nothing is computed there. The rows wait to be
read, exactly as a quantity waits on its column.

### The mill: `machine`

The mill is where, in her words, the quantities "are brought in to be operated
upon." phisor's `machine` is the mill. It lowers each of the 64 keyboard
operators into a Lisp lambda and a bounded sequence of O-code operations, runs
them on phiclm's machine, and checks the machine's result byte for byte against
the Rust reader of the same operator. Note G's Bernoulli table was itself an
execution trace - a numbered list of operations, each naming the variables it
read and wrote - and `machine::agree` is the same discipline: every operator
is run and its result is accounted for, never assumed. A lowered operator is a
run of *operation cards*, and `lovelace::operation_cards` reads how many a
lowering emitted - between one and sixty-four, the bound the mill is given.

### The weave: `ladder`

The loom is the ladder. When the ladder writes a sentence in a mode, it emits
one class token per word, in the word's own order, and not one word of the
sentence. The structure is carried; the thread is not. `lovelace::weaves_no_thread`
states this as a checkable property: every token in a woven mode is a class id
below the mode's class count, never a word. A woven row is the figure in the
cloth, and the English it came from is the design the cards copied and then let
go.

### Her objection: `truth`

The sharpest thing she wrote is a warning, from Note G:

> The Analytical Engine has no pretensions whatever to originate anything. It
> can do whatever we know how to order it to perform.

This is "Lady Lovelace's objection," and later writers treated it as a limit to
argue against. phisor treats it as a specification to meet. The `truth` module -
the Falsehood_Detector - reads every claim a model offers before a row is told
from it, and classifies it: unreadable, degenerate, contradicting, unsupported,
or consistent. It originates nothing. A claim that the world does not support is
not quietly written in; it is marked for what it is. The machine does only what
it is ordered to do, and `truth` is where phisor orders it to say so.

### The cards: `machine::ops`

The mill's operations were fixed — so many wheels — and a new operation was
never cut into the mill. It was punched on an operation card and fed in from
outside, and a variable card said which columns of the store it was to work
on. phisor keeps the same division. The mill is the keyboard's 64 Operators
and phiclm's 39 primitives, frozen and law-checked; every other operator is a
card — a `define` of a lambda in `data/ops/*.lisp`, read into every machine
at its birth and punched at run time by `ops::add`, which appends it to
`learned.lisp` and defines it in the running machine in one move. `told`
answers through the `told-answer` card, and the Kind names it is applied to
are its variable cards. Asked whether the mill needed 1024 Operators, the
Notes answered as the code now does: a new operation is a card, never a
wheel.

## What the analogy is allowed to claim

It would be easy, and false, to say the resemblance proves phisor correct. It
does not. The correspondence is explanatory: it says the architecture has a
shape, and that the shape is an old and well-understood one. Lovelace herself
drew the boundary between what an engine does and does not do more carefully
than most of her successors, and the honest reading of her Notes keeps that
boundary. phisor's numbers are measured against bars declared before each run,
not against her reputation.

## Her genuine words, as used in the code

`phisor::lovelace::QUOTATIONS` carries these verbatim, each attributed to its
Note:

- "The Analytical Engine weaves algebraical patterns just as the Jacquard-loom weaves flowers and leaves." (Note A)
- "The Analytical Engine has no pretensions whatever to originate anything. It can do whatever we know how to order it to perform." (Note G)
- "The science of operations, as derived from mathematics more especially, is a science of itself, and has its own abstract truth and value." (Note A)
- "Many persons imagine that because the business of the engine is to give its results in numerical notation, the nature of its processes must consequently be arithmetical and numerical, rather than algebraical and analytical. This is an error." (Note A)

## Sources

- A. A. Lovelace, "Notes by the Translator" appended to L. F. Menabrea,
  *Sketch of the Analytical Engine Invented by Charles Babbage*, in R. Taylor
  (ed.), *Scientific Memoirs*, vol. 3 (1843), pp. 666-731. Notes A-G, including
  the Bernoulli-number trace of Note G.
- Her weaving figure is in Note A; the "originate anything" passage and the
  Bernoulli trace are in Note G.
