//! **Property 58: The dependency-edge rule admits exactly the permitted
//! graph.**
//!
//! For any manifest graph, the rule passes exactly when every normal edge
//! between phinet crates is in the permitted set, the two dev-only edges
//! appear only as dev-dependencies, and every outside dependency is in the
//! pinned per-crate allowlist; and every failure names the depending crate,
//! the depended-on crate and the manifest path.
//!
//! The generator perturbs the permitted graph one change at a time — a
//! forbidden edge, a dev-only edge promoted to normal, a reversed edge, a
//! downward test-only dev edge, an upward dev edge, and an outside
//! dependency inside and outside the allowlist — so the expected violation
//! is known by construction rather than recomputed by a second copy of the
//! rule. Two of those shapes are expected to produce *no* violation: a
//! downward dev edge, which the dependency order permits, and an outside
//! dependency the allowlist holds. A rule that refuses what it should
//! admit fails this property as surely as one that admits what it should
//! refuse. The ten real manifests are one further case. Cases: 256, plus
//! the real ten. Oracle: the change applied.
//!
//! **Validates: Requirements 11.1, 11.2, 11.3, 11.4, 11.12**

use crate::edges::{check, report, Violation};
use crate::manifest::{Allowlist, Manifest, Section};
use crate::{pairs, Seeded};

/// What one case did to the graph, so a failure says which shape broke.
#[derive(Debug, Clone, Copy, PartialEq)]
#[rustfmt::skip]
enum Change {
    Nothing, ForbiddenEdge, DevOnlyPromoted, Reversed, DevDownward, DevUpward,
    OutsideInsideAllowlist, OutsideOutsideAllowlist,
}

#[rustfmt::skip]
const CHANGES: &[Change] = &[
    Change::Nothing, Change::ForbiddenEdge, Change::DevOnlyPromoted, Change::Reversed,
    Change::DevDownward, Change::DevUpward, Change::OutsideInsideAllowlist,
    Change::OutsideOutsideAllowlist,
];

/// The permitted graph as manifests: every permitted normal edge declared,
/// both dev-only edges declared as dev-dependencies, and one outside crate
/// per crate so the allowlist has something to admit.
fn permitted_graph() -> (Vec<Manifest>, Allowlist) {
    let mut graph = Vec::new();
    let mut allow = Allowlist::new();
    for name in crate::CRATES {
        let path = crate::manifest_path(name);
        let mut m = Manifest { crate_name: name.to_string(), path, deps: Vec::new() };
        for dep in crate::permitted_for(name) {
            m.declare(dep, Section::Normal);
        }
        for (from, to) in crate::DEV_ONLY {
            if from == name {
                m.declare(to, Section::Dev);
            }
        }
        m.declare("serde", Section::Normal);
        graph.push(m);
        allow.insert(name.to_string(), vec!["serde".to_string()]);
    }
    (graph, allow)
}

fn at<'a>(graph: &'a mut [Manifest], name: &str) -> &'a mut Manifest {
    graph.iter_mut().find(|m| m.crate_name == name).expect("one of the ten crates")
}

