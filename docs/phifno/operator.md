# operator

## Files

```mermaid
graph TD
    mod["operator/mod.rs<br/>FnoConfig · Tie · Example · MixtureExample · Layer<br/>pub use phistd cosine · stack · unstack"]
    fno["fno.rs<br/>Fno · new · params · attach_param · reset<br/>parameter_count · forward · apply"]
    resolution["resolution.rs<br/>at_resolution · circle"]
    fwdrs["graph.rs<br/>forward(tape, config, params, x)"]
    modes["modes.rs<br/>mask_modes · scale_modes"]
    modulers["module.rs<br/>trait Module · Sequential"]
    sampler["sampler.rs<br/>FrequencySampler · zeta_weights · rank"]
    train["train.rs<br/>train_epoch · train_epoch_with<br/>train_epoch_mixture · train_epoch_softmax"]
    save["save.rs<br/>APPLY_CHUNK = 256 · MAGIC PHINET<br/>Saved · SavedV1 · SavedCompact<br/>to_bytes · to_bytes_with · to_bytes_compact<br/>from_bytes · from_bytes_with"]
    compat["save/compat.rs<br/>FnoConfigNoTie · saved_v2 · saved_compact · saved_v1"]
    symmetry["symmetry.rs<br/>defects(inputs, phi) · 1 test function"]
    tests["tests.rs<br/>tests/fno.rs · 6 test functions<br/>tests/save.rs · 3<br/>tests/sphere.rs · 4"]
    mod --> fno & fwdrs & modes & modulers & sampler & train & save & symmetry & tests
    fno -->|"forward"| fwdrs
    fno -->|"APPLY_CHUNK"| save
    resolution -->|"at_resolution"| modes
    resolution --> fno
    modulers -->|"second operator"| fwdrs
    train -->|"mask_modes"| modes
    save --> compat
    symmetry -->|"apply"| fno
    sampler -->|"Example"| mod
```

## The forward graph — h ← σ(F⁻¹(R · F(h)) + W h + b)

```mermaid
flowchart TD
    X["x · b × 1 × space on the tape"] --> LIFT["lift · linear p0 · bias p1 unless Phase"]
    LIFT --> A0["σ · gelu, or squash under Phase"]
    A0 --> LAYER["layer l · R, W, b = p 2+3l, 3+3l, 4+3l"]
    LAYER --> PW["W h · bias b unless Phase"]
    LAYER --> MD{"modes #gt; 0"}
    MD -->|"no · pointwise control"| SUM["add"]
    MD -->|"yes"| GR{"tie.groups(space)"}
    GR -->|"None · Circle, Phase"| FFT["fft · mode_mix with R · ifft"]
    GR -->|"Some · Sphere, Anisotropic"| TIE["tie R across groups · mode_mix · no transform"]
    FFT & TIE & PW --> SUM
    SUM --> LAST{"last layer"}
    LAST -->|"no"| ACT["σ"] --> LAYER
    LAST -->|"yes"| PROJ["projection · linear p 2+3·layers"] --> Y["y"]
```

## Tie — what one spectral weight is shared over

```mermaid
flowchart LR
    CFG["FnoConfig constructors<br/>for_space · Circle · modes = space/5, at least 1<br/>sphere · Sphere · space HARMONICS · modes DEGREES<br/>anisotropic · Anisotropic · circle64 · Circle, modes 12"] -.-> T
    T{"Tie"} -->|"Circle · default"| C["FFT · one weight per signed frequency<br/>mask keeps m #lt; modes or m + modes ≥ space<br/>space a power of two"]
    T -->|"Phase"| P["the Circle path · no biases · squash for σ<br/>commutes with a global phase"]
    T -->|"Sphere"| S["no FFT · group = floor of √k · one weight per degree<br/>space a perfect square · modes = degrees kept"]
    T -->|"Anisotropic"| A["group = l(l+1)/2 + abs m · one weight per l and abs m<br/>36 groups at space 64"]
```

## A hinge epoch — train_epoch_with

```mermaid
sequenceDiagram
    participant C as caller
    participant F as Fno
    participant T as Tape
    participant A as Adam
    C->>F: train_epoch(table, examples, pool, batch, adam, seed)
    F->>F: order examples by mix64(i xor seed)
    loop each chunk of batch
        F->>T: truncate(keep)
        F->>T: constant x, target t, decoy d from sampler(target, r)
        F->>T: forward x gives y
        F->>T: cosine(y, t) · cosine(y, d) · hinge(margin)
        F->>T: backward(loss)
        F->>A: step(tape, params)
        F->>F: mask_modes
    end
    F->>C: mean loss
```

## Three epochs, three losses

