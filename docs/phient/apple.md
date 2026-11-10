# Apple's machine-learning strategies, mapped onto phient

Read 2026-11-10 against the working tree. Nothing was run or trained; every
line is from reading source. Lines marked (checked) were re-opened by a
second reader. `docs/apple_vs_phient.md` is the view from the top of
Apple's stack; this is the view from the headers.

## Where the strategies are written down

All under `phi/crates/macos/src/core_ml/` (from `gemphi/`):

| Short | Path |
|---|---|
| **CML** | `CoreML.framework/Versions/A/Headers/` (75 headers) |
| **CMLS** | `CoreML.framework/Versions/A/Modules/CoreML.swiftmodule/arm64e-apple-macos.swiftinterface` |
| **MLC** | `MLCompute.framework/Versions/A/Headers/` |
| **CRML** | `CreateML.framework/.../arm64e-apple-macos.swiftinterface` |
| **CMPC** | `CreateMLComponents.framework/.../arm64e-apple-macos.swiftinterface` |
| **LRN** | `phinet/crates/phient/src/lineup/learner/` |
| **PN** | `phinet/crates/` |

The Rust beside those headers (`mod.rs`, `phifly.rs`, `venus.rs`) computes
nothing: `mod.rs:136-160` returns "ok" and the other two re-export phi-oml.
The value in that folder is the headers.

## What phient is today, in six lines

- An orchestrator over several models that all answer one protocol: guess
  before being told (`PN/phient/src/study/arena.rs:11-22`).
- The live model is the **Learner**: a next-word model of unit complex
  phasors, 64 or 128 channels, trained by a hand-written sampled-softmax
  gradient with per-row Adagrad (`LRN/step.rs:18-151`, `LRN/plane.rs:34-49`).
- One position at a time on one thread (`LRN/train.rs:77-82`). CPU only.
- Saved as one bincode file: planes, accumulators, every told line, the rng
  (`LRN/store.rs:24-39`, checked). The newest is 210 MB.
- The learner's arithmetic still sits in phient; its move to
  `phical::learner` is planned, not done (`PN/phient/TODO.md:52`).
- Their own record: 24.2% top-1 against the trigram's 24.8%
  (`PN/phiano/data/curriculum/instalments.tsv:24`).

## The mapping

