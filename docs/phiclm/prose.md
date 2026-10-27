# prose and the two compilers

## Known by writing, known by learning

```mermaid
graph LR
    subgraph written["KNOWN BY WRITING — rules a person wrote down"]
        K1["BCPL<br/>the grammar in parse/<br/>the semantics in trn/ and machine/"]
    end
    subgraph learned["KNOWN BY LEARNING — rules found in a corpus"]
        K2["English, as one grammar book uses it<br/>phiano's learner and phasors"]
    end
    C1["ext/bcpl/cintcode<br/>9,524 lines of the real system"] --> K1
    C2["data/person/crick.txt<br/>Crick 2004: the expression precedence"] --> K1
    C3["data/person/murphy.txt<br/>English Grammar in Use, 129 units"] --> K2
    C4["data/person/murphy_key.txt<br/>the Key to the exercises"] --> K2
    C5["data/corpora/<br/>Austen · Carroll · Chesterton · Liskov · Lovelace"] --> K2
    K1 ==>|"right by construction: it can JUDGE"| J["the judge"]
    K2 ==>|"until docs/203 by counting · since by error"| LR["the learner"]
```

## Two compilers, one ladder

```mermaid
graph TB
    subgraph program["a program (phiclm)"]
        A1["LEX<br/>characters → tokens"]
        A2["SYN<br/>tokens → tree"]
        A3["TRN — COMPRESSION<br/>tree → O-code"]
        A4["CG — EXPANSION<br/>O-code → the machine's words"]
        A5["the machine runs it<br/>→ a Verdict"]
        A1 --> A2 --> A3 --> A4 --> A5
    end
    subgraph prose["a sentence (phiano)"]
        H1["LEX<br/>characters → words"]
        H2["SYN<br/>words → constructions"]
        H3["TRN — COMPRESSION<br/>sentence → template of TYPES (docs/200)"]
        H4["CG — EXPANSION<br/>type → which word (docs/203)"]
        H5["the intention is a kind on the ladder<br/>explain = s_ass · instruct = s_rtap · answer = s_resultis"]
        H1 --> H2 --> H3 --> H4
        H3 --> H5
    end
    H5 ==>|"an instruction becomes a program:<br/>893 of 893 compiled and ran (docs/201)"| A1
    A5 ==>|"the Verdict, told as rows"| H2
```

## The bridge, step by step

```mermaid
flowchart LR
    I["an exercise line<br/>Put the verb into the correct form"] --> TP["its template"] --> K["its kind: s_rtap"] --> PG["a program"] --> JD["the judge"]
    PG -.today.-> SH["carries only the SHAPE: result = the sentence's length"]
    PG -.next.-> AR["carries the ARGUMENTS: which grammar, which sentences"]
    AR -.waits on.-> TY["typing the closed-stratum words (docs/202)"]
```

## The rules and conditions

| | in BCPL | in prose |
|---|---|---|
| vocabulary of structure | `KEYWORDS` · closed, written | `the to a is of …` · closed, learned |
| open vocabulary | names | content words |
| a rule | a production in `parse/` | a construction family · a channel |
| a condition | `IF e`, `TEST e` | the words before, each through its operator |
| a type | a manifest kind | a word's sector on the circle (docs/200) |
| ground truth | the judge | held-out sentences · the Key · the judge |
