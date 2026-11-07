# A language model that compiles

A paper on the English reader in `phisor`, read as a compiler for sentences.

Every figure in this paper is quoted from run record
`235_2026-11-07_harness.md`, by way of `english_reader.md`. Nothing is
estimated. Where the record holds no number, the paper says so and gives none.
The record was taken on 2026-11-07 (workspace `ad92b04`, `phiano` `6e4442e`,
both dirty). The previous record is `234_2026-11-05_harness.md`.

## 1. The compiler analogy

A compiler does not guess what a program means. It reads text by rules into an
intermediate form, refuses what it cannot read, and can write the form back as
text. The English reader is built the same way, and the analogy gives each part
a test.

| compiler | the reader | what checks it |
|---|---|---|
| source text | a sentence of the Guide | the Held_Out_Slice (sec. 3) |
| lexer filter | Well_Formed: text-only operators that drop fragments | removed counts per operator |
| front end (parser) | the Reader: sentence to Frame (force, roles, words) | coverage: the share of sentences read |
| parse error | abstention: the Reader returns no Frame, never a partial one | `ladder.act.abstain.<kind>` counts |
| intermediate form | the Frame | the gold set of 150 frames |
| back end (code generator) | the Realiser: Frame to text | round trip under Canonical_Form |
| compile and decompile agree | round trip | soundness: the share of the read that round-trips |

Two properties carry the analogy. First, the Reader abstains whole. It returns
a complete Frame with every word accounted for, or nothing (Property 4 in the
design). A sentence it cannot read costs coverage and does not cost soundness.
Second, soundness is checked by the Realiser, an independent back end, not by
the Reader grading itself. A Frame counts as sound only if writing it back
gives the sentence again under Canonical_Form.

This is not a neural language model that happens to parse. The Reader is rules.
The "language model" in the title is the whole system: rules, data files, and a
small learned layer (sec. 2) that is measured and gated.

### Where the language data lives

The code holds no word lists. I read `phisor::lexicon` to confirm this, and
state exactly what I read.

- `lexicon::lists` reads each closed list from a plain-text file under
  `crates/phiano/data/lex` (control verbs, gerund verbs, complement verbs,
  performatives, set-aside adverbs and adverb phrases, imperative verbs,
  ditransitive verbs, clause conjunctions and openers, pronouns by role,
  doubling and silent-e lemmas, and others). One entry per line; `#` starts a
  comment; tagged lists share one `.tsv` per concept.
- The directory is `$PHISOR_LEX_DIR` when set, otherwise `data/lex` beside the
  `phiano` crate. Each list is read once, on first use.
- A missing file reads as an empty list, so the Reader abstains rather than
  guesses.
- `lexicon::contraction` expands contractions from `contraction_exception.tsv`
  and `contraction_clitic.tsv`. `'s` is not a rule: the following word decides
  between is, has and possessive, so the Reader resolves it.
- The open-class knowledge is primed by the harness, not read by the crate:
  `pos.tsv` (mined part-of-speech tags, kept as a set per word, so "water" is
  noun and verb and is not taken for a command), `function.txt`, and the
  Guide's Appendix 1.4 table of irregular verbs. Until primed, these queries
  answer none or false and the Reader uses its built-in rules.
- Only the foundation lexicon and the Guide's appendix are sources for priming.
  The gold set is not.

What I did not do: audit the whole of `phisor` for stray word lists outside
`lexicon`, or verify that the harness run of record 235 loaded these lists from
files. The record's working-tree line shows a dirty tree and does not say which
commits were built in. The list-to-file commits sit on the
`english-reader-coverage` branch; I did not tie record 235 to them.

## 2. The rule and learned split

The rule layer decides. The learned layer may only propose, and a proposal is
accepted only when the rule-based reading, the Realiser and Canonical_Form
agree (Property 6: with every proposal rejected, output equals the layer off).
The learned layer is trained on train sentences only, and its training indices
are disjoint from the Held_Out_Slice (Property 5). No Force classifier is
trained on the gold set.

The learned layer maps words to classes using the P16, P32 and P64 modes from
`phiton::p64`, and a sentence to a variation (first two classes plus last). It
predicts the next abstract element from an abstract prefix, and the same
positions are scored at the surface for comparison. The class count is read from
the data. The figure 64 is a tested assumption and not a limit.

Which words the rules should own is itself measured. The word-list audit asks,
for each hand-authored list, how many held-out words it covers and how many
held-out words would change if the list were learned.

