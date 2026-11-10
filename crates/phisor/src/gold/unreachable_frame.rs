//! The gold frames no text-only reader can label right, named and counted
//! apart so a bar measures the reader and not the label (spec
//! phinet-english-reader-coverage, R2).
//!
//! A frame is listed only when the probe in this file's tests finds a second
//! gold frame whose surface text is the same and whose label differs. The
//! gold file holds no two sentences that are word for word equal, so the
//! probe compares *surface shape*: pronouns, modals, auxiliaries, `to` and
//! `not` are kept, and every other run of words collapses to one slot. Two
//! sentences with one shape give a reader the same evidence.
//!
//! The list lives in code and the gold file is untouched: the exclusion
//! exists only in a reported figure (R2.3). A bare `will` is never mapped to
//! Commit by rule, because `she will arrive tomorrow` (Assert) and
//! `we will meet at eight` (Commit) read alike (R2.5).
//!
//! An index is the frame's 0-based position in the file, the one
//! [`super::part_of`] and `Gold::entries` use. The file's own ids run from 1.

/// Why the surface text does not settle the gold label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    /// A bare `will` or `shall` is a Commit or a prediction (an Assert).
    BareWill,
    /// `normalise` lower-cases, so a proper name is not told from a noun.
    NameCase,
    /// An unaccusative subject is the theme, not the agent (`she will arrive`).
    Unaccusative,
}

/// One unreachable frame and the evidence for it (R2.4).
#[derive(Clone, Copy, Debug)]
pub struct Unreachable {
    /// 0-based position of the frame in the gold file.
    pub index: usize,
    pub reason: Reason,
    /// The two readings the surface text cannot tell apart.
    pub readings: (&'static str, &'static str),
    /// A pair of gold frames with one surface shape and the labels below.
    pub witness: (usize, usize),
    /// The distinct gold labels of the witness pair, in witness order.
    pub labels: (&'static str, &'static str),
}

const WILL: Unreachable = Unreachable {
    index: 0,
    reason: Reason::BareWill,
    readings: ("Predict: she says what will happen", "Commit: she binds herself to do it"),
    witness: (58, 122),
    labels: ("ASSERT", "COMMIT"),
};

const CASE: Unreachable = Unreachable {
    index: 0,
    reason: Reason::NameCase,
    readings: ("a proper name, as in London", "a common noun, as in london"),
    witness: (2, 35),
    labels: ("name", "thing"),
};

const THEME: Unreachable = Unreachable {
    index: 0,
    reason: Reason::Unaccusative,
    readings: ("the subject undergoes the event (theme)", "the subject does it (agent)"),
    witness: (58, 122),
    labels: ("theme", "agent"),
};

const fn at(index: usize, of: Unreachable) -> Unreachable {
    Unreachable { index, ..of }
}

/// Every unreachable frame, by index. A bare `will` or `shall` Commit
/// (`i will explain the rule`), the one unaccusative subject, and each frame
/// that carries a proper name the lower-cased text cannot show.
pub const UNREACHABLE: [Unreachable; 26] = [
    at(2, CASE),
    at(5, CASE),
    at(11, CASE),
    at(14, CASE),
    at(19, CASE),
    at(26, CASE),
    at(29, CASE),
    at(31, CASE),
    at(33, CASE),
    at(46, CASE),
    at(47, CASE),
    at(58, THEME),
    at(72, CASE),
    at(80, CASE),
    at(96, CASE),
    at(112, CASE),
    at(117, WILL),
    at(119, WILL),
    at(120, WILL),
    at(121, WILL),
    at(122, WILL),
    at(124, WILL),
    at(126, WILL),
    at(131, CASE),
    at(140, CASE),
    at(145, CASE),
];

/// Why the frame at `index` cannot be labelled from its text, or `None` when
/// it can.
pub fn unreachable(index: usize) -> Option<Reason> {
    entry(index).map(|u| u.reason)
}

fn entry(index: usize) -> Option<&'static Unreachable> {
    UNREACHABLE.iter().find(|u| u.index == index)
}

#[cfg(test)]
mod tests;
