# Phiano cleanup map

Written before any file moves (Requirement 10.1). Sections 1 to 4 are the module
table (R10.2); section 5 the re-export callers (R10.3); section 6 the persona
merge (R10.4). Plural names (R10.5) and groups (R10.6) are in section 7 and 8,
which point to files in `docs/phiano_cleanup_map/` so that no file passes 300
lines.

Rule of the cleanup: phiano keeps only what belongs to phiano (its REPL, its
server, its data directory, its personas, its harness). Everything else moves
into the sibling crate that already owns that kind of thing. Nothing exists in
two places: a moved item is deleted from phiano, and its callers name the new
home. Names are singular. No file is longer than 300 lines.

Counts below come from `grep` over `crates/phiano/{src,examples,tests}` on
branch `english-reader-coverage`. "src users" means other top-level modules of
`phiano/src` that name `crate::<module>`. "ex/test files" means files under
`examples/` and `tests/` that name `phiano::<module>`.

Crate dependency order that bounds every move (from the `Cargo.toml` files):
`phistd` < `phiton` < `phifno` < `phical` < `phiecs` / `phient` < `phiclm` /
`phisor` / `philog`. A module may only move to a crate that already depends on
everything the module uses. `phiecs` and `phient` list each other in their
manifests today; moves below keep to the direction `phient -> phiecs`.

## 1. Module table: `phiano/src`

Sizes are lines of `.rs` under the entry. Target `stays` means it remains in
`phiano`.

