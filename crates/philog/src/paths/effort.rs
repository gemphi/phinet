#[allow(unused_imports)]
use super::*;

impl EffortLevel {
    pub fn max_steps(&self) -> usize {
        match self {
            EffortLevel::Instant => 1,
            EffortLevel::Quick => 4,
            EffortLevel::Standard => REASONING_MAX_STEPS,
            EffortLevel::Deep => 32,
            EffortLevel::Exhaustive => 64,
        }
    }

    pub fn n_paths(&self) -> usize {
        match self {
            EffortLevel::Instant | EffortLevel::Quick => 1,
            EffortLevel::Standard => 1,
            EffortLevel::Deep => 4,
            EffortLevel::Exhaustive => 8,
        }
    }
}
