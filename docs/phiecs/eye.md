# kit · eye

## signal · Inside signal.rs

```mermaid
graph LR
    aoc["struct Aoc<br/>phase: f64 · radians<br/>amplitude: f64 · non-negative"]
    mk["Aoc::new(phase, amplitude)"]
    seeded["Aoc::seeded(word, k)<br/>phase = seed_angles(word, k + 1)[k]<br/>amplitude 1.0"]
    to["to_complex<br/>c64::from_polar(amplitude, phase)"]
    from["from_complex · private<br/>phase = arg · amplitude = norm"]
    interfere["interfere(other)"]
    superpose["Aoc::superpose(signals)"]
    resonance["resonance(jitter) · f64<br/>(to_complex × jitter).arg()"]
    jit["fn jitter(angle) · c64<br/>c64::from_polar(1.0, angle)"]
    phistd["phistd<br/>c64 · seed_angles"]

    mk --> aoc
    seeded --> aoc
    aoc --> to
    to --> interfere --> from
    to --> superpose --> from
    to --> resonance
    jit --> resonance
    phistd -.-> seeded
    phistd -.-> to
    phistd -.-> jit
```

## signal · Interference — complex addition

```mermaid
flowchart LR
    A["Aoc a"] --> CA["a.to_complex()"]
    B["Aoc b"] --> CB["b.to_complex()"]
    CA --> SUM["+"]
    CB --> SUM
    SUM --> BACK["from_complex"] --> OUT["Aoc<br/>phase = arg · amplitude = norm"]
    MANY["superpose · slice of Aoc"] -->|"map to_complex · sum"| BACK
```

## signal · Resonance — complex multiplication

```mermaid
flowchart LR
    S["Aoc"] --> Z["to_complex()"]
    ANG["angle"] --> J["jitter(angle)<br/>unit rotation"]
    Z --> MUL["×"]
    J --> MUL
    MUL --> ARG["arg() · one atan2"] --> PH["f64<br/>the phase, rotated by angle"]
```

## signal · The type

```mermaid
classDiagram
    class Aoc {
        +f64 phase
        +f64 amplitude
        +new(phase, amplitude) Aoc
        +seeded(word, k) Aoc
        +to_complex() c64
        -from_complex(z) Aoc
        +interfere(other) Aoc
        +superpose(signals) Aoc
        +resonance(jitter) f64
    }
    class c64 {
        phistd · num_complex Complex64
    }
    Aoc ..> c64 : to_complex · resonance
```

## signal · What the test pins

```mermaid
flowchart TD
    T["interference_and_resonance_are_the_complex_moves"]
    T --> T1["Aoc(0, 1) interfere Aoc(π, 1)<br/>amplitude under 1e-12 · opposite phases cancel"]
    T --> T2["superpose c, c with c = Aoc(π/2, 2)<br/>amplitude 4 · phase π/2"]
    T --> T3["resonance(jitter(0.3))<br/>phase + 0.3"]
    T --> T4["Aoc::seeded(boat, 3).phase<br/>= seed_angles(boat, 4)[3]"]
```

## flow · Inside flow.rs

```mermaid
graph LR
    pace["enum Pace<br/>Near · Mid · Far<br/>name: near · mid · far"]
    flowt["struct Flow<br/>cells: Vec of (String, Aoc)"]
    mk["Flow::new()<br/>no cells"]
    see["see(name, signal)<br/>push one cell"]
    twitch["twitch(angle)<br/>Vec of (name, shift)"]
    paces["paces(angle)<br/>Vec of (name, Pace, shift)"]
    rows["rows(scope, angle)<br/>Vec of Row"]
    signal["signal.rs<br/>Aoc · jitter"]
    phical["phistd::wrap"]
    phiecs["crate::Row<br/>new · weighted"]

    mk --> flowt
    see --> flowt
    flowt --> twitch --> paces --> rows
    pace -.-> paces
    signal -.-> twitch
    phical -.-> twitch
    phiecs -.-> rows
```

## flow · A twitch, cell by cell

```mermaid
flowchart LR
    C["cell<br/>(name, Aoc s)"] --> B["before<br/>s.resonance(jitter(0.0))"]
    C --> A["after<br/>s.resonance(jitter(angle))"]
    B --> D["phistd::wrap(after − before)"]
    A --> D
    D --> ABS["abs × s.amplitude"] --> O["(name, shift)<br/>the parallax this thing shows"]
```

## flow · Paces — binned against the field itself

```mermaid
flowchart TD
    SH["twitch(angle)<br/>every shift"] --> SO["sorted by f64::total_cmp"]
    SO --> GET{"sorted.get(len / 3) · sorted.get(2 · len / 3)"}
    GET -->|"no cells"| EMPTY["no paces"]
    GET -->|"lo · hi"| Q{"Pace::of(v, lo, hi) · a match"}
    Q -->|"v ≥ hi"| NEAR["Pace::Near"]
    Q -->|"lo #lt; v #lt; hi"| MID["Pace::Mid"]
    Q -->|"v ≤ lo"| FAR["Pace::Far"]
```

## flow · Told as rows

```mermaid
sequenceDiagram
    participant F as Flow
    participant P as paces
    participant R as rows
    participant W as phiecs World
    F->>P: paces(angle)
    P->>R: Vec of (name, pace, v)
    loop each thing seen
        R->>R: Row::new(name, within, scope:scope)
        R->>R: Row::weighted(name, does, pace.name(), v)
        R->>R: Row::new(scope:scope, says, name)
    end
    R->>W: three rows a cell · World::from_rows in the test
```

## flow · The types

```mermaid
classDiagram
    class Pace {
        Near
        Mid
        Far
        +name() str
    }
    class Flow {
        +Vec of String and Aoc cells
        +new() Flow
        +see(name, signal)
        +twitch(angle) Vec of String and f64
        +paces(angle) Vec of String Pace f64
        +rows(scope, angle) Vec of Row
    }
    class Aoc {
        signal.rs
    }
    class Row {
        phiecs
        +String about
        +String kind
        +String other
        +f64 weight
    }
    Flow --> Aoc : cells
    Flow ..> Pace : paces
    Flow ..> Row : rows
```

## flow · What the test pins

```mermaid
flowchart LR
    S["Flow::new()<br/>hand Aoc(0.4, 3.0)<br/>table Aoc(1.1, 1.0)<br/>wall Aoc(2.0, 0.2)"] --> P["paces(0.25)<br/>hand Near · wall Far"]
    S --> R["rows(sight, 0.25)<br/>9 rows"]
    R --> R1["hand does near"]
    R --> R2["scope:sight says wall"]
    R --> W["phiecs World::from_rows<br/>lexicon has hand · near"]
```
