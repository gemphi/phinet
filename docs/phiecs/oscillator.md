# phasor · oscillator

`phiecs::phasor::oscillator` — it was phiano's `oscillator`. The sphere model itself (`Oscillator`) is `phiton::physics::oscillator`, beside Bentov's `standing`; the evaluation against the Evaluator is `phient::eval::oscillator`. The diagram shows the module as it was drawn whole.

```mermaid
graph LR
    Facet["facet::Facet<br/>lexicon: phase, amplitude, band_n"]
    Config["config::ALPHA, OSCILLATOR_*<br/>LEARNING_RATE"]
    Phiton["phiton::SpectralBand::BANDS"]
    Tok["tokenizer::Tokenizer"]
    Evaluator["eval::Evaluator<br/>coherence, novelty, resonance"]
    Users["command::oscillator::OscillatorCmd<br/>server::routes::oscillator"]

    subgraph oscillator["phiecs::phasor::oscillator"]
        Osc["mod.rs: Oscillator<br/>longitude, latitude, frequency, amplitude"]
        FromP["Oscillator::from_phasor"]
        OscOps["visibility, color, hue<br/>synchronization"]
        subgraph fieldrs["field.rs"]
            Field["OscillatorField::from_facet<br/>oscillators, index"]
            FieldOps["sentence_coherence, sentence_sync<br/>project, spectral_entropy<br/>dominant_colors, words_at_latitude"]
        end
        subgraph evalrs["eval.rs"]
            OEval["OscillatorEval::evaluate"]
            Cmp["ComparisonResult::compare<br/>transform vs oscillator"]
        end
        subgraph trainrs["train.rs"]
            OTrain["OscillatorTrainer::train<br/>warmup, weight decay, convergence"]
            TRes["OscillatorTrainResult"]
        end
        View["view.rs: SphereView<br/>render_wheel, render_sphere"]
    end

    Facet -->|"lexicon"| Field
    Field --> FromP
    Config --> FromP
    FromP --> Osc
    Osc --> OscOps
    Phiton -->|"band names"| OscOps
    OscOps --> FieldOps
    Field --> FieldOps
    Tok -->|"tokens"| OEval
    FieldOps --> OEval
    FieldOps --> Cmp
    Evaluator --> Cmp
    Tok -->|"tokens"| OTrain
    FieldOps -->|"coherence per epoch"| OTrain
    OTrain -->|"phase shift, amplitude decay"| Facet
    OTrain --> TRes
    FieldOps --> View
    Phiton --> View
    OEval --> Users
    Cmp --> Users
    TRes --> Users
    View --> Users
```
