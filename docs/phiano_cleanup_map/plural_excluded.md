# Plural names kept, and the scan rules

Requirement 10.5. The scan covered `crates/phiano/src` and `crates/phisor/src`
on branch `english-reader-coverage`: every `mod`, `struct`, `enum`, `trait`,
`type` and `fn` line outside test files and outside `#[cfg(test)]` bodies. A
name counts as plural when any `_` or CamelCase word of it ends in `s` but not
`ss`, `us` or `is`, and is not one of a short list of singular words
(`class`, `status`, `analysis`, `synthesis`, `corpus`, ...). The scan is by
text, not by the compiler; the structural test of task 15 uses the same rule
with the allowlist below. Sibling crates (`phient`, `phiecs`, `phical`,
`phiton`, `phistd`) are outside the task; rows move with their code and are
checked where they land.

Counts of names to rename (one row per crate and name; see the four lists):

| Kind | phiano | phisor | Total | List |
|---|---|---|---|---|
| module | 19 | 12 | 31 | `plural_module.md` |
| type | 8 | 22 | 30 | `plural_type.md` |
| function | 74 | 83 | 157 | `plural_function_phiano.md`, `plural_function_phisor.md` |
| all | 101 | 117 | 218 | |

## Kept: the allowlist (name, kind, why)

| Name | Kind | Why it stays |
|---|---|---|
| `sys` | module | Short for "system", not a plural. Moves to `phiecs` anyway (section 1) |
| `ecs` / `Ecs` | module, type | Acronym (entity component system) |
| `circus` | module | A singular noun that ends in `s` |
| `kudos` | function | A singular noun that ends in `s` |
| `semantics` / `SemanticsAgent` | module, type | Mass noun (the field of meaning), like "physics"; it names the Searle agent for meaning |
| `tests` | module | The Rust and Cargo name for a test module or folder; `cargo test` and `#[cfg(test)]` expect it |
| `tells`, `conserves`, `holds`, `fills`, `survives`, `reaches`, `starts`, `contains` | function | Third-person verbs naming a yes/no check. Not a plural noun |
| `contains_any`, `starts_with_any`, `looks_like_name`, `calls_for`, `meets_bar`, `weaves_no_thread` | function | A verb phrase with a verb ending in `s` |
| `speaker_vs_literal_meaning` | function | `vs` is an abbreviation |
| `civil_from_days` | function | Howard Hinnant's published name for the days-to-civil algorithm; the doc comment cites it |
| `property_NN_...` | function | Sentence-long names of property tests; each states a rule, not a thing |
| `the_two_shapes_...` and other test-sentence names | function | Names of `#[test]` functions, read as sentences |

External names are left alone because a rename would change the outside:

- HTTP paths and JSON field names in `server` (`/stats`, `/layers`); only the Rust
  handler and type names change (`stats`, `StatsResponse`, `LayersResponse`).
- REPL verbs typed by the user (`stats`, `layers`); only `LayersCmd` and the
  module names change. The verb strings are read from the `Command` impl.
- `[[bin]]` names and `phiano/data/*` file names.
- Names of items from other crates and the standard library that these crates
  call (`Iterator::take`, `HashMap::values`, `phient` items such as
  `train_extracts`'s callee).
- `Cargo` example names (see below).

## Plural names outside `src`

`examples/` holds 10 plural folders (`bands`, `builders`, `dimensions`,
`frames`, `relations`, `roles`, `senses`, `squares`, `types`, `waves`) and 3
plural files (`circus.rs`, `sets.rs`, `sides.rs`) in `crates/phiano`, plus
`kinds` in `crates/phisor/examples`. They are Cargo example names; most move to
a sibling crate (section 4). They take their singular name when they move, in
the move commit, and `circus` stays as it is (allowlist). They are not part of
the 218.

## Notes for the rename commits (task 15)

- One kind per commit: modules (`smart_relocate`), then types, then `phisor`
  functions, then `phiano` functions.
- A function and a method can share one plural name (for example `counts` is a
  free function and a method in `ladder/measure/counts.rs`); both change in the
  same commit.
- `personality_traits` has a forwarder in `likelihood.rs` and the real body in
  `traits.rs`; both take `personality_trait`. See group 12.
- `Counts` in three places holds three unlike things (libtest tallies, overlap
  tallies, transition tables); each gets its own singular name instead of one
  shared name.