| Module | What it is | Uses | Used by | Target | Reason |
|---|---|---|---|---|---|
| `arena` | `arena/ecs.rs`: the `Ecs` reader, words told as rows into a `phiecs` `World` and read from the phasors (148 lines) | `sys` (`Agent`, `Being`, `Persona`), `phient::lineup::Concept`, `phiecs::Row` | src: `seam`; 10 ex/test files | `phient` (`lineup`) | Its own doc says the other readers live in `phient::lineup`; this one only stayed because it reads a persona. Once `sys` moves, nothing app-specific is left |
| `bin` | 12 binaries and the harness (69 files, 8753 lines); see section 2 | `config`, `data`, `lab`, `tokenizer`, `persona`, `seam` | Cargo `[[bin]]` entries | split (see section 2) | Binaries that read phiano's `data/` and `config` file names stay; `helix` and `relate` use none of it and move |
| `command` | REPL commands: `chat`, `learn`, `define`, `stats`, `help`, `dispatch`, `persona` and 16 more (24 files, 1744 lines); see section 3 | `config`, `core`, `data`, `drivers`, `instruction`, `persona`, `tokenizer`, many `phient` items | src: `main`, `server`, `model`, `drivers` | stays in `phiano` | The REPL is the app. Calculation bodies inside it (`oscillator`, `wave`, `resonance`, `compose`) are listed in the groups (task 9.3), not moved wholesale |
| `config` | `PhiConfig`, constants (`constants/*`) and derived functions (`functions/*`) (9 files, 774 lines) | `phistd` constants only | src: `server`, `bin`, `command`, `persona`, `frames`, `model`; 36 ex/test files | split | Math constants (`PHI_SQUARED`, `PI_CONST`, `E`, `PRIME_*`, `COLOR_BANDS`, `primes.rs`) go to `phistd`; sector, compose and eval functions go to `phient::generate` and `phient::eval` (task 14.2 moved `sector_resolution`, `sector_exponent`, `compose_variations` and `total_compose_weight` to `phient::generate::sector` and deleted the `eval_overall` forwarder; `sector_prime` and `sector_per_color` wait for `PRIME_*` and `COLOR_BANDS` to reach `phistd`, 14.4); file names (`CHROMA_FILE`, `MEMORY_FILE`, ...), `LEARNING_RATE`, `CHECKPOINT_EVERY_TURNS` and `PhiConfig` stay as app config |
| `core` | `core/searle/*`: Searle 16-agent cognitive architecture (13 files, 1566 lines) | `config`, `data`, `tokenizer` | src: `server` (4), `command`, `instruction`, `model` | `phiton` (`p64::searle`) | `phiton::p64::searle` already reads the same speech acts; the doc says the two "are not joined yet". One Searle in one crate. Needs `data::path` replaced by a path argument first |
| `data` | `data.rs`: finds `phiano/data/` or `$PHIANO_DATA` (46 lines) | std only | src: `model`, `drivers`, `server`, `command`, `lab`, `bin` (13), `core`; 1 ex/test file | stays in `phiano` | It names phiano's own data directory |
| `drivers` | `Driver` enum plus `ingest`, `chunk`, `train` source commands (4 files, 261 lines) | `command::Context` | src: `command` | stays in `phiano` | Device-like front end of the REPL; depends on `command::Context` |
| `frames` | `frames.rs`: `shuffle_classes`, `collapse_small`, `predictive_gain` (about 100 lines). `discover_classes` moved in task 14.2 to `phient::study::class` | `config::PHASE_CHANNELS`, `phasor::{fnv1a, mix64}` | no src user; 2 ex/test files (`frames`, `riley`) | `phical` (`classes`) for the three pure functions, once `PHASE_CHANNELS` is in `phistd` (task 14.4) | `predictive_gain` reads `PHASE_CHANNELS`, which lives in `phiecs`; `phical` may not depend on `phiecs` (cycle), so the three wait for the constant to reach `phistd` |
| `instruction` | `instruction.rs` + `instruction/*`: prompt classification and grounded reply (4 files, 377 lines with `instruction.rs`) | `core`, `command`, `phient::train::grounding` | src: `server`, `command` (2) | stays in `phiano` | The REPL/server prompt pipeline |
| `lib` | `lib.rs`: module list plus 10 compatibility re-exports (41 lines) | all of the above | every caller | stays in `phiano` | Loses each re-export once its callers are fixed (section 5) |
| `main` | `main.rs`: REPL binary and server start (61 lines) | `command`, `model`, `server` | `[[bin]] phiano` | stays in `phiano` | The app entry point |
| `model` | `model.rs` + `model/run.rs`, `model/iterate.rs`: the app's whole `Model` (3 files, 334 lines) | `command`, `config`, `core`, `persona`, many `phient`/`phiecs` items | src: `main`, `server` (2), `seam` (comment only) | stays in `phiano` | The app's state and REPL loop |
| `persona` | `persona/*`: `Style`, `Dialogue`, `PersonaStore`, `Impersonator`, `Fingerprint`, `Styles` (14 files, 1148 lines) | `config`, `tokenizer`, `phient::generate`, `phient::train` | src: `bin` (3), `model`, `command` (3); 4 ex/test files | stays in `phiano` | The persona library is the one home (design default); other persona folders thin down to it (sections 5 and 6) |
| `seam` | `seam.rs`: phasor student coupled with the compiler (107 lines) | `arena::Ecs`, `phient::study::arena::Reader`, `phiclm` | src: `bin` (harness, 2) | `phient` (`study`) | The doc says it "was the phient crate, folded here"; it uses no phiano item besides `arena` |
| `server` | axum HTTP/SSE server: `api`, `intent`, `routes`, `types` (25 files, 2042 lines) | `model`, `core`, `data`, `command`, `instruction`, `wiki` | src: `main` | stays in `phiano` | The app's web front end; only phiano pulls `axum`/`tokio` |
| `sys` | `sys.rs` + `sys/*`: `Agent`, `Being`, `circus`, `eye`, `race`, `set`, `lisp`, `trace` (10 files, 813 lines) | `phiecs` (`Persona`, `World`, `Row`, `kit::eye`), `phiclm::lisp`, `phient::relate::scopes` | src: `arena`; 7 ex/test files | split | `agent`, `being`, `circus`, `eye`, `race` go to `phiecs`; `set` and the `lisp` host go to `phiclm::lisp` (it owns the machine and `lisp::set`); `trace` goes to `phient::relate` (it uses `relate::scopes`) |
| `tokenizer` | `tokenizer.rs`: one line, `pub use phiton::p64::words::Tokenizer` (5 lines) | `phiton` | src: 10 modules (`core` 7, `bin` 7, `command` 3, `persona` 3, `server` 2, `wiki`, `lab`, `frames`, `model`, `instruction`); 35 ex/test files | `phiton` (already there: delete the alias) | A second path to one type. Callers switch to `phiton::p64::words::Tokenizer` |

Outside `src`, in `crates/phiano`: `tests/` holds only `fixtures/broken` (stays).
`data/` is the data directory (stays). `personas/` is persona data (stays;
see section 6). `analysis/`, `specs/`, `tasks/`, `refs/`, `scripts/`, `web/`,
`models/`, `_to_delete/` and `jobs/` hold no Rust; they are not modules and are
not moved by this cleanup. `jobs/` holds CV and job-spec files that are not
phiano's: flagged for the maintainer, not decided here.

