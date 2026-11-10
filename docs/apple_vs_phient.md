# Apple's machine-learning stack, and what phient can take from it

Sources are the Apple pages listed at the end, read on 9 November 2026. Where a statement is from general knowledge of Apple's frameworks and not from those pages, it is marked (background). Nothing here was measured on phinet; the right-hand column states a design intent, not a result.

## 1. The stack, top to bottom

| Layer | What Apple ships | What it is for |
|---|---|---|
| Apps and system features | Apple Intelligence | features built on the models below |
| Model API | Foundation Models framework: a Swift API to the on-device model, to Private Cloud Compute models, and to any provider that conforms to the Language Model protocol | one call surface over many models |
| Models | Apple Foundation Models, third generation: a family of five models, from on-device to server models on Private Cloud Compute | the language models |
| Training and research array framework | MLX: open-source array framework for Apple silicon, built on unified memory; used for inference, fine-tuning and scientific computing, with distributed inference over several Macs | train and run on the Mac |
| Deployment runtime | Core ML (background): loads a converted model and schedules it across CPU, GPU and Neural Engine | ship a model inside an app |
| Hardware | Apple silicon with unified memory; the M5 GPU carries neural accelerators | the same memory for CPU, GPU and accelerator |

## 2. What is distinctive about it

- One memory. Unified memory means an array is not copied between CPU and GPU. MLX is designed around that.
- One call surface. The Foundation Models framework hides which model answers (on-device, Private Cloud Compute, or a third party) behind one protocol.
- Local first, cloud as overflow. The on-device model answers by default; the server model is the larger fallback with a privacy design around it.
- The vendor owns the whole path: silicon, runtime, framework, model, app API.

## 3. Compared with phient

| | Apple | phinet today |
|---|---|---|
| Call surface | one Language Model protocol over many models | the arena in phient: every model in one table, one study protocol |
| Local first, overflow second | on-device model, then Private Cloud Compute | counts and phasors first; Ollama/Phi-4 consulted only when nothing inside answers, every answer checked before it is told (phinet-64-operators-learning-ladder, requirement 2) |
| Array layer | MLX | phistd tensors, tape and FFT (smaller; no GPU path measured here) |
| Deployment runtime | Core ML schedules across CPU, GPU, Neural Engine | none; phinet is headless Rust on the CPU |
| Training | gradient fine-tuning in MLX | counts and phasor updates by error; no encoder trained (docs/ecs_vs_jepa.md) |
| Hardware path | owned silicon | not addressed |

Honest reading: phient already has Apple's call-surface idea and its local-first idea. It has no equivalent of the unified-memory array layer or of the runtime that places work on accelerators. Those two are the places to look if phient is to be "more like how Apple did it".

## 4. What to take

1. A protocol, not a class. Make "a model" in the arena a trait with one call, so a count model, a phasor model, the compiler reader and an Ollama model are interchangeable behind it. The arena already holds them in one table; the trait is the missing seam.
2. A placement rule. Decide per call where it runs (counts, phasors, reader, teacher) from cost and confidence, as Apple decides device versus server, and record the choice.
3. A single memory story for arrays in phistd before any GPU work. MLX's value comes from not copying; measure copies first.

## 5. The ECS as the main layer

The maintainer's position is that `phiecs` is the main layer beside `phiclm`: a trait-based system where entities carry components of many kinds and systems operate on them. That matches the Apple comparison: the ECS is the unified memory (one world, many readers), and the traits are the Language Model protocol (many models, one call). The compiler (`phiclm`) is the second main layer: it states and checks what the world claims. Both statements are design intent. The measured side is in docs/ecs_vs_jepa.md.

## 6. How Apple wraps a model: Core ML, Vision and metadata

Read from `phi/apple-source/ruskit/Interfaces/iPhoneOS/Vision/Headers/VNCoreMLRequest.h` and `phi/crates/coreml/src`.

Apple splits a model call into four small parts, and each has a Rust counterpart in `phi/crates/coreml`:

