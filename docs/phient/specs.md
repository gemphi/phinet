# phient — specs for the training session

What each part must do. One row is one requirement, with the Apple header
it answers. "Today" says whether phient already meets it. The reading of
each header, and what is deliberately not taken, is in
[apple.md](apple.md); the folders are in [plan.md](plan.md).

## Sources

All under `phi/crates/macos/src/core_ml/` (from `gemphi/`).

| Short | What | Where |
|---|---|---|
| **CML** | Core ML headers | `CoreML.framework/Versions/A/Headers/` |
| **MLC** | ML Compute headers | `MLCompute.framework/Versions/A/Headers/` |
| **CRML** | Create ML interface | `CreateML.framework/.../arm64e-apple-macos.swiftinterface` |
| **CMPC** | Create ML Components interface | `CreateMLComponents.framework/.../arm64e-apple-macos.swiftinterface` |
| **LRN** | the Learner | `phinet/crates/phient/src/lineup/learner/` |
| index | where each framework is on disk | `phinet/crates/phient/reference/README.md` |
| index | the view from the top of Apple's stack | [../apple_vs_phient.md](../apple_vs_phient.md) |

Standing rules these rows keep: learning accumulates (resume the newest,
always save); nothing is deleted (old files move to `_temp` with an
inventory); no data in code; phient orchestrates, phical computes; the
mill stays at 64 and a new operation is a card.

## Session

| Part | Must | Apple | Today |
|---|---|---|---|
| open | resume the newest checkpoint in the folder without being asked, or start fresh when there is none | `restoreTrainingSession`, `resume` CRML:156-160 | no: resume is a flag in an example |
| parameters | hold the folder, the report interval, the checkpoint interval and the iteration count | `MLTrainingSessionParameters` CRML:2375-2380 | no |
| checkpoint | be written every N epochs, under a temporary name and then renamed, so a reader never sees half a file | `MLCheckpoint` CRML:2200 | no: one plain write after the last epoch (`LRN/store.rs:60`) |
| header | say where it came from, the epoch, the date and the metrics, readable without loading the planes | `MLCheckpoint { url, phase, iteration, date, metrics }` CRML:2200 | no |
| bit-exact | restore planes, accumulators, told lines and rng so a resumed run equals an uninterrupted one | - | met (`LRN/store.rs:24-39`) |
| tokens | tokenise a corpus once and reuse the ids | `preprocessed(from:)` CMPC:3442-3454; `reuseExtractedFeatures` CRML:2394 | no: every run tokenises again |

## Progress and control

| Part | Must | Apple | Today |
|---|---|---|---|
| event | call the caller at each epoch's end with loss, best epoch and time | `MLUpdateProgressEvent` CML `MLUpdateProgressEvent.h:19-23` | no: `epoch()` returns a mean loss |
| control | let the caller carry on, change the rate, or stop | `resumeWithParameters` CML `MLUpdateTask.h:36-50`; `MLParameterKey.h:19-44` | no |
| stop | checkpoint, then exit, when a flag is set; checked once an epoch | `cancel` CML `MLTask.h:17-45`; `MLModelError.h:111` | no |

## Optimizer

| Part | Must | Apple | Today |
|---|---|---|---|
| descriptor | describe the optimizer as data: rate, clipping, decay | MLC `MLCOptimizerDescriptor.h:35-73` | no: hand Adagrad a row |
| decay | let the accumulator forget by itself | MLC `MLCRMSPropOptimizer.h:27-43` | no: `forget` is run by hand |
| clipping, finite check | bound a step and refuse a non-finite one | MLC `MLCOptimizerDescriptor.h:35-73` | no |
| where | live in `phical::learner`; every change is a new lever, recorded with a re-measure | - | arithmetic still in phient |

## Persistence

| Part | Must | Apple | Today |
|---|---|---|---|
| one trait | save and load every model the same way | `Estimator` CMPC:1730 | a different shape a model |
| with the optimizer | save the optimizer's state with the model when training will continue | `UpdatableEstimator` CMPC:1743-1748 | `RelationModel::save` drops Adam's moments |
| compiled asset | hold the planes alone, in f32 or 64-symbol phases, with a header readable alone | CML `MLModelAsset.h:86-111`; `MLModel+MLModelCompilation.h:48-58` | no: only the full checkpoint |

## Inference

| Part | Must | Apple | Today |
|---|---|---|---|
| off the tape | run a forward pass through a shared borrow, over the graph training uses | MLC `MLCTrainingGraph.h:52,95`, `MLCInferenceGraph.h:41,74` | no: `Fno::apply` needs the model mutably |
| state apart | keep decode state in its own value, so one model serves many decodes | CML `MLState.h:40-56`, `MLModel+MLState.h:21-36` | no: state sits in the guide |
| outputs | write scores into a buffer the caller owns | CML `MLPredictionOptions.h:89` | `scores` allocates a position |

## Cost and data

| Part | Must | Apple | Today |
|---|---|---|---|
| estimate | print positions and expected time before a run, from the record of earlier runs | CML `MLComputePlan.h:87-99`, `MLComputePlanCost.h:24-25` | no: seconds are printed after an epoch |
| record | keep positions and seconds for every run | - | no time column |
| lines | give a count and an indexed read over a corpus, with a split fixed by content hash | CML `MLBatchProvider.h:18-24` | a corpus is one `Vec` in memory |

## Not taken

An open operator vocabulary, the layer zoo, ML Compute's API (deprecated
by Apple), lazy tensors of any rank, an async runtime in phical, model
collections, and compute units while there is one backend.