No module in this table has `phisor` as its target, so task 14.1 moves
nothing. The only `phisor` mention in sections 1 to 4 is the `ladder` example
(section 4), which is an example file, not a `phiano/src` module.

## 2. `src/bin`

Every `[[bin]]` name stays the same when a binary moves; its `Cargo.toml` entry
moves with it. "Stays" means the binary reads `phiano::data` or `phiano::config`
file names, which are phiano's own.

| Entry | What it is | Uses | Target | Reason |
|---|---|---|---|---|
| `bin/evaluate.rs` | held-out perplexity runner | `data`, `tokenizer` | stays in `phiano` | Reads phiano's corpus paths; thin over `phient::metrics` |
| `bin/ingest.rs` | corpus into a facet | `config`, `data`, `tokenizer`, `wiki` | stays in `phiano` | Writes phiano's `CHROMA_FILE`; calls `wiki` through its new `phient` path |
| `bin/bootstrap/facet.rs` | builds the facet the REPL loads | `config`, `data`, `tokenizer` | stays in `phiano` | Produces the app's shipped model file |
| `bin/bpe/check.rs`, `bin/bpe/lexicon.rs` | Phi-4 tokenizer checks on Austen | `config`, `data`, `lab`, `tokenizer` | stays in `phiano` | Read `data/tokenizers/phi4`; `lab` use becomes `phient::study` |
| `bin/harness/*` | the level-by-level evaluation harness (39 files, about 6400 lines) | `bars`, `level`, `record`, `seam`; ten crates | stays in `phiano` | Its doc says it is a binary so no library names all ten crates (R1.11) |
| `bin/helix/main.rs` | write or read a `.chroma` as a helix tree | no `phiano` item | `phical` (`helix`) | `phical/src/helix.rs` is the library half; the binary goes beside it |
| `bin/persona/*` | `persona ingest\|chat\|quest` | `config`, `data`, `lab`, `persona`, `phasor`, `relate`, `tokenizer` | stays in `phiano` | Persona verbs; reduced to `main` plus calls into `src/persona` (section 6) |
| `bin/prepare/prose.rs` | WikiText-2 into a prose corpus | `data`, `tokenizer` | stays in `phiano` | Writes `data/prose_corpus.txt` |
| `bin/quest/*` | `quest ingest\|ask`: a `phiecs` quest into a facet, with measures (11 files) | `config`, `data`, `phasor` | stays in `phiano` | Reads phiano `data/quest`; its holdout and board measures go to `phient::metrics` in the groups (task 9.3) |
| `bin/relate/main.rs` | ask a `.relate` model what a word calls for | `relate` only | `phient` (`relate`); moved in task 14.2 to `phient/src/bin/relate/main.rs` | No phiano item used |
| `bin/rust/book.rs` | `train_rust_book` | `data` | stays in `phiano` | Reads `data/rust_book` |
| `bin/word/json.rs` | two-sided phasors of words as JSON | `config`, `data` | stays in `phiano` | Reads the shipped lexicon in `data/` |

## 3. `src/command`

All files stay in `phiano`: each is one REPL verb (`Command`), dispatched by
`command/dispatch.rs` through `Context`. Used by `main`, `model`, `server` and
`drivers`. Calculation bodies inside a verb are the subject of task 9.3.

| Entry | What it is | Target | Reason |
|---|---|---|---|
| `command/mod.rs`, `dispatch.rs`, `help.rs`, `tests.rs` | `Context`, `Dispatcher`, help text | stays in `phiano` | The REPL frame |
| `command/chat.rs`, `learn.rs`, `define.rs`, `save.rs`, `stats.rs`, `layers.rs`, `eval.rs`, `benchmark.rs`, `synthetic.rs`, `synonym.rs`, `generate.rs`, `compose.rs`, `instruction.rs` | thin verbs over `Model` | stays in `phiano` | Parse the line, call a library, print |
| `command/cognitive.rs`, `reasoning.rs`, `personate.rs` | verbs over `core::searle` and `persona` | stays in `phiano` | Follow `core` (to `phiton`) by calling its new path |
| `command/oscillator.rs`, `wave.rs`, `resonance.rs` | oscillator, wave and resonance verbs (122, 29, 42 lines) | stays in `phiano` | Verb stays; any math inside moves per the groups (task 9.3) |
| `command/persona/mod.rs` | `PersonaCmd` (197 lines) | stays in `phiano` | Thin dispatch into `src/persona` (section 6) |

