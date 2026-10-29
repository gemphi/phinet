# phiclm · lisp — spec

A small Lisp over the sets (docs/165): McCarthy's machine on Russell's algebra. An S-expression is a list, a row is `(kind about other)`, a relation is a set of ordered pairs. Nothing infers: the machine computes over what it was told, and only a host's `tell` changes a world.

**Every language operation is a lambda underneath** (the user's ruling, 2026-10-29). Whatever a language above this one does (phisor's operations, a host's verbs), it lowers to a `lambda` applied to its arguments, closed over the frames it needs, so the core and hosts compose it like any other value.

## Values

| value | written | what it is |
|---|---|---|
| `Nil` | `()` | the empty list, and false |
| `Bool` | `#t` `#f` | |
| `Num` | `3`, `2.5` | an `f64` |
| `Str` | `"scope:text/emma"` | text, as written between quotes |
| `Sym` | `russell`, `is-a` | a name; **unbound, it stands for itself**, so `(held russell is-a)` reads as a row does |
| `List` | `(a b c)` | |
| `Relation` | `#pairs[n]{…}` | a set of `(about, other)` pairs (`lisp::set::Relation`) |
| `Class` | `#class[n]{…}` | a set of words (`lisp::set::Class`) |
| `Lambda` | `#lambda(x)` | parameters, a body, the frames it closed over |
| `Prim` | `#prim:union` | a primitive, core or the host's |

Truth: everything but `()` and `#f` is true.

## Reading

Tokens are `(`, `)`, `'`, a `"…"` string, or a run of anything else; `;` comments to the end of the line. `'x` reads as `(quote x)`. A run that parses as a number is a `Num`, otherwise a `Sym`. `()` reads as `Nil`. Errors: `unexpected end`, `missing )`, `unexpected )`.

## Special forms

| form | does |
|---|---|
| `(quote x)` | `x`, unevaluated |
| `(if c then [else])` | `then` when `c` is true, else `else` or `()` |
| `(define name expr)` | binds in the innermost frame, else the global one; returns the name |
| `(lambda (params) body…)` | a closure over the current frames; the body is a `begin` |
| `(let ((name expr)…) body…)` | a new frame for the body |
| `(begin form…)` | each in turn; the last value |
| `(pairs (a b)…)` · `(class a b…)` | a `Relation` or a `Class` from their arguments **as written** |
| `(and form…)` · `(or form…)` | short-circuit; `#t` / `#f` when all pass |

## Core primitives (39)

| group | primitives |
|---|---|
| relations | `union` `meet` `difference` `compose` `converse` `closure` `ancestral` `(power r n)` |
| classes from relations | `domain` `range` `field` `(image r class)` `(relata r x)` `(referents r y)` `(restrict-domain r class)` `(restrict-range r class)` |
| relations from classes | `(product a b)` `(identity a)` |
| tests | `included?` `transitive?` `symmetric?` `reflexive?` |
| constructors | `pairs` `class` `members` (a class as a list of strings) `count` (of a relation, class or list) |
| lists | `list` `car` `cdr` `cons` `length` |
| numbers | `=` `<` `+` `-` `*` |
| text | `str` (joins texts) `print` (to stdout; returns `()`) |
| the judge | `(judge "BCPL source")`: compiles and runs it on phiclm's machine (1,000,000 steps), the verdict as rows `((about kind other)…)` |

A list of two-item lists is read as pairs wherever a relation is expected, and a list of words as a class.

## The host

```rust
pub trait Host {
    /// The primitives this host adds.
    fn names(&self) -> &'static [&'static str];
    /// One of them, called with its arguments evaluated.
    fn call(&mut self, name: &'static str, args: Vec<Value>) -> Result<Value, String>;
}
```

`Machine::with_host(host)` binds the 39 core names, then the host's; a host name that repeats a core name is refused (a panic at construction — a host that shadows `union` is a bug, not a choice). `()` is the host of the bare machine. `Machine::new()` exists for any host that is `Default`. A host reads its arguments with the same helpers the core primitives use: `as_relation`, `as_class`, `as_num`, `as_text`, `arg`.

## phiano's host (`phiano::sys::lisp::Sys`)

| primitive | does |
|---|---|
| `(scopes)` | the stories' scope paths |
| `(next "scope:…")` | the scope's next-word pairs, as a relation |
| `(says "scope:…")` | the words said in the scope, as a class |
| `(beings)` | the personas' slugs; a `who` below matches a slug or any hyphen-part of it (`russell` finds `bertrand-russell`) |
| `(held who kind)` | every pair `who` holds under `kind` |
| `(ask who word kind)` | `who`'s readings of `word` under `kind`: `((other weight)…)` |
| `(tell who relation kind)` | tells `who` every pair as a row under `kind`; the number told |
| `(told who)` | how many rows `who` has been told |

## Errors

Every failure is an `Err(String)` returned by `run`: `unknown primitive x`, a wrong argument (`not a pair: …`, `not a number: …`, `union: argument 2 missing`), `not a function: …`, `no scope …`, and from the host `held: who?` or `held: no persona x`. Nothing panics on input.
