# machine · ocode

## Files

```mermaid
graph LR
    mod["machine/mod.rs<br/>MEMORY · GLOBAL · Machine · Stream · load · outputs · word · set"]
    run["run.rs<br/>run · step: fetch, do, until start() returns"]
    callrs["call.rs<br/>call · back: frames in the store"]
    co["co.rs<br/>control · transfer · finished"]
    context["context.rs<br/>Context · Co · Flow"]
    arith["arith.rs<br/>unary · binary"]
    heap["heap.rs<br/>getvec · freevec"]
    ocode["ocode.rs<br/>Op · Program"]
    blib["global/ · native · control"]
    mod --> run --> arith
    run --> callrs --> blib
    callrs --> co --> heap
    context -.-> run & callrs & co
    blib --> heap
    ocode --> run
    arith -.-> fold["trn/fold.rs folds with the same arithmetic"]
```

## The store — 65,536 words

```mermaid
block-beta
    columns 1
    g0["0 – 42 · BLIB natives: negative entries"]
    g1["43 · result2"]
    g2["44 – 398 · the program's globals, functions, STATICs  (libhdr ug = 200 is slot 244)"]
    g3["399 · where the stack starts"]
    g4["400 … · strings as length then characters · TABLEs"]
    g5["… ↑ · frames, one per call"]
    g6["free"]
    g7["↓ … 65,535 · getvec's heap, from the top down"]
```

## A frame at P

```mermaid
block-beta
    columns 6
    p0["P+0<br/>saved P"] p1["P+1<br/>saved PC"] p2["P+2 …<br/>parameters"] p3["locals"] p4["VECs"] p5["hidden slots<br/>FOR limits · SWITCHON value"]
```

## A call

```mermaid
sequenceDiagram
    participant C as caller
    participant M as machine
    participant F as callee
    C->>M: push entry, push arguments, Call(args, frame)
    alt entry #lt; 0
        M->>M: native BLIB routine, result pushed
    else entry #gt; 0
        M->>F: new P = P + frame · saved P, saved PC, arguments stored
        F->>M: Fnrn (value) or Rtrn (0)
        M->>C: P and PC restored
    end
```

## The 55 ops

```mermaid
mindmap
  root((Op))
    load and store
      Num Str Table
      Lp Sp Llp
      Lg Sg
      Rv Stind Pop
      Getbyte Putbyte
    monads
      Neg Not Abs
      Float Fix Fneg Fabs
    dyads
      Add Sub Mul Div Rem
      And Or Lsh Rsh Eqv Neqv
      Eq Ne Ls Gr Le Ge
      Fadd Fsub Fmul Fdiv
      Feq Fne Fls Fgr Fle Fge
    control
      Jump Jf Jt Goto
      Call Fnrn Rtrn Finish
```

## Faults are verdicts

```mermaid
flowchart LR
    F{"fault"} --> D["division by zero · REM by zero"]
    F --> S["read or write outside the store"]
    F --> H["the stack ran into the heap"]
    F --> G["GOTO to no label"]
    F --> O["out of steps"]
    D & S & H & G & O --> V["Err in the Verdict, never a panic"]
```

## Threads — the main program and its coroutines

```mermaid
stateDiagram-v2
    [*] --> Fresh: createco(body, size) · a getvec block
    Fresh --> Running: callco or resumeco(c, v) · body(v) on its own stack
    Running --> Waiting: cowait(v) · v to its parent
    Running --> Waiting: callco or resumeco of another
    Waiting --> Running: callco or resumeco(c, v) · v as its call's result
    Running --> Fresh: body returns v · v to its parent
    Fresh --> [*]: deleteco(c) · freevec
```

```mermaid
flowchart LR
    T["transfer(target, v)"] --> K["the running Context saved in its Co"]
    K --> F{"target saved?"}
    F -->|"yes"| R["its Context back · v pushed as the result of its call"]
    F -->|"no, fresh"| S["frame at its block · OUTSIDE_CO links · v as the argument · pc = body"]
```

## level · longjump

```mermaid
flowchart LR
    L["level()"] --> P["the caller's P"]
    J["longjump(lev, lab)"] --> U{"innermost frame is lev?"}
    U -->|"no"| POP["drop that frame"] --> U
    U -->|"yes"| CUT["expression stack cut to the depth lev was entered at"] --> GO["P = lev · pc = lab"]
    U -->|"no frames left"| E["Err: no live frame"]
```
