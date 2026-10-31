//! Property 22: the three Modes are cuts of one code.

use crate::ladder::nesting;
use phistd::gen::{for_all, Gen, Seeded, CASES};
use phiton::p64::variants::Level;
use std::collections::{BTreeMap, BTreeSet};

/// One Mode-ready code. Every code is at least six bits deep, and each unit
/// may have stopped at an earlier branch whose remaining bits are zero-pad,
/// as `Halving` records stopped classes.
#[derive(Debug)]
struct NestedCode {
    depth: u32,
    codes: Vec<u16>,
}

impl Gen for NestedCode {
    fn draw(g: &mut Seeded) -> Self {
        let depth = 6 + g.below(7) as u32;
        let units = 1 + g.below(2_000) as usize;
        let codes = (0..units)
            .map(|unit| {
                // Ensure every case includes a branch that stopped before
                // the whole code, then vary that stopping depth per unit.
                let grown = match unit {
                    0 => 1 + g.below((depth - 1) as u64) as u32,
                    _ => 1 + g.below(depth as u64) as u32,
                };
                let mask = (1u16 << grown) - 1;
                ((g.u64() as u16) & mask) << (depth - grown)
            })
            .collect();
        NestedCode { depth, codes }
    }
}

#[test]
fn property_22_the_modes_are_one_code_coarsened_not_three_codes() {
    // Property 22: The Modes are one code coarsened, not three codes.
    // **Validates: Requirements 5.3**
    let run = for_all::<NestedCode>(0x22, CASES, |drawn| {
        let code = phical::classes::Halving {
            codes: drawn.codes.clone(),
            depth: drawn.depth,
            ..phical::classes::Halving::default()
        };
        let report = nesting(&code);

        // Build the exact expected containment map from the same three cuts.
        // No second or third Halving run exists in this property or in
        // `nesting`.
        let mut expected16 = BTreeMap::<u16, BTreeSet<u16>>::new();
        let mut expected32 = BTreeMap::<u16, BTreeSet<u16>>::new();
        for unit in 0..code.codes.len() {
            let c16 = code.cut(unit, Level::P16.bits());
            let c32 = code.cut(unit, Level::P32.bits());
            let c64 = code.cut(unit, Level::P64.bits());
            if c16 != c32 >> 1 || c32 != c64 >> 1 {
                return false;
            }
            expected16.entry(c16).or_default().insert(c32);
            expected32.entry(c32).or_default().insert(c64);
            if !report
                .mode16_to_mode32
                .get(&c16)
                .is_some_and(|members| members.contains(&c32))
                || !report
                    .mode32_to_mode64
                    .get(&c32)
                    .is_some_and(|members| members.contains(&c64))
            {
                return false;
            }
        }
        let expected16 = lists(expected16);
        let expected32 = lists(expected32);
        report.mode16_to_mode32 == expected16
            && report.mode32_to_mode64 == expected32
            && report
                .mode16_to_mode32
                .iter()
                .all(|(&parent, children)| children.iter().all(|child| child >> 1 == parent))
            && report
                .mode32_to_mode64
                .iter()
                .all(|(&parent, children)| children.iter().all(|child| child >> 1 == parent))
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}

fn lists(map: BTreeMap<u16, BTreeSet<u16>>) -> BTreeMap<u16, Vec<u16>> {
    map.into_iter()
        .map(|(parent, children)| (parent, children.into_iter().collect()))
        .collect()
}
