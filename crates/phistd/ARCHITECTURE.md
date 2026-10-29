# phistd — architecture

## The crate, module by module

```mermaid
graph TD
    lib["lib.rs<br/>c64 · Function · fnv1a · mix64 · decoy · GOLDEN_ANGLE<br/>seed_phase · seed_angles · normalise · unit_random · unit_phase<br/>stack · unstack · cosine"]
    subgraph engine["the engine"]
        tensor["tensor/<br/>mod · tape · fft · squash · adam<br/>ops/ basic · score · loss<br/>backward/ score · loss<br/>tests/ fft · grad · loss · optim"]
    end
    subgraph sixtyfour["the sixty-four"]
        hexagram["hexagram/<br/>mod: KING_WEN_MAP · gate · apply · hexagram_of · lines · novelty<br/>figure: Hexagram · trigram: Trigram · names: archetype_name"]
        codon["codon.rs<br/>BASES · codon · codon_index · hexagram_glyph<br/>helix_position · PITCH_PER_TURN · RISE"]
    end
    phasor["phasor/<br/>mod · functions · tests<br/>Resolution · Quantum · Range"]
    subgraph numerics["the numerics (from phient, 2026-10-28)"]
        random["random.rs<br/>unit_interval · lcg · xorshift64(*) · draw_by_weight"]
        stats["stats.rs<br/>mean · population_sd · precision · recall · f1 · log_share<br/>purity · mutual_information · z_score"]
        prob["prob.rs<br/>perplexity · softmax_z/p · log_softmax_at<br/>softmax_minus_onehot · sample_softmax"]
        real["real.rs<br/>normalise_f32 · dot_f32 · norm · re_dot_interleaved"]
        optim["optim.rs<br/>adam_step on slices"]
    end
    subgraph extern_["external crates"]
        numc["num-complex<br/>Complex64"]
        rayon["rayon"]
        serde["serde"]
    end
    subgraph users["workspace crates with phistd as a path dependency"]
        phifno["phifno"]
        phical["phical"]
        phiecs["phiecs"]
        phiton["phiton"]
        phient["phient"]
        phiano["phiano"]
    end

    lib -->|"pub use Tape · Tensor · Var · Adam · AdamState · Schedule<br/>MixtureTerms · fft_inplace · fft2_inplace · Tensor::randn"| tensor
    lib -->|"pub use Quantum · Range · Resolution"| phasor
    lib -->|"seed_phase · seed_angles · unit_phase read unit_interval"| random
    real -->|"c64"| lib
    tensor -->|"crate::c64"| lib
    hexagram -->|"fft_inplace"| tensor
    hexagram -->|"cosine · c64 · Function"| lib
    codon -->|"hexagram::glyph"| hexagram
    phasor -->|"fft_inplace"| tensor
    phasor -->|"cosine · c64 · Function"| lib
    phasor -.->|"tests: hexagram_of"| hexagram
    lib -->|"c64 = Complex64"| numc
    tensor -->|"par_chunks_mut · cosine_table"| rayon
    tensor -->|"derive on Schedule · AdamState"| serde
    users -->|"phistd = path ../phistd"| lib
```

## The module pages

```mermaid
graph LR
    A["ARCHITECTURE.md"] --> L["lib.md<br/>lib.rs"]
    A --> T["tensor.md<br/>tensor/"]
    A --> P["phasor.md<br/>phasor/"]
    A --> H["hexagram.md<br/>hexagram/"]
    A --> C["codon.md<br/>codon.rs"]
    A --> N["numerics.md<br/>random · stats · prob · real · optim"]
```

| diagram | file |
|---|---|
| the crate root: seeds, hashes, functions | [docs/phistd/lib.md](../../docs/phistd/lib.md) |
| the tensor engine: tape, ops, backward, FFT, Adam | [docs/phistd/tensor.md](../../docs/phistd/tensor.md) |
| the phasor ranges | [docs/phistd/phasor.md](../../docs/phistd/phasor.md) |
| the sixty-four gates | [docs/phistd/hexagram.md](../../docs/phistd/hexagram.md) |
| the sixty-four as three bases | [docs/phistd/codon.md](../../docs/phistd/codon.md) |
| the numerics phient used to write inline | [docs/phistd/numerics.md](../../docs/phistd/numerics.md) |

## Status

2026-10-29 · 36 lib tests (the numerics add 9; the table readings 1) · what is left: [TODO.md](TODO.md)
