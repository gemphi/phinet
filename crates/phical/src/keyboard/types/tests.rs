use super::*;
use phistd::gen::{for_all, Gen, Seeded, CASES};

mod round;

/// The declared tolerance, written here as the literal the requirement
/// states rather than read from [`PHASOR_EPSILON`], so the test pins the
/// constant instead of agreeing with whatever it happens to hold.
const DECLARED: f64 = 1e-12;

/// One case of Property 1: a value against itself, two values of any of the
/// nine types, or a phasor against a copy nudged by a multiple of the
/// declared tolerance on each component independently.
#[derive(Debug)]
enum EqCase {
    Same(Operand),
    Two(Operand, Operand),
    Nudged { base: c64, factor: [f64; 2] },
}

/// Nudge sizes as multiples of the declared tolerance: three just inside it,
/// two outside. Exactly 1.0 is left out of the drawn cases and checked on its
/// own below, because `base + 1e-12` rounds to a float whose distance from
/// `base` need not be 1e-12 to the bit, and the boundary is only exact when
/// `base` is zero.
const FACTORS: [f64; 5] = [0.0, 0.25, 0.5, 2.0, 8.0];

impl Gen for EqCase {
    fn draw(g: &mut Seeded) -> Self {
        match g.below(4) {
            0 => EqCase::Same(Operand::draw(g)),
            1 => EqCase::Nudged {
                // An ordinary magnitude, so a nudge of 1e-12 is a
                // representable distance rather than a rounding error.
                base: c64::new(g.unit() * 4.0 - 2.0, g.unit() * 4.0 - 2.0),
                factor: [*g.pick(&FACTORS), *g.pick(&FACTORS)],
            },
            _ => EqCase::Two(Operand::draw(g), Operand::draw(g)),
        }
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 1: For any two
// values of any of the nine declared types, the equality rule is reflexive
// and symmetric; two values of the eight types other than phasor are equal
// exactly when their encoded bytes are identical; and two phasors are equal
// exactly when the absolute difference of each of their two components is at
// most 1e-12.
//
// Validates: Requirements 6.16
#[test]
fn the_equality_rule_is_reflexive_and_symmetric() {
    for_all::<EqCase>(0x5EED_0E01, CASES, |case| match case {
        EqCase::Same(a) => equal(a, a),
        EqCase::Two(a, b) => holds(a, b),
        EqCase::Nudged { base, factor } => {
            let a = Operand::Phasor(*base);
            let b = Operand::Phasor(c64::new(
                base.re + factor[0] * DECLARED,
                base.im + factor[1] * DECLARED,
            ));
            let inside = factor[0] <= 1.0 && factor[1] <= 1.0;
            holds(&a, &b) && equal(&a, &b) == inside
        }
    })
    .unwrap();
}

/// Reflexivity, symmetry, and — for the eight types carrying no tolerance —
/// agreement with the values themselves, which is the oracle byte identity
/// is supposed to deliver.
fn holds(a: &Operand, b: &Operand) -> bool {
    let reflexive = equal(a, a) && equal(b, b);
    let symmetric = equal(a, b) == equal(b, a);
    let by_bytes = match (a, b) {
        (Operand::Phasor(_), _) | (_, Operand::Phasor(_)) => true,
        _ => equal(a, b) == (a == b),
    };
    reflexive && symmetric && by_bytes
}

#[test]
fn the_phasor_tolerance_is_the_declared_one() {
    assert_eq!(PHASOR_EPSILON, DECLARED);
    let p = |re: f64, im: f64| Operand::Phasor(c64::new(re, im));
    // At zero the boundary is exact: 1e-12 is representable and adding it to
    // 0.0 rounds nowhere.
    assert!(equal(&p(0.0, 0.0), &p(DECLARED, DECLARED)));
    assert!(equal(&p(0.0, 0.0), &p(-DECLARED, DECLARED)));
    // Just outside, on each component independently.
    let over = DECLARED * (1.0 + 1e-9);
    assert!(over > DECLARED);
    assert!(!equal(&p(0.0, 0.0), &p(over, 0.0)));
    assert!(!equal(&p(0.0, 0.0), &p(0.0, over)));
    assert!(!equal(&p(0.0, 0.0), &p(2.0 * DECLARED, 0.0)));
    // A tolerance is not a free pass: a component a long way off is unequal
    // however close the other one is.
    assert!(!equal(&p(1.0, 0.0), &p(1.0, 1.0)));
}

#[test]
fn a_phasor_is_equal_to_itself_whatever_bits_it_carries() {
    for bits in [
        0x0000_0000_0000_0000u64, // +0.0
        0x8000_0000_0000_0000,    // -0.0
        0x0000_0000_0000_0001,    // the smallest subnormal
        0x7FF0_0000_0000_0000,    // +inf
        0xFFF0_0000_0000_0000,    // -inf
        0x7FF8_0000_0000_0000,    // a quiet NaN
        0xFFF8_0000_0000_1234,    // a quiet NaN carrying a payload
    ] {
        let x = f64::from_bits(bits);
        let a = Operand::Phasor(c64::new(x, x));
        assert!(equal(&a, &a), "a phasor carrying {bits:#018x} is not equal to itself");
    }
    // Both signed zeros are one value under the tolerance, their bits aside.
    assert!(equal(&Operand::Phasor(c64::new(0.0, 0.0)), &Operand::Phasor(c64::new(-0.0, -0.0))));
    // An infinity is equal to itself and not to the other one.
    let inf = Operand::Phasor(c64::new(f64::INFINITY, f64::INFINITY));
    let minus = Operand::Phasor(c64::new(f64::NEG_INFINITY, f64::NEG_INFINITY));
    assert!(equal(&inf, &inf) && !equal(&inf, &minus));
}

#[test]
fn two_types_are_never_equal_however_their_bytes_read() {
    // `class` and `kind` share a layout; they are still two types.
    let c = Operand::Class(7);
    let k = Operand::Kind(7);
    assert_eq!(c.encode(), k.encode());
    assert!(!equal(&c, &k));
    assert!(!equal(&Operand::handle(Type::Row, 3), &Operand::handle(Type::Word, 3)));
}

#[test]
fn the_nine_widths_are_the_declared_ones() {
    let widths = [
        (Type::Phasor, 16usize),
        (Type::Class, 2),
        (Type::Level, 1),
        (Type::Variation, 8),
        (Type::Kind, 2),
        (Type::Act, 4),
        (Type::Frame, 4),
        (Type::Row, 4),
        (Type::Word, 4),
    ];
    assert_eq!(widths.len(), Type::ALL.len());
    for (ty, width) in widths {
        assert_eq!(ty.width(), width, "{}", ty.name());
        assert!((1..=64).contains(&ty.width()));
        for value in [ty.minimum(), ty.maximum(), ty.zero()] {
            assert_eq!(value.encode().len(), width, "{} {:?}", ty.name(), value);
            assert_eq!(value.kind(), ty);
        }
    }
    // The nine names and the nine positions are each used once.
    let mut names: Vec<&str> = Type::ALL.iter().map(|t| t.name()).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), 9);
    for (at, ty) in Type::ALL.iter().enumerate() {
        assert_eq!(ty.index(), at);
    }
}