| Apple strategy | What Apple specifies | phient / phical today | Verdict | Where it lands |
|---|---|---|---|---|
| **Training session** | `MLTrainingSessionParameters { sessionDirectory, reportInterval, checkpointInterval, iterations }` (CRML:2375-2380); `MLCheckpoint { url, phase, iteration, date, metrics }` (CRML:2200); `restoreTrainingSession` / `resume` (CRML:156-160) | `Learner::save` writes whole with a plain `fs::write` (`LRN/store.rs:60`, checked). Resume is an optional flag in an example (`PN/phiano/examples/learner/main.rs:157`, checked). Saved once, after every epoch has run (`main.rs:234`, checked) | **Adopt.** It is "learning must accumulate" made mechanical. Today a crash in epoch 7 of 8 loses the run | `LRN/session.rs` (new) |
| **Progress and control** | `MLUpdateTask` with `resumeWithParameters` (CML `MLUpdateTask.h:36-50`); events for training begin, epoch end, mini-batch end (CML `MLUpdateProgressEvent.h:19-23`); learning rate adjustable in flight, the rest at load (CML `MLParameterKey.h:19-44`); `cancel` (CML `MLTask.h:17-45`) | `Learner::epoch()` returns one mean loss and has no hook (`LRN/train.rs:68-84`). Patience, best epoch and printing live in the example (`main.rs:196-217`). No cancel anywhere | **Adopt** as closures: an epoch-end event that can stop, change the rate, forget, and checkpoint | `LRN/train.rs`, `session.rs` |
| **Optimizer descriptor** | clipping by value, norm or global norm; L1/L2; rescale (MLC `MLCOptimizerDescriptor.h:35-73`); RMSProp alpha 0.99 (MLC `MLCRMSPropOptimizer.h:27-43`); Adam with AMSGrad (MLC `MLCAdamOptimizer.h:25-50`) | Learner: hand Adagrad per row; the stride reset is run by hand (`LRN/forget.rs:16-26`). Tape: `Adam` with a schedule and saved state (`PN/phistd/src/tensor/adam.rs:7-68`). No clipping, no finite check | **Adopt.** `phinet/TODO.md:24` already says the stride was the constraint and it should forget by itself. That is RMSProp's decaying accumulator. The numbers will move, so it is a new lever in the record | `phical::learner` |
| **Estimator contract** | `Estimator.fitted / encode / decode` (CMPC:1730); `UpdatableEstimator.update / encodeWithOptimizer / decodeWithOptimizer` (CMPC:1743-1748) | A different save shape per model. `RelationModel::save` stores no optimizer (`PN/phient/src/operator/model.rs:156-161`) and its resume builds fresh Adams (`PN/phiano/examples/operator/main.rs:96-102`) | **Adopt.** One trait, two pairs: with and without the optimizer. Fixes a resume that drops Adam's moments today | trait in `PN/phient/src/study/` |
| **Compile once, then cache** | async `compileModel` (CML `MLModel+MLModelCompilation.h:48-58`); `MLModelAsset` gives the description without instantiating (CML `MLModelAsset.h:86-111`); `MLSpecializationStrategyFastPrediction` (CML `MLSpecializationStrategy.h:13-16`) | The learner has only the full training checkpoint. `phical::q64` already quantises a phase to 64 symbols (`PN/phical/src/q64.rs:20-28`) | **Adopt.** A read-only asset: no told lines, no accumulators, no rng; f32 or Q64; a header readable alone | `LRN/compile.rs` (new); quantising in `phical::q64` |
| **Training graph and inference graph** | one `MLCGraph`, compiled into a training graph with its optimizer or an inference graph (MLC `MLCTrainingGraph.h:52,95`, `MLCInferenceGraph.h:41,74`) | Written once (`PN/phifno/src/operator/graph.rs:14-45`) but inference runs on the training tape: `Fno::apply(&mut self)` pushes then truncates (`fno.rs:113-123`), so the generator holds it in a `RefCell` | **Adopt.** An off-tape `infer(&self)` over the same `graph::forward` | `PN/phifno/src/operator/` |
| **Stateful prediction** | state is a handle apart from the model, with named buffers; calls that share a state are serialised (CML `MLState.h:40-56`, `MLModel+MLState.h:21-36`) | Decode state sits inside the model objects (`PN/phient/src/generate/guide.rs:33-49`, `generator.rs:27`) | **Adopt** for the guide and generator: split into guide and `GuideState`. **Not** for the learner: a 4-word window has nothing to cache | `PN/phient/src/generate/guide.rs` |
| **Compute plan with cost** | `estimatedCost(of:)` and `deviceUsage(for:)` per operation (CML `MLComputePlan.h:87-99`); cost is a share in 0..1 (CML `MLComputePlanCost.h:24-25`) | phisor numbers every step and takes the card with the fewest (`PN/phisor/src/machine/engine.rs:31-39,100-119`). The learner prints seconds after an epoch only; the record has no time column | **Partly have.** Adopt the weight: measured cost per step in `arrange`, and an estimate before a run, read from the record | `PN/phisor/src/machine/engine.rs`; columns in `instalments.tsv` |
| **Batch provider** | count plus indexed access (CML `MLBatchProvider.h:18-24`) | A corpus is a whole `Vec<Vec<String>>` (`main.rs:76`); Tatoeba is pre-sliced into `sample80k_N` files | **Adopt** as an indexed, lazily read `Lines` with a content-hashed split (the method at `PN/phient/src/net/joint.rs:36`) | `PN/phient/src/study/` |
| **Preprocess once** | `preprocessed(from:)` then `fitted(toPreprocessed:)` (CMPC:3442-3454); `reuseExtractedFeatures` (CRML:2394) | Every resumed run tokenises its text again (`main.rs:76,97-99`) | **Adopt lightly:** cached token ids in the session folder | `session.rs` |
| **Output backings** | caller-allocated outputs (CML `MLPredictionOptions.h:89`); no-copy init (CML `MLMultiArray.h:19,148`) | The step reuses scratch (`LRN/scratch.rs`); `scores` allocates a vocabulary-sized Vec per position (`LRN/read.rs:14`) | **Mostly have.** Small: `scores_into` | `LRN/read.rs` |
| **Cancellation** | `MLModelErrorPredictionCancelled` (CML `MLModelError.h:111`) | None | **Adopt the flag only**, checked per epoch, that checkpoints and exits | `LRN/train.rs` |
| **Typed features, custom models, introspection, request and handler** | `MLFeatureDescription` constraints; `MLCustomModel`; `MLModelStructure`; Vision's handler | phical's typed operands and one checked `apply` (`PN/phical/src/keyboard/operator.rs:79-92,189-200`); `study::Reader`; the frozen keyboard index (`keyboard/index.rs:60-67`); the Arena | **Already have**, by a stricter design | - |
| **Compute units** | CPUOnly / CPUAndGPU / All / CPUAndNeuralEngine (CML `MLModelConfiguration.h:17-22`); `MLCDevice` (MLC `MLCDevice.h:34-59`) | CPU only | **Not yet.** A policy with one backend selects nothing. Take it when phical gains a second one - as wgpu kernels with CPU twins, the way phiray does | later, `phical` |

