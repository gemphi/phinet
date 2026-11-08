# Groups of related calculations and functions

Requirement 10.6. Each group is a set of functions that compute the same kind
of thing and today sit apart, or exist as several copies. Task 15 puts each
group in one module (Requirement 12.4). "Home" is the one module that keeps it.
Sites are under `crates/` and were read on branch `english-reader-coverage`.
Names are the plural-free ones from the lists in this folder.

## Copies of one calculation (merge into one)

| # | Group | Sites today | Home |
|---|---|---|---|
| 1 | Seeded shuffle and sample | `phiano/src/bin/quest/ingest/mod.rs` `shuffle` (Fisher-Yates on a splitmix stream); `phisor/src/abstraction/predict.rs` `shuffle` (LCG); `phiano/src/frames.rs` `shuffle_class`; `phiano/src/bin/persona/ingest/corpus.rs` `sample`; `phiano/src/bin/quest/ingest/categories.rs` `shuffled`; the inline shuffle in `phisor/src/ladder/measure/river.rs` `shuffled_information` | `phistd::random` (it already holds `unit_interval` and `mix64`): one `shuffle(items, seed)` and one `sample(items, n, seed)`. The callers keep their own measure and pass a seed. Section 6 already points `sample` at `persona::corpus`; that copy then calls `phistd` |
| 2 | Held-out split of a row | `phient/src/relate/quests.rs` `held_out` (the original); `phiano/src/bin/persona/quest/relate.rs` `held_out` (a copy of the hash rule); the held-out test in `examples/kind/main.rs`; `phiano/src/bin/quest/ingest/report.rs` `holdout_report` reads the split | `phient::relate::held_out`. Delete both copies (section 6). `phisor::guide::holdout` is a different split (of strings, not rows) and stays in `phisor` |
| 3 | Dash slug of a name | `phiano/src/bin/harness/transcript/page.rs` `slug`; `phiecs/src/quest/run.rs` `slug`; `phiecs/src/persona.rs` `Persona::slug` | `phistd::text::slug`; the three call it |
| 4 | Persona temperament from a fingerprint | `phiano/src/persona/fingerprint/likelihood.rs` `dominant_sector` and a forwarder `personality_trait`; `.../traits.rs` `personality_trait` | `persona::fingerprint::personality` (today `traits.rs`); `dominant_sector` stays in `likelihood.rs` and is called from it. The forwarder is deleted and callers name the real function |
| 5 | Date from the clock | `phiano/src/bin/harness/record/page.rs` `today` and `civil_from_days`; the commit and modified-path stamp in `.../record/stamp.rs` (`modified_path`, `commit_of`) | `bin/harness/record/stamp.rs` keeps the git stamp; `today` and `civil_from_days` go with it into one `record::date` module (the second caller, if one appears, moves it to `phistd`) |

## Calculations that belong together (move into one module)

| # | Group | Sites today | Home |
|---|---|---|---|
| 6 | Count of Variations and the 20%-of-mean agreement | `phisor/src/ladder/measure/counts.rs`: `count`, `all_count`, `agree_on_count`, `GrownCount`; the in-crate test generator `Triples` in `ladder/property/measure.rs`; `phisor/src/measure/twins.rs` `disagreement` counts | `phisor::ladder::measure::count` (already the file); `agree_on_count` gets its one definition there and the measure code calls it. The three `Counts` structs stay apart: `TransitionCount` (predict), `OverlapCount` (breach), `TestCount` (harness spawn) |
| 7 | Combining two readings | `phisor/src/measure/twins.rs`: `combine_probability`, `combine_amplitude`, `judge_twin`, `measured_twin`, `measure_mode`, `disagreement`, `disagreement_at` | `phisor::measure::twin`, one module (`twins.rs` is 249 lines today and stays under the cap). Only the `c64` amplitude arithmetic belongs to `phistd`; the combination of scores stays here |
| 8 | Word-list loading | `phisor/src/lexicon/lists/{path,words,phrase,adverb,spelling}.rs`: `data_line`, `data_row`, `load_pair`, `load_phrase`, `expressive_phrase`, `expressive_predicate`, `wish_form`, `set_aside_adverb_phrase`, `irregular_plural`, `article_exception`; `phiano/src/config/functions/phi.rs` `stop_word`; `phiano/src/instruction/engine.rs` `extract_topic_word` | `phisor::lexicon::list`: `path.rs` is the one reader (file, lines, rows); `phrase`, `spelling`, `adverb` read through it; `stop_word` reads the stop list from here too. `extract_topic_word` stays in `instruction` and calls the list |
| 9 | Irregular forms and contractions | `phisor/src/lexicon.rs` `Irregular`, `IrregularForm`, `parse_irregular`, `form`; `lexicon/irregular.rs` `Form`, `lemma`; `lexicon/contraction.rs` `Contraction`, `contraction` | `phisor::lexicon::irregular` for forms and lemmas; `contraction` stays its own file. `lexicon.rs` only lists modules. (Language data itself moves to files under task 11) |
| 10 | Searle marker matching | `phiano/src/core/searle/markers.rs` `contains_any`, `starts_with_any`, `count_match`; `speech.rs` `classify_with_marker`, `felicity_condition`; `social.rs` `counts_as_rule`; `selection.rs` `select_word` | One `marker` module: the three string tests and the count. The speech-act classifiers call it. The whole group moves with `core` to `phiton::p64::searle` (section 1) |
| 11 | Sector arithmetic | `phiano/src/config/functions/phi.rs`: `sector_prime`, `sector_resolution`, `sector_exponent`, `sector_per_color`, `compose_variation`, `eval_overall` | `phient::generate::sector` for the first five. Task 14.2 moved `sector_resolution`, `sector_exponent`, `compose_variations` (name kept; the singular rename is section 7) and `total_compose_weight`; `sector_prime` and `sector_per_color` wait for `PRIME_*` and `COLOR_BANDS` to reach `phistd` (14.4). `eval_overall` already forwards to `phient::eval::Evaluator::overall`, so the forwarder is deleted. Constants (`SECTOR_RESOLUTION`, `COLOR_BANDS`) go to `phistd` (section 1) |
| 12 | Quest board and category measures | `phiano/src/bin/quest/ingest/`: `board_row`, `shared_kind`, `read_row`, `repeat_count`, `document`, `category_report`, `shuffled`, `holdout_report`, `operator_report`, `score_word`, `repel_answer` | One `measure` folder of calculations under `bin/quest/ingest/` with printing kept in `report.rs`; the calculations then move to `phient::metrics` (section 2). `shuffled` calls group 1 |
| 13 | Oscillator, wave and resonance | `phiano/src/command/{oscillator,wave,resonance}.rs` (122, 29, 42 lines): verbs with maths inside | The maths goes to one `phical::wave` module (resonance and oscillation are waves); the verbs parse, call it, print. A function-level inventory is done at the move, as the files hold no plural names today |

## Order and checks

- Groups 1 to 5 are copies: each commit deletes copies and edits callers, so
  the number of definitions goes from N to 1. Groups 6 to 13 move code into one
  module without changing results; the recorded figures stay equal.
- Groups 1, 2, 3 and 5 reach other crates (`phistd`, `phient`, `phiecs`); they
  wait for the matching move in section 1.
- Groups 6 to 10 and 12 stay in `phiano` and `phisor` and can be done in task 15.
- Each commit: 0 warnings, `cargo test -p phiano -p phisor` passes, no file over
  300 lines.
