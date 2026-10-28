# phical — architecture

## The crate, module by module

```mermaid
graph TD
    lib["lib.rs<br/>CHANNELS · TAU<br/>pub use operator · heat · phistd pieces · wrap · c64"]
    subgraph learned["learned"]
        joint["joint/<br/>mod · tests<br/>Joint · IndexExample"]
        relate["relate/<br/>mod · laws · closure · inherit · structure · score · tests<br/>Triple · Law · Inverse · Inherit · Converse · Score"]
        liskov["liskov.rs<br/>better · worse · loss · forcing_choice"]
    end
    subgraph exact["exact"]
        q64["q64.rs<br/>BOUND · quantise · lift · hamming · sector_distance"]
        helix["helix.rs<br/>Codon: codon · hexagram · degree · order · position"]
    end
    subgraph below["workspace crates below"]
        phifno["phifno<br/>operator: Fno · FnoConfig · Tie · rank · Example<br/>heat"]
        phistd["phistd<br/>Tensor · Tape · Var · Adam · cosine · mix64 · normalise · decoy<br/>phasor::Quantum"]
    end
    subgraph outside["external crates"]
        numc["num-complex<br/>Complex64 as c64"]
    end
    subgraph above["workspace crates above · their Cargo.toml"]
        users["phiano · phient"]
    end
    examples["examples/<br/>apply · bench · filter · ranges · softmax · torch + torch/extra"]

    lib --> joint & relate & liskov & q64 & helix
    helix -->|"sphere::degree_of, order_of"| phifno
    helix -->|"codon::{codon, helix_position, hexagram_glyph}"| phistd
    lib -->|"pub use heat · operator"| phifno
    lib -->|"pub use"| phistd
    lib -->|"pub use"| numc
    relate -->|"Joint · IndexExample"| joint
    joint -.->|"crate::operator::Fno"| phifno
    relate -.->|"fno.apply"| phifno
    joint & relate -->|"phistd::decoy"| phistd
    joint & liskov & relate -.->|"crate:: mix64 · cosine · normalise · Adam · Function · tensor"| phistd
    q64 -->|"phistd::phasor::Quantum"| phistd
    q64 -->|"CHANNELS · c64"| lib
    phifno --> phistd
    users --> lib
    examples -.-> lib & phistd & phifno
    key["solid: a direct path<br/>dotted: through lib.rs pub use"]
```

## The module pages

```mermaid
graph LR
    A["ARCHITECTURE.md"] --> L["lib.md<br/>lib.rs"]
    A --> J["joint.md<br/>joint/"]
    A --> R["relate.md<br/>relate/"]
    A --> K["liskov.md<br/>liskov.rs"]
    A --> Q["q64.md<br/>q64.rs"]
    A --> T["TODO.md<br/>what is left"]
```

| diagram | file |
|---|---|
| the crate root and its re-exports | [docs/phical/lib.md](../../docs/phical/lib.md) |
| the operator and the table, trained together | [docs/phical/joint.md](../../docs/phical/joint.md) |
| relations, laws, closure, inheritance, the reading | [docs/phical/relate.md](../../docs/phical/relate.md) |
| better · worse · loss | [docs/phical/liskov.md](../../docs/phical/liskov.md) |
| the C64/Q64 juncture | [docs/phical/q64.md](../../docs/phical/q64.md) |

## Status

2026-10-27 · 19 test functions · what is left: [TODO.md](TODO.md)
