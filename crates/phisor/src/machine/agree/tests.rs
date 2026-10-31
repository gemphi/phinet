use std::cell::RefCell;
use std::rc::Rc;

use phical::keyboard::{apply, Arena, Fault, FrozenOperands, Operand, Operator, COUNT};

use super::run::{KeyboardDispatch, MachineBackend};
use super::{agree_with, Backend, Dispatch, MachineAnswer, STEP_CAP};
use crate::machine::{Judged, Lowering};
use crate::truth::{Cap, VerdictRecord};

struct Wrong {
    seen: Rc<RefCell<[usize; COUNT]>>,
}

impl Dispatch for Wrong {
    fn apply(
        &mut self,
        operator: &Operator,
        arguments: &[Operand],
        arena: &mut Arena,
    ) -> Result<Operand, Fault> {
        self.seen.borrow_mut()[operator.index as usize] += 1;
        let result = apply(operator, arguments, arena)?;
        if operator.index != 0 {
            return Ok(result);
        }
        let mut wrong = result.encode();
        wrong[0] ^= 1;
        Ok(Operand::decode(operator.result, &wrong).expect("the wrong result keeps its width"))
    }
}

/// A deliberately wrong lowering for one Operator reports both byte strings
/// and their first differing offset, then still runs every other Operator.
///
/// **Validates: Requirements 7.8, 7.9**
#[test]
fn a_wrong_lowering_is_a_miss_and_the_other_sixty_three_still_run() {
    let frozen = FrozenOperands::load();
    let seen = Rc::new(RefCell::new([0; COUNT]));
    let mut backend = MachineBackend::new(Wrong {
        seen: Rc::clone(&seen),
    });
    let report = agree_with(&mut Lowering::new(), &frozen, &mut backend);

    assert_eq!(report.agreeing, 63);
    assert_eq!(report.agreeing_verdict, Judged::Miss { shortfall: 1 });
    assert_eq!(report.misses.len(), 1);
    let miss = &report.misses[0];
    assert_eq!(miss.operator, "identity");
    assert_eq!(miss.rust, vec![0, 0, 0, 0]);
    assert_eq!(miss.machine, vec![1, 0, 0, 0]);
    assert_eq!(miss.first_differing, 0);
    assert!(
        miss.verdict.is_none(),
        "a completed mismatch is not noncompletion"
    );
    assert!(
        seen.borrow().iter().all(|runs| *runs > 0),
        "the mismatch stopped a later Operator: {:?}",
        seen.borrow()
    );
}

struct StopsOnce {
    inner: MachineBackend<KeyboardDispatch>,
    operator: u8,
}

impl Backend for StopsOnce {
    fn execute(
        &mut self,
        lowering: &mut Lowering,
        operator: &Operator,
        arguments: &[Operand],
        arena: &mut Arena,
    ) -> MachineAnswer {
        if operator.index == self.operator {
            return MachineAnswer {
                bytes: Vec::new(),
                verdict: VerdictRecord {
                    compiled: false,
                    error: Some("compiler stopped".to_string()),
                    out: "partial output".to_string(),
                    result: 91,
                    steps: STEP_CAP + 1,
                    capped: Some(Cap::Steps),
                },
            };
        }
        self.inner.execute(lowering, operator, arguments, arena)
    }
}

/// Noncompletion preserves every Verdict field instead of reducing the
/// failure to a boolean or dropping partial output.
///
/// **Validates: Requirements 7.14**
#[test]
fn noncompletion_records_the_full_verdict_and_continues() {
    let frozen = FrozenOperands::load();
    let mut backend = StopsOnce {
        inner: MachineBackend::new(KeyboardDispatch),
        operator: 1,
    };
    let report = agree_with(&mut Lowering::new(), &frozen, &mut backend);

    assert_eq!(report.agreeing, 63);
    assert_eq!(report.misses.len(), 1);
    let miss = &report.misses[0];
    assert_eq!(miss.operator, "reduce");
    assert_eq!(miss.first_differing, 0);
    let verdict = miss.verdict.as_ref().expect("noncompletion verdict");
    assert!(!verdict.compiled);
    assert_eq!(verdict.error.as_deref(), Some("compiler stopped"));
    assert_eq!(verdict.out, "partial output");
    assert_eq!(verdict.result, 91);
    assert_eq!(verdict.steps, STEP_CAP + 1);
    assert_eq!(verdict.capped, Some(Cap::Steps));
}
