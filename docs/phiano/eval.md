# eval

```mermaid
graph LR
    Text["text"]
    Tok["tokenizer::Tokenizer::tokenize"]
    FacetN["facet::Facet<br/>contains_word, centroid, vocabulary_size"]
    WaveN["wave::Wave<br/>sentence, text_bound"]
    MemoN["memory::Memo::novelty"]
    Config["config<br/>NOVELTY_SCALE, EVAL_WEIGHT_COHERENCE,<br/>EVAL_WEIGHT_RESONANCE, PhiConfig::eval_overall"]
    Users["compose::better, lifelong, metrics,<br/>oscillator, persona, server, command"]

    subgraph eval["phiano::eval"]
        Types["eval.rs<br/>Eval, Verdict, Evaluator"]
        EvalFn["verdict.rs<br/>Evaluator::eval"]
        Resonance["resonance<br/>known / tokens"]
        Coherence["coherence<br/>norm of wave / known"]
        Novelty["novelty<br/>angle to centroid, NaN if degenerate"]
        Combined["overall<br/>weighted, renormalised on NaN"]
        FromScores["verdict.rs<br/>Verdict::from_scores"]
        WithMemory["verdict.rs<br/>Evaluator::eval_with_memory"]
        EvalOut["Eval<br/>coherence, novelty, resonance,<br/>overall, verdict"]
    end

    Types --> EvalFn
    Text --> EvalFn
    Tok --> EvalFn
    EvalFn --> Resonance
    FacetN --> Resonance
    EvalFn --> Coherence
    WaveN --> Coherence
    EvalFn --> Novelty
    FacetN -->|"centroid"| Novelty
    Coherence --> Combined
    Resonance --> Combined
    Novelty --> Combined
    Config --> Combined
    Coherence --> FromScores
    Resonance --> FromScores
    Novelty --> FromScores
    Combined --> EvalOut
    FromScores --> EvalOut
    WithMemory -->|"base Eval"| EvalFn
    WaveN -->|"text_bound"| WithMemory
    MemoN -->|"novelty from experience"| WithMemory
    WithMemory --> EvalOut
    EvalOut --> Users
```
