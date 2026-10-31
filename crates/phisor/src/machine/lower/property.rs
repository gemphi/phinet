use super::*;
use phical::keyboard::{table, Type};
use phistd::gen::{for_all, Gen, Seeded, CASES};
use std::collections::BTreeSet;

/// One ill-typed operation: either the argument count differs or exactly one
/// argument has one of the other eight declared types.
#[derive(Debug)]
enum IllTyped {
    Arity {
        index: usize,
        len: usize,
    },
    Type {
        index: usize,
        position: usize,
        given: Type,
    },
}

impl Gen for IllTyped {
    fn draw(g: &mut Seeded) -> Self {
        let operators = table();
        let index = g.below(operators.len() as u64) as usize;
        let operator = &operators[index];
        if g.below(2) == 0 {
            let mut len = g.below(6) as usize;
            if len == operator.arity as usize {
                len = (len + 1) % 6;
            }
            IllTyped::Arity { index, len }
        } else {
            let position = g.below(operator.arity as u64) as usize;
            let declared = operator.operands[position];
            let others: Vec<Type> = Type::ALL.into_iter().filter(|ty| *ty != declared).collect();
            IllTyped::Type {
                index,
                position,
                given: *g.pick(&others),
            }
        }
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 18: Every Operator
// lowers to a bounded O-code sequence drawn from the 55. All 64 Operators are
// exhaustive for the 1..=64 bound and closed-set membership; 256 ill-typed
// tuples establish that refusal emits zero operations.
//
// **Validates: Requirements 7.5**
#[test]
fn property_18_every_operator_lowers_to_a_bounded_ocode_sequence_drawn_from_the_55() {
    let operators = table();
    assert_eq!(operators.len(), 64);
    assert_eq!(OCODE_NAMES.len(), 55);
    assert_eq!(OCODE_NAMES.into_iter().collect::<BTreeSet<_>>().len(), 55);

    let mut lowering = Lowering::new();
    for operator in &operators {
        let arguments: Vec<Operand> = operator.operands.iter().map(|ty| ty.zero()).collect();
        let lowered = lowering
            .lower_with(operator, &arguments)
            .unwrap_or_else(|fault| {
                panic!(
                    "{} refused a well-typed operation: {fault:?}",
                    operator.name
                )
            });
        assert!(
            (MIN_OCODE..=MAX_OCODE).contains(&lowered.ocode.len()),
            "{} emitted {} operations",
            operator.name,
            lowered.ocode.len()
        );
        assert!(
            lowered.ocode.iter().all(is_declared_ocode),
            "{} emitted outside phiclm's 55",
            operator.name
        );
    }

    for_all::<IllTyped>(0x18, CASES, |case| {
        let operators = table();
        let mut lowering = Lowering::new();
        let refused = match case {
            IllTyped::Arity { index, len } => {
                let operator = &operators[*index];
                let filler = operator
                    .operands
                    .first()
                    .copied()
                    .unwrap_or(Type::Word)
                    .zero();
                lowering.lower_with(operator, &vec![filler; *len])
            }
            IllTyped::Type {
                index,
                position,
                given,
            } => {
                let operator = &operators[*index];
                let mut arguments: Vec<Operand> =
                    operator.operands.iter().map(|ty| ty.zero()).collect();
                arguments[*position] = given.zero();
                lowering.lower_with(operator, &arguments)
            }
        };
        refused.is_err() && lowering.emitted().is_empty()
    })
    .unwrap();
}
