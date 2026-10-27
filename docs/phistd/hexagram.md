# hexagram

## Inside hexagram/

```mermaid
graph LR
    subgraph consts["constants"]
        HX["HEXAGRAMS = 64 · LINES = 6"]
        KW["KING_WEN_MAP<br/>binary id → King Wen number"]
    end
    subgraph naming["naming"]
        kwf["king_wen(id) u8"]
        gl["glyph(id) char"]
    end
    subgraph gates["the gate"]
        oc["octave(mode, space) Option of usize"]
        ga["gate(id, space) Vec of bool"]
        ap["apply(f, id) Function"]
        cl["changing_lines(id, mask) u8"]
    end
    subgraph reading["the reading"]
        ho["hexagram_of(f) u8"]
        po["phase_of(id) f64"]
        nv["novelty(f, id) f64"]
        li["lines(f) six f64"]
    end
    gl --> kwf --> KW
    ap --> ga --> oc
    li --> oc
    nv --> ap
    nv -->|"crate::cosine"| LIB["lib.rs"]
    ap -->|"fft_inplace"| TEN["tensor"]
    li -->|"fft_inplace"| TEN
    CO["codon::hexagram_glyph"] --> gl
    PT["phasor tests"] -.-> ho
```

## Six lines, six octaves of the spectrum

```mermaid
block-beta
    columns 7
    m0["mode 0<br/>the mean<br/>always passes"] l0["line 0 · bottom<br/>mode 1"] l1["line 1<br/>modes 2–3"] l2["line 2<br/>modes 4–7"] l3["line 3<br/>modes 8–15"] l4["line 4<br/>modes 16–31"] l5["line 5 · top<br/>32 and up to Nyquist"]
```

## A gate — apply(f, id)

```mermaid
flowchart LR
    F["f"] --> FF["fft_inplace forward"] --> M["mode m"]
    M --> O{"octave(m, n)"}
    O -->|"None · mode 0"| K["kept"]
    O -->|"Some(j)"| B{"bit j of id"}
    B -->|"1 · yang"| K
    B -->|"0 · yin"| Z["zeroed"]
    K --> IF["fft_inplace inverse"]
    Z --> IF
    IF --> H["H_id f"]
    M -.->|"m #gt; n/2 reads as n − m"| O
```

## The two ends, and the oracle's move

```mermaid
graph LR
    C["id 63 · 111111 · ䷀ the Creative"] -->|"apply"| I["the identity"]
    R["id 0 · 000000 · ䷁ the Receptive"] -->|"apply"| MN["the mean only"]
    X["changing_lines(id, mask)"] --> XO["(id xor mask) and 0b11_1111"]
```

## The hexagram a function is

```mermaid
flowchart LR
    F["f"] --> C["centroid Σ f"] --> P["arg rem τ"] --> S["floor(phase / τ · 64)"] --> ID["hexagram_of = id"]
    ID --> PO["phase_of(id) = (id + 0.5) / 64 · τ"]
    PO -.->|"round trip, tested for every id"| ID
    F --> LI["lines(f)<br/>energy per octave, normalised to sum one"]
    F --> NV["novelty(f, id)<br/>1 − cosine(f, apply(f, id)) · in [0, 2]"]
```

## Naming a hexagram

```mermaid
flowchart LR
    ID["binary id 0–63<br/>bottom line = bit 0"] --> KW["king_wen · KING_WEN_MAP"] --> N["King Wen number 1–64"] --> G["glyph<br/>U+4DC0 + number − 1"]
    E0["id 0"] --> E0N["number 2"] --> E0G["䷁"]
    E63["id 63"] --> E63N["number 1"] --> E63G["䷀"]
```

## The hexagram as a figure — figure.rs, trigram.rs, names.rs

```mermaid
classDiagram
    class Hexagram {
        +u8 id
        +u8 number
        +Trigram upper
        +Trigram lower
        +from_id(id) Hexagram
        +from_phase(phase) Hexagram
        +phase_angle() f64
        +spin(delta) Hexagram
        +changing_lines(mask) Hexagram
        +unicode_char() char
        +archetype_name() str
    }
    class Trigram {
        Unity · Lake · Fire · Thunder
        Wind · Water · Mountain · Earth
        +bits() u8
        +from_bits(bits) Trigram
        +symbol() str
        +name_pinyin() str
    }
    Hexagram --> Trigram : lower bits 0–2 · upper bits 3–5
```

```mermaid
flowchart LR
    F["Hexagram methods"] --> K["number = king_wen(id)"]
    F --> G["unicode_char = glyph(id)"]
    F --> P["phase_angle = phase_of(id)"]
    F --> C["changing_lines = changing_lines(id, mask)"]
    F --> N["archetype_name = names::archetype_name(number)"]
    PH["phiano phinum::iching"] -->|"uses"| F
    PH --> OWN["its own: of_syntax_key · speech_act"]
```