## 4. `examples` and `personas`

`personas/` (over 300 Markdown files: root, `code/`, `docs/`, `stage/`) is data
that `persona::world` and `bin/persona` read. It stays in `phiano`.

`examples/` has 40 folders and 31 single files. Rule: an example that names
`phiano::persona` or `phiano::data` stays in `phiano`. Every other example runs
an experiment on a sibling crate and moves to that crate's `examples/` once its
`phiano::` paths name the real home (sections 5 and 6). Dominant sibling crate, counted
from `use` lines, in brackets.

| Entry | Names from `phiano` | Target | Reason |
|---|---|---|---|
| `persona`, `impersonate`, `austen` | `persona`, `config`, `lab`, `tokenizer` | stays in `phiano` | Demo the persona library |
| `learner` | `arena`, `data` | stays in `phiano` | Reads `phiano::data` |
| `student`, `circus.rs`, `race.rs`, `think.rs` | `sys` | `phiecs` | Follow `agent`, `being`, `circus`, `race` |
| `lisp.rs`, `sets.rs`, `writer.rs` | `sys`, `lab` | `phiclm` | Follow the `lisp` and `set` host |
| `arena.rs`, `band.rs`, `crowd.rs`, `intent.rs`, `person.rs`, `talk.rs`, `teacher.rs`, `template.rs`, `workshop` | `arena` | `phient` | Follow `arena::Ecs` into `phient::lineup` |
| `flow.rs`, `meaning.rs` | `compose`, `facet`, `storage`, `trainer` | `phient` | Read-only today; they move or are edited with the re-exports (sections 5 and 6) |
| `anatomy`, `bands`, `builders`, `cast`, `code`, `conception`, `constructed`, `context`, `demo`, `dimensions`, `english`, `family`, `forest`, `frames`, `kind`, `ladder`, `latency`, `marked`, `mirror`, `net`, `operator`, `promoted`, `relational`, `relations`, `riley`, `roles`, `senses`, `squares`, `story`, `strand`, `sweep`, `topdown`, `types`, `waves` | only `config`, `lab`, `phasor`, `relate`, `prose`, `tokenizer`, `frames` or nothing | `phient` (most); `phical` for `kind`, `operator`, `waves`; `phiton` for `mirror` | No phiano-only item; they measure `phient` models. `ladder` also moves to `phisor` if its rungs only call `phisor::ladder` |
| `bcpl.rs`, `decode.rs`, `forgetting.rs`, `generate.rs`, `hexagram.rs`, `learn.rs`, `liskov.rs`, `novelty_fold.rs`, `order.rs`, `pairing.rs`, `phinet.rs`, `readout.rs`, `sentence.rs`, `sides.rs`, `bench.rs` | `config`, `lab`, `tokenizer` | `phient` | Single-file experiments of the same kind; `liskov.rs` to `phical` (`liskov`) |

## 5. Compatibility re-exports in `phiano/src/lib.rs`

`lib.rs` has ten `pub use` lines. Callers were found by scanning every `.rs`
file under `crates/*/src`, `crates/*/examples`, `crates/*/tests` and the root
`tests/` for `phiano::<name>` and, inside `crates/phiano`, `crate::<name>` (also
the `{...}` list form). No `use super::<name>` exists. Counts are lines, then
files. Doc-comment mentions are counted and marked, since they must change too.
Paths below are under `crates/phiano/` unless they begin with another crate.
The order of removal is fixed by design (one commit per row group, callers in
the same commit, R11.2).

