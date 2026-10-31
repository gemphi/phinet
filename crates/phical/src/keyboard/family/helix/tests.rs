//! Unit tests for the Helix family: the table is the declared six rows at
//! indices 22 to 27, a code is minted from a channel read modulo the 64, the
//! five reads agree field for field with `phical::helix::Codon`, the helix
//! position survives its handle to the bit, and a handle naming anything but
//! a code is refused rather than read as a channel.

use super::*;
use crate::helix::Codon;

use super::super::super::arena::Arena;
use super::super::super::operator::{apply, Family, Inverse};

/// A seed with no meaning of its own; every Arena here shares it.
const SEED: u64 = 0x4E11_C0DE;

fn row(name: &str) -> Operator {
    table().into_iter().find(|o| o.name == name).expect("the row is declared")
}

/// The bytes a word result names, read back out of the Arena that minted
/// them.
fn bytes_of(arena: &Arena, o: Operand) -> Vec<u8> {
    match o {
        Operand::Word(h) => arena.resolve(h).expect("the handle was just minted").to_vec(),
        other => panic!("expected a word result, got {other:?}"),
    }
}

fn text_of(arena: &Arena, o: Operand) -> String {
    String::from_utf8(bytes_of(arena, o)).expect("the text was interned as UTF-8")
}

fn re_of(o: Operand) -> f64 {
    match o {
        Operand::Phasor(z) => z.re,
        other => panic!("expected a phasor result, got {other:?}"),
    }
}

/// The code handle for channel `k`, the operand every field read takes.
fn code_of(arena: &mut Arena, k: u16) -> Operand {
    apply(&row("codon_of"), &[Operand::Class(k)], arena).expect("a class is always a channel")
}

#[test]
fn the_helix_table_is_six_rows_at_indices_twenty_two_to_twenty_seven() {
    let t = table();
    assert_eq!(t.len(), 6, "Helix declares six Operators");
    for (at, r) in t.iter().enumerate() {
        assert_eq!(r.index as usize, 22 + at, "{:?} is out of index order", r.name);
        assert_eq!(r.family, Family::Helix, "{:?} is not in Helix", r.name);
        assert_eq!(r.arity, 1, "{:?} is arity 1", r.name);
        assert_eq!(r.operands.len(), r.arity as usize, "{:?} arity", r.name);
        assert!(!r.idempotent, "{:?} declares no idempotence", r.name);
        assert_eq!(r.inverse, Inverse::None, "{:?} declares no inverse", r.name);
        assert!(r.gate.is_none(), "{:?} carries no gate until task 3.13", r.name);
    }
    assert_eq!(t[0].name, "codon_of", "codon_of opens the family at index 22");
    assert_eq!(t[5].name, "position", "position closes it at index 27");
}

#[test]
fn a_code_is_minted_from_a_channel_read_modulo_the_sixty_four() {
    let mut arena = Arena::new(SEED);

    // The canonical form is the tag then the channel, little-endian.
    let code = code_of(&mut arena, 27);
    assert_eq!(bytes_of(&arena, code), vec![CODE_TAG, 27, 0]);
    assert_eq!(bytes_of(&arena, code).len(), CODE_WIDTH);

    // Interning: the same channel twice is one handle, a different channel
    // another, so byte-equal handles mean equal codes.
    assert_eq!(code_of(&mut arena, 27), code);
    assert_ne!(code_of(&mut arena, 28), code);

    // 64 + 3 and 3 name the same channel, so every u16 a class may hold is
    // a legal channel and no drawn operand can fault or panic.
    assert_eq!(code_of(&mut arena, 3), code_of(&mut arena, 3 + CODONS as u16));
    let big = Operand::Class(u16::MAX);
    assert!(apply(&row("codon_of"), &[big], &mut arena).is_ok());
}

#[test]
fn the_five_reads_agree_with_the_codon_fields() {
    let mut arena = Arena::new(SEED);
    let (codon, hexagram) = (row("codon"), row("hexagram"));
    let (degree, order) = (row("degree"), row("order"));

    for k in 0..CODONS {
        let want = Codon::of(k);
        let code = code_of(&mut arena, k as u16);

        let got = apply(&codon, &[code], &mut arena).unwrap();
        assert_eq!(text_of(&arena, got), want.codon, "the codon of channel {k}");
        let got = apply(&hexagram, &[code], &mut arena).unwrap();
        assert_eq!(text_of(&arena, got), want.hexagram, "the hexagram of channel {k}");
        let got = apply(&degree, &[code], &mut arena).unwrap();
        assert_eq!(re_of(got), want.degree as f64, "the degree of channel {k}");
        let got = apply(&order, &[code], &mut arena).unwrap();
        assert_eq!(re_of(got), want.order as f64, "the order of channel {k}");
    }

    // The ends of the table, as the correspondence names them.
    let (first, last) = (code_of(&mut arena, 0), code_of(&mut arena, 63));
    let opens = apply(&codon, &[first], &mut arena).unwrap();
    let closes = apply(&codon, &[last], &mut arena).unwrap();
    assert_eq!(text_of(&arena, opens), "AAA");
    assert_eq!(text_of(&arena, closes), "TTT");
}

#[test]
fn the_position_handle_carries_the_three_coordinates_to_the_bit() {
    let mut arena = Arena::new(SEED);
    let position = row("position");

    for k in 0..CODONS {
        let code = code_of(&mut arena, k as u16);
        let got = apply(&position, &[code], &mut arena).unwrap();
        let bytes = bytes_of(&arena, got);
        assert_eq!(bytes.len(), POSITION_WIDTH, "three f64 at channel {k}");
        let read = coordinates(&bytes).expect("the canonical width reads back");
        // Bit identity, not a tolerance: the handle carries the coordinates
        // the existing function returned, with no narrowing and no rounding.
        let want = Codon::of(k).position;
        for axis in 0..3 {
            assert_eq!(read[axis].to_bits(), want[axis].to_bits(), "axis {axis} at {k}");
        }
    }

    // Bytes of any other length are not a position.
    assert_eq!(coordinates(&[0u8; POSITION_WIDTH - 1]), None);
}

#[test]
fn a_handle_naming_anything_but_a_code_is_refused() {
    let mut arena = Arena::new(SEED);
    let (codon, degree) = (row("codon"), row("degree"));
    let code = code_of(&mut arena, 5);

    // The codon's own text is a word handle of the declared type, so the
    // call is well typed — and refused, because the tag is not there. That
    // is also why no row here declares idempotence.
    let bases = apply(&codon, &[code], &mut arena).unwrap();
    match apply(&codon, &[bases], &mut arena) {
        Err(Fault::NotCanonical { text, reason }) => {
            let offered: String =
                Codon::of(5).codon.bytes().map(|b| format!("{b:02x}")).collect();
            assert_eq!(text, offered, "the bytes offered are named");
            assert!(reason.contains("helix code"), "the expected form is named: {reason}");
        }
        other => panic!("expected a refusal, got {other:?}"),
    }
    // A position's 24 bytes are refused the same way.
    let place = apply(&row("position"), &[code], &mut arena).unwrap();
    assert!(matches!(
        apply(&degree, &[place], &mut arena),
        Err(Fault::NotCanonical { .. })
    ));

    // A handle naming nothing in this Arena is the Arena's own fault, not a
    // wrong reading.
    let stray = Operand::handle(Type::Word, u32::MAX);
    assert!(matches!(
        apply(&degree, &[stray], &mut arena),
        Err(Fault::NoSuchHandle { .. })
    ));
}
