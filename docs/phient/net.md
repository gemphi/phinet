# net

`phient::net`: the lexicon read as a function space, beside the operators.
It was phiano's `phinet`; phiano keeps the model-file reader and the
perplexity scorer, which were `phiano::net`, are here too (below).

```mermaid
graph LR
    Facet["phiecs::phasor::facet::Facet<br/>lexicon of SpectralPhasor"]
    Phistd["phistd<br/>Function, Adam, c64, fnv1a"]
    Hrr["phiton::hrr<br/>role, bind, unbind, compose"]
    Words["phiton::p64::words::Tokenizer<br/>phiton::units::PHASE_CHANNELS"]
    Phical["phical<br/>Fno, Example, Joint, Tie::Circle"]
    Users["phiano: net::lm, generate::guide<br/>command::generate, persona::fingerprint"]

    subgraph net["phient::net"]
        subgraph space["space.rs, space/"]
            FS["FunctionSpace<br/>words, index, funcs, table<br/>Sides: Source, Target, Both, Directed"]
            Enc["encode.rs<br/>encode, encode_side, encode_both<br/>from_facet_sides, context, pool, examples"]
        end
        Joint["joint.rs<br/>index_examples, write_back"]
        subgraph compose["compose/"]
            Comp["mod.rs: Composition<br/>role(k), from_corpus_keyed, PairKey"]
            Recover["recover.rs<br/>identities, recover, cleanup_examples"]
        end
    end

    Facet -->|"phasor per word"| Enc
    Words --> Enc
    Enc -->|"unit-energy functions"| FS
    Phistd --> FS
    FS -->|"(context, next word) examples"| Joint
    Phical -->|"Joint training"| Joint
    Joint -->|"learned functions"| Facet
    FS -->|"offset fillers d 1..4"| Comp
    Hrr -->|"bind, unbind, compose"| Comp
    Comp --> Recover
    Recover -->|"noisy unbind to clean filler"| Phical
    FS --> Users
    Enc --> Users
```

## The reader and the scorer

```mermaid
graph LR
    FS["phient::net::FunctionSpace<br/>context functions"]
    Phical["phical::Fno<br/>a trained operator"]
    Files["data/*.chroma<br/>storage::Storage, memory::Memo"]
    Harness["metrics::harness::PhianoLM<br/>gamma_sweep_scored"]

    subgraph net["phient::net"]
        LM["lm.rs: OperatorScorer<br/>score, score_identity"]
        subgraph reader["reader.rs, reader/"]
            Report["ModelReport, ModelKind"]
            Version["version.rs<br/>inspect, header_version, render"]
        end
    end

    FS -->|"context function"| LM
    Phical --> LM
    LM -->|"cosine scores per word"| Harness
    Files -->|"Storage::load, Memo"| Version
    Version --> Report
```
