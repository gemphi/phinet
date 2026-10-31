//! The counts and AtMost-zero judgement produced by `Lowering::lower_all`.

use super::Lowered;
use phical::keyboard::Fault;

/// One operation `lower_all` could not lower. `emitted` is recorded so a run
/// can prove a type refusal happened before O-code emission.
#[derive(Clone, Debug, PartialEq)]
pub struct Unlowered {
    pub operator: u8,
    pub name: &'static str,
    pub fault: Fault,
    pub emitted: usize,
}

/// Which way a count Bar faces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sense {
    AtLeast,
    AtMost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bar {
    pub value: usize,
    pub sense: Sense,
}

impl Bar {
    pub const fn at_least(value: usize) -> Self {
        Self {
            value,
            sense: Sense::AtLeast,
        }
    }

    pub const fn at_most(value: usize) -> Self {
        Self {
            value,
            sense: Sense::AtMost,
        }
    }

    pub const fn judge(self, value: usize) -> Judged {
        match self.sense {
            Sense::AtLeast if value < self.value => Judged::Miss {
                shortfall: self.value - value,
            },
            Sense::AtMost if value > self.value => Judged::Miss {
                shortfall: value - self.value,
            },
            Sense::AtLeast | Sense::AtMost => Judged::Pass,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Judged {
    Pass,
    Miss { shortfall: usize },
}

/// Every lowering and refusal, with both counts and the required AtMost-zero
/// judgement beside the unlowered count.
#[derive(Clone, Debug, PartialEq)]
pub struct LoweringReport {
    pub lowered: Vec<Lowered>,
    pub unlowered: Vec<Unlowered>,
    pub lowered_count: usize,
    pub unlowered_count: usize,
    pub unlowered_bar: Bar,
    pub unlowered_verdict: Judged,
}
