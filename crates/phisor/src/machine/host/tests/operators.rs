//! The keyboard Operators reached from the Lisp by index.

use super::*;

// ----- phisor-apply: the 64 Operators from the Lisp -----

/// Evaluates `src` on a machine over the fixture host and returns the last
/// value.
fn eval(src: &str) -> Result<Value, String> {
    let mut machine = checked(host()).expect("a checked machine");
    let mut env = phiclm::lisp::Env::new();
    let mut last = Value::Nil;
    for form in read(src)? {
        last = machine.eval(&form, &mut env)?;
    }
    Ok(last)
}

#[test]
fn the_operators_are_reached_from_the_lisp_by_index() {
    // 0 identity: a word handle comes back as itself.
    assert_eq!(eval("(phisor-apply 0 5)"), Ok(Value::Num(5.0)));
    // 11 centroid of two unit phasors a right angle apart: a phasor between them.
    let Ok(Value::List(z)) = eval("(phisor-apply 11 (list 1 0) (list 0 1))") else {
        panic!("centroid answers a phasor");
    };
    let (Value::Num(re), Value::Num(im)) = (&z[0], &z[1]) else { panic!("a phasor is (re im)") };
    assert!((re - im).abs() < 1e-9 && *re > 0.0, "the centroid lies between: {re} {im}");
    // The lowering's own lambda, defined then applied, reaches the same door.
    let lowered = Lowering::new().lower(&table()[11]).expect("centroid lowers");
    let src = format!("{} (phisor-op-11-centroid (list 1 0) (list 0 1))", lowered.lisp);
    assert_eq!(eval(&src), eval("(phisor-apply 11 (list 1 0) (list 0 1))"));
}

#[test]
fn a_wrong_call_is_an_error_naming_the_operator_not_a_panic() {
    let e = eval("(phisor-apply 11 (list 1 0))").expect_err("one operand short");
    assert!(e.contains("centroid") && e.contains("given 1"), "{e}");
    let e = eval("(phisor-apply 11 7 (list 0 1))").expect_err("a number is not a phasor");
    assert!(e.contains("centroid") && e.contains("operand 0"), "{e}");
    let e = eval("(phisor-apply 64 1)").expect_err("the keyboard ends at 63");
    assert!(e.contains("no Operator at index 64"), "{e}");
}