#[test]
fn a_zero_is_the_width_in_zero_bytes_and_a_level_zero_is_invalid() {
    for ty in Type::ALL {
        assert_eq!(ty.zero().encode(), vec![0u8; ty.width()], "{}", ty.name());
    }
    // level's zero is the invalid 0, not its minimum.
    assert_eq!(Type::Level.zero(), Operand::Level(0));
    assert_eq!(Type::Level.minimum(), Operand::Level(4));
    assert_eq!(Type::Level.maximum(), Operand::Level(6));
}

#[test]
fn a_variation_pads_with_a_zero_byte_and_reads_back_little_endian() {
    let v = Operand::Variation([0x0102, 0x0304, 0x0506], 5);
    assert_eq!(v.encode(), vec![0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x05, 0x00]);
    assert_eq!(Operand::Class(0x0102).encode(), vec![0x02, 0x01]);
    assert_eq!(Operand::handle(Type::Word, 0x0102_0304).encode(), vec![0x04, 0x03, 0x02, 0x01]);
    let z = Operand::Phasor(c64::new(1.0, 2.0));
    assert_eq!(&z.encode()[0..8], &1.0f64.to_bits().to_le_bytes());
    assert_eq!(&z.encode()[8..16], &2.0f64.to_bits().to_le_bytes());
}

#[test]
fn a_slice_of_the_wrong_width_is_refused_by_name() {
    let bad = Operand::decode(Type::Phasor, &[0u8; 8]).unwrap_err();
    assert_eq!(bad.ty, Type::Phasor);
    assert_eq!((bad.declared, bad.given), (16, 8));
    assert!(bad.to_string().contains("phasor"));
    for ty in Type::ALL {
        assert!(Operand::decode(ty, &vec![0u8; ty.width() + 1]).is_err());
    }
}
