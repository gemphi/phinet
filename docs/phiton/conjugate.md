# conjugate

`phiton::conjugate`, beside `chiton`; it was phical's.

## Files

```mermaid
graph LR
    mod_["conjugate/mod.rs<br/>Mirror · Told · Saved · SavedUnpumped<br/>new · reindex · kind · filter · wave · correlate · read · save · load"]
    train["train.rs<br/>Moment · adam · B1 · B2 · EPS<br/>score · step_word · train_epoch"]
    tests["tests.rs<br/>3 test functions"]
    lib["phistd<br/>c64 · mix64 · unit_random · normalise · decoy"]
    ser["serde · bincode"]
    mod_ -->|"pub use Moment"| train
    train -->|"super::Mirror · Told"| mod_
    tests -.-> mod_
    mod_ --> lib
    train --> lib
    mod_ --> ser
```

## A kind's filter, raised to a power

```mermaid
flowchart LR
    k["kind"] --> th["phases θ per mode"]
    k --> g["gains g per mode<br/>all zero unless pumped"]
    th --> f["filter(kind, p)<br/>from_polar(exp(g · abs p), θ · p)"]
    g --> f
    f --> p1["p = 1<br/>H · the relation forward"]
    f --> pm["p = −1<br/>H̄ · read backwards<br/>the inverse while gains are zero · the adjoint once pumped"]
    f --> p2["p = 2<br/>H² · two steps on"]
```

## A reading

```mermaid
flowchart LR
    q["read(word, kind, power)"] --> i{"word in index<br/>kind in kind_index"}
    i -->|"no"| none["None"]
    i -->|"yes"| w["wave(w, k, p)<br/>H^p · a_w, mode by mode"]
    w --> c["correlate(q)<br/>Re of Σ q · conj(a_v), for every word v"]
    c --> s["Some(a score per word)"]
```

## One told row in train_epoch

```mermaid
flowchart TD
    o["rows ordered by mix64(i ^ seed)"] --> d["decoy d = phistd::decoy(pool, other)"]
    d --> u["u = H · a_about<br/>n = norm of u"]
    u --> sc["sp = Re⟨u, a_other⟩ / n<br/>sd = Re⟨u, a_d⟩ / n"]
    sc --> l{"loss = margin − sp + sd<br/>above zero"}
    l -->|"no"| skip["next row"]
    l -->|"yes"| gr["g = ∂loss/∂u<br/>pumped: (b − s·u/n)/n · unit mirror: b"]
    gr --> gw["about: H̄ · g"]
    gr --> gp["other: −u/n"]
    gr --> gd["decoy: u/n"]
    gw --> sw["step_word × 3<br/>Adam on re and im · then normalise"]
    gp --> sw
    gd --> sw
    gr --> gt["phases: −Im(conj(g) · u)"] --> ap["adam on phases of the kind"]
    gr --> gg["gains: Re(conj(g) · u)"] --> pp{"pumped"}
    pp -->|"yes"| ag["adam on gains of the kind"]
    sw --> mean["mean hinge over rows"]
    ap --> mean
    ag --> mean
```

## Save and load

```mermaid
flowchart LR
    m["Mirror"] -->|"save"| s["Saved<br/>spectra as (re, im) pairs · phases · gains · pumped"] --> b["bincode bytes"] --> file["file"]
    file -->|"load"| t{"reads as Saved"}
    t -->|"yes"| back["Mirror · reindex"]
    t -->|"no"| u["SavedUnpumped<br/>the layout before gains"] --> z["gains all zero · pumped false"] --> back
```

## The types

```mermaid
classDiagram
    class Mirror {
        +usize modes
        +Vec of String names
        +Vec of Vec of c64 spectra
        +Vec of String kinds
        +Vec of Vec of f64 phases
        +Vec of Vec of f64 gains
        +bool pumped
        +HashMap of String to usize index
        -HashMap of String to usize kind_index
        +new(names, kinds, modes, pumped, seed) Mirror
        -reindex()
        +kind(name) Option of usize
        +filter(kind, power) Vec of c64
        +wave(word, kind, power) Vec of c64
        +correlate(q) Vec of f64
        +read(word, kind, power) Option of Vec of f64
        +score(w, kind, v) f64
        -step_word(mo, w, g, lr)
        +train_epoch(rows, pool, margin, lr, mo, seed) f64
        +save(path) io Result
        +load(path) io Result of Mirror
    }
    class Told {
        +usize about
        +usize kind
        +usize other
    }
    class Moment {
        -spectra Adam moments per word and mode
        -phases Adam moments per kind and mode
        -gains Adam moments per kind and mode
        -steps per word
        -kind_steps per kind
        +new(m) Moment
    }
    class Saved {
        modes · names · spectra as re-im pairs
        kinds · phases · gains · pumped
    }
    class SavedUnpumped {
        modes · names · spectra as re-im pairs
        kinds · phases
    }
    Mirror ..> Told : train_epoch rows
    Mirror ..> Moment : train_epoch mo
    Mirror ..> Saved : save and load
    Mirror ..> SavedUnpumped : load fallback
```
