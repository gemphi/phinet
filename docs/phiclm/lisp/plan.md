# phiclm · lisp — plan

**Why.** phiano keeps only apps. Its Lisp is a language and a machine, the kind of thing phiclm is (BCPL's architecture, its machine, its judge); only eight of its 47 primitives touch phiano's world. So the machine moves to phiclm, and phiano plugs its world in.

**Where things land.**

| from phiano | to | lines |
|---|---|---|
| `sys/lisp.rs` (Value, read) | `phiclm::lisp` (`src/lisp/mod.rs`) | 159 |
| `sys/lisp/eval.rs` (Machine) | `phiclm::lisp::eval`, made generic over a `Host`; the personas and scopes leave it | 174 |
| `sys/lisp/prims.rs` (47 primitives) | `phiclm::lisp::prims` keeps 39; the 8 that touch personas and scopes stay as phiano's host | 159 |
| — | `phiclm::lisp::host`: the `Host` trait, `()` as the bare host | new |
| `sys/set.rs`: `Relation`, `union`, `meet`, `compose`, `converse` | `phiclm::lisp::set` | ~30 of 91 |
| `sys/set/algebra.rs` | `phiclm::lisp::set::algebra` | 121 |
| `sys/set/tests.rs`: `principia_on_a_chain` | `phiclm::lisp::tests` | ~20 |
| `sys/lisp/tests.rs`: `reads_and_evaluates` | `phiclm::lisp::tests` (on the bare machine) | ~8 |

**What stays in phiano.** `sys::set`'s `held`, `Thought`, `think`, `render` (they read a `Being`), reading the algebra from phiclm. `sys::lisp` becomes the host: `Sys { beings, scopes }`, `impl Host for Sys` with the eight primitives, `pub type Machine = phiclm::lisp::Machine<Sys>`, and `with_beings` / `with_scopes` on a small trait so the example and the tests keep their shape. `sys::trace` reads `Relation`, `meet`, `union` from phiclm. The tests `russell_on_the_machine` and `stories_on_the_machine` stay with the host.

## Steps

1. **phiclm::lisp::set** — the pure algebra, its test; `cargo test -p phiclm`.
2. **phiclm::lisp** — `Value`, `read`, `Host`, `Machine<H>` (no `Being`, no `Scope`), the 39 core primitives and the helpers hosts read their arguments with; the bare-machine test.
3. **phiano** — `sys::set` keeps its four, reads the rest from phiclm; `sys::lisp` is the host; `sys::trace` repointed; the example's `m.scopes` reads `m.host.scopes`.
4. **Verify** — phiclm, phiano (lib, tests, examples, bins); the REPL smoke: `cargo run --example lisp -- -e "(count (closure (pairs (a b) (b c))))"` prints `3`.
5. **Record** — originals parked in `_temp/phinet-root` with an INVENTORY row; phiclm's ARCHITECTURE, README and TODO; phiano's sys page; the jobs ticked.

## Risks

- **A host name repeating a core name** would silently replace it. Refused at construction instead (see the spec).
- **`judge` in the core** makes the Lisp depend on the judge — phiclm's own, so no new dependency.
- **Name collision** — phiclm already has a `machine` module (BCPL's word machine); the Lisp's is `lisp::Machine`, always reached through `lisp::`.
- **Files over 200 lines** — none: the largest after the split is `eval.rs`, ~170.
