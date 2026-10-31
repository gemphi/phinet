//! Growth of the one learned word code, and its three Mode cuts (R5.2,
//! R5.3). The code is grown once. Mode 16, Mode 32 and Mode 64 are views of
//! that one [`Halving`] at four, five and six bits; [`nesting`] never runs
//! the exchange routine itself.

use phical::classes::{halve_with, Grow, Halving, Settings};
use phiton::p64::variants::Level;
use std::collections::{BTreeMap, BTreeSet};

/// What the data paid for at one accepted depth of the code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DepthReport {
    /// The number of leading bits read at this depth.
    pub depth: u32,
    /// The occupied classes after all splits through this depth were paid.
    pub classes: usize,
    /// The splits paid for at this depth. Each kept split adds one class.
    pub splits_paid_for: usize,
}

/// The learned code and the account of how far the data paid to grow it.
#[derive(Clone, Debug)]
pub struct CodeReport {
    /// The six settings used by the run, retained for the no-new-setting
    /// comparison at the next height.
    pub settings: Settings,
    /// The occupied-class count and newly paid splits at every accepted
    /// depth, in increasing depth order.
    pub depths: Vec<DepthReport>,
    /// The final accepted depth: the depth at which growth stopped.
    pub stopped_at: u32,
    /// The one code from which every Mode and every containment edge is read.
    pub halving: Halving,
}

/// The occupied child classes of each occupied parent class at the two Mode
/// boundaries. Keys and members are sorted, and a member occurs only once.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Nesting {
    /// Each Mode-16 class and its occupied Mode-32 members.
    pub mode16_to_mode32: BTreeMap<u16, Vec<u16>>,
    /// Each Mode-32 class and its occupied Mode-64 members.
    pub mode32_to_mode64: BTreeMap<u16, Vec<u16>>,
}

/// Grow one code as far as its data pays, using the complete task 4.1
/// settings, and report its occupied classes at every accepted depth (R5.2).
pub fn grow_code(units: usize, sequences: &[Vec<usize>], settings: &Settings) -> CodeReport {
    let halving = halve_with(units, sequences, Grow::Paid, settings);
    let depths = (1..=halving.depth)
        .map(|depth| {
            let classes = (0..halving.codes.len())
                .map(|unit| halving.cut(unit, depth))
                .collect::<BTreeSet<_>>()
                .len();
            DepthReport {
                depth,
                classes,
                splits_paid_for: halving.kept.get(depth as usize - 1).copied().unwrap_or(0),
            }
        })
        .collect();
    CodeReport {
        settings: *settings,
        stopped_at: halving.depth,
        depths,
        halving,
    }
}

/// Read Mode-16→Mode-32 and Mode-32→Mode-64 membership from one code by
/// calling [`Halving::cut`] at four, five and six bits (R5.3).
pub fn nesting(code: &Halving) -> Nesting {
    let mut sixteen: BTreeMap<u16, BTreeSet<u16>> = BTreeMap::new();
    let mut thirty_two: BTreeMap<u16, BTreeSet<u16>> = BTreeMap::new();
    for unit in 0..code.codes.len() {
        let c16 = code.cut(unit, Level::P16.bits());
        let c32 = code.cut(unit, Level::P32.bits());
        let c64 = code.cut(unit, Level::P64.bits());
        sixteen.entry(c16).or_default().insert(c32);
        thirty_two.entry(c32).or_default().insert(c64);
    }
    Nesting {
        mode16_to_mode32: lists(sixteen),
        mode32_to_mode64: lists(thirty_two),
    }
}

fn lists(map: BTreeMap<u16, BTreeSet<u16>>) -> BTreeMap<u16, Vec<u16>> {
    map.into_iter()
        .map(|(class, members)| (class, members.into_iter().collect()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alternating(n: usize) -> Vec<Vec<usize>> {
        let mut state = 7u64;
        let mut draw = |m: u64| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((state >> 33) % m) as usize
        };
        (0..n)
            .map(|_| (0..8).map(|i| draw(3) + 3 * (i % 2)).collect())
            .collect()
    }

    #[test]
    fn growth_reports_each_paid_depth_and_the_depth_where_it_stopped() {
        let settings = Settings {
            max_passes: 1,
            ..Settings::default()
        };
        let report = grow_code(6, &alternating(200), &settings);
        assert_eq!(report.settings, settings);
        assert_eq!(report.stopped_at, report.halving.depth);
        assert_eq!(report.depths.len(), report.stopped_at as usize);
        assert_eq!(
            report.depths.last().map(|d| d.classes),
            Some(report.halving.classes)
        );
        assert!(report
            .halving
            .passes
            .iter()
            .all(|passes| passes.len() == settings.max_passes));
        for (index, depth) in report.depths.iter().enumerate() {
            assert_eq!(depth.depth as usize, index + 1);
            assert_eq!(depth.splits_paid_for, report.halving.kept[index]);
        }
    }

    #[test]
    fn nesting_reports_the_occupied_members_of_both_mode_boundaries() {
        let code = Halving {
            codes: vec![0, 1, 2, 3, 4, 5, 32, 33, 63],
            depth: 6,
            ..Halving::default()
        };
        let got = nesting(&code);
        assert_eq!(
            got.mode16_to_mode32,
            BTreeMap::from([(0, vec![0, 1]), (1, vec![2]), (8, vec![16]), (15, vec![31])])
        );
        assert_eq!(
            got.mode32_to_mode64,
            BTreeMap::from([
                (0, vec![0, 1]),
                (1, vec![2, 3]),
                (2, vec![4, 5]),
                (16, vec![32, 33]),
                (31, vec![63]),
            ])
        );
        assert_eq!(nesting(&Halving::default()), Nesting::default());
    }
}
