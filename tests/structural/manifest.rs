//! Reading a `Cargo.toml` as the Structure_Check needs it: the crate's name,
//! the manifest's path and every declared dependency with the section it sat
//! in. Nothing else is read, and no parser crate is linked — R11.12 forbids
//! this target an outside dependency as firmly as it forbids the ten crates
//! one, and the manifests it reads are line-per-declaration TOML.

use std::collections::BTreeMap;
use std::path::PathBuf;

/// Which dependency section a declaration sat in. `[build-dependencies]`
/// counts as normal: it ships in the build, so an edge there is an edge.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Section {
    Normal,
    Dev,
}

impl Section {
    pub fn name(self) -> &'static str {
        match self {
            Section::Normal => "[dependencies]",
            Section::Dev => "[dev-dependencies]",
        }
    }
}

/// One crate's manifest, reduced to what the dependency rules read.
#[derive(Clone, Debug)]
pub struct Manifest {
    pub crate_name: String,
    pub path: PathBuf,
    /// Every declared dependency, in declaration order, with its section.
    pub deps: Vec<(String, Section)>,
}

impl Manifest {
    /// Read one of the ten crates' manifests from disk.
    pub fn read(crate_name: &str) -> Manifest {
        let path = crate::manifest_path(crate_name);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is unreadable: {e}", path.display()));
        Manifest::parse(crate_name, path, &text)
    }

    pub fn parse(crate_name: &str, path: PathBuf, text: &str) -> Manifest {
        let mut deps = Vec::new();
        let mut section = None;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some(header) = line.strip_prefix('[') {
                let header = header.trim_end_matches(']').trim_start_matches('[');
                assert!(
                    !header.contains("dependencies."),
                    "{}: the dependency sub-table `[{header}]` is past this reader; \
                     extend tests/structural/manifest.rs rather than let an edge hide",
                    path.display()
                );
                section = section_of(header);
                continue;
            }
            let Some(section) = section else { continue };
            if let Some(name) = key_of(line) {
                deps.push((name, section));
            }
        }
        Manifest { crate_name: crate_name.to_string(), path, deps }
    }

    /// The declarations in one section.
    pub fn in_section(&self, want: Section) -> Vec<&str> {
        self.deps.iter().filter(|(_, s)| *s == want).map(|(d, _)| d.as_str()).collect()
    }

    /// The phinet crates this crate declares in one section — its edges.
    pub fn edges(&self, want: Section) -> Vec<&str> {
        self.in_section(want).into_iter().filter(|d| crate::is_phinet_crate(d)).collect()
    }

    /// Every outside crate this crate declares, in either section, sorted.
    pub fn outside(&self) -> Vec<&str> {
        let mut out: Vec<&str> = self
            .deps
            .iter()
            .map(|(d, _)| d.as_str())
            .filter(|d| !crate::is_phinet_crate(d))
            .collect();
        out.sort();
        out.dedup();
        out
    }

    /// Add a declaration. Used by Property 58 to perturb a graph; the ten
    /// real manifests are never written, only read.
    pub fn declare(&mut self, dep: &str, section: Section) {
        self.deps.push((dep.to_string(), section));
    }

    /// Remove a declaration, if it is there.
    pub fn undeclare(&mut self, dep: &str, section: Section) {
        self.deps.retain(|(d, s)| !(d == dep && *s == section));
    }
}

/// The section a header names, or `None` for a header that holds no
/// dependency. `[target.'cfg(unix)'.dependencies]` ends in the section name,
/// so the last dotted segment decides.
fn section_of(header: &str) -> Option<Section> {
    let last = header.rsplit('.').next().unwrap_or("").trim_matches(|c| c == '\'' || c == '"');
    match last {
        "dependencies" | "build-dependencies" => Some(Section::Normal),
        "dev-dependencies" => Some(Section::Dev),
        _ => None,
    }
}

/// The dependency a line declares: the key left of the first `=`, with any
/// dotted tail (`serde.workspace = true`) and any quoting removed.
fn key_of(line: &str) -> Option<String> {
    let key = line.split('=').next()?.trim();
    let key = key.split('.').next()?.trim().trim_matches('"');
    if key.is_empty() {
        None
    } else {
        Some(key.to_string())
    }
}

/// The per-crate outside-dependency allowlist: a crate's name against the
/// outside crates it may declare.
pub type Allowlist = BTreeMap<String, Vec<String>>;

/// Read the golden record. One line per crate, `<crate> = <dep>, <dep>`, an
/// empty right-hand side meaning the crate declares no outside dependency.
pub fn allowlist(text: &str) -> Allowlist {
    let mut out = Allowlist::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let (name, rest) = line
            .split_once('=')
            .unwrap_or_else(|| panic!("{}: `{line}` is not `<crate> = <deps>`", crate::ALLOWLIST_PATH));
        let deps: Vec<String> =
            rest.split(',').map(str::trim).filter(|d| !d.is_empty()).map(str::to_string).collect();
        out.insert(name.trim().to_string(), deps);
    }
    out
}

/// The ten manifests as they stand on disk.
pub fn ten_manifests() -> Vec<Manifest> {
    crate::CRATES.iter().map(|c| Manifest::read(c)).collect()
}
