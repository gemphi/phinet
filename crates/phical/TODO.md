# phical — TODO

```mermaid
graph LR
    subgraph open_["open"]
        O1["nothing open found in the code"]
    end
    subgraph done_["done"]
        D1["README · Cargo.toml · lib.rs doc · the real modules, phistd, Moment"]
        D2["joint/ · window ≥ 1 asserted in new · each example's own weights"]
        D3["q64.rs · under a hundredth, measured 0.0099"]
        D4["examples/torch.rs · phistd's torch folder"]
    end
```

## lib.rs
- nothing open found in the code

## conjugate/
- [x] The phase-conjugate mirror moved to `phiton::conjugate`, beside `chiton`; its decoy draw is `phistd::decoy`.

## joint/
- [x] `context_on_tape` reads each example's own weight (a constant row per example, `Tape::mul`); identical to before for today's position-only schedules, tested with two that differ.
- [x] `Joint::new` asserts `window ≥ 1` with a message, before any epoch.

## liskov.rs
- nothing open found in the code

## q64.rs
- [x] The q64 test is `…_is_under_a_hundredth` and asserts `< 0.01`: measured 0.0099, the module doc's figure.

## relate/
- nothing open found in the code

## README.md · Cargo.toml
- [x] Module table: `sphere`, `mobius`, `winding` removed; `q64`, `heat` (phifno's) and the crate root's re-exports listed; Cargo.toml's description to match.
- [x] The example imports `phical::unit_random`; phistd, not phinet, is named as the engine.
- [x] `Moments` is `Moment`.
- [x] The examples named as `examples/` holds them: `apply`, `bench`, `filter`, `ranges`, `softmax`, `torch`.
- [x] lib.rs's crate doc names phistd as the engine.

## examples/
- [x] `torch.rs` points to `crates/phistd/torch/phinet_torch.py` and runs `-p phical --example torch` from crates/phistd.

## Moved in (2026-10-28)

- [x] helix: phiano's helix code (`Codon`) is `helix` (docs/phiano/distribution).
