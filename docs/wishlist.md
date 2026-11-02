# Wish list: connecting the dots

What would let the little pieces that already dance in phinet resonate into
one picture. Each item names the pieces that exist, the gap between them, and
the one wire that would close it. Nothing here is speculative architecture; it
is connective tissue over parts that are already built and tested.

The ordering is by leverage: the first few turn separate green numbers into one
honest verdict, the later ones deepen what the learner can resonate with.

## 1. One walk, every number (nearly done)

- Have: the harness walks the ten crates; the ladder run now measures function
  purity, terminator purity and the River z-scores on the real Guide; the
  curriculum measures top-1 and perplexity; the settings sweep tunes on
  held-out purity.
- Gap: top-1 and perplexity are measured by the Curriculum_Runner but not yet
  folded into the same `--run` walk, so the one record does not yet carry all
  eighteen numbers at once.
- Wire: call the curriculum from `run_walk` beside the ladder run, judged
  against `curriculum.top1` and `curriculum.perplexity`. One function call.

## 2. Close the function-purity gap honestly

- Have: a settings sweep over the exchange routine's knobs, scored on held-out
  function purity, with the Bar and the labels fixed.
- Gap: the best grid point reached 52.6% against the 56.7% Bar before the
  sweep; whether the wider grid clears it is a measurement to run, not to
  assume.
- Wire: run the tuned walk to completion and read the number. If it clears, the
  verdict passes on its own; if not, the record shows the honest miss and the
  gap is a research item, not a wiring one.

## 3. The dictionary as relations, not just definitions (landed: adapter plus training wire)

- Have: phient's `DefinitionGraph` (Dict2vec strong/weak pairs), the
  `DefinitionGrounder`, controlled negative sampling, and a `ChunkStore` that
  ingests `{word: definition}` JSON. A word is already learned as its
  relations, carried as phasors.
- Gap: the richest free, licensed structure - synonyms, antonyms,
  part-of-speech, hypernyms - is not yet ingested; only definitions are.
- Wire: a Kaikki/Wiktextract source adapter (Wiktionary, CC BY-SA) that reads
  the machine-readable JSON and feeds synonyms and hypernyms into the relation
  channels, not just the definition graph. This is the piece that makes a word
  resonate with its kin, not only with its gloss.

## 4. The River as Bentov's standing wave

- Have: `phiton::physics::standing` already carries Bentov's rule - a bounded
  medium of n channels sustains exactly n modes - and the River already
  measures how one kind of sentence follows another (z-score 7.58).
- Gap: the two are not yet read as the same thing. The River's kind-to-kind
  transitions are a standing wave over the sentence stream; naming them so
  would let the mutual-information measure and the standing-wave modes check
  each other.
- Wire: express the River's transition matrix as modes of a bounded medium and
  assert the mode count matches the occupied-kind count, the way
  `oscillator.rs` and `standing.rs` already stand beside each other.

## 5. Lovelace's loom closing the act height

- Have: the reversible Act text (`ir::text`) writes and reads an Act with no
  field lost (100% round trip over 3,781 Acts); the ladder weaves structure
  with no word carried.
- Gap: the English reader (sentence -> Act) is not built, so the four
  act-height numbers stay unjudged and the overall verdict stays an honest
  miss.
- Wire: the reader is the loom read backwards - text to structure to Act. It is
  the one unbuilt piece between the woven mode and the grounded rows, and
  building it lets the verdict become a true pass rather than an honest miss.

## 6. The teacher seam, consulted only as overflow

- Have: the Oracle_Seam reaches a local Phi-4 through Ollama, checks every
  claim before a row is told, and records the transcript; the
  Falsehood_Detector classifies before anything is learned.
- Gap: it is wired for the labelled-set measurement but not yet run as a live
  overflow source during a training walk.
- Wire: let the curriculum consult the seam only when nothing held answers,
  every answer checked, so the teacher fills gaps without ever overwriting what
  the learner earned.

## 7. The two learnings reading each other

- Have: the word code and the sentence kinds are grown by one routine
  (`phical::classes::halve`); the "learned how to learn" measure checks the two
  heights use the same settings.
- Gap: the measure is computed but not surfaced in the walk record, so the
  claim that the second height learns with no new setting is not yet on the
  page a partner reads.
- Wire: record the settings-drift comparison beside the ladder numbers, so the
  record shows not just what was learned but that it was learned the same way
  twice - the small proof that the method transfers.

## The shape of the whole

Every item above is one wire between two things that already work. That is the
state worth being plain about: phinet is not a pile of parts waiting for an
architecture, it is an architecture whose parts are mostly connected, with a
short list of named wires left. The picture the next section draws is the one
these wires complete.
