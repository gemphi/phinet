//! What the one routine is handed to group the templates, in two readings.
//!
//! - **The river:** each training run as a sequence of templates, so two
//!   templates fall together when they stand between the same neighbours.
//! - **Inside:** one pair per class pair a training sentence holds, from its
//!   template to that pair (`f:c6c8`, with the edges `f:<s>c6` and
//!   `f:c3</s>`). Two templates fall together when their sentences hold the
//!   same pairs, and the pairs fall together when the same templates hold
//!   them. The class-pair likelihood of these pairs is their mutual
//!   information, so this is information-theoretic co-clustering (Dhillon,
//!   Mallela and Modha 2003), with the same routine and the same stop.

use crate::units::Units;

/// What a template is grouped by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Reading {
    /// Which template follows which: the neighbours.
    River,
    /// Which class pairs a template's sentences hold.
    Inside,
}

impl Reading {
    pub const ALL: [Reading; 2] = [Reading::River, Reading::Inside];

    pub fn name(self) -> &'static str {
        match self {
            Reading::River => "river",
            Reading::Inside => "inside",
        }
    }
}

/// The edges of a run, as units of their own.
pub const START: &str = "<s>";
pub const END: &str = "</s>";

/// Each training run as `<s>`, its templates' units, `</s>`.
pub fn river(runs: &[Vec<usize>], unit: &[usize], units: &mut Units) -> Vec<Vec<usize>> {
    runs.iter()
        .map(|run| {
            let mut seq = vec![units.id(START)];
            seq.extend(run.iter().map(|&i| unit[i]));
            seq.push(units.id(END));
            seq
        })
        .collect()
}

/// A sentence's class pairs, its edges among them, as feature names.
pub fn features(classes: &[u16]) -> Vec<String> {
    let name = |c: Option<&u16>, edge: &str| c.map_or(edge.to_string(), |x| format!("c{x}"));
    (0..=classes.len()).map(|i| format!("f:{}{}", name(i.checked_sub(1).and_then(|j| classes.get(j)), START), name(classes.get(i), END))).collect()
}

/// One pair `[template, feature]` for every class pair of every training
/// sentence; the features are coined after the templates.
pub fn inside(runs: &[Vec<usize>], unit: &[usize], written: &[Vec<u16>], units: &mut Units) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for &i in runs.iter().flatten() {
        for f in features(&written[i]) {
            out.push(vec![unit[i], units.id(&f)]);
        }
    }
    out
}
