# Jobs — `phiclm::lisp`

What the Lisp does, each job with the test that holds it. Moved from phiano on 2026-10-28; every job passes in its new home.

| State | Open | Done |
|---|---|---|
| 🟢 done | 0 | 12 |

## Done — the machine (phiclm)

- [x] **Read** S-expressions: lists, quote, strings, `#t`/`#f`, numbers, symbols, `;` comments; `unexpected end`, `missing )`, `unexpected )` as errors (`lisp::tests::reads_and_evaluates`)
- [x] **Evaluate** symbols (an unbound symbol is the word itself), lists, and the special forms `quote if define lambda let begin and or` (`reads_and_evaluates`)
- [x] **Closures**: `lambda` closes over its frames; `define` binds in the innermost frame (`reads_and_evaluates`: `(define sq (lambda (x) (* x x))) (sq 7)` → 49)
- [x] **Relations** as values: `pairs`, the relation algebra (`union meet difference compose converse closure ancestral power`), the classes it yields (`domain range field image relata referents restrict-*`), `product identity`, and the four tests (`lisp::tests::principia_on_a_chain`)
- [x] **Lists and numbers**: `list car cdr cons length = < + - *`, `str`, `print` (`reads_and_evaluates`)
- [x] **The judge**: `(judge "BCPL")` compiles and runs BCPL on phiclm's machine, the verdict as rows (`lisp::tests::the_judge_runs_bcpl`: `LET start() = VALOF RESULTIS 6 * 7` returns 42)
- [x] **The host seam**: `Machine::with_host` binds a host's primitives after the core's, refuses one that repeats a core name, and routes calls by name; `()` is the bare machine (`lisp::tests::a_host_adds_its_primitives`, `a_host_may_not_shadow_the_core`)

## Done — phiano's host

- [x] **Personas**: `held ask tell told beings` on `Being`s keyed by slug (`sys::lisp::tests::russell_on_the_machine`)
- [x] **Stories**: `scopes next says` on the scopes `trace::scopes_of` builds (`sys::lisp::tests::stories_on_the_machine`)
- [x] **Two beings think together**: `sys::set::{held, think, render}` read the algebra from phiclm (`sys::set::tests::relations_are_sets_of_pairs`)
- [x] **The REPL**: `examples/lisp` with `--text`, `--bcpl`, `--personas`, `-e` (smoke, 2026-10-28: `(count (closure (pairs (a b) (b c))))` printed `3`; `(judge …)` printed its verdict rows)

## Done — the records

- [x] Originals parked with an INVENTORY row; phiclm's ARCHITECTURE links this folder; phiano's `docs/phiano/sys.md` shows the host, not the machine.
