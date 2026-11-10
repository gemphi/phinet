//! What a run of the engine invoked, counted (phisor TODO: "have the
//! harness count which primitives and Operators the guide's questions
//! invoke" before any Operator is added to the keyboard).
//!
//! The machine's tally (phiclm: every primitive and every named lambda it
//! applied) is read against what phisor knows the names to be: a core
//! primitive, one of the host's calls, or a card (a named lambda, the
//! question's own included); the trace says which of
//! the 64 keyboard Operators were reached through `phisor-apply`. Usages add,
//! so a harness can sum them over every question it asked.

use std::collections::BTreeMap;
use std::fmt;

use super::engine::Step;
use super::PhisorHost;

/// What one run (or many, summed) invoked, by name and count.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    /// phiclm's core primitives.
    pub primitives: BTreeMap<String, usize>,
    /// The host's calls: `phisor-rows`, `phisor-apply`, …
    pub host: BTreeMap<String, usize>,
    /// Keyboard Operators reached through `phisor-apply`, by their names.
    pub operators: BTreeMap<String, usize>,
    /// Cards: named lambdas, from `data/ops` or punched.
    pub cards: BTreeMap<String, usize>,
    /// Runs summed into this usage.
    pub runs: usize,
}

impl Usage {
    /// One run's usage: the machine's tally sorted by what each name is -
    /// a core primitive, one of `host_names`, else a card - and the
    /// Operators the trace shows applied.
    pub fn of(tally: BTreeMap<String, usize>, steps: &[Step], host_names: &[&str]) -> Self {
        let core = PhisorHost::core_names();
        let mut u = Usage { runs: 1, ..Usage::default() };
        for (name, n) in tally {
            let bucket = match (core.contains(&name.as_str()), host_names.contains(&name.as_str())) {
                (true, _) => &mut u.primitives,
                (false, true) => &mut u.host,
                (false, false) => &mut u.cards,
            };
            bucket.insert(name, n);
        }
        for step in steps.iter().filter(|s| s.operation != "rows") {
            *u.operators.entry(step.operation.clone()).or_insert(0) += 1;
        }
        u
    }

    /// This usage with another's counts added.
    pub fn add(&mut self, other: &Usage) {
        let sum = |into: &mut BTreeMap<String, usize>, from: &BTreeMap<String, usize>| {
            for (k, n) in from {
                *into.entry(k.clone()).or_insert(0) += n;
            }
        };
        sum(&mut self.primitives, &other.primitives);
        sum(&mut self.host, &other.host);
        sum(&mut self.operators, &other.operators);
        sum(&mut self.cards, &other.cards);
        self.runs += other.runs;
    }

    /// How many distinct names of each kind were invoked.
    pub fn distinct(&self) -> (usize, usize, usize, usize) {
        (self.primitives.len(), self.host.len(), self.operators.len(), self.cards.len())
    }
}

/// A markdown table: each name, what it is, how often, and how often per run.
impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let runs = self.runs.max(1) as f64;
        writeln!(f, "| invoked | kind | times | per run |")?;
        writeln!(f, "|---|---|---|---|")?;
        let kinds = [("primitive", &self.primitives), ("host", &self.host), ("operator", &self.operators), ("card", &self.cards)];
        for (kind, names) in kinds {
            let mut by_count: Vec<(&String, &usize)> = names.iter().collect();
            by_count.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            for (name, n) in by_count {
                writeln!(f, "| `{name}` | {kind} | {n} | {:.1} |", *n as f64 / runs)?;
            }
        }
        let (p, h, o, c) = self.distinct();
        write!(f, "\n{} runs · {p} of {} primitives · {h} host calls · {o} of 64 Operators · {c} cards", self.runs, PhisorHost::core_names().len())
    }
}

#[cfg(test)]
mod tests;
