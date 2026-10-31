//! The frozen operand set: the whole input to the agreement check, written
//! once by a seeded run and then left alone.
//!
//! Requirement 7 criterion 6 asks for at least 8 operands of each of the
//! nine declared types, each type's declared minimum, its declared maximum
//! and its zero among them, so at least 72 operands overall, and asks that
//! the set be written before the first agreement run. So it is a file —
//! [`PATH`] — rather than a generator call: a generator reseeded between two
//! runs would hand phiclm's machine and the Rust reader different work, and
//! the comparison the criterion asks for would be of two different
//! questions rather than of two answers to one.
//!
//! The file is a TSV of three tab-separated columns, one operand per line,
//! `#` starting a comment and a blank line carrying nothing:
//!
//! | column | what |
//! |---|---|
//! | type | one of the nine names of [`Type::name`] |
//! | role | why the operand is in the set: `minimum`, `maximum`, `zero`, one of the four phasor bit shapes, or `drawn`. Two roles held by one value are joined with `+`, as `minimum+zero` for the six types whose declared minimum *is* their zero |
//! | bytes | the operand's canonical little-endian encoding, lower-case hex, exactly `2 * ty.width()` digits |
//!
//! An operand is carried as its encoded bytes and not as a decimal, because
//! four of the values this set exists for — the other signed zero, the
//! subnormal, the infinity and the NaN bit pattern — have either no decimal
//! spelling or a lossy one, and those are exactly where a float carried as
//! bits either survives or does not.
//!
//! The seeded run that wrote the file is [`record::generate`], kept beside
//! the loader so the record is reproducible rather than merely asserted.
//!
//! [`FrozenOperands::fingerprint`] is [`phistd::mix64`] folded over the
//! file's bytes and is pinned in `phinet/tests/structural/allowlist.golden`,
//! so a modification of the set fails the Structure_Check until it is
//! re-recorded. The file sits under `phical/tests/`, not `phical/src/`, so
//! the standing rule that phisor does not edit `phical/src` is untouched.

use super::{BadWidth, Operand, Type};
use phistd::mix64;

pub mod record;

pub use record::generate;

/// The frozen set's path, as a failure must name it.
pub const PATH: &str = "phinet/crates/phical/tests/keyboard/operands.tsv";

/// The frozen set's text, compiled in so the agreement check reads the same
/// bytes whatever directory it is run from.
pub const TEXT: &str = include_str!("../../tests/keyboard/operands.tsv");

/// The seed the set was generated under. It has no meaning of its own; it is
/// recorded so the run that wrote the file is reproducible.
pub const SEED: u64 = 0x5EED_0312;

/// The least count of operands of each type (R7.6), hence a floor of 72
/// across the nine.
pub const PER_TYPE: usize = 8;

/// The frozen operand set, read from [`TEXT`].
#[derive(Clone, Debug)]
pub struct FrozenOperands {
    /// The operands of each type, indexed by [`Type::index`]: at least
    /// [`PER_TYPE`] of each, so at least 72 across the nine.
    pub by_type: [Vec<Operand>; 9],
    /// [`phistd::mix64`] folded over the file's bytes, pinned in the golden
    /// record so a modification fails the run.
    pub fingerprint: u64,
}

impl FrozenOperands {
    /// The frozen set as it stands in the file. Panics on a malformed line:
    /// the set is a compiled-in golden record, so a malformation is a broken
    /// record rather than bad input, and it must not be read past.
    pub fn load() -> FrozenOperands {
        FrozenOperands::parse(TEXT).unwrap_or_else(|flaw| panic!("{PATH}: {flaw}"))
    }

    /// The set `text` spells, or the first [`Flaw`] in it.
    pub fn parse(text: &str) -> Result<FrozenOperands, Flaw> {
        let mut by_type: [Vec<Operand>; 9] = std::array::from_fn(|_| Vec::new());
        for (at, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let at = at + 1;
            let column: Vec<&str> = line.split('\t').map(str::trim).collect();
            if column.len() != 3 {
                return Err(Flaw { line: at, reason: Reason::Count(column.len()) });
            }
            let ty = Type::ALL
                .into_iter()
                .find(|t| t.name() == column[0])
                .ok_or_else(|| Flaw { line: at, reason: Reason::Type(column[0].to_string()) })?;
            let bytes = unhex(column[2]).ok_or(Flaw { line: at, reason: Reason::Hex })?;
            let operand = Operand::decode(ty, &bytes)
                .map_err(|bad| Flaw { line: at, reason: Reason::Width(bad) })?;
            by_type[ty.index()].push(operand);
        }
        Ok(FrozenOperands { by_type, fingerprint: fingerprint(text) })
    }

    /// The operands of one type, in file order.
    pub fn of(&self, ty: Type) -> &[Operand] {
        &self.by_type[ty.index()]
    }

    /// Every operand in the set, in [`Type::ALL`] order and in file order
    /// within a type — the order the agreement check runs them in.
    pub fn all(&self) -> Vec<Operand> {
        Type::ALL.into_iter().flat_map(|ty| self.of(ty).iter().copied()).collect()
    }

    /// The count of operands across the nine types.
    pub fn count(&self) -> usize {
        self.by_type.iter().map(Vec::len).sum()
    }
}

/// [`phistd::mix64`] folded byte by byte over the record.
///
/// A carriage return is not part of the record: a checkout that stores the
/// line ending as CRLF holds the same operand set, and the fingerprint the
/// golden record pins must not turn on which way the file was written out.
pub fn fingerprint(text: &str) -> u64 {
    let mut hash = 0u64;
    for byte in text.bytes().filter(|&b| b != b'\r') {
        hash = mix64(hash ^ byte as u64);
    }
    hash
}

/// A line of the record that is not an operand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flaw {
    /// The line it is, counted from 1 as an editor counts.
    pub line: usize,
    pub reason: Reason,
}

/// What is wrong with the line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reason {
    /// The count of tab-separated columns given, which is not 3.
    Count(usize),
    /// A type name outside the nine.
    Type(String),
    /// A byte column that is not an even run of hex digits.
    Hex,
    /// Hex of the wrong length for the type's declared width.
    Width(BadWidth),
}

impl std::fmt::Display for Flaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: ", self.line)?;
        match &self.reason {
            Reason::Count(given) => write!(f, "{given} tab-separated columns given, not 3"),
            Reason::Type(name) => write!(f, "`{name}` is not one of the nine declared types"),
            Reason::Hex => write!(f, "the byte column is not an even run of hex digits"),
            Reason::Width(bad) => write!(f, "{bad}"),
        }
    }
}

impl std::error::Error for Flaw {}

/// The bytes as lower-case hex, two digits a byte.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The bytes `text` spells, or `None` for an odd length or a digit outside
/// `0..9a..fA..F` — `from_str_radix` would take a sign, and a signed byte is
/// not a byte.
pub fn unhex(text: &str) -> Option<Vec<u8>> {
    if text.len() % 2 != 0 || !text.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let digit: Vec<u8> = text.bytes().collect();
    digit
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok())
        .collect()
}

#[cfg(test)]
mod tests;
