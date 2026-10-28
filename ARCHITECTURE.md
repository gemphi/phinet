# phinet — architecture

## The crates, as dependencies run (downward only)

```mermaid
graph TD
    phistd["phistd — the standard floor"]
    phifno["phifno — Fourier neural operators"]
    phical["phical — the paradigm: X ⇒ Y, phasors on the circle"]
    phiecs["phiecs — the world: entity, node, Row, World::tell,<br/>the phasor (packed form, facet lexicon, wave), the kit (body/track/device/eye)"]
    phient["phient — the BRAIN: arena and its lineup (the learner),<br/>study protocol and its one Score, prose knowing, ledgers"]
    phiclm["phiclm — the compiler: CPL→BCPL→B→C→phiclm, the judge"]
    phiton["phiton — quantum light: α ≈ 1/137, P64,<br/>Chiton and the conjugate mirror, Bentov's standing waves, holography"]
    phiano["phiano — the instrument: agents, personas, seam, examples, servers"]

    phistd --> phifno --> phical --> phiecs
    phistd --> phiton
    phiton --> phiecs
    phiton --> phient
    phiecs --> phient
    phiecs --> phiclm
    phient --> phiano
    phiclm --> phiano
    phiton --> phiano
```

## Where learning lives — counted, and corrected (docs/203)

```mermaid
graph TB
    L["a line is told<br/>phient::prose::tokens"]
    subgraph counted["COUNTED — phiecs::World (rounds 1–202)"]
        C1["each pair a ⇒ b under a kind<br/>(next, a family, after &lt;word&gt;)"]
        C2["Phasor::couple<br/>a's angle → b's − TURN<br/>a running mean: step 1/(n+1)"]
        C3["World::reads<br/>the words nearest one TURN past"]
        C1 --> C2 --> C3
    end
    subgraph corrected["CORRECTED — phient::lineup::learner (round 203)"]
        E1["the condition z<br/>the words before, each through<br/>its distance's operator, plus the pair"]
        E2["predict<br/>softmax over every word,<br/>by agreement with z"]
        E3["compare<br/>with the word that came:<br/>error = p − truth"]
        E4["move<br/>toward the word that came,<br/>away from the words expected instead;<br/>angles back on the unit circle"]
        E1 --> E2 --> E3 --> E4 --> E1
    end
    A["phient::study::Arena<br/>every reader guesses before it is told;<br/>one table, the same positions"]
    L --> C1
    L --> E1
    C3 --> A
    E2 --> A
```

## The single layer (docs/197)

```mermaid
graph LR
    ASK["an ask"]
    subgraph phient["phient — the orchestrator"]
        INV["the inventory<br/>a phiano in phinet,<br/>a phiora database"]
        ROUTE["the router<br/>which model answers"]
        ARENA["the arena<br/>every model, one table"]
    end
    subgraph base["one phasor world — the base"]
        FEAT["a feature = a channel<br/>a construction family, opened lazily"]
        ACT["an action = a kit row<br/>typed, told, never scraped"]
    end
    ORACLE["an outside oracle<br/>only as overflow"]
    ASK --> ROUTE
    INV --> ROUTE
    ROUTE --> FEAT
    ROUTE --> ACT
    ROUTE -.->|"when nothing inside answers"| ORACLE
    ARENA --> INV
```

## The repositories, as submodules run

```mermaid
graph LR
    subgraph github.com/gemphi
        phinetR["phinet.git — the workspace"]
        phianoR["phiano.git"]
        phifnoR["phifno.git"]
        phiecsR["phiecs.git"]
        phiclmR["phiclm.git"]
        phientR["phient.git — the brain"]
        phikitR["phikit.git — the reality kit"]
        phiberR["phiber.git — worlds"]
        phirayR["phiray.git — rendering + phiphx physics"]
        phiaceR["phiace.git — robotics"]
        puikitR["puikit.git — UI apps + fonts"]
        pediumR["pedium.git — open science on all of it"]
        phihubR["phihub.git — the manifest of everything"]
    end
    phinetR -. submodule .-> phianoR
    phinetR -. submodule .-> phifnoR
    phinetR -. submodule .-> phiecsR
    phinetR -. submodule .-> phiclmR
    phinetR -. submodule .-> phientR
    phikitR -. submodule .-> phiberR
    phikitR -. submodule .-> phirayR
    phikitR -. submodule .-> phiecsR
```

## Where it stands (docs/203)

```mermaid
xychart-beta
    title "Held-out sentences of the English guide, next word top-1 % (docs/203)"
    x-axis ["floor", "phasors counted", "counts", "learner", "trigram"]
    y-axis "top-1 %" 0 --> 30
    bar [4.5, 13.6, 17.1, 21.3, 24.8]
```

## Each crate, drawn

- [phistd](crates/phistd/ARCHITECTURE.md) · [phifno](crates/phifno/ARCHITECTURE.md) · [phical](crates/phical/ARCHITECTURE.md) · [phiecs](crates/phiecs/ARCHITECTURE.md)
- [phient](crates/phient/ARCHITECTURE.md) · [phiclm](crates/phiclm/ARCHITECTURE.md) · [phiton](crates/phiton/ARCHITECTURE.md) · [phiano](crates/phiano/ARCHITECTURE.md)

Notes: [docs/notes.md](docs/notes.md) · What is left to do: [TODO.md](TODO.md)
