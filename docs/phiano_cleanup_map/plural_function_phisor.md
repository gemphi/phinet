# Plural function names: phisor

Requirement 10.5, kind: function, crate `phisor` (free functions and methods). A name that would clash with a singular name already in the same scope takes the suffix `_list` (or `_count` for a number) instead; the rename commit checks the scope.

| Crate | Plural name | Where (under `src`) | Singular |
|---|---|---|---|
| phisor | `expressive_phrases` | `lexicon/lists/phrase.rs` | `expressive_phrase` |
| phisor | `pairs_in` | `gold/mod.rs` | `pair_in` |
| phisor | `clauses` | `ground/unfold.rs` | `clause` |
| phisor | `per_phase_reads` | `guide/holdout/guard.rs` | `per_phase_read` |
| phisor | `counts` | `ladder/measure/counts.rs` | `count` |
| phisor | `acts` | `gold/mod.rs` | `act` |
| phisor | `force_kinds` | `machine/told.rs` | `force_kind` |
| phisor | `parse_irregulars` | `lexicon.rs` | `parse_irregular` |
| phisor | `constraints` | `machine/ask.rs` | `constraint` |
| phisor | `templates` | `kind/mod.rs` | `template` |
| phisor | `labels` | `truth/set/mod.rs` | `label` |
| phisor | `variations` | `kind/mod.rs`, `ladder/kinds.rs` | `variation` |
| phisor | `answer_told_kinds` | `machine/told.rs` | `answer_told_kind` |
| phisor | `held_roles` | `guide/holdout/guard.rs` | `held_role` |
| phisor | `members` | `code/learned.rs` | `member` |
| phisor | `word_lists` | `ladder/act/measure.rs` | `word_list` |
| phisor | `kind_pairs` | `ladder/measure/river.rs` | `kind_pair` |
| phisor | `pairs` | `abstraction/predict.rs`, `gold/mod.rs` | `pair` |
| phisor | `classes` | `abstraction/mod.rs` | `class` |
| phisor | `wish_forms` | `lexicon/lists/phrase.rs` | `wish_form` |
| phisor | `cases` | `machine/agree/input.rs` | `case` |
| phisor | `bytes` | `ladder/property/independence.rs`, `machine/agree/run.rs` | `byte_list` |
| phisor | `irregular_plurals` | `lexicon/lists/spelling.rs` | `irregular_plural` |
| phisor | `disagreements_at` | `measure/twins.rs` | `disagreement_at` |
| phisor | `kinds` | `ladder/kinds.rs`, `machine/host.rs` | `kind` |
| phisor | `operation_cards` | `lovelace/mod.rs` | `operation_card` |
| phisor | `grown_classes` | `ladder/property/independence.rs` | `grown_class` |
| phisor | `tokens` | `guide/split.rs` | `token_list` |
| phisor | `features` | `ground.rs`, `kind/learn.rs`, `kind/resemblance.rs` | `feature` |
| phisor | `set_aside_adverb_phrases` | `lexicon/lists/adverb.rs` | `set_aside_adverb_phrase` |
| phisor | `preposition_roles` | `machine/told.rs` | `preposition_role` |
| phisor | `seeds` | `ladder/mod.rs`, `measure/seeds.rs` | `seed` |
| phisor | `pieces` | `guide/units.rs` | `piece` |
| phisor | `all_counts` | `ladder/measure/counts.rs` | `all_count` |
| phisor | `data_lines` | `lexicon/lists/path.rs` | `data_line` |
| phisor | `contractions` | `lexicon/contraction.rs` | `contraction` |
| phisor | `bounds` | `machine/word.rs` | `bound` |
| phisor | `load_pairs` | `lexicon/lists/words.rs` | `load_pair` |
| phisor | `answer_kinds` | `machine/ask.rs` | `answer_kind` |
| phisor | `names` | `machine/host.rs`, `units.rs` | `name` |
| phisor | `files_in` | `machine/ops.rs` | `file_in` |
| phisor | `same_fields` | `ir/text.rs` | `same_field` |
| phisor | `carry_arguments` | `machine/agree/run.rs` | `carry_argument` |
| phisor | `words` | `guide/mod.rs` | `word` |
| phisor | `combine_probabilities` | `measure/twins.rs` | `combine_probability` |
| phisor | `openings` | `kind/shape.rs` | `opening` |
| phisor | `link_handles` | `machine/agree/input.rs` | `link_handle` |
| phisor | `expressive_predicates` | `lexicon/lists/phrase.rs` | `expressive_predicate` |
| phisor | `run_measures` | `measure/seeds/blocked.rs` | `run_measure` |
| phisor | `rows` | `machine/host.rs` | `row` |
| phisor | `numbers` | `machine/host/apply.rs`, `measure/seeds.rs` | `number` |
| phisor | `article_exceptions` | `lexicon/lists/spelling.rs` | `article_exception` |
| phisor | `lists` | `ladder/grow.rs`, `ladder/property/nesting.rs` | `list` |
| phisor | `files` | `machine/ops.rs` | `file` |
| phisor | `check_names` | `machine/host.rs` | `check_name` |
| phisor | `arms` | `ladder/property.rs` | `arm` |
| phisor | `judge_twins` | `measure/twins.rs` | `judge_twin` |
| phisor | `lemmas` | `lexicon/irregular.rs` | `lemma` |
| phisor | `clean_strings` | `guide/split.rs` | `clean_string` |
| phisor | `frame_with_features` | `ir/show.rs` | `frame_with_feature` |
| phisor | `answer_told_traced_kinds` | `machine/told.rs` | `answer_told_traced_kind` |
| phisor | `events_and_hole` | `machine/told.rs` | `event_and_hole` |
| phisor | `template_kinds` | `kind/mod.rs` | `template_kind` |
| phisor | `roles` | `gold/read.rs` | `role` |
| phisor | `pairs_table` | `kind/read.rs` | `pair_table` |
| phisor | `counts_agree` | `ladder/measure/counts.rs` | `agree_on_count` |
| phisor | `load_phrases` | `lexicon/lists/words.rs` | `load_phrase` |
| phisor | `measured_twins` | `measure/twins.rs` | `measured_twin` |
| phisor | `caps` | `truth/program.rs` | `cap` |
| phisor | `training_runs` | `guide/split.rs` | `training_run` |
| phisor | `rank_candidates` | `ladder/act/rank.rs` | `rank_candidate` |
| phisor | `held_triples` | `truth/read.rs` | `held_triple` |
| phisor | `roles_agree` | `guide/holdout/guard.rs` | `agree_on_role` |
| phisor | `core_names` | `machine/host.rs` | `core_name` |
| phisor | `all_kinds` | `ladder/kinds.rs` | `all_kind` |
| phisor | `strings` | `guide/split.rs` | `string_list` |
| phisor | `disagreements` | `measure/twins.rs` | `disagreement` |
| phisor | `forms` | `lexicon.rs` | `form` |
| phisor | `slots` | `ir/text/parse.rs` | `slot` |
| phisor | `direct_differences` | `measure/property.rs` | `direct_difference` |
| phisor | `measure_modes` | `measure/twins.rs` | `measure_mode` |
| phisor | `data_rows` | `lexicon/lists/path.rs` | `data_row` |
| phisor | `combine_amplitudes` | `measure/twins.rs` | `combine_amplitude` |

