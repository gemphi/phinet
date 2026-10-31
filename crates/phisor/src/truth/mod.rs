//! The Falsehood_Detector: how a claim stands before a row is told from it.
//!
//! A claim out of an oracle is text, not knowledge. Before [`phiecs::World`]
//! is told a row, the detector reads the claim against the Laws the kinds are
//! declared to obey and classifies it: unreadable, degenerate, contradicting,
//! unsupported or consistent — in that order, stopping at the first test that
//! holds, so the five classes are mutually exclusive and jointly exhaustive
//! and a claim qualifying for two takes the earlier one (R8.1).
//!
//! This file is the classifier core: the degenerate, contradicting,
//! unsupported and consistent tests, over the triples the World already
//! holds. The judge-backed program path ([`program`], R8.6/R8.13) and the
//! per-class scoring ([`score`], R8.8/R8.9) sit in sibling files; the
//! novelty split ([`novelty`], R8.11) in another. `Reading::verdict` is
//! populated by the program path and stays `None` here.
//!
//! phiecs declares the class names and the [`Checker`] trait, so [`Detector`]
//! reaches the quest's tell point through that trait and phiecs gains no edge
//! for it. The Law reading is phical's ([`phical::relate`]): a functional or
//! one-to-one kind already holding a different other, a declared converse
//! held with a different other, or an inherit edge closing a cycle, over the
//! closure of the relations already held.

use phical::relate::Law;
use phiecs::quest::{Checker, Claim, Degeneracy, Standing};
use phiecs::world::World;

pub mod program;
mod record;
pub mod score;
pub use record::{Cap, VerdictRecord};

/// The Laws a kind is declared to obey, against which a claim is read.
///
/// phical's `relate::{Converse, Inherit}` carry the joint-table index form
/// the learner trains on; the detector reads the String-named form below,
/// since a quest's kinds are named, not indexed into phical's joint. The
/// counted [`Law`]s are kept for the record.
pub struct Declared {
    /// Kinds holding at most one other per about: a second, different other
    /// for the same about contradicts (R8.2).
    pub functional: Vec<String>,
    /// Kinds holding at most one about per other: a second, different about
    /// for the same other contradicts (R8.2).
    pub one_to_one: Vec<String>,
    /// Converse kind pairs `(k, k')`: `a k b` held contradicts a claim
    /// `b k' c` for `c != a`, since the converse already names `b k' a`.
    pub converse: Vec<(String, String)>,
    /// Inheriting (transitive) kinds: a claim closing a cycle over the held
    /// edges of such a kind contradicts (R8.2).
    pub inherit: Vec<String>,
    /// The counted laws, kept for the record; the operative contradiction
    /// tests are the four String-named ones above.
    pub laws: Vec<Law>,
}

impl Declared {
    /// Nothing declared: every claim is readable-or-degenerate-or-unsupported,
    /// never contradicting. The starting point a quest declares its kinds
    /// against.
    pub fn none() -> Self {
        Self {
            functional: Vec::new(),
            one_to_one: Vec::new(),
            converse: Vec::new(),
            inherit: Vec::new(),
            laws: Vec::new(),
        }
    }

