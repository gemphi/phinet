//! The Structure_Check — the workspace's standing structural rules
//! (Requirement 11), held as tests rather than as a convention.
//!
//! This is the workspace root's one target: integration tests, not a crate.
//! It links none of the ten crates it polices and declares no dependency of
//! any kind, so it may name all ten freely and nothing can depend on it. Its
//! whole input is files — the ten `Cargo.toml` manifests and, for the rules
//! that land later, the source text under each crate's `src`.
//!
//! What lands here in task 2.1 is the permitted-edge table and the
//! outside-dependency allowlist; the line-count band, the arithmetic home
//! and the inventory rows join it later, each in its own file.
//!
//! The files: `manifest` reads a `Cargo.toml`, `pairs` enumerates the crate
//! pairs the rules are stated over, `edges` holds the rule and the three
//! tests pinning the tables to the manifests, `edges_named` the named
//! failures, `dev_edges` the direction rule for `[dev-dependencies]`, and
//! `edges_property` Property 58, and `frozen` the frozen operand set's
//! fingerprint, pinned in the same golden record.

mod dev_edges;
mod edges;
mod edges_named;
mod edges_property;
mod frozen;
mod manifest;
mod pairs;

/// The ten phinet crates, in dependency order: the order a level's tests
/// compile in, and the order the tables below are written in. The order is
/// load-bearing, not decorative — `is_below` reads a crate's position as its
/// height, and `dev_edges::the_crate_order_is_a_topological_order_of_the_permitted_graph`
/// holds the order to the permitted graph so it cannot drift out of step.
pub const CRATES: &[&str] = &[
    "phistd", "phifno", "phiton", "phical", "phiecs", "phiclm", "phient", "philog", "phisor",
    "phiano",
];

/// The permitted normal edges between phinet crates (R11.2), read from the
/// ten manifests and pinned here. A crate's list is what it may declare as a
/// normal dependency, not what it must: the rule permits these edges and
/// forbids every other one, so a dropped edge is not a violation.
///
/// Two amendments are coming and each must land in the same task as the
/// manifest line it describes — `phisor` on `phiclm` (the Lowering and the
/// Falsehood_Detector) and `phiano` on `phisor` (the Harness). Until then
/// this table is the manifests' true state, which
/// `the_permitted_table_is_the_manifests_true_state` holds it to.
pub const PERMITTED: &[(&str, &[&str])] = &[
    ("phistd", &[]),
    ("phifno", &["phistd"]),
    ("phiton", &["phistd"]),
    ("phical", &["phistd", "phifno"]),
    ("phiecs", &["phistd", "phiton", "phical"]),
    ("phiclm", &["phiecs"]),
    ("phient", &["phistd", "phical", "phiton", "phiecs", "phiclm"]),
    ("philog", &["phient", "phiecs", "phiton"]),
    ("phisor", &["phiecs", "phiton", "phical", "phistd", "phiclm"]),
    (
        "phiano",
        &["phistd", "phiton", "phical", "phifno", "phient", "philog", "phiecs", "phiclm"],
    ),
];

/// The two dev-only edges (R11.4): edges permitted as a dev-dependency and
/// as a dev-dependency alone. `phiecs` on `phient` is the workspace's one
/// standing cycle — phient depends on phiecs normally — which cargo permits
/// only as a dev-dependency. Either edge appearing as a normal dependency
/// fails as a promotion, naming both crates and the manifest path.
///
/// This table is not the list of permitted dev edges, and never was the
/// list of every declared one: it is the list of edges whose permission
/// stops at `[dev-dependencies]`. A test-only edge that runs *downward* is
/// permitted by the dependency order itself and belongs to no table here —
/// see `dev_permitted`.
pub const DEV_ONLY: &[(&str, &str)] = &[("phical", "phiton"), ("phiecs", "phient")];

/// The per-crate outside-dependency allowlist (R11.12), pinned as a golden
/// record so a novel outside dependency fails until it is recorded. It also
/// carries the frozen operand set's fingerprint (R7.6) on a comment line,
/// which `frozen` reads and the allowlist reader skips.
pub const ALLOWLIST_GOLDEN: &str = include_str!("allowlist.golden");

/// The golden record's path, for a failure that must say where to look.
pub const ALLOWLIST_PATH: &str = "phinet/tests/structural/allowlist.golden";

