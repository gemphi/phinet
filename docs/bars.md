# The declared Bars

This is the declared-Bars input the Recorder reads once before the first level
runs (Requirement 2 criterion 7). Each row names one measured number, the Bar
value it is judged against, the `Sense` the Bar faces, and the number's unit.

A name absent from this table is `Unjudged`, never a pass (Requirement 2
criterion 6). The sense is declared with the Bar and never inferred from the
unit: `lowering.unlowered` and `curriculum.perplexity` are the two Bars met by
falling at or below their value, so they carry `AtMost`; every other Bar is a
floor and carries `AtLeast`. A judge that assumed every Bar is a floor would
pass a perplexity of 500, which is why the sense is written down here rather
than guessed.

The reader (`phiano/src/bin/harness/bars.rs`) parses every row whose first cell
is a backtick-quoted name. The `value` cell is the declared Bar value in the
number's own unit. Where a Bar is relative to a control arm — the terminator
majority baseline, the Hash_Null's z-score, the baseline's act coverage or the
better single-end Twins reading — the declared `value` is the offset added to
that control at judging time (0.0 where the Bar is exactly the control, 10.0
for "baseline + 10 points", and so on); the Ladder resolves the control and
offsets this value before calling `judge` (task 17.5). The `control` column
below names that control arm for the human reader and is not parsed.

| name | value | sense | unit | control |
|---|---|---|---|---|
| `ladder.function_purity.learned` | 56.7 | AtLeast | Percentage | — |
| `ladder.function_purity.gap` | 5.0 | AtLeast | Percentage | hash function purity |
| `ladder.terminator_purity.learned` | 0.0 | AtLeast | Percentage | terminator majority baseline |
| `ladder.river_z.learned` | 3.0 | AtLeast | ZScore | — |
| `ladder.river_z.above_hash` | 0.0 | AtLeast | ZScore | hash z-score |
| `ladder.act.coverage` | 10.0 | AtLeast | Percentage | baseline act coverage |
| `ladder.act.round_trip` | 91.3 | AtLeast | Percentage | — |
| `ladder.act.force_purity` | 92.8 | AtLeast | Percentage | — |
| `keyboard.operator_count` | 64.0 | AtLeast | Count | — |
| `lowering.unlowered` | 0.0 | AtMost | Count | — |
| `lowering.agreeing` | 64.0 | AtLeast | Count | — |
| `truth.recall.degenerate` | 0.95 | AtLeast | F1 | — |
| `truth.recall.contradicting` | 0.80 | AtLeast | F1 | — |
| `learn.twins.both` | 2.0 | AtLeast | Percentage | better single-end reading |
| `learn.amplitudes` | 0.0 | AtLeast | Percentage | probability combination purity |
| `curriculum.top1` | 24.8 | AtLeast | Percentage | trigram top-1 |
| `curriculum.perplexity` | 84.7 | AtMost | Perplexity | trigram perplexity |
| `ladder.act.per_field` | 0.0 | AtLeast | Percentage | baseline per-field score |

The eighteen rows above cover every Bar the requirements name. `lowering.cintcode_agreeing` is deliberately absent: the O-code to Cintcode stage is not built, so that number is recorded `unjudged` with the stage marked not built (Requirement 7 criterion 10), which an absent Bar produces exactly.