/// Apply one change and return the violations it must produce.
fn perturb(
    change: Change,
    rng: &mut Seeded,
    graph: &mut [Manifest],
    allow: &mut Allowlist,
) -> Vec<Violation> {
    match change {
        Change::Nothing => Vec::new(),
        Change::ForbiddenEdge => {
            let (from, to) = *rng.pick(&pairs::forbidden_normal());
            at(graph, from).declare(to, Section::Normal);
            vec![Violation::ForbiddenEdge {
                from: from.to_string(),
                to: to.to_string(),
                section: Section::Normal,
                path: crate::manifest_path(from),
            }]
        }
        Change::DevOnlyPromoted => {
            let (from, to) = *rng.pick(crate::DEV_ONLY);
            at(graph, from).declare(to, Section::Normal);
            vec![Violation::DevOnlyPromoted {
                from: from.to_string(),
                to: to.to_string(),
                path: crate::manifest_path(from),
            }]
        }
        Change::Reversed => {
            let (from, to) = *rng.pick(&pairs::permitted());
            at(graph, from).undeclare(to, Section::Normal);
            at(graph, to).declare(from, Section::Normal);
            assert!(
                !crate::permitted_for(to).contains(&from),
                "{to} on {from} is permitted too, so the permitted graph holds a cycle"
            );
            // The one reversal that lands on a dev-only edge: phient on
            // phiecs reversed is phiecs on phient, the standing cycle.
            if crate::is_dev_only(to, from) {
                vec![Violation::DevOnlyPromoted {
                    from: to.to_string(),
                    to: from.to_string(),
                    path: crate::manifest_path(to),
                }]
            } else {
                vec![Violation::ForbiddenEdge {
                    from: to.to_string(),
                    to: from.to_string(),
                    section: Section::Normal,
                    path: crate::manifest_path(to),
                }]
            }
        }
        // A test-only edge running downward: phiclm on phistd is the live
        // one, and the later property tests add the same shape to phical,
        // phisor and phiano. It is permitted on its direction alone, so no
        // table names it and the expected violation set is empty.
        Change::DevDownward => {
            let (from, to) = *rng.pick(&pairs::downward_test_only());
            at(graph, from).declare(to, Section::Dev);
            Vec::new()
        }
        // The same declaration pointing the other way: a cycle waiting for
        // a second line, which `[dev-dependencies]` excuses for the
        // standing cycle alone. `pairs::upward` sets that one aside, so
        // every pair drawn here must fail.
        Change::DevUpward => {
            let (from, to) = *rng.pick(&pairs::upward());
            at(graph, from).declare(to, Section::Dev);
            vec![Violation::ForbiddenEdge {
                from: from.to_string(),
                to: to.to_string(),
                section: Section::Dev,
                path: crate::manifest_path(from),
            }]
        }
        Change::OutsideInsideAllowlist | Change::OutsideOutsideAllowlist => {
            let from = *rng.pick(crate::CRATES);
            let dep = format!("outside-{}", rng.below(1000));
            let section = if rng.below(2) == 0 { Section::Normal } else { Section::Dev };
            at(graph, from).declare(&dep, section);
            if change == Change::OutsideInsideAllowlist {
                allow.get_mut(from).expect("one of the ten crates").push(dep);
                Vec::new()
            } else {
                vec![Violation::OutsideDependency {
                    from: from.to_string(),
                    dep,
                    section,
                    path: crate::manifest_path(from),
                }]
            }
        }
    }
}

#[test]
fn property_58_the_dependency_edge_rule_admits_exactly_the_permitted_graph() {
    let (clean, clean_allow) = permitted_graph();
    let found = check(&clean, &clean_allow);
    assert!(found.is_empty(), "the permitted graph itself must pass, got:{}", report(&found));

    let mut seen = Vec::new();
    for case in 0..crate::CASES as u64 {
        let mut rng = Seeded::for_case(crate::SEED, case);
        let change = *rng.pick(CHANGES);
        seen.push(change);
        let (mut graph, mut allow) = permitted_graph();
        let mut expected = perturb(change, &mut rng, &mut graph, &mut allow);
        expected.sort();
        expected.dedup();
        let found = check(&graph, &allow);
        assert_eq!(
            found,
            expected,
            "seed {:#x}, case {case}, change {change:?}: the rule admitted the wrong set, \
             got:{}\nexpected:{}",
            crate::SEED,
            report(&found),
            report(&expected)
        );
        for v in &found {
            let said = v.message();
            assert!(said.contains("Cargo.toml"), "case {case}: {said} omits the manifest path");
        }
    }

    // Every shape drawn, or the property tested less than it claims to.
    for change in CHANGES {
        assert!(seen.contains(change), "seed {:#x} drew no {change:?} case", crate::SEED);
    }

    // The real ten manifests as one further case.
    let real = check(&crate::manifest::ten_manifests(), &crate::edges::golden());
    assert!(real.is_empty(), "the real ten manifests must pass, got:{}", report(&real));
}