| Re-export | Owner path callers switch to | Callers (lines / files) | Where |
|---|---|---|---|
| `phasor` | `phiecs::phasor::spectral` for `SpectralPhasor`, `TorusPhasor`, `Rounding`; `phistd::{fnv1a, mix64}` for the hashes | 29 / 27 | `src`: `frames.rs`, `core/searle/selection.rs`, `bin/quest/ingest/{mod,repel}.rs` (`Rounding`), `bin/persona/quest/relate.rs`; `lib.rs` doc line. `examples`: `bands` (doc only), `builders/{follow,graded,weighted,wordposts}`, `code/reader/mod`, `code/relations/newfn`, `code/write/model`, `constructed`, `demo`, `dimensions`, `family/moses`, `kind`, `latency` (`SpectralPhasor`), `promoted/{main,pair,profiles}`, `relational`, `riley`, `roles` (`SpectralPhasor`), `senses` |
| `facet` | `phiecs::phasor::facet` | 3 / 3 | `examples/flow.rs`, `examples/meaning.rs`; `lib.rs` doc line |
| `storage` | `phiecs::phasor::facet::storage` | 2 / 2 | `examples/flow.rs`; `lib.rs` doc line |
| `trainer` | `phient::train::trainer` | 1 / 1 | `examples/meaning.rs` |
| `compose` | `phient::generate::compose` | 2 / 2 | `examples/flow.rs` (`compose::flow::RiverFlow`); `lib.rs` doc line |
| `prose` | `phient::prose` | 27 / 12 | `examples`: `context` (3), `forest/{casts,main,words}`, `ladder`, `mirror`, `operator`, `story/{main (doc, 14),operator,reader,strata}`, `waves/strata` |
| `operator` | `phient::operator` | 0 / 0 | none. The example folder `examples/operator` names `phiano::prose` and `phiano::relate`, not this alias. Delete the line with no caller change |
| `relate` | `phient::relate` | 57 / 30 | `src`: `bin/relate/main.rs`, `bin/persona/quest/{mod (doc),relate}.rs`. `examples`: `builders/phient`, `code/relations/{counts (doc),mod,newfn,owner,readings}`, `code/student/{mod,student (doc)}`, `context`, `demo/{main,train}`, `family/{main,moses,rough}`, `forest/{casts,waves,words}`, `kind`, `ladder`, `mirror`, `operator`, `story/{main (doc),operator,reader,strata}`, `waves/strata`. Other crate: `crates/phient/src/lib.rs` doc line says "`phiano::relate` is this crate's"; reword it |
| `code` | `phient::code` | 6 / 4 | `examples/code/relations/{mod,newfn}.rs` (`Reader`, `read_files`); `examples/code/student/{rows,student}.rs` (doc only) |
| `phiton` | `::phiton` (the crate itself) | 1 / 1 | `lib.rs` doc line only. No file names `phiano::phiton` or `crate::phiton`; the line goes with no caller change |

Notes for tasks 9.4 and later:

- `examples/flow.rs` and `examples/meaning.rs` are the only callers of `facet`,
  `storage`, `trainer` and `compose`. `lib.rs` calls them read-only; editing
  them is part of the removal commit, not a reason to keep the alias.
- `phasor` is the largest group (27 files). The hash calls (`fnv1a`, `mix64`)
  already come from `phistd` (`phiecs::phasor::spectral::widen` re-exports
  them), so the callers name `phistd`, which many already do beside it
  (`phistd::mix64(phiano::phasor::fnv1a(..))`).
- Examples that name only `phiano::relate` or `phiano::prose` move to `phient`
  later (section 4); their paths change now, so the move later is a pure move.
- `crate::tokenizer` is not a `lib.rs` re-export (it is a one-line module);
  its callers are listed in section 1, not here.

## 6. The four `persona` folders

Today (lines from the working tree):

| Folder | Files | What is there |
|---|---|---|
| `src/persona` | 14 files, 1148 lines | The library: `Styles` (the world), `Style`, `Fingerprint`, `Impersonator`, `Dialogue`, `Reply`, `PersonaStore` |
| `src/command/persona` | 1 file, 197 lines | `PersonaCmd`: REPL verbs `add\|from\|list\|show\|compare\|impersonate\|match\|chat`; plus a private `parse_quoted` |
| `src/bin/persona` | 9 files, 539 lines | `main` (16) dispatching `chat`, `ingest`, `quest`; each verb carries its own corpus, sampling and constants |
| `examples/persona` | 2 files, 135 lines | An evaluation of persona conditioning; `world.rs` rebuilds the three personas "exactly as `bin/austen` builds them" |

The one home is `src/persona` (design default). Nothing is copied: each
duplicate below is deleted and its callers name the one copy.

Duplicated today, and the one place each goes:

