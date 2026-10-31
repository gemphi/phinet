//! The dependency rules: the permitted edges between phinet crates (R11.1,
//! R11.2, R11.3), the two dev-only edges (R11.4) and the per-crate
//! outside-dependency allowlist (R11.12). Every failure names the depending
//! crate, the depended-on crate and the manifest path.

use crate::manifest::{allowlist, ten_manifests, Allowlist, Manifest, Section};
use std::path::PathBuf;

/// One broken rule, named as the criteria require.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Violation {
    ForbiddenEdge { from: String, to: String, section: Section, path: PathBuf },
    DevOnlyPromoted { from: String, to: String, path: PathBuf },
    OutsideDependency { from: String, dep: String, section: Section, path: PathBuf },
}

impl Violation {
    pub fn message(&self) -> String {
        match self {
            Violation::ForbiddenEdge { from, to, section: Section::Normal, path } => format!(
                "{from} declares {to} in [dependencies], an edge the permitted table does not \
                 hold for {from}: {}",
                path.display()
            ),
            Violation::ForbiddenEdge { from, to, section: Section::Dev, path } => format!(
                "{from} declares {to} in [dev-dependencies], an upward edge: {to} is not below \
                 {from} in the dependency order, and {from} on {to} is not a declared dev-only \
                 edge: {}",
                path.display()
            ),
            Violation::DevOnlyPromoted { from, to, path } => format!(
                "{from} declares {to} in [dependencies], but {from} on {to} is permitted as a \
                 dev-dependency alone: {}",
                path.display()
            ),
            Violation::OutsideDependency { from, dep, section, path } => format!(
                "{from} declares the outside crate {dep} in {}, which {from}'s allowlist in {} \
                 does not hold: {}",
                section.name(),
                crate::ALLOWLIST_PATH,
                path.display()
            ),
        }
    }
}

/// The rule. Passes exactly when every normal edge between phinet crates is
/// permitted, the two dev-only edges appear only as dev-dependencies, every
/// dev edge runs downward or is one of those two, and every outside
/// dependency is in that crate's pinned allowlist.
///
/// The two sections are judged by two different rules on purpose. A normal
/// edge must be in `PERMITTED` by name, because it ships in every build
/// that reaches the crate and so is part of the architecture. A dev edge
/// ships in nothing but that crate's own test targets, so what it has to
/// answer for is direction alone: `crate::dev_permitted` admits it when it
/// runs downward and refuses it when it runs upward, the one named
/// exception being a `DEV_ONLY` pair.
///
/// Against the table this replaces for dev edges, the admitted set grows by
/// exactly the downward edges no table names — phiclm on phistd and the
/// ones like it — and the refused set loses nothing that can do harm, since
/// a downward edge cannot take part in a cycle. What the rule refuses it
/// now refuses on a structural ground rather than on absence from a list,
/// which is why the refusal holds for edges nobody has thought to write
/// down.
pub fn check(manifests: &[Manifest], allow: &Allowlist) -> Vec<Violation> {
    let mut found = Vec::new();
    for m in manifests {
        let from = &m.crate_name;
        for (dep, section) in &m.deps {
            if !crate::is_phinet_crate(dep) {
                let held = allow.get(from).is_some_and(|l| l.iter().any(|a| a == dep));
                if !held {
                    found.push(Violation::OutsideDependency {
                        from: from.clone(),
                        dep: dep.clone(),
                        section: *section,
                        path: m.path.clone(),
                    });
                }
                continue;
            }
            let dev_only = crate::is_dev_only(from, dep);
            let permitted = crate::permitted_for(from).contains(&dep.as_str());
            match section {
                Section::Normal if dev_only => found.push(Violation::DevOnlyPromoted {
                    from: from.clone(),
                    to: dep.clone(),
                    path: m.path.clone(),
                }),
                Section::Normal if !permitted => found.push(Violation::ForbiddenEdge {
                    from: from.clone(),
                    to: dep.clone(),
                    section: Section::Normal,
                    path: m.path.clone(),
                }),
                Section::Dev if !crate::dev_permitted(from, dep) => {
                    found.push(Violation::ForbiddenEdge {
                        from: from.clone(),
                        to: dep.clone(),
                        section: Section::Dev,
                        path: m.path.clone(),
                    })
                }
                _ => {}
            }
        }
    }
    found.sort();
    found.dedup();
    found
}

