//! Fixed-width carriage between phical's typed operands and phiclm's one
//! `i64` machine word.
//!
//! The type declaration decides both directions. A phasor is its two `f64`
//! bit patterns in two words; every other declared type is at most eight
//! bytes and occupies one zero-padded word. Bytes within a word are always
//! little-endian. Handle payloads remain four-byte indices, while the
//! [`ArenaId`] stamped into the machine's global vector says which Arena
//! those indices belong to.

use std::fmt;

use phical::keyboard::{Arena, ArenaId, BadWidth, Canon, Fault, Handle, Operand, Operator, Type};
use phiclm::machine::Machine;
use phiclm::ocode::Program;

/// phiclm's declared machine-word width.
pub const MACHINE_WORD_BYTES: usize = std::mem::size_of::<i64>();

/// The global holding the Arena identity for every handle in the machine.
/// The dispatcher owns the first phisor global; carriage owns the next one.
pub const ARENA_ID_SLOT: usize = super::DISPATCH_SLOT + 1;

/// A store access that cannot read or write the type's complete declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreError {
    Outside {
        at: usize,
        words: usize,
        store_words: usize,
    },
    Layout {
        ty: Type,
        width: usize,
    },
    Width(BadWidth),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Outside {
                at,
                words,
                store_words,
            } => write!(
                f,
                "{words} machine words at {at} lie outside a {store_words}-word store"
            ),
            StoreError::Layout { ty, width } => write!(
                f,
                "{} declares {width} bytes, which has no machine-word carriage",
                ty.name()
            ),
            StoreError::Width(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for StoreError {}

/// A canonical value that cannot become the fixed-width operand an Operator
/// declares. The context is retained around the originating keyboard fault,
/// so `TooDeep` and `NotCanonical` name the Operator, position and type at
/// the lowering boundary rather than losing where the value was used.
#[derive(Clone, Debug, PartialEq)]
pub struct Unrepresentable {
    pub operator: &'static str,
    pub position: usize,
    pub ty: Type,
    pub source: Fault,
}

impl fmt::Display for Unrepresentable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operator {} operand {} of type {} is unrepresentable: {:?}",
            self.operator,
            self.position,
            self.ty.name(),
            self.source
        )
    }
}

impl std::error::Error for Unrepresentable {}

/// The smallest whole number of machine words holding `ty`'s width.
pub const fn word_count(ty: Type) -> usize {
    (ty.width() + MACHINE_WORD_BYTES - 1) / MACHINE_WORD_BYTES
}

/// Load a phiclm machine and stamp the Arena identity after the program's own
/// global initialisers have run.
pub fn load(program: &Program, arena: &Arena) -> Machine {
    let mut machine = Machine::load(program);
    machine.mem[ARENA_ID_SLOT] = id_word(arena.id);
    machine
}

/// The Arena identity currently stamped in the machine's globals.
pub fn arena_id(machine: &Machine) -> ArenaId {
    word_id(machine.mem[ARENA_ID_SLOT])
}

/// Resolve a machine handle against `arena`. The stamp is read and checked
/// on every call; no Arena identity is cached beside the machine.
pub fn resolve<'a>(machine: &Machine, arena: &'a Arena, handle: Handle) -> Result<&'a [u8], Fault> {
    arena.resolve_from(arena_id(machine), handle)
}

/// Canonicalise one handle-backed operand in its Operator context. This is
/// the boundary before lowering: a value that cannot canonicalise is refused
/// as [`Unrepresentable`] and no machine word is emitted for it.
pub fn represent<C: Canon>(
    operator: &Operator,
    position: usize,
    arena: &mut Arena,
    value: &C,
) -> Result<Operand, Unrepresentable> {
    let declared = match operator.operands.get(position).copied() {
        Some(ty) => ty,
        None => {
            return Err(unrepresentable(
                operator,
                position,
                C::TYPE,
                Fault::Arity {
                    operator: operator.name,
                    declared: operator.arity,
                    given: position + 1,
                },
            ))
        }
    };
    if declared != C::TYPE {
        return Err(unrepresentable(
            operator,
            position,
            declared,
            Fault::OperandType {
                operator: operator.name,
                position,
                declared,
                given: C::TYPE,
            },
        ));
    }
    if !carriable(declared) {
        return Err(unrepresentable(
            operator,
            position,
            declared,
            Fault::NotCanonical {
                text: declared.name().to_string(),
                reason: format!(
                    "the declared {}-byte layout has no exact carriage",
                    declared.width()
                ),
            },
        ));
    }
    let handle = crate::ground::mint(arena, value)
        .map_err(|source| unrepresentable(operator, position, declared, source))?;
    Ok(Operand::handle(declared, handle.index))
}

/// Write `operand` at word address `at`, returning the exact words consumed.
pub fn write(store: &mut Vec<i64>, at: usize, operand: &Operand) -> Result<usize, StoreError> {
    let ty = operand.kind();
    let words = layout(ty)?;
    bounds(store.len(), at, words)?;
    match operand {
        Operand::Phasor(z) => {
            // These casts change signed interpretation only; all 64 bits stay.
            store[at] = i64::from_le_bytes(z.re.to_bits().to_le_bytes());
            store[at + 1] = i64::from_le_bytes(z.im.to_bits().to_le_bytes());
        }
        _ => {
            let bytes = operand.encode();
            let mut word = [0u8; MACHINE_WORD_BYTES];
            word[..bytes.len()].copy_from_slice(&bytes);
            store[at] = i64::from_le_bytes(word);
        }
    }
    Ok(words)
}

/// Read type `ty` at word address `at`, returning the value and exact words
/// consumed under the same width, byte order and layout used by [`write`].
pub fn read(store: &[i64], at: usize, ty: Type) -> Result<(Operand, usize), StoreError> {
    let words = layout(ty)?;
    bounds(store.len(), at, words)?;
    let operand = if ty == Type::Phasor {
        let component = |word: i64| f64::from_bits(u64::from_le_bytes(word.to_le_bytes()));
        Operand::Phasor(phical::c64::new(
            component(store[at]),
            component(store[at + 1]),
        ))
    } else {
        let word = store[at].to_le_bytes();
        Operand::decode(ty, &word[..ty.width()]).map_err(StoreError::Width)?
    };
    Ok((operand, words))
}

fn layout(ty: Type) -> Result<usize, StoreError> {
    match (ty, ty.width()) {
        (Type::Phasor, 16) => Ok(2),
        (_, 1..=MACHINE_WORD_BYTES) => Ok(1),
        (_, width) => Err(StoreError::Layout { ty, width }),
    }
}

fn carriable(ty: Type) -> bool {
    layout(ty).is_ok()
}

fn bounds(len: usize, at: usize, words: usize) -> Result<(), StoreError> {
    match at.checked_add(words).filter(|end| *end <= len) {
        Some(_) => Ok(()),
        None => Err(StoreError::Outside {
            at,
            words,
            store_words: len,
        }),
    }
}

fn unrepresentable(
    operator: &Operator,
    position: usize,
    ty: Type,
    source: Fault,
) -> Unrepresentable {
    Unrepresentable {
        operator: operator.name,
        position,
        ty,
        source,
    }
}

fn id_word(id: ArenaId) -> i64 {
    i64::from_le_bytes(id.0.to_le_bytes())
}

fn word_id(word: i64) -> ArenaId {
    ArenaId(u64::from_le_bytes(word.to_le_bytes()))
}

#[cfg(test)]
mod property;
#[cfg(test)]
mod tests;
