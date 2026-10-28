# entity · node

`entity` is level 0 of phiecs; `node` holds levels 1 and 2. `Entity`,
`Names`, `Component`, `Dense`, `Graph`, `Edge`, `Kind` and `Kinds` are the
ECS names of `Word`, `Lexicon`, `Aspect`, `Dense`, `relation::Graph`,
`Edge`, `Relation` and `Register`; `Tree` and `State` are new.

## The levels

```mermaid
graph BT
    subgraph l0["0 · identity"]
        ent["entity::Entity · Names<br/>= word::Word · Lexicon"]
    end
    subgraph l1["1 · data"]
        comp["node::component<br/>Component = Aspect · Dense"]
        stt["node::state<br/>State: now and next, commit"]
    end
    subgraph l2["2 · structure"]
        tree["node::tree<br/>Tree: one parent, no loops"]
        graph_["node::graph<br/>Graph · Edge · Kind · Kinds"]
    end
    subgraph l3["3 · the world"]
        world["world · system · query"]
    end
    subgraph l4["4 · domains"]
        lang["phasor · facet · wave<br/>a language"]
        kit["kit<br/>bodies, devices, the eye"]
        sim["a simulator<br/>planned, MuJoCo-like"]
    end
    comp --> ent
    stt --> ent
    tree --> ent
    graph_ --> ent
    world --> comp & graph_
    lang --> world
    kit --> world
    sim -.-> tree & stt & comp
```

## Tree

```mermaid
flowchart LR
    a["attach(child, parent)"] --> q{"parent is child,<br/>or stands under it"}
    q -->|"yes"| no["false · nothing changed"]
    q -->|"no"| d["detach(child) from its old parent"]
    d --> s["parent[child] = parent<br/>children[parent] += child"]
    s --> yes["true · its subtree comes with it"]
    r["reads"] --> p["parent · children"]
    r --> an["ancestors: nearest first, to the root"]
    r --> de["descendants: depth first, attach order"]
    r --> ro["roots: children and no parent"]
```

## State

```mermaid
sequenceDiagram
    participant S as a system
    participant St as State
    S->>St: init(e, v0): now and next
    S->>St: get(e): now, v0
    S->>St: set(e, v1): next only
    S->>St: get(e): still v0
    S->>St: commit(): now = next, step += 1
    S->>St: get(e): v1
    Note over St: an entity not written in a step keeps its state
```
