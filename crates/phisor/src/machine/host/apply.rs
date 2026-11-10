//! `phisor-apply` — the 64 keyboard Operators, callable from the Lisp.
//!
//! The lowering (`machine::lower`) defines each Operator as a lambda whose
//! body is `(phisor-apply index a0 a1 …)`; until this arm the host answered
//! that call with "the lowering is not built yet", so no Operator could be
//! reached from a Lisp program — the mill (docs/lovelace) ran only inside
//! the agreement check. Here the call is answered: the Operator at `index`,
//! its arguments read as the Operand types its row declares, applied through
//! the one door [`phical::keyboard::apply`] against the host's Arena, and the
//! result carried back as a Lisp value.
//!
//! The carriage, by type: a phasor is a two-number list `(re im)`; class,
//! level and kind are numbers; a variation is a four-number list, its three
//! classes then the level's bit count; a handle type (act, frame, row, word)
//! is its index as a number — the Arena the host holds is the one the index
//! belongs to. A Fault — arity, operand type, or the family's own — comes
//! back as the error text, as every host error does.

use phical::c64;
use phical::keyboard::{apply, table, Arena, Operand, Type};
use phiclm::lisp::Value;

/// Answers `(phisor-apply index a0 a1 …)` against `arena`.
pub(super) fn call(arena: &mut Arena, args: &[Value]) -> Result<Value, String> {
    let Some(first) = args.first() else {
        return Err("phisor-apply: which Operator? (phisor-apply index a0 a1 …)".into());
    };
    let index = number(first, "phisor-apply: the Operator index")? as usize;
    let operators = table();
    let Some(operator) = operators.get(index) else {
        return Err(format!("phisor-apply: no Operator at index {index}; the keyboard has {}", operators.len()));
    };
    // The arity first, so a short or long call is named as such and never
    // silently truncated to the declared operands.
    let given = args.len() - 1;
    if given != operator.arity as usize {
        return Err(format!("phisor-apply {}: declared {} operands, given {given}", operator.name, operator.arity));
    }
    let operands = operator
        .operands
        .iter()
        .zip(&args[1..])
        .enumerate()
        .map(|(i, (ty, value))| operand(*ty, value).map_err(|e| format!("phisor-apply {}: operand {i} {e}", operator.name)))
        .collect::<Result<Vec<Operand>, String>>()?;
    apply(operator, &operands, arena).map(value).map_err(|fault| format!("phisor-apply {}: {fault:?}", operator.name))
}

/// A Lisp value read as an operand of the declared type.
fn operand(ty: Type, value: &Value) -> Result<Operand, String> {
    match ty {
        Type::Phasor => {
            let [re, im] = numbers::<2>(value, "a phasor is (re im)")?;
            Ok(Operand::Phasor(c64::new(re, im)))
        }
        Type::Class => Ok(Operand::Class(number(value, "a class is a number")? as u16)),
        Type::Level => Ok(Operand::Level(number(value, "a level is a number")? as u8)),
        Type::Kind => Ok(Operand::Kind(number(value, "a kind is a number")? as u16)),
        Type::Variation => {
            let [a, b, c, bits] = numbers::<4>(value, "a variation is (first second last bits)")?;
            Ok(Operand::Variation([a as u16, b as u16, c as u16], bits as u8))
        }
        Type::Act | Type::Frame | Type::Row | Type::Word => {
            let index = number(value, &format!("{} is a handle index", ty.name()))?;
            Ok(Operand::handle(ty, index as u32))
        }
    }
}

/// An operand carried back as a Lisp value.
fn value(result: Operand) -> Value {
    match result {
        Operand::Phasor(z) => Value::List(vec![Value::Num(z.re), Value::Num(z.im)]),
        Operand::Class(c) => Value::Num(c as f64),
        Operand::Level(l) => Value::Num(l as f64),
        Operand::Kind(k) => Value::Num(k as f64),
        Operand::Variation([a, b, c], bits) => {
            Value::List([a, b, c].iter().map(|&x| Value::Num(x as f64)).chain(std::iter::once(Value::Num(bits as f64))).collect())
        }
        Operand::Act(h) | Operand::Frame(h) | Operand::Row(h) | Operand::Word(h) => Value::Num(h.index as f64),
    }
}

fn number(value: &Value, what: &str) -> Result<f64, String> {
    match value {
        Value::Num(n) => Ok(*n),
        other => Err(format!("{what}, not {other}")),
    }
}

fn numbers<const N: usize>(value: &Value, what: &str) -> Result<[f64; N], String> {
    let Value::List(items) = value else {
        return Err(format!("{what}, not {value}"));
    };
    if items.len() != N {
        return Err(format!("{what}, not a list of {}", items.len()));
    }
    let mut out = [0.0; N];
    for (slot, item) in out.iter_mut().zip(items) {
        *slot = number(item, what)?;
    }
    Ok(out)
}