```mermaid
flowchart LR
    EX["Example · input, target"] --> H & SM
    MEX["MixtureExample · input, target, k, coef, other"] --> MX
    H["train_epoch · train_epoch_with<br/>cosine to target and to decoy · hinge(margin)"]
    MX["train_epoch_mixture<br/>cosine_table over the table · mixture_nll<br/>MixtureTerms: targets, k, coef, gamma, other, beta"]
    SM["train_epoch_softmax<br/>cosine_table · cross_entropy(targets, beta)"]
    D1["phistd::decoy · uniform over pool"] -.-> H
    D2["FrequencySampler::draw · weight^alpha · 16 tries"] -.-> H
    H & MX & SM --> ST["backward · Adam step · mask_modes · mean loss"]
```

## After a step — the mask and the zeta prior

```mermaid
flowchart LR
    R["R of each layer"] --> MM["mask_modes · zero every slot where mask is false"]
    ZW["zeta_weights(modes, s)<br/>1 at m = 0 · m^-s / ζ(s) above<br/>ζ(s) over 200 000 terms when s #gt; 1, else 1"] --> SC["scale_modes(weights)<br/>slot m times weights at f<br/>f = m, or space − m above space/2"]
    R --> SC
```

## at_resolution(space)

```mermaid
flowchart LR
    A["Fno at old space"] --> CFG["space = new<br/>circle: modes ≤ space/2 · sphere: modes ≤ √space degrees"] --> N["Fno::new(cfg, 0)<br/>checks a power of two, or a perfect square"]
    N --> P{"each param · lift, lift bias, then R W b per layer, then projection"}
    P -->|"R on the circle"| R["new mode m takes signed frequency f<br/>copied when abs f ≤ old space/2"]
    P -->|"R on the sphere"| RS["groups numbered degree first<br/>the shared prefix copied · new groups zero"]
    P -->|"lift · W · b · projection"| C["copied as is"]
    R & RS & C --> M["mask_modes"]
```

## Saving and loading

```mermaid
flowchart TD
    TB["to_bytes · to_bytes_with(adam)"] --> V2["PHINET · u16 2 · bincode Saved<br/>config · params as f64 pairs · AdamState or none"]
    TC["to_bytes_compact"] --> V3["PHINET · u16 3 · bincode SavedCompact<br/>params as f32 pairs · no optimiser"]
    FB["from_bytes_with(bytes)"] --> MG{"starts with PHINET"}
    MG -->|"shorter than header"| NONE["None"]
    MG -->|"version 2"| S2["saved_v2 · Saved, else SavedNoTie"]
    MG -->|"version 3"| S3["saved_compact · SavedCompact, else SavedCompactNoTie<br/>f32 pairs to f64"]
    MG -->|"other version"| NONE
    MG -->|"no header · format 1"| S1["saved_v1 · SavedV1, else SavedV1NoTie"]
    NT["FnoConfigNoTie into FnoConfig · tie = Circle"] -.-> S2 & S3 & S1
    S2 & S3 & S1 --> NEW["Fno::new(config, 0) · param count must match · set_value each"]
    NEW --> OUT["Some · Fno and Option of Adam"]
```

## Symmetry defects and ranking

```mermaid
flowchart LR
    X["defects(inputs, phi) · each x"] --> AP["apply to x · e^iφ x · conj x"]
    AP --> M["mean of norm(y1 − e^iφ y0) / norm(y0)<br/>and norm(y2 − conj y0) / norm(y0)<br/>(phase, conjugation)"]
    P["rank(table, preds, examples, pool)"] --> AB["above · pool items other than the target<br/>whose cosine to the pred beats the target's"]
    AB --> OUT["mean over examples<br/>mrr = 1 / (1 + above) · hit at 10 when above #lt; 10"]
```

## The types

```mermaid
classDiagram
    class FnoConfig {
        +usize space
        +usize width
        +usize layers
        +usize modes
        +f64 margin
        +f64 lr
        +Tie tie
    }
    class Tie {
        Circle
        Phase
        Sphere
        Anisotropic
        +groups(space) Option of Vec of usize
        +degrees(space) usize
    }
    class Fno {
        +FnoConfig config
        +Tape tape
        lift Var
        lift_b Var
        layers Vec of Layer
        proj Var
        keep usize
        mask Vec of bool
    }
    class Layer {
        r Var
        w Var
        b Var
    }
    class Module {
        tape() Tape
        parameters() Vec of Var
        forward(x) Var
        parameter_count() usize
    }
    class Sequential {
        +Fno first
        second_params Vec of Var
        second Fno
    }
    FnoConfig *-- Tie
    Fno *-- FnoConfig
    Fno *-- Layer
    Fno ..|> Module
    Sequential ..|> Module
    Sequential *-- Fno
```
