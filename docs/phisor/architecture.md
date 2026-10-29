# phisor — architecture

## The ladder: one language at several heights

```mermaid
graph TB
    W["words<br/>were you waiting in the rain"]
    M64["mode 64<br/>each word's class at 64 levels"]
    M32["mode 32"]
    M16["mode 16<br/>c9 c2 c14 c5 c1 c11 · structure only"]
    V["variation<br/>how the sentence opens and closes<br/>c9 c2 … c11"]
    K["kind<br/>the variations that play one part in the flow"]
    A["act<br/>ASK(wait(agent: you, in: rain), past, progressive)"]
    R["rows<br/>e17 is-a wait · e17 in rain@17 · e17 force ask"]
    MC["the machine<br/>phiclm: the Lisp · the judge"]
    W -->|"the code, 6 bits"| M64
    M64 -->|"drop a bit"| M32
    M32 -->|"drop a bit"| M16
    M16 -->|"the shape"| V
    V -->|"the same routine, one level up: the river"| K
    K -->|"read by the clause skeleton (step 5)"| A
    A -->|"ground"| R
    R --> MC
    A -->|"ASK → expression"| MC
```

## The code: three arms in the same modes

```mermaid
graph LR
    word["a word"]
    subgraph hash_["hash · the null"]
        h["phiton P16/P32/P64::classify_str<br/>the spelling's hash, its low 4, 5, 6 bits<br/>random buckets"]
    end
    subgraph given_["given · the book by hand"]
        g["phiton PosDictionary<br/>11 parts of speech + suffix fallback"]
    end
    subgraph learned_["learned · bottom-up"]
        e1["split in two by exchange<br/>move a word · keep if the class-pair<br/>likelihood is better"]
        e2["each half split again<br/>… six times"]
        code["a 6-bit code per word<br/>4 bits → 16 · 5 → 32 · 6 → 64<br/>nested: mode 16 is mode 64 coarsened"]
        e1 --> e2 --> code
    end
    word --> h
    word --> g
    word --> e1
    lik["phical::classes<br/>halve · move_gain · apply_move · split_cost"]
    lik -.-> e1
```

## The first experiment: the kinds of intention and their variations

```mermaid
graph TB
    guide["the guide's sentences, in order<br/>units' text · the Key (all held out)"]
    split["docs/203's split<br/>i % 10 = 0 held out · i % 20 = 5 dev"]
    code["the word code<br/>exchange over word pairs, training only"]
    mode["each sentence in a mode<br/>16 · 32 · 64 · per arm"]
    var["its variation<br/>open · close · both (the twins)"]
    river["training runs as sequences of variations"]
    kinds["kinds, as many as pay<br/>exchange over variation pairs: the river"]
    held["held-out sentences<br/>their kinds by their variations"]
    subgraph checks["read afterwards, never told"]
        fn_["function purity · explain · instruct · answer"]
        mk["mark purity · stop · question · exclamation"]
        rv["the river's mutual information<br/>against 1,000 shuffles · z"]
        sense["each label sensed by the kinds<br/>that answer to it on dev · F1"]
    end
    guide --> split --> code --> mode --> var --> river --> kinds
    kinds --> held
    held --> fn_
    held --> mk
    held --> rv
    held --> sense
```

## The twins and the amplitudes

```mermaid
graph LR
    s["a sentence in mode 16"]
    fwd["the opening twin<br/>c9 c2 …"]
    bwd["the closing twin, the mirror<br/>… c5 c11"]
    both["both ends<br/>c9 c2 … c11"]
    diff["where the twins disagree<br/>Babbage: listed, tried first"]
    prob["the twins' kinds as probabilities added"]
    amp["as amplitudes added, then squared<br/>phases learned by better and worse<br/>Feynman: the second run"]
    out["purity for each"]
    s --> fwd
    s --> bwd
    s --> both
    fwd --> diff
    bwd --> diff
    fwd --> prob
    bwd --> prob
    fwd --> amp
    bwd --> amp
    both --> out
    prob --> out
    amp --> out
```

## How the variations relate

```mermaid
graph TB
    a["a held-out sentence"]
    p["the sentence before it<br/>same unit, same run"]
    ka["its kind and variation"]
    kp["that sentence's kind and variation"]
    t["kind × kind<br/>which way of doing one thing<br/>is followed by which way of doing another"]
    mi["mutual information against<br/>1,000 shuffles of the sentence before · z"]
    loop["learning how to learn<br/>the routine set once at the words<br/>runs unchanged at the sentences"]
    a --> ka
    p --> kp
    ka --> t
    kp --> t
    t --> mi
    loop -.-> t
```