    /// Declares `kinds` functional (at most one other per about).
    pub fn functional(mut self, kinds: &[&str]) -> Self {
        self.functional = kinds.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Declares `kinds` one-to-one (at most one about per other).
    pub fn one_to_one(mut self, kinds: &[&str]) -> Self {
        self.one_to_one = kinds.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Declares converse pairs `(k, k')`.
    pub fn converse(mut self, pairs: &[(&str, &str)]) -> Self {
        self.converse = pairs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
        self
    }

    /// Declares `kinds` inheriting (transitive): a cycle over them is broken.
    pub fn inherit(mut self, kinds: &[&str]) -> Self {
        self.inherit = kinds.iter().map(|s| s.to_string()).collect();
        self
    }
}

/// The classifier. Carries the Laws, the judge's caps for the program path,
/// and every [`Reading`] it has made so the run record can attribute each
/// classification to it.
pub struct Detector {
    pub declared: Declared,
    /// `judge`'s step cap (R8.6); matches phiclm's own 1,000,000.
    pub step_cap: usize,
    /// `judge`'s wall-clock cap in seconds (R8.6).
    pub seconds_cap: u64,
    pub seen: Vec<Reading>,
}

/// What the detector concluded about one claim, for the record.
#[derive(Clone, Debug)]
pub struct Reading {
    pub about: String,
    pub claim: Claim,
    pub standing: Standing,
    /// Which degeneracy test held, when one did (R8.5).
    pub degeneracy: Option<Degeneracy>,
    /// The judge's verdict fields, when a program was offered (R8.6). The
    /// program path (task 9.3) fills this; the classifier core leaves it
    /// `None`.
    pub verdict: Option<VerdictRecord>,
    /// Absent — not `Some(0.0)` — for an unsupported claim (R8.4).
    pub agreement: Option<f64>,
}

impl Detector {
    /// A detector reading claims against `declared`, under phiclm's own
    /// 1,000,000-step cap and a 10-second wall-clock cap (R8.6).
    pub fn new(declared: Declared) -> Self {
        Self { declared, step_cap: 1_000_000, seconds_cap: 10, seen: Vec::new() }
    }

    /// The four class tests in precedence order — degenerate, contradicting,
    /// unsupported, consistent — stopping at the first that holds, after the
    /// readability test that runs before any class test (R8.1, R8.12). The
    /// classes are mutually exclusive and jointly exhaustive, and a claim
    /// qualifying for two takes the earlier.
    pub fn classify(&mut self, about: &str, claim: &Claim, world: &World) -> Standing {
        let (standing, degeneracy) = self.read(about, claim, world);
        let agreement = None; // Absent for every class here; never Some(0.0) (R8.4).
        self.seen.push(Reading {
            about: about.to_string(),
            claim: claim.clone(),
            standing: standing.clone(),
            degeneracy,
            verdict: None,
            agreement,
        });
        standing
    }

    /// The precedence ladder itself, split out so `classify` only records.
    fn read(&self, about: &str, claim: &Claim, world: &World) -> (Standing, Option<Degeneracy>) {
        // Unreadable, before any class test (R8.12): a weight off the
        // 0..=1 scale, or a field that is not one row-shaped column.
        if let Some(reason) = unreadable(about, claim) {
            return (Standing::Unreadable { reason }, None);
        }
        // Degenerate (R8.5): a self-edge or an empty field.
        if let Some(test) = degeneracy(about, claim) {
            return (Standing::Degenerate { test }, Some(test));
        }
        // Contradicting (R8.2): a broken Law over the held triples.
        let broken = self.contradiction(about, claim, world);
        if !broken.is_empty() {
            return (Standing::Contradicting { rows: broken }, None);
        }
        // Unsupported (R8.3): nothing supports it and nothing it breaks.
        // Consistent is the last class; with no support found here the
        // classifier core returns Unsupported, the normal case for a claim
        // about something the World has never heard of.
        (Standing::Unsupported, None)
    }

    /// Every held triple that a Law reads as broken by this claim, named in
    /// spelling form (R8.2). Empty when no declared Law is broken.
    ///
    /// The tests run over the closure of the relations already held:
    /// - a functional kind already holding `(about, kind, other')` with
    ///   `other' != other`;
    /// - a one-to-one kind already holding `(about', kind, other)` with
    ///   `about' != about`;
    /// - a declared converse of the kind held with a different other;
    /// - an inherit edge the claim would close into a cycle.
    fn contradiction(&self, about: &str, claim: &Claim, world: &World) -> Vec<(String, String, String)> {
        let held = held_triples(world);
        let kind = &claim.kind;
        let other = &claim.other;
        let mut rows: Vec<(String, String, String)> = Vec::new();

        if self.declared.functional.iter().any(|k| k == kind) {
            for t in held.iter().filter(|t| &t.0 == about && &t.1 == kind && &t.2 != other) {
                push_unique(&mut rows, t.clone());
            }
        }
        if self.declared.one_to_one.iter().any(|k| k == kind) {
            for t in held.iter().filter(|t| &t.2 == other && &t.1 == kind && &t.0 != about) {
                push_unique(&mut rows, t.clone());
            }
        }
        for (k, k_inv) in &self.declared.converse {
            // The claim `about k other` says the converse `other conv about`,
            // so it answers `about`'s `k` with `other`. A held
            // `x conv about` for `x != other` already answers it the other
            // way, and the two cannot both stand: a contradiction, and the
            // held triple is named.
            let conv = if k == kind {
                Some(k_inv.as_str())
            } else if k_inv == kind {
                Some(k.as_str())
            } else {
                None
            };
            if let Some(conv) = conv {
                for t in held.iter().filter(|t| &t.2 == about && t.1 == conv && &t.0 != other) {
                    push_unique(&mut rows, t.clone());
                }
            }
        }
        if self.declared.inherit.iter().any(|k| k == kind) {
            // The claim `about kind other` closes a cycle when `other` already
            // reaches `about` along the same kind. Every edge on that path is
            // named.
            if let Some(path) = reaches(&held, kind, other, about) {
                for t in path {
                    push_unique(&mut rows, t);
                }
            }
        }
        rows
    }
}

/// `impl Checker for Detector`: the quest's tell point reaches the detector
/// through this. `name` stamps the run record with this crate and version so
/// every number produced from the transcript is attributed to it.
impl Checker for Detector {
    fn check(&mut self, about: &str, claim: &Claim, world: &World) -> Standing {
        self.classify(about, claim, world)
    }

    fn name(&self) -> String {
        format!("phisor::truth v{}", env!("CARGO_PKG_VERSION"))
    }
}

mod read;
#[cfg(test)]
mod tests;

use read::{degeneracy, held_triples, push_unique, reaches, unreadable};
