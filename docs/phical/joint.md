# joint

## Files

```mermaid
graph LR
    mod_["joint/mod.rs<br/>IndexExample · Joint<br/>new · with_tether · settle · context_on_tape · train_epoch · functions"]
    tests["tests.rs<br/>1 test function"]
    fno["phifno · operator::Fno<br/>attach_param · params · reset · forward · mask_modes · decoy · tape · config"]
    std_["phistd<br/>tensor::Adam · Tensor · Var · mix64 · Function · c64"]
    relate["relate/<br/>example · closure · inherit · structure · score"]
    mod_ --> fno
    mod_ --> std_
    tests -.-> mod_
    relate -->|"Joint · IndexExample"| mod_
```

## The table on the tape — Tensor [v + 1, 1, space], fno.attach_param

```mermaid
block-beta
    columns 4
    r0["row 0<br/>table[0]"] r1["row 1 …"] rl["row v − 1<br/>table[v − 1]"] pad["row v · pad<br/>all zero, re-zeroed after every step"]
```

## A batch's context, built on the tape

```mermaid
flowchart LR
    ex["IndexExample<br/>ctx: (item, weight) pairs · target"] --> j{"for j in 0..window"}
    j --> idx["per example: its j-th most recent item<br/>or pad when ctx is shorter"]
    j --> wt["per example: the weight of its j-th most recent item<br/>0 at the pad · one constant row each"]
    idx --> ga["tape.gather(table, idx)"] --> sc["tape.mul(g, weights)"]
    wt --> sc
    sc --> acc["tape.add into the sum"] --> un["tape.unit(sum)<br/>the batch's input x"]
```

## One batch of train_epoch

```mermaid
sequenceDiagram
    participant E as train_epoch
    participant J as Joint
    participant F as Fno and its tape
    participant A as adam_op and adam_table
    E->>E: order by mix64(i ^ seed), chunks of batch
    E->>F: reset
    E->>J: context_on_tape(batch)
    J->>F: gather · mul by each example's weights · add · unit
    E->>F: gather the table at targets, and at decoys from Fno::decoy
    E->>F: forward(x), cosine to target, cosine to decoy, hinge(margin)
    E->>F: backward(loss)
    E->>A: adam_op on fno.params, adam_table on the table
    E->>F: mask_modes
    E->>J: settle
    E->>E: mean loss over batches
```

## settle

```mermaid
flowchart LR
    v["tape.value(table)"] --> z["pad row set to zero"] --> t{"tether above zero"}
    t -->|"yes"| pull["every z += (z0 − z) · tether<br/>z0 from init"] --> set["tape.set_value(table)"]
    t -->|"no"| set
```

## Reading it back

```mermaid
flowchart LR
    tab["tape.value(table)"] --> rows["rows 0..pad"] --> fs["functions()<br/>Vec of Function, in table order"]
```

## The types

```mermaid
classDiagram
    class Joint {
        +Fno fno
        +Var table
        -usize pad
        -usize window
        -Tensor init
        +f64 tether
        +new(fno, table, window ≥ 1) Joint
        +with_tether(tether) Joint
        -settle()
        -context_on_tape(batch) Var
        +train_epoch(examples, pool, batch, adam_op, adam_table, seed) f64
        +functions() Vec of Function
    }
    class IndexExample {
        +Vec of item-weight pairs ctx
        +usize target
    }
    class Fno {
        phifno operator
    }
    Joint --> Fno : owns
    Joint ..> IndexExample : trains on
```
