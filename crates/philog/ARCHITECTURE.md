# philog — architecture

## The modules and what they use

```mermaid
graph TB
    subgraph crate["philog"]
        m_abstraction["abstraction"]
        m_analogy["analogy"]
        m_chain["chain"]
        m_comparison["comparison"]
        m_eval["eval"]
        m_hybrid["hybrid"]
        m_interject["interject"]
        m_pathfinding["pathfinding"]
        m_paths["paths"]
        m_planning["planning"]
        m_program["program"]
        m_sorting["sorting"]
    end
    m_analogy --> m_program
    m_comparison --> m_eval
    m_comparison --> m_hybrid
    m_comparison --> m_pathfinding
    m_eval --> m_pathfinding
    m_hybrid --> m_analogy
    m_hybrid --> m_pathfinding
    m_paths --> m_eval
    m_paths --> m_pathfinding
    X_phiecs["phiecs (crate)"]
    m_abstraction -.-> X_phiecs
    m_analogy -.-> X_phiecs
    m_comparison -.-> X_phiecs
    m_eval -.-> X_phiecs
    m_hybrid -.-> X_phiecs
    m_interject -.-> X_phiecs
    m_pathfinding -.-> X_phiecs
    m_paths -.-> X_phiecs
    m_planning -.-> X_phiecs
    m_program -.-> X_phiecs
    m_sorting -.-> X_phiecs
    X_phient["phient (crate)"]
    m_interject -.-> X_phient
    m_pathfinding -.-> X_phient
    m_paths -.-> X_phient
    m_planning -.-> X_phient
    X_phiton["phiton (crate)"]
    m_abstraction -.-> X_phiton
    m_analogy -.-> X_phiton
    m_hybrid -.-> X_phiton
    m_interject -.-> X_phiton
    m_pathfinding -.-> X_phiton
    m_paths -.-> X_phiton
    m_planning -.-> X_phiton
    m_program -.-> X_phiton
    m_sorting -.-> X_phiton
```

## The crate in one diagram

- [reasoning](../../docs/philog/reasoning.md): the engines, the chain, and what reads them

What is left to do: [TODO.md](TODO.md)
