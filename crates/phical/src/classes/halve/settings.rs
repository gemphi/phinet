//! The six settings the halving reads.

use super::{MAX_PASSES, STOP};

/// The six settings the halving reads, one struct so a word-height value
/// carries unchanged to a sentence height and reads back for comparison
/// (Requirement 9.1). The default equals today's consts exactly, so a run
/// left at the default is byte-identical to the run before this struct
/// existed: 53 word classes at depth 7, and
/// `the_numbers_on_the_guide_do_not_move` stays green.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// A target class count to stop at, or `0` for no target (the default):
    /// `grow` alone says how many classes the data pays for.
    pub target_classes: usize,
    /// The most passes at one depth; `MAX_PASSES` by default.
    pub max_passes: usize,
    /// A pass gaining less than this share of the first pass's gain stops
    /// the depth; `STOP` by default.
    pub stop_fraction: f64,
    /// A unit told fewer times than this does not move; `0` by default, so
    /// every unit may move, as today.
    pub min_unit_frequency: usize,
    /// Added to each split's cost before the keep decision; `0.0` by
    /// default, so no penalty is charged, as today.
    pub split_cost_penalty: f64,
    /// The seed for the move order; `0` by default, the fixed
    /// most-told-first order the routine has always used.
    pub move_order_seed: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            target_classes: 0,
            max_passes: MAX_PASSES,
            stop_fraction: STOP,
            min_unit_frequency: 0,
            split_cost_penalty: 0.0,
            move_order_seed: 0,
        }
    }
}
