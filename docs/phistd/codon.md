# codon

## Inside codon.rs

```mermaid
graph LR
    B["BASES = A · C · G · T"]
    N["CODONS = 64"]
    cd["codon(k) String"]
    ci["codon_index(s) Option of usize"]
    hg["hexagram_glyph(k) String"]
    cd --> B
    cd --> N
    ci --> B
    hg -->|"hexagram::glyph"| H["hexagram.rs<br/>King Wen order"]
```

## k as three bases

```mermaid
flowchart LR
    K["k"] --> M["k mod 64"]
    M --> A["k / 16"] --> BA["first base"]
    M --> Bm["(k / 4) mod 4"] --> BB["second base"]
    M --> C["k mod 4"] --> BC["third base"]
    BA --> S["three letters from BASES"]
    BB --> S
    BC --> S
```

## Back again — codon_index(s)

```mermaid
flowchart LR
    S["s"] --> CH["each char, uppercased"] --> P{"position in BASES"}
    P -->|"none"| NO["None"]
    P -->|"p"| ACC["k = 4k + p"]
    ACC --> L{"length 3"}
    L -->|"yes"| SOME["Some(k)"]
    L -->|"no"| NO
```

## The sixty-four, three ways — from the test

```mermaid
graph LR
    k0["k 0"] --> a0["AAA"] --> g0["䷁"]
    k27["k 27"] --> a27["CGT"]
    k63["k 63"] --> a63["TTT"] --> g63["䷀"]
    k0 -.-> six["the same six bits a hexagram reads as lines"]
```

## Two readings — from the module doc

```mermaid
graph LR
    K["codon k"] --> HE["phiano's helix<br/>a word's relation channel k"]
    K --> ST["phient's strand<br/>level k of a text<br/>AAA the word … TTT the whole story"]
```

## The helix — where codon k sits

```mermaid
flowchart LR
    K["codon k"] --> TH["angle 2πk / PITCH_PER_TURN · 10.5 a turn, B-DNA"]
    K --> Z["rise RISE · k · 0.34 nm"]
    TH --> XY["x, y on a radius of 1 nm"]
    XY & Z --> POS["helix_position(k) · [x, y, z]"]
    POS -.->|"phiano helix · Codon::of"| APP["the codon table, with phifno's degree and order"]
```