| Part | Apple (Vision header) | Rust bridge in `phi/crates/coreml/src/core_ml` |
|---|---|---|
| The model | `MLModel`, wrapped by `VNCoreMLModel`, which refuses a model that takes no image as input | `MLModel`, `MLModelDescription` |
| What the model says it needs | feature descriptions and constraints (image size, array shape, dictionary key type) | `MLFeatureDescription`, `MLImageConstraint`, `MLMultiArrayConstraint`, `MLDictionaryConstraint` |
| How inputs arrive | `inputImageFeatureName` names the input Vision fills; an `MLFeatureProvider` supplies every other input | `MLFeatureProvider`, `MLDictionaryFeatureProvider`, `MLBatchProvider` |
| What comes back | the observation type follows the model kind: classification, pixel buffer, recognised object, or a generic feature value; confidences are passed on as they are, not forced into 0..1 | `MLPrediction { outputs, prediction_time_ms }`, `MLFeatureValue` |
| Where it runs | the compute-unit choice | `MLComputeUnits`, `MLModelConfiguration`, `MLOptimizationHints` |

The idea to take: the model carries a description of itself (its metadata), and the caller reads that description before it calls. Vision checks the description when it wraps the model and fails early with an error if the model cannot be used.

## 7. phiora as the metadata layer

`phiadk/phiora` already exists as a Python service ("the one Postgres database and API behind every phi* app", FastAPI, Alembic migrations, tables for kinds, items, links, meta, log, accounts, tokens and ledger). The maintainer's reading is that phinet's `phiora` holds the metadata coming from that Python version, used like retrieval (RAG-style) so that phient can look up what a model, word, kind or asset is before it acts, the way Vision reads a Core ML model's description first.

This is a proposal. Not yet decided: whether phinet's `phiora` is a Rust client of the Python service, a Rust port of its tables, or a read-only export; and how its `tokens` and `ledger` tables relate to `phipad` and `phiput`. I read only the README and `main.py`, not the models or the migrations.
## 8. One crate, one thing, linked by a thin request

The maintainer's rule: one crate does one thing, and crates are linked the way Apple links Vision to Core ML. In Apple's headers, Vision does not contain a model runtime. `VNCoreMLRequest` is a small request type in Vision that holds a `VNCoreMLModel`, which in turn wraps a Core ML `MLModel`. The link is one type, in the consumer, that holds a handle from the provider.

What the workspace has today (read from the files, not run):

- `phi/crates/iosar/src/vision/coreml.rs` has a `VNCoreMLRequest` that holds only a base `VNRequest`. It has no model field and no reference to Core ML. It is a stub.
- `phi/crates/macos/src/core_ml` holds the framework headers, `phifly.rs`, `venus.rs` and `Resonance.metal`.
- `phi/crates/coreml/src/core_ml` is the Rust bridge with `MLModel`, `MLModelDescription` and the feature types.
- Neither `iosar/Cargo.toml` nor `macos/Cargo.toml` mentions the coreml crate, so the Vision request is not yet linked to the bridge.

For phinet, the same shape would be: `phiora` is the provider (a Rust client of the Python service, giving descriptions of models, words, kinds and assets). A consumer such as `phisor` would hold a small request type with a `phiora` handle and ask it for a description before acting. Any answer is a proposal that the consumer checks, in the way the Reader already checks learned proposals. This is a proposal for the maintainer to confirm; nothing here is implemented.
## Sources

- Apple Machine Learning Research, third generation of Apple Foundation Models: https://machinelearning.apple.com/research/introducing-third-generation-of-apple-foundation-models
- Apple Machine Learning Research, MLX distributed inference: https://machinelearning.apple.com/research/mlx-distributed-inference
- Apple Machine Learning Research, MLX and the M5 neural accelerators: https://machinelearning.apple.com/research/exploring-llms-mlx-m5
- Apple Developer, AI and Machine Learning: https://developer.apple.com/machine-learning/
- Apple Developer, WWDC26 Machine Learning guide: https://developer.apple.com/wwdc26/guides/machine-learning/

Content was rephrased for compliance with licensing restrictions.


