use super::*;
use phical::keyboard::types::draw::arbitrary;
use phical::keyboard::{equal, Arena, Operand, Type};
use phiclm::ocode::Program;
use phistd::gen::{for_all, Gen, Seeded, CASES};

/// One operand of every declared type. Drawing all nine in every case makes
/// the property universal over the table rather than hoping a random type is
/// selected; the phasor draw is weighted toward awkward IEEE-754 patterns.
#[derive(Debug)]
struct Case([Operand; 9]);

impl Gen for Case {
    fn draw(g: &mut Seeded) -> Self {
        Case(std::array::from_fn(|i| {
            let ty = Type::ALL[i];
            match g.below(8) {
                0 => ty.minimum(),
                1 => ty.maximum(),
                2 => ty.zero(),
                _ => arbitrary(g, ty),
            }
        }))
    }
}

// Feature: phinet-64-operators-learning-ladder, Property 15: A value
// survives the machine's store, across as many words as its width needs.
// The phasor generator gives three draws in four to signed zero, subnormal,
// infinity, NaN and other awkward bit patterns.
//
// **Validates: Requirements 7.7, 7.12**
#[test]
fn property_15_a_value_survives_the_machines_store_across_as_many_words_as_its_width_needs() {
    let run = for_all::<Case>(0x15, CASES, |case| {
        let arena = Arena::new(0x15);
        let mut machine = load(&Program::default(), &arena);
        let at = phiclm::machine::GLOBAL + 8;
        const UNTOUCHED: i64 = 0x1357_2468_1020_3040;

        case.0.iter().all(|operand| {
            machine.mem[at..at + 3].fill(UNTOUCHED);
            let expected = (operand.kind().width() + MACHINE_WORD_BYTES - 1) / MACHINE_WORD_BYTES;
            let Ok(written) = write(&mut machine.mem, at, operand) else {
                return false;
            };
            let Ok((read_back, read_words)) = read(&machine.mem, at, operand.kind()) else {
                return false;
            };

            written == expected
                && read_words == expected
                && word_count(operand.kind()) == expected
                && machine.mem[at + expected] == UNTOUCHED
                && equal(operand, &read_back)
                // The equality rule permits a phasor tolerance; carriage is
                // stricter and must preserve every component bit exactly.
                && operand.encode() == read_back.encode()
                && match operand {
                    Operand::Phasor(_) => expected == 2,
                    _ => expected == 1,
                }
        })
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
