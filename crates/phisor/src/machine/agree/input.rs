//! Turning the frozen record into typed agreement cases.

use phical::keyboard::family::helix::CODE_TAG;
use phical::keyboard::{Arena, Fault, FrozenOperands, Operand, Operator, Type};

/// Diagonal traversal of the complete per-type lists. It is exhaustive over
/// each declared argument position without replacing the frozen file with a
/// random 256-case sample.
pub(super) fn cases(operator: &Operator, frozen_set: &FrozenOperands) -> Vec<Vec<Operand>> {
    let count = operator
        .operands
        .iter()
        .map(|&ty| frozen_set.of(ty).len())
        .max()
        .unwrap_or(0);
    (0..count)
        .map(|case| {
            operator
                .operands
                .iter()
                .map(|&ty| {
                    let values = frozen_set.of(ty);
                    values[case % values.len()]
                })
                .collect()
        })
        .collect()
}

/// A handle has meaning only with its Arena. The frozen Word bytes seed a
/// canonical helix code when one of the five code-reading Operators needs to
/// dereference it; the exact frozen handle itself remains the operand for
/// Operators such as identity that compare the handle bytes directly.
pub(super) fn link_handles(
    operator: &Operator,
    source: &[Operand],
    arena: &mut Arena,
) -> Result<Vec<Operand>, Fault> {
    if !(23..=27).contains(&operator.index) {
        return Ok(source.to_vec());
    }
    source
        .iter()
        .map(|operand| match operand {
            Operand::Word(handle) => {
                let channel = (handle.index % phical::CHANNELS as u32) as u16;
                let [lo, hi] = channel.to_le_bytes();
                arena
                    .mint(Type::Word, &[CODE_TAG, lo, hi])
                    .map(Operand::Word)
            }
            other => Ok(*other),
        })
        .collect()
}

pub(super) fn describe(arguments: &[Operand]) -> String {
    arguments
        .iter()
        .map(|argument| {
            format!(
                "{}:{}",
                argument.kind().name(),
                phical::keyboard::frozen::hex(&argument.encode())
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn first_difference(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b)
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| a.len().min(b.len()))
}
