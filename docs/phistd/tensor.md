# tensor

## Files

```mermaid
graph TD
    mod["tensor/mod.rs<br/>Tensor: zeros · from_vec · randn · len · is_empty · idx · get · set · norm_sq<br/>pub use Adam · AdamState · Schedule · fft_inplace · fft2_inplace · MixtureTerms · Tape · Var"]
    tape["tape.rs<br/>Var · Op · Node · Tape<br/>new · push · param · constant · value · grad · set_value · truncate · any_grad"]
    subgraph ops_["ops/ — forward"]
        basic["basic.rs<br/>add · add_bias · mul · scale · linear · mode_mix<br/>fft · ifft · fft2 · ifft2 · gelu"]
        score["score.rs<br/>cosine · hinge · mean_real · tie · gather<br/>unit · cosine_table · mobius"]
        loss["loss.rs<br/>MixtureTerms · mixture_nll · cross_entropy"]
    end
    subgraph back_["backward/ — reverse"]
        bmod["backward.rs<br/>accumulate · backward · back_basic · clone_for_test"]
        bscore["score.rs<br/>back_score"]
        bloss["loss.rs<br/>back_loss"]
    end
    fft["fft.rs<br/>fft_inplace · fft2_inplace · fft_tensor · fft2_tensor<br/>gelu · gelu_prime"]
    squash["squash.rs<br/>shrink · shrink_slope · squash · squash_grad"]
    adam["adam.rs<br/>Adam · Schedule · AdamState"]
    tests["tests/<br/>fft · grad · loss · mul_tie · optim"]
    rayon["rayon · par_chunks_mut"]
    tape -->|"Tensor"| mod
    tape -->|"MixtureTerms"| loss
    basic -->|"Op · push"| tape
    score -->|"Op · push"| tape
    loss -->|"Op · push"| tape
    squash -->|"Op::Squash"| tape
    basic -->|"fft_tensor · fft2_tensor · gelu"| fft
    bmod -->|"fft_tensor · fft2_tensor · gelu_prime"| fft
    bmod -->|"squash_grad"| squash
    bmod -->|"other ops"| bscore
    bscore -->|"other ops"| bloss
    adam -->|"Tape · Var"| tape
    score --> rayon
    bscore --> rayon
    tests -.-> mod
```

## Shapes — every tensor is [batch, width, space]

```mermaid
graph LR
    subgraph moving["ops/basic.rs · squash.rs"]
        s1["add · mul · scale · gelu · squash<br/>same shape"]
        s2["add_bias · [b,w,s] + [1,w,1] → [b,w,s]<br/>linear · [b,i,s] × [i,o,1] → [b,o,s]<br/>mode_mix · [b,i,m] × [i,o,m] → [b,o,m]"]
        s3["fft · ifft · each row of s, unitary<br/>fft2 · ifft2 · each row as an s1 × s2 torus"]
    end
    subgraph reading["ops/score.rs"]
        r1["cosine · [b,1,s] and [b,1,s] → [b,1,1]<br/>hinge · [b,1,1] and [b,1,1] → [1,1,1]<br/>mean_real · [b,1,1] → [1,1,1]"]
        r2["gather · [V,1,s] by idx → [b,1,s]<br/>unit · each row to unit energy<br/>tie · [i,o,g] by group → [i,o,len of group]"]
        r3["cosine_table · [b,1,s] and [V,1,s] → [b,V,1]<br/>mobius · x and abcd [1,1,4] → shape of x"]
    end
    subgraph losses["ops/loss.rs"]
        l1["cross_entropy · [b,V,1], targets, β → [1,1,1]<br/>mixture_nll · [b,V,1], MixtureTerms → [1,1,1]"]
    end
    r3 --> l1
```

## Recording on the tape

```mermaid
sequenceDiagram
    participant U as caller
    participant T as Tape
    participant N as nodes
    U->>T: param(t) or constant(t)
    T->>N: push a Leaf, requires_grad true or false
    T-->>U: Var(index)
    U->>T: an op on Vars, for one linear(x, w)
    T->>T: output Tensor from the input values · any_grad(inputs)
    T->>N: push value, Op::Linear(x, w), requires_grad
    T-->>U: Var(index)
    U->>T: truncate(keep)
    T->>N: drop every node after keep, clear every grad
```

## backward(out) — and which file holds which Op

