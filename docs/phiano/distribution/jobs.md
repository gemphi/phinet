# Jobs — phiano's distribution

| State | Open | Done |
|---|---|---|
| 🟢 done | 0 | 16 |

## Done — level 1 (phiton, phical)

- [x] `bpe` → `phiton::p64::bpe`
- [x] `oscillator` sphere model (`Oscillator`) → `phiton::physics::oscillator` — `SphereView` reads the field, so it went to phiecs with it
- [x] `helix` code (`Codon`) → `phical::helix`

## Done — level 2 (phiecs)

- [x] `layers` → `phiecs::phasor::layers`
- [x] `attention` → `phiecs::phasor::attention`
- [x] `flow` → `phiecs::phasor::flow`, with `LEARNING_RATE`, `PHASE_REPULSION` and `SYNTACTIC_MOMENTUM_DEFAULT`
- [x] `helix` read and write → `phiecs::phasor::facet::helix`
- [x] `oscillator` field and trainer → `phiecs::phasor::oscillator`

## Done — level 3 (phient)

- [x] `kinds` → `phient::train::kinds`
- [x] `synthetic`, `curriculum`, `correction`, `envision` → `phient::train`
- [x] `synthesis`, then `lifelong` → `phient::train`
- [x] `oscillator` eval → `phient::eval::oscillator`

## The checks and the records

- [x] The workspace builds and every test count is conserved: 462 before, 462 after (phiano 69 → 36, phient 132 → 155, phiecs 124 → 131, phiton 27 → 30; the rest unchanged), 2026-10-28
- [x] Originals parked with INVENTORY rows
- [x] The module pages moved, each crate's ARCHITECTURE, README and TODO updated
- [x] phiano's ARCHITECTURE regenerated (the parent session's script): 19 modules, cycles unchanged (compose ↔ generate ↔ learn ↔ persona, command ↔ drivers), 2026-10-28
