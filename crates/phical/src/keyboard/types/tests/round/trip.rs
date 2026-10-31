use crate::c64;
use crate::keyboard::types::{equal, Operand, Type};
use phistd::gen::{for_all, CASES};

// Feature: phinet-64-operators-learning-ladder, Property 14: For all
// operands of each of the nine declared types, encoding to bytes under the
// declared little-endian layout then decoding returns a value equal to the
// original under the equality rule, and the encoded length equals that
// type's declared width.
//
// Validates: Requirements 6.13
#[test]
fn encode_decode_round_trips_at_the_declared_width() {
    for_all::<Operand>(0x5EED_0E14, CASES, |x| {
        let ty = x.kind();
        let bytes = x.encode();
        bytes.len() == ty.width()
            && match Operand::decode(ty, &bytes) {
                Ok(back) => equal(&back, x) && back.kind() == ty && back.encode() == bytes,
                Err(_) => false,
            }
    })
    .unwrap();
}

/// The generator draws the three declared edges three times in eight, but a
/// property is not a proof that each one was drawn; the edges are checked
/// here exhaustively as well.
#[test]
fn each_type_min_max_and_zero_round_trip() {
    for ty in Type::ALL {
        for value in [ty.minimum(), ty.maximum(), ty.zero()] {
            let bytes = value.encode();
            assert_eq!(bytes.len(), ty.width(), "{} {:?}", ty.name(), value);
            let back = Operand::decode(ty, &bytes).expect("the declared width decodes");
            assert!(equal(&back, &value), "{} {:?} came back {:?}", ty.name(), value, back);
            assert_eq!(back, value, "{} {:?}", ty.name(), value);
        }
    }
}

/// The phasor bit patterns the round trip either survives or does not. The
/// assertion is on the *bits*, which is stronger than the equality rule: a
/// NaN payload or a negative zero that came back as something else would
/// pass the tolerance and fail here.
#[test]
fn a_phasor_round_trips_bit_for_bit() {
    const BITS: [u64; 10] = [
        0x0000_0000_0000_0000, // +0.0
        0x8000_0000_0000_0000, // -0.0
        0x0000_0000_0000_0001, // the smallest subnormal
        0x800F_FFFF_FFFF_FFFF, // the largest negative subnormal
        0x0010_0000_0000_0000, // the smallest normal
        0x7FF0_0000_0000_0000, // +inf
        0xFFF0_0000_0000_0000, // -inf
        0x7FF8_0000_0000_0000, // a quiet NaN
        0xFFF8_0000_0000_1234, // a quiet NaN carrying a payload
        0x7FF0_0000_0000_0001, // a signalling NaN
    ];
    for re in BITS {
        for im in BITS {
            let z = c64::new(f64::from_bits(re), f64::from_bits(im));
            let bytes = Operand::Phasor(z).encode();
            assert_eq!(bytes.len(), 16);
            // The declared layout, read independently of the decoder.
            assert_eq!(&bytes[0..8], &re.to_le_bytes());
            assert_eq!(&bytes[8..16], &im.to_le_bytes());
            let back = Operand::decode(Type::Phasor, &bytes).expect("16 bytes decode");
            let Operand::Phasor(w) = back else { panic!("a phasor decoded as {back:?}") };
            assert_eq!((w.re.to_bits(), w.im.to_bits()), (re, im));
            assert!(equal(&back, &Operand::Phasor(z)));
        }
    }
}

/// Every byte pattern of the declared width decodes to something, and that
/// something encodes back to the same bytes — the round trip read from the
/// other end, which is what the one pad byte has to be zero for.
#[test]
fn every_byte_pattern_of_the_width_decodes_and_re_encodes() {
    let patterns: [&[u8]; 4] = [
        &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        &[0xFF; 8],
        &[0x00; 8],
        &[0x80, 0x00, 0x7F, 0xFF, 0x01, 0xFE, 0x55, 0xAA],
    ];
    for ty in Type::ALL {
        if ty == Type::Phasor {
            continue; // 16 bytes wide; covered bit for bit above.
        }
        for pattern in patterns {
            let bytes = &pattern[..ty.width()];
            let value = Operand::decode(ty, bytes).expect("the declared width decodes");
            if ty == Type::Variation {
                // The pad byte is not read, so the re-encoding zeroes it.
                let mut expected = bytes.to_vec();
                expected[7] = 0x00;
                assert_eq!(value.encode(), expected, "{}", ty.name());
            } else {
                assert_eq!(value.encode(), bytes, "{}", ty.name());
            }
        }
    }
}
