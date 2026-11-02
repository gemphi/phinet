//! The per-level compile-closure check (R11.11) — the structural-side mirror
//! of phiano's `level::compiled_for`.
//!
//! Running one level's tests (`cargo test -p <level>`) compiles that level's
//! own crate and the transitive closure of its dependencies in the
//! permitted-edge graph, nothing above it, and reports the count of crates
//! compiled. phiano's Harness owns the heavy path — it shells out to cargo —
//! so what this target owns is the claim that path rests on: the closure the
//! permitted-edge graph *defines*. This file computes that closure from the
//! structural `PERMITTED` table, the source of truth, and holds it to the
//! dependency-order expectation exhaustively over all ten levels.
//!
//! The two copies — this one and phiano's `level::PERMITTED` — are held equal
//! by phiano's `the_level_graph_matches_the_structural_table`, so a drift in
//! the harness's copy fails there, not here. Here, `PERMITTED` is read
//! straight from `crate::PERMITTED` through `crate::permitted_for`, so there
//! is no second copy to drift.

/// The crates `cargo test -p <level>` compiles: the transitive closure of the
/// level's dependencies in the permitted-edge graph together with the level's
/// own crate, in dependency order (R11.11). The result is `CRATES` order
/// restricted to the closure — every crate before any crate that depends on
/// it, so the level itself is last. A name that is not one of the ten closes
/// to nothing, naming nothing to compile.
pub fn compiled_for(level: &str) -> Vec<&'static str> {
    if crate::height_of(level).is_none() {
        return Vec::new();
    }
    let mut reached: Vec<&'static str> = Vec::new();
    let mut stack: Vec<&'static str> =
        vec![crate::CRATES.iter().copied().find(|c| *c == level).unwrap()];
    while let Some(c) = stack.pop() {
        if reached.contains(&c) {
            continue;
        }
        reached.push(c);
        for &dep in crate::permitted_for(c) {
            if !reached.contains(&dep) {
                stack.push(dep);
            }
        }
    }
    // Dependency order: a crate before anything that depends on it. `CRATES`
    // is dependency order (phistd 0, phiano 9) and the graph is acyclic, so
    // ordering by `height_of` is a topological order, and the level — which
    // depends on every other member of its own closure — lands last.
    reached.sort_by_key(|c| crate::height_of(c).unwrap_or(usize::MAX));
    reached
}

/// An independent transitive closure over `PERMITTED`: `level` and everything
/// it reaches, as a sorted unordered set. The oracle `compiled_for` is held
/// to, written as a plain reachability fixpoint so it shares no code with the
/// stack walk above.
fn reach(level: &'static str) -> Vec<&'static str> {
    let mut set: Vec<&'static str> = vec![level];
    let mut changed = true;
    while changed {
        changed = false;
        for c in set.clone() {
            for &dep in crate::permitted_for(c) {
                if !set.contains(&dep) {
                    set.push(dep);
                    changed = true;
                }
            }
        }
    }
    set.sort_unstable();
    set
}

// --- The ten levels, exhaustively ------------------------------------------

/// **Validates: Requirements 11.11**
///
/// For all ten levels: the closure the permitted-edge graph defines contains
/// the level's own crate, holds only crates below it in dependency order,
/// compiles nothing above it, is in dependency order, agrees with the
/// reachability oracle, and the reported count equals the set's size.
#[test]
fn a_level_compiles_its_crate_and_only_the_crates_below_it() {
    for (i, &level) in crate::CRATES.iter().enumerate() {
        let got = compiled_for(level);

        // No crate is compiled twice, so the count is a set count.
        let mut sorted = got.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), got.len(), "{level}: the closure holds a crate twice");

        // The level's own crate is in the closure.
        assert!(got.contains(&level), "{level}: its own crate is absent from the closure");

        // Nothing strictly above the level in dependency order is compiled,
        // and every other member is strictly below it.
        for &other in &got {
            if other != level {
                assert!(
                    crate::is_below(other, level),
                    "{level}: {other} is in the closure yet is not below it"
                );
            }
        }
        for &higher in &crate::CRATES[i + 1..] {
            assert!(!got.contains(&higher), "{level}: {higher} is above it yet was compiled");
        }

        // The closure agrees with the independent reachability oracle.
        assert_eq!(sorted, reach(level), "{level}: the closure disagreed with the oracle");

        // Dependency order: each crate's height never decreases along the
        // result, and the level — which closes over all its deps — lands last.
        let mut last: Option<usize> = None;
        for &c in &got {
            let h = crate::height_of(c).unwrap();
            if let Some(p) = last {
                assert!(h > p, "{level}: {c} is out of dependency order");
            }
            last = Some(h);
        }
        assert_eq!(*got.last().unwrap(), level, "{level}: the level itself is not last");

        // The reported count is the size of the compiled set (R11.11).
        assert_eq!(got.len(), reach(level).len(), "{level}: the reported count is not the set size");
    }
}

// --- The two ends and the unaccepted name ----------------------------------

#[test]
fn the_bottom_compiles_alone_and_the_top_pulls_the_stack() {
    // phistd depends on nothing, so its run compiles one crate: itself.
    assert_eq!(compiled_for("phistd"), vec!["phistd"], "phistd compiles itself alone");
    assert_eq!(compiled_for("phistd").len(), 1, "phistd's compiled count is one");

    // phiano sits at the top and closes over all ten.
    let all = compiled_for("phiano");
    assert_eq!(all.len(), crate::CRATES.len(), "phiano closes over all ten crates");
    assert_eq!(*all.last().unwrap(), "phiano", "the level itself is last");
    for &name in crate::CRATES {
        assert!(all.contains(&name), "phiano's closure omits {name}");
    }
}

#[test]
fn an_unaccepted_name_compiles_nothing() {
    assert!(compiled_for("nope").is_empty(), "an unknown crate compiles nothing");
    assert!(compiled_for("PHISTD").is_empty(), "the match is case-exact");
    assert!(compiled_for(" phistd ").is_empty(), "the match rejects padding");
}
