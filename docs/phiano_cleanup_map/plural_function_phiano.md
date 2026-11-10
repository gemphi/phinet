# Plural function names: phiano

Requirement 10.5, kind: function, crate `phiano` (free functions and methods). A name that would clash with a singular name already in the same scope takes the suffix `_list` (or `_count` for a number) instead; the rename commit checks the scope.

| Crate | Plural name | Where (under `src`) | Singular |
|---|---|---|---|
| phiano | `load_sentences` | `bin/bootstrap/facet.rs` | `load_sentence` |
| phiano | `classify_with_markers` | `core/searle/speech.rs` | `classify_with_marker` |
| phiano | `scopes_of` | `sys/trace.rs` | `scope_of` |
| phiano | `from_examples` | `persona/mod.rs` | `from_example` |
| phiano | `with_beings` | `sys/lisp.rs` | `with_being` |
| phiano | `count_matches` | `core/searle/markers.rs` | `count_match` |
| phiano | `asks` | `bin/harness/transcript.rs` | `ask_list` |
| phiano | `personality_traits` | `persona/fingerprint/likelihood.rs`, `persona/fingerprint/traits.rs` | `personality_trait` |
| phiano | `default_max_steps` | `server/types/defaults.rs` | `default_max_step` |
| phiano | `board_rows` | `bin/quest/ingest/board.rs` | `board_row` |
| phiano | `totals` | `bin/harness/transcript.rs` | `total` |
| phiano | `stats` | `server/routes/core/ingest.rs` | `stat` |
| phiano | `layers_info` | `server/routes/core/eval.rs` | `layer_info` |
| phiano | `parse_counts` | `bin/harness/spawn.rs` | `parse_count` |
| phiano | `outcomes` | `bin/harness/seam.rs` | `outcome_list` |
| phiano | `dominant_sectors` | `persona/fingerprint/likelihood.rs` | `dominant_sector` |
| phiano | `felicity_conditions` | `core/searle/speech.rs` | `felicity_condition` |
| phiano | `operators_report` | `bin/quest/ingest/report.rs` | `operator_report` |
| phiano | `dev_scores_of` | `bin/harness/ladder/tune.rs` | `dev_score_of` |
| phiano | `beats_trigram_numbers` | `bin/harness/curriculum.rs` | `beat_trigram_number` |
| phiano | `standings` | `sys/race.rs` | `standing_list` |
| phiano | `arm_records` | `bin/harness/act/predict.rs` | `arm_record` |
| phiano | `counts_as_rules` | `core/searle/social.rs` | `counts_as_rule` |
| phiano | `previous_bars` | `bin/harness/record/parse.rs` | `previous_bar` |
| phiano | `categories_report` | `bin/quest/ingest/categories.rs` | `category_report` |
| phiano | `channels` | `bin/harness/curriculum/provenance.rs` | `channel` |
| phiano | `repeats` | `bin/quest/ingest/rows.rs` | `repeat_count` |
| phiano | `stop_words` | `config/functions/phi.rs` | `stop_word` |
| phiano | `select_words` | `core/searle/selection.rs` | `select_word` |
| phiano | `bootstrap_bigrams` | `model/run.rs` | `bootstrap_bigram` |
| phiano | `documents` | `bin/quest/ingest/rows.rs` | `document` |
| phiano | `fingerprints` | `persona/world/matching.rs` | `fingerprint` |
| phiano | `names` | `sys/lisp.rs` | `name` |
| phiano | `fetch_curriculum_extracts` | `wiki/extracts.rs` | `fetch_curriculum_extract` |
| phiano | `compose_variations` | `config/functions/phi.rs` | `compose_variation` |
| phiano | `parse_args` | `command/compose.rs` | `parse_arg` |
| phiano | `split_claims` | `bin/harness/transcript.rs` | `split_claim` |
| phiano | `numbers` | `bin/harness/curriculum/provenance.rs`, `bin/harness/record.rs` | `number` |
| phiano | `from_quests` | `sys/being.rs` | `from_quest` |
| phiano | `words_of` | `sys/trace.rs` | `word_of` |
| phiano | `failing_names` | `bin/harness/spawn.rs` | `failing_name` |
| phiano | `with_scopes` | `sys/lisp.rs` | `with_scope` |
| phiano | `epochs` | `bin/harness/curriculum.rs` | `epoch` |
| phiano | `lookup_definitions` | `instruction/kind.rs` | `lookup_definition` |
| phiano | `words` | `seam.rs` | `word` |
| phiano | `sectors_per_color` | `config/functions/phi.rs` | `sector_per_color` |
| phiano | `construct_intentional_states` | `core/searle/synthesis.rs` | `construct_intentional_state` |
| phiano | `learn_unknown_definitions` | `server/routes/chat.rs` | `learn_unknown_definition` |
| phiano | `grams` | `persona/dialog.rs` | `gram` |
| phiano | `repel_answers` | `bin/quest/ingest/repel.rs` | `repel_answer` |
| phiano | `persona_files` | `sys/being.rs` | `persona_file` |
| phiano | `discover_classes` | `frames.rs` | `discover_class` |
| phiano | `shared_kinds` | `bin/quest/ingest/board.rs` | `shared_kind` |
| phiano | `beats_trigram` | `bin/harness/curriculum.rs` | `beat_trigram` |
| phiano | `dir_sentences` | `lab.rs` | `dir_sentence` |
| phiano | `strip_quotes` | `command/dispatch.rs` | `strip_quote` |
| phiano | `train_file_sentences` | `bin/ingest.rs` | `train_file_sentence` |
| phiano | `gate_records` | `bin/harness/act/predict.rs` | `gate_record` |
| phiano | `extract_topic_words` | `instruction/engine.rs` | `extract_topic_word` |
| phiano | `sentences_of` | `lab.rs` | `sentence_of` |
| phiano | `read_rows` | `bin/quest/ingest/rows.rs` | `read_row` |
| phiano | `beats` | `bin/harness/act/predict.rs` | `beat` |
| phiano | `word_lists` | `bin/harness/act.rs` | `word_list` |
| phiano | `kinds` | `bin/quest/ingest/rows.rs` | `kind` |
| phiano | `train_extracts` | `wiki/extracts.rs` | `train_extract` |
| phiano | `authors` | `bin/persona/ingest/corpus.rs` | `author` |
| phiano | `score_words` | `bin/quest/ingest/liskov.rs` | `score_word` |
| phiano | `shuffle_classes` | `frames.rs` | `shuffle_class` |
| phiano | `bars` | `bin/harness/harness.rs` | `bar` |
| phiano | `submodules` | `bin/harness/record/stamp.rs` | `submodule` |
| phiano | `pairs` | `sys/trace.rs` | `pair` |
| phiano | `gold_figures` | `bin/harness/ladder/gold.rs` | `gold_figure` |
| phiano | `hash_weights` | `bin/harness/curriculum.rs` | `hash_weight` |
| phiano | `modified_paths` | `bin/harness/record/stamp.rs` | `modified_path` |

