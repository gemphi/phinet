# Plural module names

Requirement 10.5, kind: module. Each row is a `mod` (a file or a folder) whose name ends in a plural noun; "Where" is the file that declares it. A name that would clash with a singular name already in the same scope takes the suffix `_list` (or `_count` for a number) instead; the rename commit checks the scope.

| Crate | Plural name | Where (under `src`) | Singular |
|---|---|---|---|
| phiano | `rows` | `bin/quest/ingest/mod.rs` | `row` |
| phisor | `labels` | `ladder/measure.rs` | `label` |
| phiano | `requests` | `server/types.rs` | `request` |
| phisor | `kinds` | `ladder/mod.rs`, `ladder/property.rs` | `kind` |
| phiano | `categories` | `bin/quest/ingest/mod.rs` | `category` |
| phisor | `counts` | `ladder/measure.rs` | `count` |
| phiano | `personas` | `persona/world.rs` | `persona` |
| phiano | `frames` | `lib.rs` | `frame` |
| phiano | `extracts` | `wiki.rs` | `extract` |
| phiano | `primes` | `config/constants.rs` | `prime` |
| phiano | `layers` | `command/mod.rs` | `layer` |
| phiano | `routes` | `server/mod.rs` | `route` |
| phiano | `traits` | `persona/fingerprint/mod.rs` | `personality` |
| phiano | `types` | `config/constants.rs`, `core/searle/mod.rs`, `server/mod.rs` | `shape` |
| phiano | `drivers` | `lib.rs` | `driver` |
| phisor | `seeds` | `measure/mod.rs` | `seed` |
| phiano | `stats` | `command/mod.rs` | `stat` |
| phiano | `defaults` | `server/types.rs` | `default` |
| phiano | `bars` | `bin/harness/main.rs` | `bar` |
| phisor | `unreachable_frames` | `gold/mod.rs` | `unreachable_frame` |
| phisor | `lists` | `lexicon.rs` | `list` |
| phiano | `bands` | `bin/harness/curric.rs` | `band` |
| phisor | `ops` | `machine/mod.rs` | `op` |
| phisor | `words` | `lexicon/lists/mod.rs` | `word` |
| phiano | `functions` | `config/mod.rs` | `function` |
| phisor | `twins` | `measure/mod.rs` | `twin` |
| phisor | `features` | `ir/mod.rs` | `feature` |
| phiano | `constants` | `config/mod.rs` | `constant` |
| phisor | `units` | `guide/mod.rs`, `lib.rs` | `unit` |
| phisor | `names` | `ladder/act.rs` | `name` |
| phiano | `markers` | `core/searle/mod.rs` | `marker` |


## Status (task 15.1)
All rows renamed except two that no longer exist. Renames: `types` became `shape` (not `type`, a keyword) in `config/constant`, `core/searle` and `server`; `traits` became `personality` as the table says. `frames` (`lib.rs`) and `extracts` (`wiki.rs`) are gone: neither module is in the tree. `lists` is now the folder `lexicon/list/` and `words` is `lexicon/list/word.rs`. The phiano and phisor module names in the table are the old names.
