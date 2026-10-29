# philog — the logic of Phient systems

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle; meaning is a side effect of pairs of words moving under a condition, and they move by the error of what was expected — toward what came, away from what did not.

Reasoning in phase space, taken out of phient (the brain orchestrates; the logic is its own crate, and its calculations belong in phical):

| module | what |
|---|---|
| `pathfinding` | `ReasoningEngine::solve`: the nearest unvisited phase per step, toward convergence; `ReasoningChain` |
| `paths` | `MultiPath`: solving at a depth, several paths, the best of them |
| `analogy`, `program` | analogy by phase offsets, and program analogy |
| `planning`, `abstraction`, `sorting`, `hybrid` | planning, abstraction, sorting and the hybrid reasoner |
| `interject` | the hypothetical: swap a word's phase and re-evaluate (`Counterfactual`) |
| `eval` | the evaluation of a chain: convergence, oscillation, divergence (`Diagnostic`) |
| `comparison` | a chain read against the facet, step by step |
| `chain` | `Chain<S>`: a problem, its steps, whether it converged, the answer |

Diagrams: [ARCHITECTURE.md](ARCHITECTURE.md). What is left to do: [TODO.md](TODO.md).
