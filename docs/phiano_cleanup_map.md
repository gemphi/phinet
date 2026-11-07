# Phiano cleanup map

Written before any file moves (Requirement 10.1). Sections: module table (this
file, part 1), re-export callers, persona merge, plural names, groups (parts 2
and 3 are added by tasks 9.2 and 9.3).

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
| `config` | `PhiConfig`, constants (`constants/*`) and derived functions (`functions/*`) (9 files, 774 lines) | `phistd` constants only | src: `server`, `bin`, `command`, `persona`, `frames`, `model`; 36 ex/test files | split | Math constants (`PHI_SQUARED`, `PI_CONST`, `E`, `PRIME_*`, `COLOR_BANDS`, `primes.rs`) go to `phistd`; sector, compose and eval functions (`sector_*`, `compose_variations`, `eval_overall`) go to `phient::generate` and `phient::eval`; file names (`CHROMA_FILE`, `MEMORY_FILE`, ...), `LEARNING_RATE`, `CHECKPOINT_EVERY_TURNS` and `PhiConfig` stay as app config |
| `core` | `core/searle/*`: Searle 16-agent cognitive architecture (13 files, 1566 lines) | `config`, `data`, `tokenizer` | src: `server` (4), `command`, `instruction`, `model` | `phiton` (`p64::searle`) | `phiton::p64::searle` already reads the same speech acts; the doc says the two "are not joined yet". One Searle in one crate. Needs `data::path` replaced by a path argument first |
| `data` | `data.rs`: finds `phiano/data/` or `$PHIANO_DATA` (46 lines) | std only | src: `model`, `drivers`, `server`, `command`, `lab`, `bin` (13), `core`; 1 ex/test file | stays in `phiano` | It names phiano's own data directory |
| `drivers` | `Driver` enum plus `ingest`, `chunk`, `train` source commands (4 files, 261 lines) | `command::Context` | src: `command` | stays in `phiano` | Device-like front end of the REPL; depends on `command::Context` |
| `frames` | `frames.rs`: `shuffle_classes`, `discover_classes`, `collapse_small`, `predictive_gain` (192 lines) | `config::PHASE_CHANNELS`, `phiecs` `Facet`, `phasor::{fnv1a, mix64}`, `Tokenizer` | no src user; 2 ex/test files (`frames`, `riley`) | `phical` (`classes`) for the three pure functions; `phient` (`study`) for `discover_classes` | Class statistics beside `phical::classes`; the one function that reads a `Facet` goes where `Facet` is allowed |
| `instruction` | `instruction.rs` + `instruction/*`: prompt classification and grounded reply (4 files, 377 lines with `instruction.rs`) | `core`, `command`, `phient::train::grounding` | src: `server`, `command` (2) | stays in `phiano` | The REPL/server prompt pipeline |
| `lab` | `lab.rs`: experiment bench: `flag`, `switch`, sentence loaders (92 lines) | `data`, `tokenizer` | src: `bin` (2); 21 ex/test files | `phient` (`study`) | Used by experiments, not by the app. Its `data::path` use becomes a path argument |
| `lib` | `lib.rs`: module list plus 12 compatibility re-exports (41 lines) | all of the above | every caller | stays in `phiano` | Loses each re-export once its callers are fixed (section 2 of task 9.2) |
| `main` | `main.rs`: REPL binary and server start (61 lines) | `command`, `model`, `server` | `[[bin]] phiano` | stays in `phiano` | The app entry point |
| `model` | `model.rs` + `model/run.rs`, `model/iterate.rs`: the app's whole `Model` (3 files, 334 lines) | `command`, `config`, `core`, `persona`, many `phient`/`phiecs` items | src: `main`, `server` (2), `seam` (comment only) | stays in `phiano` | The app's state and REPL loop |
| `persona` | `persona/*`: `Style`, `Dialogue`, `PersonaStore`, `Impersonator`, `Fingerprint`, `Styles` (14 files, 1148 lines) | `config`, `tokenizer`, `phient::generate`, `phient::train` | src: `bin` (3), `model`, `command` (3); 4 ex/test files | stays in `phiano` | The persona library is the one home (design default); other persona folders thin down to it (task 9.2) |
| `seam` | `seam.rs`: phasor student coupled with the compiler (107 lines) | `arena::Ecs`, `phient::study::arena::Reader`, `phiclm` | src: `bin` (harness, 2) | `phient` (`study`) | The doc says it "was the phient crate, folded here"; it uses no phiano item besides `arena` |
| `server` | axum HTTP/SSE server: `api`, `intent`, `routes`, `types` (25 files, 2042 lines) | `model`, `core`, `data`, `command`, `instruction`, `wiki` | src: `main` | stays in `phiano` | The app's web front end; only phiano pulls `axum`/`tokio` |
| `sys` | `sys.rs` + `sys/*`: `Agent`, `Being`, `circus`, `eye`, `race`, `set`, `lisp`, `trace` (10 files, 813 lines) | `phiecs` (`Persona`, `World`, `Row`, `kit::eye`), `phiclm::lisp`, `phient::relate::scopes` | src: `arena`; 7 ex/test files | split | `agent`, `being`, `circus`, `eye`, `race` go to `phiecs`; `set` and the `lisp` host go to `phiclm::lisp` (it owns the machine and `lisp::set`); `trace` goes to `phient::relate` (it uses `relate::scopes`) |
| `tokenizer` | `tokenizer.rs`: one line, `pub use phiton::p64::words::Tokenizer` (5 lines) | `phiton` | src: 10 modules (`core` 7, `bin` 7, `command` 3, `persona` 3, `server` 2, `wiki`, `lab`, `frames`, `model`, `instruction`); 35 ex/test files | `phiton` (already there: delete the alias) | A second path to one type. Callers switch to `phiton::p64::words::Tokenizer` |
| `wiki` | `wiki.rs` + `wiki/extracts.rs`: Wikipedia bulk ingestion with `reqwest` (181 + 70 lines) | `phient::train::{chunker, trainer}`, `Facet`, `Tokenizer` | src: `server`, `bin` (`ingest`) | `phient` (`train`) | It is a training source over `ChunkStore` and `Trainer`. `reqwest` moves with it to the `phient` manifest. Alternative if that dependency is unwanted: stays as a driver (decide at move) |

