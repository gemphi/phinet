//! The crate pairs the dependency rules are stated over, enumerated from
//! `CRATES`, `PERMITTED` and `DEV_ONLY` rather than written out a second
//! time. A test that needs "an edge the rule must refuse" draws from these,
//! so it cannot accidentally draw an edge the rule permits, and a table
//! amendment moves the tests with it instead of leaving them behind.
//!
//! The four sets are disjoint in the way the rules need: `permitted` and
//! `forbidden_normal` partition the normal edges between the ten, and
//! `downward_test_only` and `upward` are the two directions a dev edge can
//! take once the pairs the tables already name are set aside.

type Pair = (&'static str, &'static str);

/// Every normal edge `PERMITTED` holds, as pairs.
pub fn permitted() -> Vec<Pair> {
    crate::CRATES
        .iter()
        .flat_map(|from| crate::permitted_for(from).iter().map(move |to| (*from, *to)))
        .collect()
}

/// Every edge between two of the ten that is neither permitted as a normal
/// edge nor named in `DEV_ONLY`: the edges a `[dependencies]` line must not
/// hold. Includes each crate on itself, which cargo would reject anyway.
pub fn forbidden_normal() -> Vec<Pair> {
    cross(|from, to| !crate::permitted_for(from).contains(&to) && !crate::is_dev_only(from, to))
}

/// Every downward edge that no table names: the shape a test-only
/// dependency takes, phiclm on phistd being the live one. Downward and
/// unpinned is exactly the case the rule has to admit on direction alone,
/// so pairs already in `PERMITTED` or `DEV_ONLY` are set aside — they would
/// pass for a reason that is not the one under test.
pub fn downward_test_only() -> Vec<Pair> {
    cross(|from, to| {
        crate::is_below(to, from)
            && !crate::permitted_for(from).contains(&to)
            && !crate::is_dev_only(from, to)
    })
}

/// Every edge that runs upward — the depended-on crate at or above the
/// depending one — bar the declared standing cycle. These must fail in
/// either section: upward is how a cycle is built, and `[dev-dependencies]`
/// buys an upward edge nothing, since cargo resolves it into the same
/// graph.
pub fn upward() -> Vec<Pair> {
    cross(|from, to| !crate::is_below(to, from) && !crate::is_dev_only(from, to))
}

/// Every ordered pair of the ten that holds, in `CRATES` order.
fn cross(holds: impl Fn(&'static str, &'static str) -> bool) -> Vec<Pair> {
    let mut out = Vec::new();
    for from in crate::CRATES {
        for to in crate::CRATES {
            if holds(from, to) {
                out.push((*from, *to));
            }
        }
    }
    out
}