## Order of work

1. **Session folder.** Resume-newest and save become the default, not a
   flag. Checkpoint every N epochs, written to a temp name then renamed.
   Lineage in the header. Old files move to `_temp` with an inventory,
   never deleted. ~200-250 lines.
2. **Epoch event and control.** Patience, best epoch, rate and `forget`
   move out of the phiano example into phient; a stop flag checkpoints and
   exits. ~120 lines.
3. **Optimizer descriptor in phical.** Decaying accumulator, clipping, a
   finite check. Replaces the hand-run `--forget`. ~150 lines plus a
   recorded re-measure.
4. **One persistence trait, with and without the optimizer.** ~150 lines.
5. **Compiled learner asset.** ~150-200 lines. Expected far smaller than
   210 MB; not measured.
6. **Off-tape `Fno::infer(&self)` and `GuideState`.** ~280 lines. Removes
   the `RefCell` and allows parallel decoding.
7. **Cost plan.** Positions and seconds in `instalments.tsv`, an estimate
   printed before a run. ~120 lines. Serves "don't saturate the machine".
8. **Indexed `Lines` with a hashed split.** ~150 lines.
9. **Cached token ids.** ~100 lines.
10. **Persona delta over a frozen base** (CML `MLParameterKey.h:60-80`).
    ~200 lines, only if personas get their own learners.

## Do not take

- **An open operator vocabulary or compiled custom layers.** The mill is
  fixed at 64 and a new operation is a card in `data/ops`.
- **The layer zoo** (attention, LSTM, convolution). Relations replace it.
- **MLCompute as an API.** Apple deprecated it in macOS 14
  (MLC `MLCTrainingGraph.h:27`). Its designs are worth reading; its
  surface is not.
- **Lazy `MLTensor` of any rank and type.** A second engine beside one that
  is rank 3 on purpose.
- **An async runtime in phical.** Closures and a stop flag are enough.
- **Model collections** (sunset by Apple, CML `MLModelCollection.h:19`),
  **data frames, random splits.** The deterministic split is the better one.

## Where the transcription disagrees with the headers

If the headers are the spec, these lines in `core_ml/mod.rs` are wrong:
`MLComputeUnits::NeuralEngineOnly` (`mod.rs:28`) does not exist - the case
is CPUAndNeuralEngine; `batch_size` (`mod.rs:55`) and
`prefer_background_execution` (`mod.rs:38`) are not in the headers; low
precision accumulation defaults to true (`mod.rs:45`) where the header says
NO (CML `MLModelConfiguration.h:56`).

## Not verified

- Scores, checkpoint sizes and "an hour an epoch" are quoted from
  `TODO.md` and `instalments.tsv`, not reproduced.
- The compiled-asset saving and "one thread per position sets the epoch
  time" are estimates.
- The CreateML and CreateMLComponents interfaces were read by search hits,
  not whole. No Vision headers are in that folder.
- Read in part only: phient's `generate`, `metrics`, `prose`, `relate`,
  `memory`, `scribe`, `code`, most of `train`; phisor's ladder and truth.
