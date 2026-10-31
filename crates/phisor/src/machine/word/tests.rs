use super::*;
use crate::ground::Word;
use crate::ir::{Act, Filler, Force, Frame, Role, Tense};
use phical::keyboard::{Family, Inverse, MAX_CLAUSE_DEPTH};
use phiclm::ocode::Program;

const ACT_OPERAND: &[Type] = &[Type::Act];

fn first(arguments: &[Operand], _arena: &mut Arena) -> Result<Operand, Fault> {
    Ok(arguments[0])
}

fn act_operator() -> Operator {
    Operator {
        index: 0,
        name: "nested_act",
        family: Family::Circle,
        arity: 1,
        operands: ACT_OPERAND,
        result: Type::Act,
        idempotent: false,
        inverse: Inverse::None,
        gate: None,
        compute: first,
    }
}

/// An Act whose theme is a clause nested `depth` times.
fn deep(depth: usize) -> Act {
    let mut frame = Frame::new("sing", Tense::Present);
    for _ in 0..depth {
        frame = Frame::new("want", Tense::Present).with(Role::Theme, Filler::clause(frame));
    }
    Act::new(Force::Wish, frame)
}

#[test]
fn an_act_past_the_clause_cap_names_the_operator_position_and_type() {
    let operator = act_operator();
    let mut arena = Arena::new(0x15);
    let error = represent(&operator, 0, &mut arena, &deep(MAX_CLAUSE_DEPTH + 1))
        .expect_err("an over-deep Act must be refused before carriage");

    assert_eq!(error.operator, "nested_act");
    assert_eq!(error.position, 0);
    assert_eq!(error.ty, Type::Act);
    assert_eq!(
        error.source,
        Fault::TooDeep {
            depth: MAX_CLAUSE_DEPTH + 1,
            cap: MAX_CLAUSE_DEPTH,
        }
    );
    let message = error.to_string();
    assert!(message.contains("nested_act"));
    assert!(message.contains("operand 0"));
    assert!(message.contains("type act"));
}

#[test]
fn a_noncanonical_word_names_the_operator_position_and_type() {
    let operator = phical::keyboard::identity();
    let mut arena = Arena::new(0x15);
    let error = represent(&operator, 0, &mut arena, &Word(" word".to_string()))
        .expect_err("untrimmed text must be refused before carriage");

    assert_eq!(error.operator, "identity");
    assert_eq!(error.position, 0);
    assert_eq!(error.ty, Type::Word);
    match &error.source {
        Fault::NotCanonical { text, reason } => {
            assert_eq!(text, " word");
            assert!(reason.starts_with("untrimmed"));
        }
        other => panic!("expected NotCanonical, got {other:?}"),
    }
    let message = error.to_string();
    assert!(message.contains("identity"));
    assert!(message.contains("operand 0"));
    assert!(message.contains("type word"));
}

#[test]
fn the_machine_stamp_is_checked_on_every_handle_resolve() {
    let mut home = Arena::new(0x15);
    let mut foreign = Arena::new(0x16);
    assert_ne!(home.id, foreign.id);

    let handle = home.mint(Type::Word, b"home bytes").unwrap();
    foreign.mint(Type::Word, b"wrong bytes").unwrap();
    let machine = load(&Program::default(), &home);

    assert_eq!(arena_id(&machine), home.id);
    assert_eq!(resolve(&machine, &home, handle).unwrap(), b"home bytes");
    match resolve(&machine, &foreign, handle) {
        Err(Fault::ForeignHandle { expected, given }) => {
            assert_eq!(expected, foreign.id);
            assert_eq!(given, home.id);
        }
        other => panic!("expected ForeignHandle, got {other:?}"),
    }
}
