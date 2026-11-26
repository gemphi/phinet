//! Which kinds sense which label. Each kind answers to one label, named on
//! dev: dev only names which kinds answer to which label, it never shapes a
//! kind. The held-out sentences score it. Dev holds no answers (the Key is
//! never told), so no kind can answer to `answer`.
//!
//! Two rules name a kind's label. `Majority`: the label most of its dev
//! sentences carry, the first rule (2026-10-29). `Lift`: the label it holds
//! the largest share of, `n(kind, label) / n(label)` on dev, so a rare label
//! a kind gathers is named even when the kind holds more of a common one.
//! The guide's instructions are a rare label (98 held out against 1,641
//! explanations): under the majority one kind holding a third of them tipped
//! to `explain` on a few dev sentences and the F1 fell from 35% to 0
//! (docs/phisor/learn.md, "Sensing a rare label").

use super::{Kinds, Tables};
use crate::guide::{Part, Split};
use phistd::stats::f1;

/// How a kind's label is named on dev.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    Majority,
    Lift,
}

/// One label's sensing on the held-out sentences.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sense {
    /// How many kinds answer to the label.
    pub kinds: usize,
    /// Held-out sentences that carry the label.
    pub expected: f64,
    /// Held-out sentences in the kinds that answer to it.
    pub predicted: f64,
    /// Held-out sentences that carry it and fall in those kinds.
    pub hit: f64,
}

impl Sense {
    pub fn f1(&self) -> f64 {
        f1(self.hit, self.predicted as usize, self.expected as usize)
    }
}

/// The label each kind answers to under `rule`; `None` when dev holds none
/// of its sentences.
fn answer(dev: &[Vec<f64>], rule: Rule) -> Vec<Option<usize>> {
    let labels = dev.first().map_or(0, Vec::len);
    let total: Vec<f64> = (0..labels).map(|j| dev.iter().map(|r| r[j]).sum()).collect();
    let weight = |j: usize, c: f64| match rule {
        Rule::Majority => c,
        Rule::Lift => c / total[j],
    };
    dev.iter()
        .map(|r| r.iter().enumerate().filter(|(_, &c)| c > 0.0).map(|(j, &c)| (j, weight(j, c))).max_by(|a, b| a.1.total_cmp(&b.1)).map(|(j, _)| j))
        .collect()
}

/// Each label's sensing, from kinds × labels tables on dev and held out.
pub fn sense(dev: &[Vec<f64>], held: &[Vec<f64>], rule: Rule) -> Vec<Sense> {
    let labels = held.first().map_or(0, Vec::len);
    let answer = answer(dev, rule);
    (0..labels)
        .map(|j| {
            let sensers: Vec<usize> = (0..answer.len()).filter(|&k| answer[k] == Some(j)).collect();
            Sense {
                kinds: sensers.len(),
                expected: held.iter().map(|r| r[j]).sum::<f64>() + 0.0,
                predicted: sensers.iter().map(|&k| held[k].iter().sum::<f64>()).sum::<f64>() + 0.0,
                hit: sensers.iter().map(|&k| held[k][j]).sum::<f64>() + 0.0,
            }
        })
        .collect()
}

/// Each function's and each mark's sensing for `kinds` under `rule`, in that order.
pub fn sensing(kinds: &Kinds, split: &Split, rule: Rule) -> [Vec<Sense>; 2] {
    let (dev, held) = (Tables::read(kinds, split, Part::Dev), Tables::read(kinds, split, Part::Test));
    [sense(&dev.function, &held.function, rule), sense(&dev.mark, &held.mark, rule)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Kind 0 holds most of the common label; kind 1 holds a third of the
    /// rare one but more of the common. The majority names both common; the
    /// lift names kind 1 for the rare label it gathers.
    #[test]
    fn the_lift_names_a_rare_label_a_kind_gathers_and_the_majority_does_not() {
        let dev = vec![vec![300.0, 2.0], vec![40.0, 30.0], vec![60.0, 4.0]];
        let held = vec![vec![290.0, 3.0], vec![38.0, 33.0], vec![60.0, 5.0]];
        assert_eq!(answer(&dev, Rule::Majority), [Some(0), Some(0), Some(0)]);
        assert_eq!(answer(&dev, Rule::Lift), [Some(0), Some(1), Some(0)]);
        let rare = sense(&dev, &held, Rule::Lift)[1];
        assert_eq!((rare.kinds, rare.hit, rare.predicted, rare.expected), (1, 33.0, 71.0, 41.0));
        assert_eq!(sense(&dev, &held, Rule::Majority)[1].f1(), 0.0);
    }
}
