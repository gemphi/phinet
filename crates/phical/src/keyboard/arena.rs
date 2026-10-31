//! The interning byte arena: four append-only tables, one handle minted per
//! distinct canonical byte form, and the stamp that keeps one Arena's handles
//! from being misread against another.
//!
//! The Arena stores **canonical bytes**, never Rust types. phical cannot know
//! `phisor::ir::Act` — phisor depends on phical and not the reverse — so
//! phisor writes an `Act` to its canonical form and hands the bytes over; the
//! Arena interns bytes and never learns what they mean. The one thing it
//! guarantees is the thing [`equal`](super::equal) rests on: two structurally
//! equal values share one handle, because minting a form already present
//! returns the handle already minted, so byte-equal handles are structural
//! equality of what they name (Requirement 6 criterion 16).
//!
//! The Arena is append-only and never removes, so within one Arena a handle
//! once minted stays valid for the Arena's life: interning more values moves
//! no index and invalidates no handle. Across Arenas a stray handle is caught
//! rather than misread — every handle carries nothing but a `u32` index, and
//! resolving it against an Arena whose [`ArenaId`] differs from the one it was
//! minted under returns [`Fault::ForeignHandle`] rather than wrong bytes.

use std::collections::HashMap;

use super::Type;

pub use super::Handle;

/// Minted once per Arena from the run's declared seed, via
/// [`phistd::mix64`]. Stamped into the machine's global vector at load and
/// checked on every resolve, so a handle minted against one Arena cannot be
/// silently misread against another.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArenaId(pub u64);

/// The declared cap on clause nesting, which bounds an Act's canonical form
/// and gives Requirement 7 criterion 13's refusal path a real trigger.
pub const MAX_CLAUSE_DEPTH: usize = 8;

/// What phisor implements for each of its four handle types. The trait lives
/// in phical so the Arena can be generic over it; the implementations live in
/// phisor, which owns `Act`, `Frame`, `Row` and `Word`.
///
/// A nested value becomes a nested *handle* inside the bytes, so
/// `Filler::Clause(Box<Frame>)` — a recursive tree — becomes a DAG of
/// fixed-width handles rather than an inline struct of unbounded size.
pub trait Canon: Sized {
    /// Which of the four handle types this value interns as.
    const TYPE: Type;

    /// Write the canonical form, minting handles for any nested value.
    fn write(&self, arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault>;

    /// Read the value back from its canonical bytes.
    fn read(arena: &Arena, bytes: &[u8]) -> Result<Self, Fault>;
}

/// One interning table: the byte forms in mint order, and the index of each
/// so a repeat form finds the handle already minted without a scan.
#[derive(Clone, Debug, Default)]
struct Table {
    bytes: Vec<Vec<u8>>,
    index: HashMap<Vec<u8>, u32>,
}

impl Table {
    /// The index of `canon`, minting a fresh one only when the form is new.
    fn intern(&mut self, canon: &[u8]) -> u32 {
        if let Some(&i) = self.index.get(canon) {
            return i;
        }
        let i = self.bytes.len() as u32;
        self.bytes.push(canon.to_vec());
        self.index.insert(canon.to_vec(), i);
        i
    }
}

/// The interning byte arena. It stores canonical byte forms, never Rust
/// types, and never removes: every table is append-only.
#[derive(Clone, Debug)]
pub struct Arena {
    pub id: ArenaId,
    words: Table,
    rows: Table,
    frames: Table,
    acts: Table,
}

impl Arena {
    /// A fresh, empty Arena whose [`ArenaId`] is minted from the run's
    /// declared `seed` via [`phistd::mix64`], so two runs with the same seed
    /// stamp the same id and two with different seeds do not.
    pub fn new(seed: u64) -> Self {
        Arena {
            id: ArenaId(phistd::mix64(seed)),
            words: Table::default(),
            rows: Table::default(),
            frames: Table::default(),
            acts: Table::default(),
        }
    }

    /// The table a handle type names. The four handle types are the only ones
    /// the Arena interns; an inline type is a caller error, not a fault, and
    /// panics.
    fn table(&mut self, ty: Type) -> &mut Table {
        match ty {
            Type::Word => &mut self.words,
            Type::Row => &mut self.rows,
            Type::Frame => &mut self.frames,
            Type::Act => &mut self.acts,
            inline => panic!("{} is an inline type, not interned", inline.name()),
        }
    }

    fn table_ref(&self, ty: Type) -> &Table {
        match ty {
            Type::Word => &self.words,
            Type::Row => &self.rows,
            Type::Frame => &self.frames,
            Type::Act => &self.acts,
            inline => panic!("{} is an inline type, not interned", inline.name()),
        }
    }

    /// Mint a handle for a canonical byte form. Interning: a form already
    /// present returns the handle already minted, so two structurally equal
    /// values share one handle and byte equality of handles is structural
    /// equality of what they name.
    pub fn mint(&mut self, ty: Type, canon: &[u8]) -> Result<Handle, Fault> {
        let index = self.table(ty).intern(canon);
        Ok(Handle::new(ty, index))
    }

    /// The byte form a handle names. A handle from another Arena — one whose
    /// [`ArenaId`] differs — is refused as [`Fault::ForeignHandle`] rather
    /// than misread; an index past the table's length (unreachable within one
    /// append-only Arena, kept as a guard) is [`Fault::NoSuchHandle`].
    pub fn resolve(&self, h: Handle) -> Result<&[u8], Fault> {
        let table = self.table_ref(h.ty);
        match table.bytes.get(h.index as usize) {
            Some(bytes) => Ok(bytes),
            None => Err(Fault::NoSuchHandle { handle: h, len: table.bytes.len() }),
        }
    }

    /// Resolve a handle minted against the Arena `from`. The ids are compared
    /// first, so a handle carried over from a different Arena returns
    /// [`Fault::ForeignHandle`] instead of indexing wrong bytes.
    pub fn resolve_from(&self, from: ArenaId, h: Handle) -> Result<&[u8], Fault> {
        if from != self.id {
            return Err(Fault::ForeignHandle { expected: self.id, given: from });
        }
        self.resolve(h)
    }

    /// Per-table lengths in the order `[words, rows, frames, acts]`, recorded
    /// so a run reproduces (Requirement 9 criterion 15).
    pub fn epoch(&self) -> [usize; 4] {
        [
            self.words.bytes.len(),
            self.rows.bytes.len(),
            self.frames.bytes.len(),
            self.acts.bytes.len(),
        ]
    }
}

/// The ways an operand, a handle or a canonical form can be wrong. Each
/// variant names the fields its criterion requires, so a fault is a complete
/// account of what went wrong rather than a bare code.
#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// Requirement 6 criterion 8: an operator given the wrong number of
    /// operands.
    Arity { operator: &'static str, declared: u8, given: usize },
    /// Requirement 6 criterion 8: an operand of the wrong type at a position.
    OperandType { operator: &'static str, position: usize, declared: Type, given: Type },
    /// A handle minted against a different Arena, caught by its [`ArenaId`].
    ForeignHandle { expected: ArenaId, given: ArenaId },
    /// A handle past the table's length — unreachable within one append-only
    /// Arena, kept as a guard.
    NoSuchHandle { handle: Handle, len: usize },
    /// Requirement 7 criterion 13, first trigger: an Act nested past
    /// [`MAX_CLAUSE_DEPTH`].
    TooDeep { depth: usize, cap: usize },
    /// Requirement 7 criterion 13, second trigger: text that will not
    /// canonicalise.
    NotCanonical { text: String, reason: String },
}

#[cfg(test)]
mod tests;
