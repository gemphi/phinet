//! Differential agreement between the Rust keyboard reader and its lowered
//! phiclm execution (Requirement 7 criteria 6, 8, 9 and 14).
//!
//! The frozen operand file is the whole input. For one Operator, case `i`
//! takes operand `i` from each declared type (cycling only when a future
//! frozen record gives the types different lengths), so every frozen value
//! of every declared operand type is exercised and no generated sample can
//! replace an inconvenient edge value. One mismatch makes that Operator a
//! Miss, but never prevents the next Operator from running.

use phical::keyboard::{apply, Arena, Fault, FrozenOperands, Operand, Operator, COUNT};

use super::{Bar, Judged, Lowering};
use crate::truth::VerdictRecord;

mod input;
mod run;

use input::{cases, describe, first_difference, link_handles};

/// The machine step cap required by Requirement 7 criterion 14.
pub const STEP_CAP: usize = phiclm::STEP_CAP;

/// One Operator application on which the two readers did not agree.
#[derive(Clone, Debug)]
pub struct Mismatch {
    pub operator: &'static str,
    /// The exact typed argument bytes, in argument order.
    pub operand: String,
    pub rust: Vec<u8>,
    pub machine: Vec<u8>,
    pub first_differing: usize,
    /// Present only when the machine did not complete. It retains the full
    /// compiled/error/output/result/steps account (and the cap, when any).
    pub verdict: Option<VerdictRecord>,
}

/// The 64-Operator result and its declared AtLeast-64 Bar.
#[derive(Clone, Debug)]
pub struct Agreement {
    pub agreeing: usize,
    pub misses: Vec<Mismatch>,
    pub agreeing_bar: Bar,
    pub agreeing_verdict: Judged,
    /// Applications actually completed before each Operator agreed or
    /// produced its first diagnostic Miss.
    pub cases: usize,
}

/// Run the agreement check with a fresh lowerer.
pub fn agree(frozen_set: &FrozenOperands) -> Agreement {
    Lowering::new().agree(frozen_set)
}

impl Lowering {
    /// Run all 64 Operators over the complete frozen input. Agreement is
    /// exact encoded-byte identity even for phasors; the keyboard's tolerant
    /// equality predicate is deliberately not consulted here.
    pub fn agree(&mut self, frozen_set: &FrozenOperands) -> Agreement {
        let mut backend = run::MachineBackend::new(run::KeyboardDispatch);
        agree_with(self, frozen_set, &mut backend)
    }
}

/// One linked machine execution. Successful executions carry bytes; failed
/// executions carry the full verdict that explains why no result exists.
pub(super) struct MachineAnswer {
    pub bytes: Vec<u8>,
    pub verdict: VerdictRecord,
}

impl MachineAnswer {
    fn completed(&self) -> bool {
        self.verdict.compiled
            && self.verdict.error.is_none()
            && self.verdict.capped.is_none()
            && self.verdict.steps <= STEP_CAP
    }
}

/// The machine-side implementation behind the lowered dispatch call. It is
/// a seam so the required wrong-lowering test can alter exactly one Operator
/// without changing the Rust reader used as the differential oracle.
pub(super) trait Dispatch {
    fn apply(
        &mut self,
        operator: &Operator,
        arguments: &[Operand],
        arena: &mut Arena,
    ) -> Result<Operand, Fault>;
}

/// A backend executes one already-typed application on phiclm.
pub(super) trait Backend {
    fn execute(
        &mut self,
        lowering: &mut Lowering,
        operator: &Operator,
        arguments: &[Operand],
        arena: &mut Arena,
    ) -> MachineAnswer;
}

fn agree_with<B: Backend>(
    lowering: &mut Lowering,
    frozen_set: &FrozenOperands,
    backend: &mut B,
) -> Agreement {
    let mut agreeing = 0;
    let mut misses = Vec::new();
    let mut cases_run = 0;

    for operator in phical::keyboard::table() {
        let mut operator_agrees = true;
        for source in cases(&operator, frozen_set) {
            cases_run += 1;
            let mut arena = Arena::new(
                frozen_set.fingerprint ^ ((operator.index as u64) << 32) ^ cases_run as u64,
            );
            let arguments = match link_handles(&operator, &source, &mut arena) {
                Ok(arguments) => arguments,
                Err(fault) => {
                    misses.push(Mismatch {
                        operator: operator.name,
                        operand: format!("{}; link fault: {fault:?}", describe(&source)),
                        rust: Vec::new(),
                        machine: Vec::new(),
                        first_differing: 0,
                        verdict: None,
                    });
                    operator_agrees = false;
                    break;
                }
            };
            let shown = describe(&arguments);
            let mut reader_arena = arena.clone();
            let rust = match apply(&operator, &arguments, &mut reader_arena) {
                Ok(value) => value.encode(),
                Err(fault) => {
                    misses.push(Mismatch {
                        operator: operator.name,
                        operand: format!("{shown}; Rust reader fault: {fault:?}"),
                        rust: Vec::new(),
                        machine: Vec::new(),
                        first_differing: 0,
                        verdict: None,
                    });
                    operator_agrees = false;
                    break;
                }
            };

            let answer = backend.execute(lowering, &operator, &arguments, &mut arena);
            if !answer.completed() {
                let first_differing = first_difference(&rust, &answer.bytes);
                misses.push(Mismatch {
                    operator: operator.name,
                    operand: shown,
                    rust,
                    machine: answer.bytes,
                    first_differing,
                    verdict: Some(answer.verdict),
                });
                operator_agrees = false;
                break;
            }
            if rust != answer.bytes {
                let first_differing = first_difference(&rust, &answer.bytes);
                misses.push(Mismatch {
                    operator: operator.name,
                    operand: shown,
                    rust,
                    machine: answer.bytes,
                    first_differing,
                    verdict: None,
                });
                operator_agrees = false;
                break;
            }
        }
        if operator_agrees {
            agreeing += 1;
        }
    }

    let agreeing_bar = Bar::at_least(COUNT);
    Agreement {
        agreeing,
        misses,
        agreeing_bar,
        agreeing_verdict: agreeing_bar.judge(agreeing),
        cases: cases_run,
    }
}

#[cfg(test)]
mod property;
#[cfg(test)]
mod tests;