/// The workspace root — this target's manifest directory.
pub fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// One crate's manifest path, as a failure must name it.
pub fn manifest_path(crate_name: &str) -> std::path::PathBuf {
    workspace_root().join("crates").join(crate_name).join("Cargo.toml")
}

/// The crates a crate may declare as a normal dependency; empty for a name
/// the table does not hold, so an unknown crate may declare nothing.
pub fn permitted_for(crate_name: &str) -> &'static [&'static str] {
    PERMITTED.iter().find(|(c, _)| *c == crate_name).map(|(_, d)| *d).unwrap_or(&[])
}

/// Is this edge one of the two dev-only edges?
pub fn is_dev_only(from: &str, to: &str) -> bool {
    DEV_ONLY.iter().any(|(f, t)| *f == from && *t == to)
}

/// A crate's height: its position in `CRATES`, which is dependency order.
/// phistd is 0, the bottom; phiano is 9, the top. `None` for a name that is
/// not one of the ten.
pub fn height_of(crate_name: &str) -> Option<usize> {
    CRATES.iter().position(|c| *c == crate_name)
}

/// Is `lower` strictly below `higher` in dependency order? A name that is
/// not one of the ten is below nothing, so an unknown crate gains nothing
/// from this rule.
pub fn is_below(lower: &str, higher: &str) -> bool {
    match (height_of(lower), height_of(higher)) {
        (Some(l), Some(h)) => l < h,
        _ => false,
    }
}

/// Is this edge permitted as a `[dev-dependencies]` declaration?
///
/// Two ways, and the second is the general one. A `DEV_ONLY` pair is
/// permitted by name, because a normal declaration of it would close a
/// cycle. Any other edge is permitted exactly when it runs *downward* — the
/// depended-on crate is below the depending one in dependency order — and
/// is forbidden when it runs upward or sideways.
///
/// The reason a downward dev edge is no violation is that it adds no edge
/// to the graph R11.3 governs: cargo builds a dev-dependency only for that
/// crate's own test targets, so it cannot appear in any other crate's
/// build, and a downward edge cannot take part in a cycle because every
/// cycle needs at least one upward step. The live case is phiclm's judge
/// property test drawing its source texts from `phistd::gen`, the
/// workspace's one seeded generator; phical, phisor, phiano and the rest
/// need the same generator for the property tests the later tasks add, so
/// this is a rule about the direction of an edge rather than a list of the
/// crates that happen to have reached for it yet. Pinning each such edge by
/// name would make every new property test a structural amendment and buy
/// nothing: the direction is the whole of what makes it safe.
///
/// Note that every edge in `PERMITTED` is downward — the topological-order
/// test holds it so — hence a permitted normal edge is also permitted as a
/// dev one, and no separate case is needed for it.
pub fn dev_permitted(from: &str, to: &str) -> bool {
    is_dev_only(from, to) || is_below(to, from)
}

/// Is this name one of the ten phinet crates?
pub fn is_phinet_crate(name: &str) -> bool {
    CRATES.contains(&name)
}

// ---------------------------------------------------------------------------
// The seeded generator.
//
// `phistd::gen` is the workspace's generator, but this target links no crate
// it polices, so the same three lines — the mix, the per-case seed and the
// bounded draw — are written here over the same constants. A failure is
// reported as (seed, case) and replays from those two numbers alone.
// ---------------------------------------------------------------------------

/// phistd's mix: the one hash, written out rather than linked.
pub fn mix64(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// A case's draw sequence, seeded so case 412 reconstructs without running
/// cases 0 to 411.
pub struct Seeded(u64);

impl Seeded {
    pub fn for_case(seed: u64, case: u64) -> Self {
        Seeded(mix64(seed ^ case).max(1))
    }

    pub fn u64(&mut self) -> u64 {
        self.0 = mix64(self.0);
        self.0
    }

    /// A draw in `0..n`; `n == 0` draws 0, which no caller asks for.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.u64() % n as u64) as usize
    }

    pub fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.below(xs.len())]
    }
}

/// The case count every property test in the workspace runs at.
pub const CASES: usize = 256;

/// The run's declared seed. A failure names it beside the case.
pub const SEED: u64 = 0x5747_4f4c_4341_494c;
