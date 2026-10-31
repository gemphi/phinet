//! The named failures: for each rule in `edges::check`, one perturbation of
//! the ten real manifests that must fail, and must say which crate, which
//! depended-on crate and which manifest path (R11.1, R11.4, R11.12).
//!
//! Each test perturbs a read copy, so the manifests on disk are untouched,
//! and each asserts the whole violation set rather than a membership: a
//! violation these tests do not ask for is as much a failure as a missing
//! one, which is how a rule that has grown too loose or too keen shows up.

use crate::edges::{at, check, golden, report, Violation};
use crate::manifest::{ten_manifests, Section};

/// R11.1: phisor on phient, philog or phiano fails, naming all three parts.
#[test]
fn phisor_declaring_phient_philog_or_phiano_fails_by_name() {
    for dep in ["phient", "philog", "phiano"] {
        let mut ms = ten_manifests();
        at(&mut ms, "phisor").declare(dep, Section::Normal);
        let found = check(&ms, &golden());
        assert_eq!(found.len(), 1, "phisor on {dep} must fail once, got:{}", report(&found));
        let said = found[0].message();
        for part in ["phisor", dep, "crates\\phisor\\Cargo.toml"] {
            assert!(
                said.contains(part) || said.contains(&part.replace('\\', "/")),
                "{said} omits {part}"
            );
        }
    }
}

/// R11.4: either dev-only edge as a normal dependency fails, naming both
/// crates and the manifest path — and fails as a `DevOnlyPromoted`, not as
/// a plain forbidden edge, because the reason is that its permission stops
/// at `[dev-dependencies]` rather than that the edge is unheard of.
///
/// This is the test the downward-dev-edge rule must not weaken, and does
/// not: `dev_permitted` says what may appear in `[dev-dependencies]`, and
/// says nothing about `[dependencies]`, which `PERMITTED` alone governs.
/// phical on phiton is downward and still fails here when promoted.
#[test]
fn a_dev_only_edge_promoted_to_normal_fails_by_name() {
    for (from, to) in crate::DEV_ONLY {
        let mut ms = ten_manifests();
        at(&mut ms, from).declare(to, Section::Normal);
        let found = check(&ms, &golden());
        assert_eq!(
            found,
            vec![Violation::DevOnlyPromoted {
                from: from.to_string(),
                to: to.to_string(),
                path: crate::manifest_path(from),
            }],
            "{from} on {to} promoted to normal must fail as a promotion"
        );
        let said = found[0].message();
        assert!(said.contains(from) && said.contains(to), "{said} omits a crate");
    }
}

/// The allowlist is per crate: phiano's server and parser crates pass where
/// the same names on phistd do not.
#[test]
fn the_allowlist_is_per_crate_not_workspace_wide() {
    let golden = golden();
    for dep in ["axum", "tokio", "reqwest", "syn"] {
        assert!(golden["phiano"].iter().any(|d| d == dep), "phiano's allowlist omits {dep}");
        let mut ms = ten_manifests();
        at(&mut ms, "phistd").declare(dep, Section::Normal);
        let found = check(&ms, &golden);
        assert_eq!(found.len(), 1, "phistd on {dep} must fail once, got:{}", report(&found));
        assert!(found[0].message().contains(dep), "the failure omits {dep}");
    }
}