| list | covers | would change if learned |
|---|---|---|
| `control_verbs` | 15 | 375 |
| `gerund_verbs` | 11 | 11 |
| `complement_verbs` | 9 | 381 |
| `performatives` | 1 | 389 |
| `set_aside_adverbs` | 22 | 154 |
| `clause_conjunctions` | 15 | 139 |

The record gives the counts and does not say which direction is better. They
show that a learned replacement would touch far more held-out words than the
lists cover today. They do not show it would be right. That has not been run,
and no figure is claimed for it.

## 3. The held-out discipline and the Well_Formed operators

The Reader is measured on the Held_Out_Slice only: the Test part of the Guide,
one sentence in ten, never tuned on. Coverage is the share of sentences read.
Soundness is the share of the read that round-trips under Canonical_Form.

Three rules keep the figures honest. The Split rule, Canonical_Form and the
scorer are not changed to raise a figure. The gold files are not edited (their
fingerprint and count of 150 are asserted by test). A metric that cannot be
computed is written as a named absence, not zero.

Well_Formed is a table of named, text-only operators that drop sentences which
are not sentences. Each is a pure function of the text; with no lexicon primed,
its verdict is the same (Property 2). The view after Well_Formed is the
"clean" view: 1201 kept, 114 removed. A sentence can fall to more than one
operator, so the counts may sum past 114.

| operator | removes | removed |
|---|---|---|
| `no_etc` | a sentence with an `etc` token (a fragment) | 53 |
| `no_stray_letter` | a single letter other than `a` or `i` (an item label or split stem) | 51 |
| `no_doubling` | a token equal to the next (`shut shut`) | 6 |
| `no_leading_or` | a first token `or` (a list label) | 10 |
| `no_label_a` | `a` followed by a pronoun or pronoun contraction | 10 |

Well_Formed removes sentences that no reader should be asked to read. It sits
outside the Reader so it cannot raise coverage by dropping hard sentences the
Reader could have attempted. Blanked exercise stems are removed here and not by
the Reader.

The clean bars are declared in `phiano/src/bin/harness/act.rs`, starting at
31.2 clean coverage and 76.8 clean soundness. A run is flagged as a regression
when clean soundness drops by more than 1.0 point from the previous record.
They are not in `bars.md`.

## 4. Measured results, including failures

### Held-out coverage and soundness

| measure | record 234 | record 235 | bar | verdict |
|---|---|---|---|---|
| `foundation_coverage` | 29.5 | 31.1 | none declared | unjudged |
| `foundation_soundness` | 75.0 | 77.0 | none declared | unjudged |
| `clean_coverage` | 31.2 | 32.9 | 31.2 at least | pass |
| `clean_soundness` | 76.8 | 78.7 | 76.8 at least | pass |

`clean_soundness_regression` is 0. The two bars passed here were set at the
previous record's figures, so the pass shows no regression and not a high
level. About two thirds of held-out sentences are not read (clean coverage
32.9), and roughly one in five of those read does not round-trip (clean
soundness 78.7).

### Why sentences are not read

Abstention kinds on the held-out foundation view:

| kind | count |
|---|---|
| `simple` | 305 |
| `one_join` | 267 |
| `infinitive` | 122 |
| `ing_form` | 76 |
| `multi_clause` | 26 |
| `very_long` | 10 |

The largest bucket is `simple`: sentences the Reader abstains on with no
structural excuse. Record 235 does not say why. I give no explanation.

### The gold set

| measure | all 150 | reachable only (124) | bar (all 150) | verdict |
|---|---|---|---|---|
| `force_purity` | 86.8 | 92.0 | 92.8 at least | miss by 6 on all 150 |
| `round_trip` | 86.0 | 84.7 | 100 at least | miss by 14 on all 150 |

Both bars are missed. Reachable-only force purity (92.0) is still below 92.8,
and the reachable-only figures carry no bar. The reachable-only round trip
(84.7) is lower than the all-150 figure (86.0). Taking out the frames no
text-only reader can label did not lift the round trip.

26 of 150 frames are Unreachable_Frames: another gold frame has the same
surface shape and a different label.

| reason | count | the two readings |
|---|---|---|
| `NameCase` | 18 | a proper name (London) or a common noun (london); `name` / `thing` |
| `BareWill` | 7 | a prediction or a binding; `ASSERT` / `COMMIT` |
| `Unaccusative` | 1 | the subject undergoes the event or does it; `theme` / `agent` |

A bare `will` is not mapped to Commit by rule, and the gold files are
untouched.

