
# The English reader: latest harness figures

Every figure below is copied from run record `235_2026-11-07_harness.md`
(workspace `ad92b04`, `phiano` `6e4442e`, run on 2026-11-07). The previous
record is `234_2026-11-05_harness.md`. Nothing here is estimated. Where the
record holds no number, this page says so.

The reader is measured on the Guide's Held_Out_Slice only (the Test part, one
sentence in ten, never tuned on). Coverage is the share of sentences read.
Soundness is the share of the read that round-trips under Canonical_Form.

## Run caveat

The run walked ten levels. Nine passed. The `phiano` level reported
`BuildFailed` (`field exercise is never read`, in `examples/intent` and
`examples/workshop`). That is the known unused-field warning under
`-D warnings`. It is not fixed here. The harness overall verdict is therefore
`Miss: level phiano did not build`. The `ladder.act.*` figures were measured
regardless, in the harness binary itself.

## Coverage and soundness (held-out)

| measure | record 234 | record 235 | bar | verdict |
|---|---|---|---|---|
| `foundation_coverage` | 29.5 | 31.1 | none declared | unjudged |
| `foundation_soundness` | 75.0 | 77.0 | none declared | unjudged |
| `clean_coverage` | 31.2 | 32.9 | 31.2 at least | pass |
| `clean_soundness` | 76.8 | 78.7 | 76.8 at least | pass |

The clean view is the foundation view after Well_Formed: 1201 kept, 114
removed. `clean_soundness_regression` is 0 (no drop beyond 1.0 point). The
clean bars are declared in `phiano/src/bin/harness/act.rs`, not in `bars.md`.

## Gold set: force purity and round-trip

| measure | all 150 | reachable only (124) | bar (all 150) | verdict |
|---|---|---|---|---|
| `force_purity` | 86.8 | 92.0 | 92.8 at least | miss by 6 on all 150 |
| `round_trip` | 86.0 | 84.7 | 100 at least | miss by 14 on all 150 |

The reachable-only figures carry no bar. Reachable-only force purity (92.0)
is still below 92.8. Reachable-only round-trip (84.7) is below the all-150
figure of 86.0. Both bars are missed.

## Unreachable_Frames (26 of 150)

No text-only reader can label these right: another gold frame has the same
surface shape and a different label. The list is in
`phisor/src/gold/unreachable_frames.rs`. The gold files are untouched. A bare
`will` is never mapped to Commit by rule.

| reason | count | indices (0-based) | the two readings |
|---|---|---|---|
| `NameCase` | 18 | 2, 5, 11, 14, 19, 26, 29, 31, 33, 46, 47, 72, 80, 96, 112, 131, 140, 145 | a proper name (London) or a common noun (london); labels `name` / `thing` |
| `BareWill` | 7 | 117, 119, 120, 121, 122, 124, 126 | a prediction (Assert) or a binding (Commit); labels `ASSERT` / `COMMIT` |
| `Unaccusative` | 1 | 58 | the subject undergoes the event or does it; labels `theme` / `agent` |

## Well_Formed operators and removed counts

Text-only filters over the held-out slice. A sentence can fall to more than
one operator, so the counts may sum past 114.

| operator | removes | removed |
|---|---|---|
| `no_etc` | a sentence with an `etc` token (a fragment) | 53 |
| `no_stray_letter` | a single letter other than `a` or `i` (an item label or split stem) | 51 |
| `no_doubling` | a token equal to the next (`shut shut`) | 6 |
| `no_leading_or` | a first token `or` (a list label) | 10 |
| `no_label_a` | `a` followed by a pronoun or pronoun contraction | 10 |

## Abstention kinds (held-out, foundation view)

| kind | count |
|---|---|
| `simple` | 305 |
| `one_join` | 267 |
| `infinitive` | 122 |
| `ing_form` | 76 |
| `multi_clause` | 26 |
| `very_long` | 10 |

## Word-list audit

For each hand-authored list: held-out words it covers, and held-out words
that would change if the list were learned.

| list | covers | would change if learned |
|---|---|---|
| `control_verbs` | 15 | 375 |
| `gerund_verbs` | 11 | 11 |
| `complement_verbs` | 9 | 381 |
| `performatives` | 1 | 389 |
| `set_aside_adverbs` | 22 | 154 |
| `clause_conjunctions` | 15 | 139 |

## Learned layer: prediction and gate

Next-element prediction on the Held_Out_Slice. The layer predicted 252
positions and abstained on 23404 (23656 in total).

| level | arm | top-1 (%) | perplexity | predicted |
|---|---|---|---|---|
| abstract | learned | 54.4 | 22.43 | 252 |
| abstract | shuffled control | absent | 47.36 | 0 |
| abstract | frequency control | absent | 47.02 | 0 |
| surface | learned | 3.6 | 484.06 | 252 |
| surface | shuffled control | absent | 993.29 | 0 |
| surface | frequency control | absent | 1014.56 | 0 |

The control arms answered no position, so their top-1 is a named absence
(`nan`), not zero. The gate therefore judged on perplexity alone.

Gate verdict: `abstract_gate_enabled` = 1. The learned layer beats the
shuffled control (`abstract_gate_beats_shuffled` = 1) and the frequency
control (`abstract_gate_beats_frequency` = 1) on abstract perplexity.

Not measured in record 235: the Reader's coverage and soundness change with
the layer on. The record holds no such figure, so none is given.

## Other bars in the same record

| measure | value | bar | verdict |
|---|---|---|---|
| `ladder.function_purity.learned` | 53.4 | 56.7 at least | miss by 3.3 |
| `ladder.terminator_purity.learned` | 85.4 | 0 at least | pass |
| `ladder.river_z.learned` | 3.39 | 3 at least | pass |
| `ladder.river_z.above_hash` | 3.39 | 0 at least | pass |
| `curriculum.perplexity` | 129.1 | 84.7 at most | miss by 44.4 |

## Bars missed

`force_purity` (86.8 against 92.8), `round_trip` (86.0 against 100),
`function_purity.learned` (53.4 against 56.7) and `curriculum.perplexity`
(129.1 against 84.7 at most). The `bars.md` file lists none of the
`ladder.act.clean_*` or learned-layer measures, so those are declared in code
or unjudged.
