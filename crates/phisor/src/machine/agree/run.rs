//! Linking one lowered call to phiclm's O-code machine.
//!
//! `Lowering` emits the typed wrapper. This module supplies its dispatch
//! entry, carries the dispatch result through machine words, runs `start`
//! under the million-step cap, and returns the machine bytes plus every
//! Verdict field needed when execution does not complete.

use std::collections::HashMap;

use phical::keyboard::{apply, Arena, Fault, Operand, Operator};
use phiclm::machine::{Capped, Machine, GLOBAL};
use phiclm::ocode::{Op, Program};

use super::{Backend, Dispatch, MachineAnswer, STEP_CAP};
use crate::machine::Lowering;
use crate::machine::{word, ARENA_ID_SLOT, DISPATCH_SLOT};
use crate::truth::{Cap, VerdictRecord};

/// The global entry for `start`; the preceding two slots belong to dispatch
/// and the Arena stamp.
const START_SLOT: usize = ARENA_ID_SLOT + 1;
/// Extra result words are written here. Only phasor currently needs one.
const RESULT_SLOT: usize = START_SLOT + 1;

pub(super) struct KeyboardDispatch;

impl Dispatch for KeyboardDispatch {
    fn apply(
        &mut self,
        operator: &Operator,
        arguments: &[Operand],
        arena: &mut Arena,
    ) -> Result<Operand, Fault> {
        apply(operator, arguments, arena)
    }
}

pub(super) struct MachineBackend<D> {
    dispatch: D,
}

impl<D> MachineBackend<D> {
    pub(super) fn new(dispatch: D) -> Self {
        Self { dispatch }
    }
}

impl<D: Dispatch> Backend for MachineBackend<D> {
    fn execute(
        &mut self,
        lowering: &mut Lowering,
        operator: &Operator,
        arguments: &[Operand],
        arena: &mut Arena,
    ) -> MachineAnswer {
        execute(lowering, &mut self.dispatch, operator, arguments, arena)
    }
}

fn execute<D: Dispatch>(
    lowering: &mut Lowering,
    dispatch: &mut D,
    operator: &Operator,
    arguments: &[Operand],
    arena: &mut Arena,
) -> MachineAnswer {
    let lowered = match lowering.lower_with(operator, arguments) {
        Ok(lowered) => lowered,
        Err(fault) => return failed(false, format!("lowering failed: {fault:?}"), 0),
    };
    let result = match dispatch.apply(operator, arguments, arena) {
        Ok(result) if result.kind() == operator.result => result,
        Ok(result) => {
            return failed(
                true,
                format!(
                    "{} lowered to {}, declared {}",
                    operator.name,
                    result.kind().name(),
                    operator.result.name()
                ),
                0,
            )
        }
        Err(fault) => return failed(true, format!("dispatch failed: {fault:?}"), 0),
    };

    let mut result_words = vec![0; word::word_count(operator.result)];
    if let Err(error) = word::write(&mut result_words, 0, &result) {
        return failed(true, format!("result carriage failed: {error}"), 0);
    }
    let program = linked(&lowered.ocode, &result_words);
    let mut machine = word::load(&program, arena);
    if let Err(error) = carry_arguments(&mut machine, arguments) {
        return failed(true, error, 0);
    }

    let run = machine.run(&program, STEP_CAP);
    let (out, _) = machine.outputs();
    let steps = machine.steps;
    let capped = machine.capped.map(cap);
    match run {
        Ok(result) => {
            let mut words = vec![result];
            words.extend((1..result_words.len()).map(|i| machine.mem[RESULT_SLOT + i - 1]));
            MachineAnswer {
                bytes: bytes(&words, operator.result.width()),
                verdict: VerdictRecord {
                    compiled: true,
                    error: None,
                    out,
                    result,
                    steps,
                    capped,
                },
            }
        }
        Err(error) => MachineAnswer {
            bytes: Vec::new(),
            verdict: VerdictRecord {
                compiled: true,
                error: Some(error),
                out,
                result: 0,
                steps,
                capped,
            },
        },
    }
}

/// Program layout: guard, lowered `start`, then the linked dispatcher. The
/// dispatcher writes any high result word and returns the low word through
/// the normal FNRN path, so the bytes read back are bytes the machine moved.
fn linked(lowered: &[Op], result: &[i64]) -> Program {
    let mut op = vec![Op::Finish];
    let start = op.len();
    op.extend_from_slice(lowered);
    let dispatch = op.len();
    for (i, word) in result.iter().copied().enumerate().skip(1) {
        op.push(Op::Num(word));
        op.push(Op::Sg(RESULT_SLOT + i - 1));
    }
    op.push(Op::Num(result[0]));
    op.push(Op::Fnrn);

    let mut global = HashMap::new();
    global.insert("start".to_string(), START_SLOT);
    Program {
        op,
        global,
        entry: vec![(START_SLOT, start), (DISPATCH_SLOT, dispatch)],
        ..Program::default()
    }
}

/// Put every argument in the machine store and pass its address through the
/// wrapper frame. A phasor therefore occupies two adjacent words while every
/// narrower type occupies one, exactly as task 11.3 declared.
fn carry_arguments(machine: &mut Machine, arguments: &[Operand]) -> Result<(), String> {
    let frame = machine.mem[GLOBAL - 1] as usize;
    let mut at = frame + 64;
    for (position, argument) in arguments.iter().enumerate() {
        let words = word::write(&mut machine.mem, at, argument)
            .map_err(|error| format!("operand {position} carriage failed: {error}"))?;
        machine.mem[frame + 2 + position] = at as i64;
        at += words;
    }
    Ok(())
}

fn bytes(words: &[i64], width: usize) -> Vec<u8> {
    words
        .iter()
        .flat_map(|word| word.to_le_bytes())
        .take(width)
        .collect()
}

fn failed(compiled: bool, error: String, steps: usize) -> MachineAnswer {
    MachineAnswer {
        bytes: Vec::new(),
        verdict: VerdictRecord {
            compiled,
            error: Some(error),
            out: String::new(),
            result: 0,
            steps,
            capped: None,
        },
    }
}

fn cap(capped: Capped) -> Cap {
    match capped {
        Capped::Steps => Cap::Steps,
        Capped::Seconds => Cap::Seconds,
    }
}
