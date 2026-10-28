# relate

## Files

```mermaid
graph TD
    mod_["relate/mod.rs<br/>Triple · example · pools · pub use"]
    laws["laws.rs<br/>Law · Inverse · find_laws · find_inverses · implied"]
    closure["closure.rs<br/>closure_pairs · closure_epoch"]
    inherit["inherit.rs<br/>Inherit · from_told · with_transitive · add · of<br/>inherit_epoch · context · predictions · score_inherit"]
    structure["structure.rs<br/>Converse · none · declared · laws_epoch · score_laws"]
    score["score.rs<br/>Score · chance · rank · tally · context · score · prior · two_hop"]
    tests["tests.rs<br/>8 test functions"]
    joint["joint/<br/>Joint · IndexExample"]
    mod_ --> laws & closure & inherit & structure & score
    mod_ -->|"IndexExample"| joint
    closure -->|"Law"| laws
    inherit -->|"tally · Score"| score
    inherit -->|"laws_epoch · Converse::none"| structure
    structure -->|"context · predictions · Inherit"| inherit
    structure -->|"tally · Score"| score
    closure & structure & inherit & score -->|"Joint"| joint
    laws & closure & inherit & structure & score -.->|"super::Triple"| mod_
    tests -.-> mod_
```

## A triple becomes an example

```mermaid
flowchart LR
    t["Triple<br/>about · kind · other"] -->|"example"| e["IndexExample<br/>ctx: (kind, 1.0), (about, 1.0) · target: other"]
    ts["told triples"] -->|"pools"| p["pool: every item named as an answer<br/>by_kind: per kind, the items named under it"]
```

## Laws, counted not written

```mermaid
flowchart TD
    told["told triples"] --> fl["find_laws(told, min_support)"]
    fl --> ans["answer sets per (about, kind)"]
    ans --> pair["each kind pair k1, k2<br/>each told w k1 v, w ≠ v, both told under k2"]
    pair --> sup["support + 1<br/>agree + 1 when the two answer sets meet"]
    sup --> law["Law k1 · k2 · support · agree<br/>kept at support ≥ min_support · strongest first"]
    told --> fi["find_inverses(told, min_support)"]
    fi --> inv["Inverse k · k_inv · support · agree<br/>agree: v k_inv w is told"]
    inv --> imp["implied(told, inverses)<br/>w k v for every told v k_inv w, not yet told"]
    law --> cp["closure_pairs(told, laws)<br/>(w, v, k2) for every told w k1 v"]
```

## Three ways to train the Joint

```mermaid
flowchart LR
    cp["closure pairs (w, v, k2)"] --> ce["closure_epoch<br/>read w, v and a random item u under k2<br/>hinge(cos(yw, yv), cos(yw, yu)) · alpha"]
    told["told triples"] --> le["laws_epoch<br/>one-way rows first, converse rows after<br/>chunks of 32"]
    told --> ie["inherit_epoch"]
    ie -->|"Converse::none"| le
    ce --> step["backward · adam_op on fno.params<br/>adam_table on the table · mask_modes"]
    le --> step
```

## One batch of laws_epoch

```mermaid
flowchart TD
    rows["batch: w · k · target<br/>decoy from phistd::decoy"] --> y["y = F(unit(w + k))"]
    y --> inh{"Inherit given"}
    inh -->|"yes"| yp["y = y + lambda · F(unit(p + k))<br/>p: one told parent drawn by seed, or w"]
    inh -->|"no"| sc["sp = cos(y, target)<br/>sn = cos(y, decoy)"]
    yp --> sc
    sc --> cv{"k has a declared converse k'"}
    cv -->|"yes"| bk["sp += mu · cos(F(unit(target + k')), w)<br/>sn += mu · cos(F(unit(decoy + k')), w)"]
    cv -->|"no"| h["hinge(sp, sn, margin)"]
    bk --> h
```

## Inheritance — who reads a parent

```mermaid
flowchart LR
    r["row w · k"] --> v{"k in via"}
    v -->|"no"| own["own reading only"]
    v -->|"yes"| pa["parents of (via k, w)<br/>told rows under the via kind"]
    pa -->|"empty"| own
    pa -->|"some"| add["own + lambda · mean of the parents' readings under k"]
    ft["from_told(told, isa, kinds, lambda)<br/>kinds inherit along is-a"] --> v
    wt["with_transitive(told, kinds)<br/>a kind reads along itself"] --> v
```

## Reading back

```mermaid
flowchart LR
    tally["tally(rows, pool, by_kind, a score per item)"]
    s1["score<br/>cos(F(unit(about + kind)), item)"]
    s2["score_inherit<br/>own + lambda · mean of the parents"]
    s3["score_laws<br/>predictions + mu · the converse reading"]
    s4["prior<br/>how often the kind named the item in told"]
    s5["two_hop<br/>predictions through told neighbours, each unit length"]
    rk["rank of the truth in the pool<br/>and within the kind's own answers · ties count half"]
    out["Score<br/>mrr · within · hit5 · per_kind"]
    ch["chance(n)<br/>MRR of a uniformly random ranking"]
    s1 & s2 & s3 & s4 & s5 --> tally
    tally --> rk --> out
    ch -.->|"the floor"| out
```

## The types

```mermaid
classDiagram
    class Triple {
        +usize about
        +usize kind
        +usize other
    }
    class Law {
        +usize k1
        +usize k2
        +usize support
        +usize agree
        +confidence() f64
    }
    class Inverse {
        +usize k
        +usize k_inv
        +usize support
        +usize agree
        +confidence() f64
    }
    class Inherit {
        +HashMap of kind to via kind via
        +HashMap of via kind and item to parents parents
        +f64 lambda
        +from_told(told, isa, kinds, lambda) Inherit
        +with_transitive(told, kinds) Inherit
        -add(told, via, kinds)
        +of(t) slice of usize
    }
    class Converse {
        +HashMap of kind to converse kind of
        +f64 mu
        +none() Converse
        +declared(pairs, mu) Converse
    }
    class Score {
        +f64 mrr
        +f64 within
        +f64 hit5
        +Vec of kind n mrr within per_kind
    }
    Law ..> Triple : counted over
    Inverse ..> Triple : implies
    Inherit ..> Triple : of
    Converse ..> Triple : read both ways
```
