# Plural type names

Requirement 10.5, kind: type (`struct`, `enum`, `type`). `Triples`, `Lengths`, `Runs`, `Classes`, `GainCases`, `LearningCases` and `CombinationCases` are generators of the in-crate property tests. A name that would clash with a singular name already in the same scope takes the suffix `_list` (or `_count` for a number) instead; the rename commit checks the scope.

| Crate | Plural name | Where (under `src`) | Singular |
|---|---|---|---|
| phisor | `GrownCounts` | `ladder/measure/counts.rs` | `GrownCount` |
| phisor | `Triples` | `ladder/property/measure.rs` | `Triple` |
| phisor | `GainCases` | `measure/property.rs` | `GainCase` |
| phisor | `Forms` | `lexicon/irregular.rs` | `Form` |
| phisor | `Counts` | `abstraction/predict.rs` | `TransitionCount` |
| phisor | `Counts` | `guide/holdout/breach.rs` | `OverlapCount` |
| phisor | `Scores` | `abstraction/predict.rs` | `Score` |
| phiano | `Counts` | `bin/harness/spawn.rs` | `TestCount` |
| phiano | `Styles` | `persona/world.rs` | `StyleSet` |
| phisor | `IrregularForms` | `lexicon.rs` | `IrregularForm` |
| phisor | `Lengths` | `ladder/property.rs` | `Length` |
| phisor | `Runs` | `ladder/property/kinds.rs` | `Run` |
| phisor | `Numbers` | `measure/seeds.rs` | `Number` |
| phiano | `Labels` | `bin/quest/ingest/board.rs` | `Label` |
| phisor | `Words` | `lexicon/lists/words.rs` | `WordList` |
| phisor | `LearningCases` | `measure/property.rs` | `LearningCase` |
| phisor | `Seeds` | `ladder/mod.rs` | `Seed` |
| phisor | `Kinds` | `kind/mod.rs` | `Kind` |
| phisor | `Classes` | `ladder/property/kinds.rs` | `Class` |
| phisor | `Contractions` | `lexicon/contraction.rs` | `Contraction` |
| phisor | `Irregulars` | `lexicon.rs` | `Irregular` |
| phisor | `Units` | `units.rs` | `Unit` |
| phiano | `LayersResponse` | `server/types/requests.rs` | `LayerResponse` |
| phisor | `KindRows` | `machine/host.rs` | `KindRow` |
| phiano | `Bars` | `bin/harness/bars.rs` | `BarSet` |
| phisor | `Tables` | `kind/read.rs` | `Table` |
| phiano | `Caps` | `bin/harness/harness.rs` | `Cap` |
| phiano | `StatsResponse` | `server/types/requests.rs` | `StatResponse` |
| phisor | `CombinationCases` | `measure/twins/property.rs` | `CombinationCase` |
| phiano | `LayersCmd` | `command/layers.rs` | `LayerCmd` |

