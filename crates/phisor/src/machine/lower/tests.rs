use super::*;
use phical::keyboard::{table, Type};
use phiclm::lisp::{read, Value};

#[test]
fn each_lowering_defines_a_lambda_that_applies_the_declared_arguments() {
    let mut lowering = Lowering::new();
    for operator in table() {
        let lowered = lowering
            .lower(&operator)
            .unwrap_or_else(|fault| panic!("{} refused: {fault:?}", operator.name));
        let forms = read(&lowered.lisp)
            .unwrap_or_else(|error| panic!("{} did not parse: {error}", operator.name));
        assert_eq!(
            forms.len(),
            1,
            "{} emitted more than one form",
            operator.name
        );

        let Value::List(define) = &forms[0] else {
            panic!("{} emitted no form", operator.name)
        };
        assert_eq!(
            define.first(),
            Some(&Value::Sym("define".into())),
            "{} emitted no define",
            operator.name
        );
        assert_eq!(
            define.get(1),
            Some(&Value::Sym(lowered.binding.clone())),
            "{} bound the wrong name",
            operator.name
        );

        let Some(Value::List(lambda)) = define.get(2) else {
            panic!("{} defined no lambda", operator.name)
        };
        assert_eq!(
            lambda.first(),
            Some(&Value::Sym("lambda".into())),
            "{} defined no lambda",
            operator.name
        );
        let Some(Value::List(parameters)) = lambda.get(1) else {
            panic!("{} has no parameter list", operator.name)
        };
        assert_eq!(
            parameters.len(),
            operator.arity as usize,
            "{} has the wrong lambda arity",
            operator.name
        );

        let Some(Value::List(application)) = lambda.get(2) else {
            panic!("{} has no application body", operator.name)
        };
        assert_eq!(
            application.first(),
            Some(&Value::Sym("phisor-apply".into()))
        );
        assert_eq!(application.get(1), Some(&Value::Num(operator.index as f64)));
        assert_eq!(
            &application[2..],
            parameters.as_slice(),
            "{} did not apply its own arguments",
            operator.name
        );
    }
}

#[test]
fn a_type_refusal_clears_every_emitted_operation() {
    let operators = table();
    let identity = &operators[0];
    let mut lowering = Lowering::new();
    assert!(lowering.lower(identity).is_ok());
    assert!(!lowering.emitted().is_empty());

    let refused = lowering.lower_with(identity, &[Operand::Class(0)]);
    assert!(matches!(
        refused,
        Err(Fault::OperandType {
            operator: "identity",
            position: 0,
            declared: Type::Word,
            given: Type::Class,
        })
    ));
    assert_eq!(lowering.emitted().len(), 0);
}

#[test]
fn lower_all_reports_both_counts_and_the_at_most_zero_bar() {
    let report = Lowering::new().lower_all();
    assert_eq!(report.lowered_count, 64);
    assert_eq!(report.lowered.len(), report.lowered_count);
    assert_eq!(report.unlowered_count, 0);
    assert_eq!(report.unlowered.len(), report.unlowered_count);
    assert_eq!(
        report.unlowered_bar,
        Bar {
            value: 0,
            sense: Sense::AtMost
        }
    );
    assert_eq!(report.unlowered_verdict, Judged::Pass);
    assert_eq!(Bar::at_most(0).judge(1), Judged::Miss { shortfall: 1 });
}