### The learned layer

The layer predicted 252 positions and abstained on 23404 (23656 in total).

| level | arm | top-1 (%) | perplexity | predicted |
|---|---|---|---|---|
| abstract | learned | 54.4 | 22.43 | 252 |
| abstract | shuffled control | absent | 47.36 | 0 |
| abstract | frequency control | absent | 47.02 | 0 |
| surface | learned | 3.6 | 484.06 | 252 |
| surface | shuffled control | absent | 993.29 | 0 |
| surface | frequency control | absent | 1014.56 | 0 |

`abstract_gate_enabled` is 1: the layer beats the shuffled control
(`abstract_gate_beats_shuffled` = 1) and the frequency control
(`abstract_gate_beats_frequency` = 1) on abstract perplexity.

Read this result with its limits.

- The learned arm answered 252 of 23656 positions. The controls answered none,
  so their top-1 is a named absence (`nan`) and the gate judged on perplexity
  alone. The record does not say that the perplexities are taken over the same
  positions. I do not claim they are, so the gate pass is weaker than it looks.
- The abstract top-1 of 54.4 is over those 252 positions only.
- The surface top-1 is 3.6. The abstract level predicts well on the positions
  it answers; the surface level, on the same positions, mostly does not.
- Not measured in record 235: the Reader's coverage and soundness with the
  layer on. The record holds no such figure. Whether the layer helps the Reader
  is therefore unknown, and the gate has enabled it without that evidence.

### Other bars in the same record

| measure | value | bar | verdict |
|---|---|---|---|
| `ladder.function_purity.learned` | 53.4 | 56.7 at least | miss by 3.3 |
| `ladder.terminator_purity.learned` | 85.4 | 0 at least | pass |
| `ladder.river_z.learned` | 3.39 | 3 at least | pass |
| `ladder.river_z.above_hash` | 3.39 | 0 at least | pass |
| `curriculum.perplexity` | 129.1 | 84.7 at most | miss by 44.4 |

### Failures, listed

- `force_purity`: 86.8 against 92.8.
- `round_trip`: 86.0 against 100.
- `function_purity.learned`: 53.4 against 56.7.
- `curriculum.perplexity`: 129.1 against 84.7 at most.
- The run's `phiano` level reported `BuildFailed` (`field exercise is never
  read`, in `examples/intent` and `examples/workshop`), a known unused-field
  warning under `-D warnings`. The overall verdict is `Miss: level phiano did
  not build`. The `ladder.act.*` figures were measured by the harness binary
  regardless.
- Totals for the record: 66 recorded, 5 passes, 4 misses, 57 unjudged. Most
  measures have no bar, so most of this record is description and not a verdict.

## 5. The comparison with JEPA

This section is limited to `docs/ecs_vs_jepa.md` and to figures on the same
slice. There are no same-slice figures for a JEPA model, so there is no
numerical comparison, and the paper does not claim to beat JEPA. No run exists
that would support such a claim.

What `ecs_vs_jepa.md` argues, in brief, is about the entity-component tree of
Phient, not about the reader. It sets a JEPA encoder, which predicts a learned
representation and needs an anti-collapse term, beside a tree whose unit is
already a word with relations under conditions, whose information content is
computed from counts, and which has no encoder to collapse. That document names
what JEPA does that the tree does not: perception. It calls a JEPA encoder a
candidate front end to the tree.

The reader's relation to that argument is narrow. Both predict at an abstract
level and not at the surface, and in record 235 the abstract level is far
better than the surface one on the positions the layer answered (perplexity
22.43 against 484.06; top-1 54.4 against 3.6). That is the only point of
contact the record supports. Its figures come from a different measure
(next-element prediction over Guide sentences) than anything in
`ecs_vs_jepa.md`, whose held-out figures are measured elsewhere and are not
quoted here because they are not in a harness record. Putting them beside the
figures above would compare different slices, so I do not.

A comparison would need: a JEPA-style model trained on the same train sentences,
scored on the same Held_Out_Slice, with the same abstention accounting, and
recorded by the harness. None of that has been done.

## 6. Missing and open

- Reader coverage and soundness with the learned layer on: not in record 235.
- Any bar for the foundation, learned-layer and word-list measures: none
  declared, so unjudged.
- Why 305 `simple` sentences are abstained on: not in the record.
- Whether the abstract and control perplexities cover the same positions: not
  in the record.
- Whether record 235 was measured with the file-loaded lists: not in the record.
- Any JEPA figure on the Held_Out_Slice: no run.