```mermaid
flowchart TD
    B["backward(out)"] --> C["clear every grad"] --> A{"out is [1,1,1]"}
    A -->|"no"| P["assert fails"]
    A -->|"yes"| G["grad of out = 1"]
    G --> L["node i, from out down to 0"]
    L --> Q{"has a grad and requires_grad"}
    Q -->|"no"| L
    Q -->|"yes"| BB["back_basic · backward.rs<br/>Leaf · Add · AddBias · Mul · Scale · Linear · ModeMix<br/>Fft · Ifft · Fft2 · Ifft2 · Gelu · Squash"]
    BB -->|"handled"| AC["accumulate into the inputs<br/>skipped where requires_grad is false"]
    BB -->|"Some(op, g)"| BS["back_score · backward/score.rs<br/>Cosine · Hinge · MeanReal · Tie · Gather<br/>Unit · CosineTable · Mobius"]
    BS -->|"handled"| AC
    BS -->|"other"| BL["back_loss · backward/loss.rs<br/>CrossEntropy · MixtureNll"]
    BL -->|"handled"| AC
    BL -->|"anything else"| UR["unreachable: every op has a backward"]
    AC --> L
```

## The gradient convention

```mermaid
flowchart LR
    G["g = ∂L/∂Re z + i · ∂L/∂Im z"] --> M["y = x · w<br/>g_x = g_y · conj w"]
    G --> U["y = U x, U unitary · fft<br/>g_x = U⁻¹ g_y, the inverse transform"]
    G --> R["gelu on Re and Im apart<br/>each part on its own"]
    G --> H["mobius, holomorphic<br/>g_z = g · conj(∂w/∂z)"]
    G --> S["squash<br/>real 2×2 Jacobian on (Re, Im)"]
```

## squash and cosine_table

```mermaid
flowchart LR
    Z["z"] --> R["r = |z|"] --> S{"r #lt; 1e-6"}
    S -->|"yes"| A["s = 1 − r²/3"]
    S -->|"no"| B["s = tanh r / r"]
    A --> O["squash · z · s<br/>phase kept · modulus tanh r"]
    B --> O
    CT{"cosine_table"} -->|"forward"| CF["par_chunks_mut over batch rows"]
    CT -->|"pred requires_grad"| BP["backward pass 1 · par over prediction rows"]
    CT -->|"table requires_grad"| BT["backward pass 2 · par over table rows"]
    CT -->|"table constant"| SK["table pass skipped"]
```

## Adam::step and Schedule::factor

```mermaid
flowchart TD
    S["step(tape, params)"] --> N{"m.len == params.len"}
    N -->|"no"| Z["m, v zeroed · t = 0"]
    N -->|"yes"| T["t += 1"]
    Z --> T
    T --> LR["lr = lr · schedule.factor(t)"]
    LR --> P["each param"]
    P --> G{"grad on the tape"}
    G -->|"none"| P
    G -->|"some"| WD{"weight_decay #gt; 0"}
    WD -->|"yes · AdamW"| D["value times 1 − lr · weight_decay"]
    WD -->|"no"| U["m, v on Re and Im apart<br/>bias-corrected by 1 − β1^t and 1 − β2^t"]
    D --> U
    U --> V["value −= lr · m̂ / (√v̂ + eps), per part · tape.set_value"]
    F["factor(step)"] --> C1["Constant → 1"]
    F --> C2["Cosine · warmup · total · floor"]
    C2 -->|"warmup #gt; 0 and step ≤ warmup"| W["step / warmup"]
    C2 -->|"total ≤ warmup or step ≥ total"| FL["floor"]
    C2 -->|"else"| CO["floor + (1 − floor) · ½(1 + cos πp)"]
    F --> C3["Step · every · gamma → gamma^(step / every)"]
    LR -.-> F
```

## The types

```mermaid
classDiagram
    class Tensor {
        +shape : three usize
        +data : Vec of c64
        +zeros(shape) · from_vec(shape, data) · randn(shape, scale, seed)
        +get(b, w, s) · set(b, w, s, v) · norm_sq()
    }
    class Node {
        value · grad · op · requires_grad
    }
    class Tape {
        nodes : Vec of Node
        +param(t) · constant(t) · value(v) · grad(v)
        +truncate(keep) · backward(out)
    }
    class Op {
        Leaf · Add · AddBias · Mul · Scale · Linear · ModeMix
        Fft · Ifft · Fft2 · Ifft2 · Gelu · Squash · Cosine · Hinge
        MeanReal · Gather · Unit · CosineTable · MixtureNll · Mobius · CrossEntropy · Tie
    }
    class MixtureTerms {
        +targets · k · coef · gamma · other · beta
    }
    class Adam {
        +lr · beta1 · beta2 · eps · weight_decay · schedule
        +new(lr) · with_schedule · with_decay · current_lr · steps
        +state() · from_state(s) · step(tape, params)
    }
    class Schedule {
        Constant · Cosine · Step
        +factor(step) f64
    }
    class AdamState {
        +lr · beta1 · beta2 · eps · weight_decay · schedule · m · v · t
    }
    Tape *-- Node
    Node --> Tensor
    Node --> Op
    Tape ..> Var : hands out a usize index
    Op --> MixtureTerms
    Adam --> Schedule
    AdamState --> Schedule
    Adam ..> AdamState : state and from_state
    Adam ..> Tape : reads grads and sets values
```