Outside `src`, in `crates/phiano`: `tests/` holds only `fixtures/broken` (stays).
`data/` is the data directory (stays). `personas/` is persona data (stays;
see section 4). `analysis/`, `specs/`, `tasks/`, `refs/`, `scripts/`, `web/`,
`models/`, `_to_delete/` and `jobs/` hold no Rust; they are not modules and are
not moved by this cleanup. `jobs/` holds CV and job-spec files that are not
phiano's: flagged for the maintainer, not decided here.

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
| `bin/persona/*` | `persona ingest\|chat\|quest` | `config`, `data`, `lab`, `persona`, `phasor`, `relate`, `tokenizer` | stays in `phiano` | Persona verbs; reduced to `main` plus calls into `src/persona` (section 4) |
| `bin/prepare/prose.rs` | WikiText-2 into a prose corpus | `data`, `tokenizer` | stays in `phiano` | Writes `data/prose_corpus.txt` |
| `bin/quest/*` | `quest ingest\|ask`: a `phiecs` quest into a facet, with measures (11 files) | `config`, `data`, `phasor` | stays in `phiano` | Reads phiano `data/quest`; its holdout and board measures go to `phient::metrics` in the groups (task 9.3) |
| `bin/relate/main.rs` | ask a `.relate` model what a word calls for | `relate` only | `phient` (`relate`) | No phiano item used |
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
| `command/persona/mod.rs` | `PersonaCmd` (197 lines) | stays in `phiano` | Thin dispatch into `src/persona` (section 4) |

## 4. `examples` and `personas`

`personas/` (over 300 Markdown files: root, `code/`, `docs/`, `stage/`) is data
that `persona::world` and `bin/persona` read. It stays in `phiano`.

`examples/` has 40 folders and 31 single files. Rule: an example that names
`phiano::persona` or `phiano::data` stays in `phiano`. Every other example runs
an experiment on a sibling crate and moves to that crate's `examples/` once its
`phiano::` paths name the real home (task 9.2). Dominant sibling crate, counted
from `use` lines, in brackets.

| Entry | Names from `phiano` | Target | Reason |
|---|---|---|---|
| `persona`, `impersonate`, `austen` | `persona`, `config`, `lab`, `tokenizer` | stays in `phiano` | Demo the persona library |
| `learner` | `arena`, `data` | stays in `phiano` | Reads `phiano::data` |
| `student`, `circus.rs`, `race.rs`, `think.rs` | `sys` | `phiecs` | Follow `agent`, `being`, `circus`, `race` |
| `lisp.rs`, `sets.rs`, `writer.rs` | `sys`, `lab` | `phiclm` | Follow the `lisp` and `set` host |
| `arena.rs`, `band.rs`, `crowd.rs`, `intent.rs`, `person.rs`, `talk.rs`, `teacher.rs`, `template.rs`, `workshop` | `arena` | `phient` | Follow `arena::Ecs` into `phient::lineup` |
| `flow.rs`, `meaning.rs` | `compose`, `facet`, `storage`, `trainer` | `phient` | Read-only today; they move or are edited with the re-exports (task 9.2) |
| `anatomy`, `bands`, `builders`, `cast`, `code`, `conception`, `constructed`, `context`, `demo`, `dimensions`, `english`, `family`, `forest`, `frames`, `kind`, `ladder`, `latency`, `marked`, `mirror`, `net`, `operator`, `promoted`, `relational`, `relations`, `riley`, `roles`, `senses`, `squares`, `story`, `strand`, `sweep`, `topdown`, `types`, `waves` | only `config`, `lab`, `phasor`, `relate`, `prose`, `tokenizer`, `frames` or nothing | `phient` (most); `phical` for `kind`, `operator`, `waves`; `phiton` for `mirror` | No phiano-only item; they measure `phient` models. `ladder` also moves to `phisor` if its rungs only call `phisor::ladder` |
| `bcpl.rs`, `decode.rs`, `forgetting.rs`, `generate.rs`, `hexagram.rs`, `learn.rs`, `liskov.rs`, `novelty_fold.rs`, `order.rs`, `pairing.rs`, `phinet.rs`, `readout.rs`, `sentence.rs`, `sides.rs`, `bench.rs` | `config`, `lab`, `tokenizer` | `phient` | Single-file experiments of the same kind; `liskov.rs` to `phical` (`liskov`) |

## 5. The four persona folders (summary; detail is task 9.2)

| Folder | Today | Goes to |
|---|---|---|
| `src/persona` (14 files, 1148 lines) | the library | stays: the one home |
| `src/command/persona` (197 lines) | REPL verb | stays, thin dispatch into `src/persona` |
| `src/bin/persona` (9 files) | binary with its own chat, ingest and quest logic | stays, keeps `main` and calls the library; shared logic moves into `src/persona` |
| `examples/persona` (2 files) | demo that rebuilds the three personas by hand | stays, demo code only |
