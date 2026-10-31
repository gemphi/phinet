//! Lowering a typed phical keyboard operation to the two machine-facing
//! forms: a Lisp `define` of a lambda and a bounded O-code call sequence.
//!
//! O-code is deliberately typeless. [`Lowering::lower`] therefore validates
//! the argument count and every argument type against the Operator row before
//! it creates either form. A refusal clears the transaction's emission buffer,
//! making its observable O-code count zero (Requirement 7 criterion 5).

use phical::keyboard::{table, Fault, Operand, Operator};
use phiclm::ocode::Op;

mod report;

pub use report::{Bar, Judged, LoweringReport, Sense, Unlowered};

/// The bounds Requirement 7 criterion 5 places on one lowered sequence.
pub const MIN_OCODE: usize = 1;
pub const MAX_OCODE: usize = 64;

/// The user-global slot through which the machine-side wrapper calls the
/// `phisor-apply` dispatcher. The agreement task links the dispatcher; this
/// task emits its typed call site.
pub const DISPATCH_SLOT: usize = phiclm::global::USER;

/// The 55 operation names declared by [`phiclm::ocode::Op`]. Keeping the
/// closed list beside the lowering gives Property 18 a direct membership
/// oracle rather than treating any arbitrary mnemonic as O-code.
pub const OCODE_NAMES: [&str; 55] = [
    "num", "str", "table", "lp", "sp", "llp", "lg", "sg", "rv", "stind", "pop", "neg", "not",
    "abs", "float", "fix", "fneg", "fabs", "add", "sub", "mul", "div", "rem", "and", "or", "lsh",
    "rsh", "eqv", "neqv", "getbyte", "putbyte", "eq", "ne", "ls", "gr", "le", "ge", "fadd", "fsub",
    "fmul", "fdiv", "feq", "fne", "fls", "fgr", "fle", "fge", "jump", "goto", "jf", "jt", "call",
    "fnrn", "rtrn", "finish",
];

/// One concrete operation: an Operator declaration and the operands supplied
/// to it. `From<&Operator>` supplies one zero value of every declared type,
/// which is the operation `lower_all` uses to lower each declaration once.
#[derive(Clone, Debug)]
pub struct Operation {
    pub operator: Operator,
    pub arguments: Vec<Operand>,
}

impl Operation {
    pub fn new(operator: &Operator, arguments: &[Operand]) -> Self {
        Self {
            operator: operator.clone(),
            arguments: arguments.to_vec(),
        }
    }
}

impl From<&Operator> for Operation {
    fn from(operator: &Operator) -> Self {
        let arguments = operator.operands.iter().map(|ty| ty.zero()).collect();
        Self {
            operator: operator.clone(),
            arguments,
        }
    }
}

/// One successfully lowered operation.
#[derive(Clone, Debug, PartialEq)]
pub struct Lowered {
    pub operator: u8,
    pub name: &'static str,
    pub binding: String,
    pub lisp: String,
    pub ocode: Vec<Op>,
}

/// A transactional lowerer. `emitted` is replaced only after validation and
/// is empty after every refusal.
#[derive(Default)]
pub struct Lowering {
    emitted: Vec<Op>,
}

impl Lowering {
    pub fn new() -> Self {
        Self::default()
    }

    /// Lower one concrete operation. Passing `&Operator` lowers its typed
    /// declaration with zero values; [`Operation::new`] supplies real values.
    pub fn lower(&mut self, operation: impl Into<Operation>) -> Result<Lowered, Fault> {
        self.emitted.clear();
        let operation = operation.into();
        check(&operation.operator, &operation.arguments)?;

        let binding = binding(&operation.operator);
        let lisp = lisp(&operation.operator, &binding);
        let ocode = emit(&operation.operator);
        debug_assert!((MIN_OCODE..=MAX_OCODE).contains(&ocode.len()));
        debug_assert!(ocode.iter().all(is_declared_ocode));
        self.emitted.clone_from(&ocode);

        Ok(Lowered {
            operator: operation.operator.index,
            name: operation.operator.name,
            binding,
            lisp,
            ocode,
        })
    }

    /// Lower one Operator with explicit operands.
    pub fn lower_with(
        &mut self,
        operator: &Operator,
        arguments: &[Operand],
    ) -> Result<Lowered, Fault> {
        self.lower(Operation::new(operator, arguments))
    }

    /// O-code emitted by the last attempt; empty after a refusal.
    pub fn emitted(&self) -> &[Op] {
        &self.emitted
    }

    /// Lower all 64 declarations and judge the count left unlowered against
    /// the required Bar of zero under its `AtMost` sense.
    pub fn lower_all(&mut self) -> LoweringReport {
        let mut lowered = Vec::new();
        let mut unlowered = Vec::new();
        for operator in table() {
            match self.lower(&operator) {
                Ok(item) => lowered.push(item),
                Err(fault) => unlowered.push(Unlowered {
                    operator: operator.index,
                    name: operator.name,
                    fault,
                    emitted: self.emitted.len(),
                }),
            }
        }
        let lowered_count = lowered.len();
        let unlowered_count = unlowered.len();
        let unlowered_bar = Bar::at_most(0);
        let unlowered_verdict = unlowered_bar.judge(unlowered_count);
        LoweringReport {
            lowered,
            unlowered,
            lowered_count,
            unlowered_count,
            unlowered_bar,
            unlowered_verdict,
        }
    }
}

fn check(operator: &Operator, arguments: &[Operand]) -> Result<(), Fault> {
    if arguments.len() != operator.arity as usize {
        return Err(Fault::Arity {
            operator: operator.name,
            declared: operator.arity,
            given: arguments.len(),
        });
    }
    for (position, (argument, &declared)) in arguments.iter().zip(operator.operands).enumerate() {
        let given = argument.kind();
        if given != declared {
            return Err(Fault::OperandType {
                operator: operator.name,
                position,
                declared,
                given,
            });
        }
    }
    Ok(())
}

fn binding(operator: &Operator) -> String {
    format!(
        "phisor-op-{}-{}",
        operator.index,
        operator.name.replace('_', "-")
    )
}

fn lisp(operator: &Operator, binding: &str) -> String {
    let arguments: Vec<String> = (0..operator.arity).map(|i| format!("a{i}")).collect();
    let applied = std::iter::once(operator.index.to_string())
        .chain(arguments.iter().cloned())
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "(define {binding} (lambda ({}) (phisor-apply {applied})))",
        arguments.join(" ")
    )
}

fn emit(operator: &Operator) -> Vec<Op> {
    let mut code = Vec::with_capacity(operator.arity as usize + 4);
    code.push(Op::Lg(DISPATCH_SLOT));
    code.push(Op::Num(operator.index as i64));
    code.extend((0..operator.arity as usize).map(|i| Op::Lp(2 + i)));
    code.push(Op::Call {
        args: operator.arity as usize + 1,
        frame: operator.arity as usize + 2,
    });
    code.push(Op::Fnrn);
    code
}

/// Whether an emitted operation is one of phiclm's closed 55.
pub fn is_declared_ocode(op: &Op) -> bool {
    OCODE_NAMES.contains(&op.name())
}

#[cfg(test)]
mod property;
#[cfg(test)]
mod tests;
