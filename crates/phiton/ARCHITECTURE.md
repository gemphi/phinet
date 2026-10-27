# phiton — architecture

## The crate, module by module

```mermaid
graph TD
    subgraph phiton_["phiton"]
        lib["lib.rs<br/>struct Phiton · C · H · VISIBLE_MIN_NM · VISIBLE_MAX_NM<br/>phase_to_wavelength · wavelength_to_phase<br/>frequency_thz · energy_ev · sector_color"]
        units["units.rs<br/>ALPHA = 1/137 · PHI · PHI_CONJUGATE · TWO_PI · BAND_N_EFFECTIVE_MAX<br/>PHASE_CHANNELS = 64 · CHANNEL_QUANTA · quantum · effective_phase"]
        bands["bands.rs<br/>SpectralBand · BANDS 16"]
        types["types.rs<br/>PhitonColor · LightQuantum · wavelength_to_rgb"]
        chiton["chiton.rs<br/>Chiton"]
        spectrum["spectrum.rs<br/>PhitonSpectrum"]
        hrr["hrr.rs<br/>role · bind · unbind · compose · holography"]
        physics["physics/<br/>Physics · PhysicsOp · ColorSpaceTimeManifold · Topology<br/>standing: harmonic_rotation"]
        p64["p64/<br/>Level P16 P32 P64 · syntax · searle · lexicon · topology · iching · words"]
        tests["tests.rs<br/>7 test functions"]
    end
    serde["serde<br/>Serialize · Deserialize"]
    phistd["phistd<br/>c64 · Function · GOLDEN_ANGLE · hexagram"]
    phiano["phiano<br/>pub use ::phiton<br/>gemgum · compose · config"]

    lib -->|"quantum · ALPHA · TWO_PI"| units
    lib -->|"SpectralBand::from_wavelength"| bands
    lib -->|"PhitonColor::new"| types
    lib -.->|"pub use Chiton"| chiton
    lib -.->|"pub use PhitonSpectrum"| spectrum
    lib -.->|"cfg test · mod tests"| tests
    types -->|"SpectralBand"| bands
    types -->|"PHI_CONJUGATE"| units
    types -->|"super::Phiton"| lib
    chiton -->|"LightQuantum"| types
    chiton -->|"ALPHA · PHI · TWO_PI"| units
    chiton -->|"super::Phiton"| lib
    hrr -->|"c64 · unit_phase · normalise"| phistd
    physics -->|"ALPHA · quantum · effective_phase"| units
    physics -->|"LightQuantum · Chiton"| chiton
    physics -->|"GOLDEN_ANGLE"| phistd
    p64 -->|"Hexagram · Trigram · fnv1a"| phistd
    spectrum -->|"SpectralBand"| bands
    spectrum -->|"LightQuantum · PhitonColor"| types
    spectrum -->|"ALPHA · PHI · TWO_PI"| units
    spectrum -->|"super::Phiton"| lib
    tests -->|"use super::*"| lib
    bands --> serde
    types --> serde
    chiton --> serde
    phiano -.->|"depends on"| lib
```

## The module pages

```mermaid
graph LR
    A["ARCHITECTURE.md"] --> L["lib.md<br/>lib.rs · Phiton"]
    A --> U["units.md<br/>units.rs"]
    A --> B["bands.md<br/>bands.rs"]
    A --> T["types.md<br/>types.rs"]
    A --> C["chiton.md<br/>chiton.rs"]
    A --> S["spectrum.md<br/>spectrum.rs"]
    A --> X["tests.md<br/>tests.rs"]
    A --> HR["hrr.md<br/>hrr.rs"]
    A --> PY["physics.md<br/>physics/"]
    A --> P6["p64.md<br/>p64/"]
```

| diagram | file |
|---|---|
| Phiton · phase to light | [docs/phiton/lib.md](../../docs/phiton/lib.md) |
| the constants and the clamp | [docs/phiton/units.md](../../docs/phiton/units.md) |
| the 16 spectral bands | [docs/phiton/bands.md](../../docs/phiton/bands.md) |
| colour and quantum | [docs/phiton/types.md](../../docs/phiton/types.md) |
| the wave | [docs/phiton/chiton.md](../../docs/phiton/chiton.md) |
| the spectrum mapper | [docs/phiton/spectrum.md](../../docs/phiton/spectrum.md) |
| the tests | [docs/phiton/tests.md](../../docs/phiton/tests.md) |
| holographic binding | [docs/phiton/hrr.md](../../docs/phiton/hrr.md) |
| colour, space and time · standing waves | [docs/phiton/physics.md](../../docs/phiton/physics.md) |
| the 16/32/64 variation engine | [docs/phiton/p64.md](../../docs/phiton/p64.md) |

## Status

2026-10-27 · 24 test functions · what is left: [TODO.md](TODO.md)
