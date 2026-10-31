//! The six exchange settings read side by side at two learning heights.

use super::Settings;

impl Settings {
    /// The complete, stable setting-name order recorded at each height.
    pub const NAMES: [&'static str; 6] = [
        "target_classes",
        "max_passes",
        "stop_fraction",
        "min_unit_frequency",
        "split_cost_penalty",
        "move_order_seed",
    ];

    /// Every setting's word-height value beside its sentence-height value.
    pub fn beside(word: &Self, sentence: &Self) -> [(&'static str, f64, f64); 6] {
        [
            (
                Self::NAMES[0],
                word.target_classes as f64,
                sentence.target_classes as f64,
            ),
            (
                Self::NAMES[1],
                word.max_passes as f64,
                sentence.max_passes as f64,
            ),
            (Self::NAMES[2], word.stop_fraction, sentence.stop_fraction),
            (
                Self::NAMES[3],
                word.min_unit_frequency as f64,
                sentence.min_unit_frequency as f64,
            ),
            (
                Self::NAMES[4],
                word.split_cost_penalty,
                sentence.split_cost_penalty,
            ),
            (
                Self::NAMES[5],
                word.move_order_seed as f64,
                sentence.move_order_seed as f64,
            ),
        ]
    }

    /// Every setting that drifted, in [`Self::NAMES`] order.
    pub fn differing(word: &Self, sentence: &Self) -> Vec<&'static str> {
        let differs = [
            word.target_classes != sentence.target_classes,
            word.max_passes != sentence.max_passes,
            word.stop_fraction != sentence.stop_fraction,
            word.min_unit_frequency != sentence.min_unit_frequency,
            word.split_cost_penalty != sentence.split_cost_penalty,
            word.move_order_seed != sentence.move_order_seed,
        ];
        Self::NAMES
            .into_iter()
            .zip(differs)
            .filter_map(|(name, differs)| differs.then_some(name))
            .collect()
    }
}
