//! Writing the frozen operand set: the one seeded run that produced
//! [`super::TEXT`], kept beside the loader so the record is reproducible
//! rather than merely asserted.
//!
//! `the_file_is_exactly_what_the_seeded_run_wrote` holds the file to
//! [`generate`], so this is not a path that ran once and drifted. A
//! re-recording is `std::fs::write(PATH, generate(SEED))` with the previous
//! fingerprint, the new one and the stated reason written beside the pin in
//! `phinet/tests/structural/allowlist.golden` (R11.10).

use super::{hex, PER_TYPE};
use crate::c64;
use crate::keyboard::types::draw::arbitrary;
use crate::keyboard::{Operand, Type};
use phistd::gen::Seeded;

/// The record the seeded run writes: the header, then each type's declared
/// edges and bit shapes, then drawn operands until the type holds
/// [`PER_TYPE`] distinct encodings.
///
/// Each type draws from its own case of `seed`, so adding a type or raising
/// [`PER_TYPE`] moves no other type's operands.
pub fn generate(seed: u64) -> String {
    let mut out = header(seed);
    for ty in Type::ALL {
        for (role, operand) in rows(seed, ty) {
            out.push_str(ty.name());
            out.push('\t');
            out.push_str(&role);
            out.push('\t');
            out.push_str(&hex(&operand.encode()));
            out.push('\n');
        }
    }
    out
}

/// One type's rows: the declared ones, then drawn ones, each a row only if
/// its encoding is new. A repeated encoding would leave the type short of
/// [`PER_TYPE`] operands while looking as though it had them, so it joins
/// the row it repeats instead — `minimum+zero`, which is the honest reading
/// for the six types whose declared minimum *is* their zero.
fn rows(seed: u64, ty: Type) -> Vec<(String, Operand)> {
    let mut kept: Vec<(String, Operand)> = Vec::new();
    let mut seen: Vec<Vec<u8>> = Vec::new();
    for (role, operand) in declared(ty) {
        keep(&mut kept, &mut seen, role, operand);
    }
    let mut g = Seeded::for_case(seed, ty.index());
    for _ in 0..10_000 {
        if kept.len() >= PER_TYPE {
            return kept;
        }
        keep(&mut kept, &mut seen, "drawn", arbitrary(&mut g, ty));
    }
    panic!("{} offers fewer than {PER_TYPE} distinct operands", ty.name())
}

/// A row for the operand, or `+role` on the row that already holds its
/// bytes. A drawn operand that repeats one is dropped rather than joined:
/// `drawn` says nothing a role does not already say.
fn keep(kept: &mut Vec<(String, Operand)>, seen: &mut Vec<Vec<u8>>, role: &str, operand: Operand) {
    let bytes = operand.encode();
    if let Some(at) = seen.iter().position(|held| *held == bytes) {
        if role != "drawn" {
            kept[at].0.push('+');
            kept[at].0.push_str(role);
        }
        return;
    }
    seen.push(bytes);
    kept.push((role.to_string(), operand));
}

/// The operands a type is in the set for, whatever a draw offers: its
/// declared minimum, its declared maximum and its zero for all nine, and for
/// `phasor` the four bit shapes R7.6 turns on — the other signed zero, a
/// subnormal, an infinity and a NaN bit pattern.
fn declared(ty: Type) -> Vec<(&'static str, Operand)> {
    // `minimum`, `maximum`, `zero` is the order R7.6 names them in.
    let mut out = vec![("minimum", ty.minimum()), ("maximum", ty.maximum()), ("zero", ty.zero())];
    if ty == Type::Phasor {
        out.push(("negative_zero", phasor(0x8000_0000_0000_0000, 0x8000_0000_0000_0000)));
        out.push(("subnormal", phasor(0x0000_0000_0000_0001, 0x800F_FFFF_FFFF_FFFF)));
        out.push(("infinity", phasor(0x7FF0_0000_0000_0000, 0xFFF0_0000_0000_0000)));
        out.push(("nan", phasor(0x7FF8_0000_0000_0000, 0xFFF8_0000_0000_1234)));
    }
    out
}

/// A phasor from its two components' bit patterns, which is the only way to
/// name a particular NaN payload or a particular subnormal exactly.
fn phasor(re: u64, im: u64) -> Operand {
    Operand::Phasor(c64::new(f64::from_bits(re), f64::from_bits(im)))
}

/// The record's header. The prose carries the two numbers the record was
/// generated under, and `the_header_states_the_numbers_it_was_written_under`
/// holds the prose to them.
fn header(seed: u64) -> String {
    format!(
        "{PREAMBLE}# seed {seed:#018x}, at least {PER_TYPE} distinct operands of each of \
         the nine types\n#\n# type\trole\tbytes\n"
    )
}

const PREAMBLE: &str = "# The frozen operand set (R7.6): the whole input to the agreement check,
# generated once by a seeded run and then frozen. The loader, the format and
# the generator that wrote it are in phical::keyboard::frozen; a re-recording
# writes the previous fingerprint, the new one and the stated reason.
#
# One operand per line: its type, why it is in the set, and its canonical
# little-endian encoding in lower-case hex. Each type's declared minimum, its
# declared maximum and its zero are among its operands. `phistd::mix64`
# folded over these bytes is pinned in
# phinet/tests/structural/allowlist.golden, so a modification fails the run.
#
";
