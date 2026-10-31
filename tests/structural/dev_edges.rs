//! The direction rule for `[dev-dependencies]` (R11.3, R11.4).
//!
//! A normal edge must be in `PERMITTED` by name: it ships in every build
//! that reaches the crate, so it is part of the architecture and worth an
//! amendment. A dev edge ships in nothing but that crate's own test
//! targets, and what makes one safe or unsafe is its direction:
//!
//! * **Downward is permitted.** The depended-on crate is below the
//!   depending one in dependency order, so the edge cannot close a cycle —
//!   every cycle needs at least one upward step — and it adds no edge to
//!   any other crate's build. phiclm's judge property test drawing source
//!   texts from `phistd::gen` is the live case; phical, phisor and phiano
//!   reach for the same generator as the later property tests land. Each is
//!   the same shape, so the rule is stated once over the direction rather
//!   than pinned once per crate.
//! * **Upward fails by name.** An upward dev edge is a cycle waiting for a
//!   second declaration, and cargo resolves it into the same graph, so
//!   `[dev-dependencies]` buys it nothing.
//! * **One named exception**, `phiecs` on `phient`: the workspace's
//!   standing cycle, which cargo permits as a dev-dependency alone.
//!
//! The rule rests on `CRATES` being a topological order of `PERMITTED`, so
//! that is the first test here.

use crate::edges::{at, check, golden, report, Violation};
use crate::manifest::{ten_manifests, Section};
use crate::pairs;

/// `CRATES` is dependency order, and `is_below` reads a crate's position in
/// it as a height. That only means anything while every permitted normal
/// edge runs downward in the same order — were a permitted edge to run
/// upward, "below" would stop matching "depends on" and the dev rule would
/// admit an edge that really does close a cycle. So the two tables are held
/// to each other here rather than assumed to agree.
#[test]
fn the_crate_order_is_a_topological_order_of_the_permitted_graph() {
    for (from, to) in pairs::permitted() {
        assert!(
            crate::is_below(to, from),
            "PERMITTED holds {from} on {to}, but {to} is not below {from} in CRATES: either the \
             edge is wrong or CRATES is no longer dependency order"
        );
    }
    for (from, to) in crate::DEV_ONLY {
        assert!(
            !crate::permitted_for(from).contains(to),
            "{from} on {to} is in DEV_ONLY and in PERMITTED, so the promotion rule can never fire"
        );
    }
}

/// The real ten pass, and phiclm's test-only edge on phistd is the reason
/// this file exists: it is downward, unpinned, and no violation.
#[test]
fn phiclms_test_only_edge_on_phistd_is_permitted() {
    let ms = ten_manifests();
    let phiclm = ms.iter().find(|m| m.crate_name == "phiclm").expect("phiclm");
    assert!(
        phiclm.edges(Section::Dev).contains(&"phistd"),
        "phiclm no longer declares phistd as a dev-dependency, so this test guards nothing"
    );
    assert!(crate::is_below("phistd", "phiclm"), "phistd must be below phiclm");
    let found = check(&ms, &golden());
    assert!(found.is_empty(), "the ten manifests must pass, got:{}", report(&found));
}

/// Every downward test-only edge is permitted, not phiclm's alone: the four
/// crates the later property tests give a `phistd::gen` dev edge pass the
/// same way, and so does every other downward pair.
#[test]
fn a_downward_dev_dependency_is_permitted() {
    for (from, to) in pairs::downward_test_only() {
        let mut ms = ten_manifests();
        at(&mut ms, from).declare(to, Section::Dev);
        let found = check(&ms, &golden());
        assert!(
            found.is_empty(),
            "{from}'s test-only edge on {to} runs downward and must pass, got:{}",
            report(&found)
        );
    }
    for from in ["phical", "phisor", "phiano", "phiecs"] {
        assert!(crate::is_below("phistd", from), "phistd must be below {from}");
    }
}

/// R11.3: an upward dev edge fails, naming the depending crate, the
/// depended-on crate and the manifest path. This is the test that keeps the
/// rule from being "a dev edge is always fine".
#[test]
fn an_upward_dev_dependency_fails_by_name() {
    for (from, to) in pairs::upward() {
        let mut ms = ten_manifests();
        at(&mut ms, from).declare(to, Section::Dev);
        let found = check(&ms, &golden());
        assert_eq!(
            found,
            vec![Violation::ForbiddenEdge {
                from: from.to_string(),
                to: to.to_string(),
                section: Section::Dev,
                path: crate::manifest_path(from),
            }],
            "{from}'s dev edge on {to} runs upward and must fail"
        );
        let said = found[0].message();
        for part in [from, to, "Cargo.toml", "[dev-dependencies]"] {
            assert!(said.contains(part), "{said} omits {part}");
        }
    }
}

/// The three upward dev edges a reader is most likely to try, spelled out:
/// the bottom crate reaching for the top, a middle crate reaching for the
/// one above it, and a crate reaching for itself.
#[test]
fn the_named_upward_dev_edges_fail() {
    for (from, to) in [("phistd", "phiano"), ("phiclm", "phient"), ("phical", "phical")] {
        assert!(pairs::upward().contains(&(from, to)), "{from} on {to} must count as upward");
        let mut ms = ten_manifests();
        at(&mut ms, from).declare(to, Section::Dev);
        assert_eq!(check(&ms, &golden()).len(), 1, "{from}'s dev edge on {to} must fail once");
    }
}

/// `phiecs` on `phient` is upward and still permitted, because it is the
/// declared standing cycle — the one dev edge the direction rule defers to
/// `DEV_ONLY` over. It is already declared in phiecs's manifest, so the
/// test asserts the shape and that the ten pass with it in place.
#[test]
fn the_standing_cycle_is_the_one_permitted_upward_dev_edge() {
    assert!(!crate::is_below("phient", "phiecs"), "phiecs on phient must be an upward edge");
    assert!(crate::dev_permitted("phiecs", "phient"), "the standing cycle must be permitted");
    assert!(
        !pairs::upward().contains(&("phiecs", "phient")),
        "the standing cycle must be the upward set's one exclusion"
    );
    let upward_permitted: Vec<(&str, &str)> = crate::CRATES
        .iter()
        .flat_map(|from| crate::CRATES.iter().map(move |to| (*from, *to)))
        .filter(|(from, to)| !crate::is_below(to, from) && crate::dev_permitted(from, to))
        .collect();
    assert_eq!(
        upward_permitted,
        vec![("phiecs", "phient")],
        "exactly one upward dev edge is permitted, and it is the declared standing cycle"
    );
}