| Duplicate | Copies | Goes to |
|---|---|---|
| `sample(v, n, seed)`, a seeded xorshift shuffle taking `n` | `src/bin/persona/ingest/corpus.rs:43`, `examples/persona/world.rs:10`, an inline copy in `examples/austen/main.rs` (about line 85) | One `persona::corpus::sample` in `src/persona`. The three callers use it. (The `bin` copy clamps `n` to `v.len()`; take that form.) |
| Author for a corpus file (`austen` is Jane Austen; `chesterton*` is G. K. Chesterton; `carroll*` is Lewis Carroll) | `src/bin/persona/ingest/corpus.rs:13` (`author_of`), `examples/persona/world.rs` (twice, as an `if`), `examples/austen/main.rs:100`; the names alone are also printed in `examples/impersonate`, `examples/austen/sections.rs` | One `persona::corpus::author_of` and `authors()` in `src/persona`. `examples/persona` and `examples/austen` read `authors()`. The printed name columns use the names `authors()` returns |
| `LEXICON = "personas.chroma"`, `STORE = "personas.json"` | `src/bin/persona/chat/mod.rs:25`, `src/bin/persona/ingest/mod.rs:31` | `persona::store` as `pub const LEXICON` and `STORE`, beside `PersonaStore` |
| The 80/10/10 split, seed 7, then 600-sentence personas with seeds 11 and 13 (the "three personas") | `bin/persona/ingest` (`authors()`), `examples/persona/world.rs::build`, `examples/austen/main.rs` | `persona::corpus::three_personas(facet, split, contrast)` returns `(Styles, Facet)`. It is what `examples/persona/world.rs::build` is today, moved. The held-out prompt helper `held_prompts` stays in the example (only that example needs it) but calls `authors()` |
| Held-out test of a quest row (hash of about, kind, other; below 0.2) | `src/bin/persona/quest/relate.rs:12`, `examples/kind/main.rs:41`; the original is `phient::relate::held_out(&Row)` (`phient/src/relate/quests.rs`), same rule, already used by other examples | Delete both local copies; call `phient::relate::held_out`. Nothing new is written |
| `parse_quoted(&str)` | `src/command/persona/mod.rs:173` only | Stays; one copy. Moves to `src/persona` only if a second caller appears |

What stays where after the merge:

| Folder | Keeps | Loses |
|---|---|---|
| `src/persona` | The library; gains `corpus.rs` (`sample`, `author_of`, `authors`, `three_personas`) and the two store constants | Nothing |
| `src/command/persona` | `PersonaCmd`, the verbs and `parse_quoted`; each verb is parse, call the library, print | Nothing today; it already holds no logic besides `parse_quoted` |
| `src/bin/persona` | `main`, `chat/{mod,script,judge}`, `ingest/{mod,measure}`, `quest/{mod,relate}`: the three verbs and their printing | `ingest/corpus.rs` (moves to `src/persona/corpus.rs`); the two constants in `chat/mod.rs` and `ingest/mod.rs`; `held_out` in `quest/relate.rs` |
| `examples/persona` | `main.rs` (the conditioning measurement) and `held_prompts` | `sample`, the author `if`, and the build loop in `world.rs`; `world.rs` shrinks to `held_prompts`, or folds into `main.rs` |

Order of work (task 9.4 or later): first add `src/persona/corpus.rs` and the
constants; then switch `bin/persona/ingest`, `examples/persona`,
`examples/austen`; then delete the old copies, in the same commit as the
switch. Each commit builds with 0 warnings and `cargo test -p phiano` passes.
`src/persona` stays under 300 lines per file (its largest file is 180).

`personas/` (the Markdown files) is data, read by `persona::world`; it stays.

## 7. Plural names (R10.5)

218 plural module, type and function names in `phiano/src` and `phisor/src`,
each with its singular replacement, in `docs/phiano_cleanup_map/`:

| File | Kind | Rows |
|---|---|---|
| `plural_module.md` | module | 31 (phiano 19, phisor 12) |
| `plural_type.md` | type | 30 (phiano 8, phisor 22) |
| `plural_function_phisor.md` | function | 83 |
| `plural_function_phiano.md` | function | 74 |
| `plural_excluded.md` | names kept (singular words ending in `s`, verbs, acronyms, external names), the scan rule, the counts | |

## 8. Groups of related calculations (R10.6)

13 groups in `docs/phiano_cleanup_map/group.md`: five calculations that exist as
several copies (seeded shuffle, held-out split, slug, personality traits, date
from the clock) and eight that sit apart (variation counts, twin combination,
word-list loading, irregular forms, Searle markers, sector arithmetic, quest
measures, oscillator and wave).

## 9. The rule applied (R10.7, R10.8)

`.kiro/steering` holds no `phinet` naming or line-cap rule. The rule applied is
singular names, 300 lines per file, and `act.rs` split by role. See
`docs/phiano_cleanup_map/rule.md`. `tests/structural/cleanup.rs` fails if a
top-level entry of `phiano/src` has no row in section 1.