pub fn report(found: &[Violation]) -> String {
    found.iter().map(|v| format!("\n  {}", v.message())).collect()
}

pub fn golden() -> Allowlist {
    allowlist(crate::ALLOWLIST_GOLDEN)
}

// --- The ten manifests as they stand ---------------------------------------

#[test]
fn the_ten_manifests_break_no_dependency_rule() {
    let found = check(&ten_manifests(), &golden());
    assert!(found.is_empty(), "the Structure_Check found {} violations:{}", found.len(), report(&found));
}

/// `PERMITTED` is held to the manifests exactly: a normal edge landing in
/// any of the ten manifests without the table being amended in the same
/// task fails here, by name, which is what tasks 8.1 and 16.1 lean on.
///
/// `DEV_ONLY` is held to the manifests in the one direction that is its
/// meaning: both pairs it names must really be declared as
/// dev-dependencies, so the table cannot name an edge that has gone away.
/// It is deliberately *not* held to be every declared dev edge. A test-only
/// downward edge — phiclm on phistd for the judge's generator, and the same
/// edge on phical, phisor and phiano as the later property tests land — is
/// permitted by the dependency order, so pinning it here would turn each
/// new property test into a structural amendment. What every dev edge is
/// held to instead is its direction.
#[test]
fn the_permitted_table_is_the_manifests_true_state() {
    let mut dev_only_declared = Vec::new();
    for m in ten_manifests() {
        let mut declared: Vec<&str> = m.edges(Section::Normal);
        declared.sort();
        let mut pinned: Vec<&str> = crate::permitted_for(&m.crate_name).to_vec();
        pinned.sort();
        let (name, path) = (&m.crate_name, m.path.display());
        assert_eq!(declared, pinned, "{name}'s normal phinet edges differ from PERMITTED: {path}");
        for d in m.edges(Section::Dev) {
            assert!(
                crate::dev_permitted(name, d),
                "{name}'s dev edge on {d} runs upward and is no declared dev-only edge: {path}"
            );
            if crate::is_dev_only(name, d) {
                dev_only_declared.push((name.clone(), d.to_string()));
            }
        }
    }
    dev_only_declared.sort();
    let mut pinned: Vec<(String, String)> =
        crate::DEV_ONLY.iter().map(|(f, t)| (f.to_string(), t.to_string())).collect();
    pinned.sort();
    assert_eq!(dev_only_declared, pinned, "DEV_ONLY names an edge no manifest declares as a dev-dependency");
}

#[test]
fn the_allowlist_golden_is_the_manifests_true_outside_state() {
    let golden = golden();
    let mut names: Vec<&str> = golden.keys().map(String::as_str).collect();
    names.sort();
    let mut crates: Vec<&str> = crate::CRATES.to_vec();
    crates.sort();
    assert_eq!(names, crates, "{} must name the ten crates", crate::ALLOWLIST_PATH);
    for m in ten_manifests() {
        let outside = m.outside();
        let pinned: Vec<&str> = golden[&m.crate_name].iter().map(String::as_str).collect();
        let name = &m.crate_name;
        assert_eq!(outside, pinned, "{name}'s outside dependencies differ from the golden record");
    }
}

/// One of the ten, by name, for a test that perturbs a read copy. The ten
/// real manifests are only ever read; the copy is what gets written.
pub fn at<'a>(ms: &'a mut [Manifest], name: &str) -> &'a mut Manifest {
    ms.iter_mut().find(|m| m.crate_name == name).expect("one of the ten crates")
}