## The thinkers, each a small run

```mermaid
graph LR
    subgraph runs["runs on the guide, recorded straight"]
        se["Searle<br/>function from structure: I1, I3, M4"]
        wi["Wittgenstein<br/>family resemblance within a kind"]
        be["Bentov<br/>the counts at 16, 32, 64 · the slopes"]
        mi["the mirror<br/>the closing twin against the opening"]
        ba["Babbage<br/>the passes' differences · the stop"]
        lo["Lovelace<br/>generated, never told, attested"]
        fe["Feynman<br/>amplitudes against probabilities"]
        wh["Whitehead<br/>a question answered by relations: M6"]
        vn["von Neumann<br/>a kind as a small program<br/>on phiclm's machine"]
    end
    rec["docs/phisor/thinkers.md"]
    se --> rec
    wi --> rec
    be --> rec
    mi --> rec
    ba --> rec
    lo --> rec
    fe --> rec
    wh --> rec
    vn --> rec
```

## The one loop

```mermaid
graph TB
    pos["a position<br/>the code · the kinds · the phases · the lexicon"]
    move["a move<br/>one unit to its sibling class"]
    every["every pair it stands in<br/>(every reply)"]
    better{"better?<br/>the likelihood gains"}
    play["play it"]
    drop["refuted"]
    stop{"the pass gained under<br/>1/1000 of the first?"}
    curve["the held-out reading<br/>afterwards, never used to accept"]
    gain["phical::classes<br/>move_gain · apply_move"]
    pos --> move --> every --> better
    better -->|"no"| drop
    better -->|"yes"| play --> pos
    play --> stop
    stop -->|"yes"| curve
    gain -.-> better
    gain -.-> play
```

## The act, at the top

```mermaid
classDiagram
    class Act {
        force: Force
        frame: Frame
    }
    class Force {
        <<enum>>
        ASSERT · ASK · DIRECT · COMMIT
        EXPRESS · WISH · DECLARE
        point() Point
        fit() Fit
    }
    class Frame {
        pred: lemma
        slots: Role → Filler
        tense · aspect · modal
        voice · polarity
    }
    class Filler {
        <<enum>>
        Thing · Name · Pronoun · Word
        Hole(Wh)
        Clause(Frame)
    }
    class Template {
        the act, fillers blinded
        construction() pred blinded
    }
    Act --> Force
    Act --> Frame
    Frame --> Filler : each role
    Filler --> Frame : Clause, operators over operators
    Frame ..> Template : abstract
```

## English → the act → rows → the machine (steps 5 and 6)

```mermaid
graph LR
    S["English<br/>'who was the first king of France?'"]
    K["the closed words, the chunker<br/>the clause skeleton"]
    L["the open lexicon<br/>learned by the one loop"]
    A["ASK(be(theme: ?who,<br/>as: king{order: first, of: France}), past)"]
    G["rows · the event as an entity"]
    H["Lisp host · (rows kind)"]
    M["phiclm Machine · the set algebra"]
    Ans["#class{clovis}"]
    Z["realiser"]
    S --> K --> A
    L --> K
    A --> G --> H --> M --> Ans
    A --> Z -->|"M2: the same sentence?"| S
```

## The crate and the workspace

```mermaid
graph TB
    subgraph phisor_["phisor"]
        ir["ir (built)"]
        guide["guide (built)"]
        code["code (built)"]
        units["units (built)"]
        kind["kind (built)"]
        english["english"]
        learn["learn"]
        ground["ground"]
        machine["machine"]
        measure["measure"]
    end
    code --> units
    kind --> code
    kind --> units
    kind --> guide
    english --> ir
    english --> learn
    ground --> ir
    machine --> ground
    measure --> kind
    measure --> machine
    phiton["phiton<br/>p64 Level · classify_str · PosDictionary"]
    phical["phical<br/>classes: halve · Grow · Halving · split_cost<br/>liskov::staged"]
    phistd["phistd<br/>stats: purity · mutual_information · z_score · f1"]
    phiecs["phiecs<br/>speak::tokens · Row · World"]
    phiclm["phiclm<br/>lisp::Machine · Host · judge"]
    code --> phiton
    code -->|"halve"| phical
    kind -->|"halve"| phical
    measure --> phistd
    guide --> phiecs
    ground --> phiecs
    machine --> phiclm
```
