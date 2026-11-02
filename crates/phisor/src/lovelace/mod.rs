//! `lovelace` — the Note G analytic model: a reading of phisor through
//! Augusta Ada Lovelace's 1843 Notes on the Analytical Engine.
//!
//! This module is an analytic model, not a runtime. It adds no execution of
//! its own; it names the correspondence between the four ideas Lovelace set
//! down in her Notes and the four parts of phisor that already carry them,
//! and it offers two thin readings over the live outputs of those parts so
//! the correspondence can be checked rather than merely asserted.
//!
//! Lovelace translated L. F. Menabrea's sketch of Charles Babbage's
//! Analytical Engine and appended seven Notes, A through G, longer than the
//! paper itself. Note G carried a step-by-step table for computing the
//! Bernoulli numbers — the first algorithm published for a machine. The four
//! ideas below are hers, in her own words where the words are famous, and
//! each lands in the phisor module that does the same work.
//!
//! | Note G idea | her term | phisor module | what it does |
//! |---|---|---|---|
//! | [`Store`](Correspondence::Store) | the store | [`ground`](crate::ground) | an Act held as rows, apart from the mill |
//! | [`Mill`](Correspondence::Mill) | the mill | [`machine`](crate::machine) | the 64 operators lowered and operated, checked against the reader |
//! | [`Weave`](Correspondence::Weave) | the weave | [`ladder`](crate::ladder) | structure woven from text, no word of the text carried |
//! | [`Objection`](Correspondence::Objection) | her objection | [`truth`](crate::truth) | the machine originates nothing; it classifies what it is given |
//!
//! Her architecture was already the one this crate is built on: a store that
//! holds quantities apart from a mill that operates on them, an engine that
//! "weaves algebraic patterns just as the Jacquard-loom weaves flowers and
//! leaves" (Note A), and a machine with "no pretensions whatever to
//! originate anything" (Note G). phisor keeps the store ([`ground`]) apart
//! from the mill ([`machine`]), weaves structure without carrying a word
//! ([`ladder`]), and refuses to let the mill originate a claim it was not
//! given ([`truth`]).

#[cfg(test)]
mod tests;

/// One correspondence between an idea in Lovelace's Notes and the phisor
/// module that carries it. The four are exhaustive over the architecture's
/// spine: what is held, what operates, what is woven, and what the machine
/// may not do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Correspondence {
    /// The store: the Analytical Engine's columns held its quantities apart
    /// from the mill that operated on them. phisor's [`ground`](crate::ground)
    /// holds an Act as rows — the event and what is said about it — before
    /// any operation reads them.
    Store,
    /// The mill: where, in Lovelace's words, the quantities "are brought in
    /// to be operated upon." phisor's [`machine`](crate::machine) lowers the
    /// 64 operators onto phiclm's Lisp and O-code and checks the result byte
    /// for byte against the Rust reader, as Note G's own trace table stepped
    /// one operation at a time.
    Mill,
    /// The weave: Lovelace's Jacquard-loom figure. phisor's
    /// [`ladder`](crate::ladder) weaves a sentence's structure — its word
    /// classes, its variation, its kind — carrying no word of the text, as a
    /// woven pattern carries no thread of the design it copies.
    Weave,
    /// Her objection: the engine "has no pretensions whatever to originate
    /// anything." phisor's [`truth`](crate::truth) detector classifies a
    /// claim before a row is told from it and originates none; the mill does
    /// only what it is ordered to do.
    Objection,
}

impl Correspondence {
    /// The four correspondences, in the architecture's order: what is held,
    /// what operates, what is woven, what may not be originated.
    pub const ALL: [Correspondence; 4] = [
        Correspondence::Store,
        Correspondence::Mill,
        Correspondence::Weave,
        Correspondence::Objection,
    ];

    /// Lovelace's own term for the idea.
    pub fn lovelace_term(self) -> &'static str {
        match self {
            Correspondence::Store => "the store",
            Correspondence::Mill => "the mill",
            Correspondence::Weave => "the weave",
            Correspondence::Objection => "Lady Lovelace's objection",
        }
    }

    /// The phisor module that carries the idea, by its one-word name.
    pub fn phisor_module(self) -> &'static str {
        match self {
            Correspondence::Store => "ground",
            Correspondence::Mill => "machine",
            Correspondence::Weave => "ladder",
            Correspondence::Objection => "truth",
        }
    }

    /// A one-line gloss of the correspondence in plain prose.
    pub fn note(self) -> &'static str {
        match self {
            Correspondence::Store => {
                "the store holds the quantities apart from the mill; ground holds an Act as rows"
            }
            Correspondence::Mill => {
                "the mill operates on what the store holds; machine lowers and runs the 64 operators"
            }
            Correspondence::Weave => {
                "the engine weaves patterns; the ladder weaves structure and carries no word"
            }
            Correspondence::Objection => {
                "the engine originates nothing; the detector classifies, it does not invent"
            }
        }
    }

    /// Lovelace's verbatim words for the idea, where she gave the idea a
    /// famous sentence of its own. The store and the mill are her terms
    /// rather than quoted sentences, so they return `None`.
    pub fn quotation(self) -> Option<&'static str> {
        match self {
            Correspondence::Weave => Some(QUOTATIONS[0].0),
            Correspondence::Objection => Some(QUOTATIONS[1].0),
            Correspondence::Store | Correspondence::Mill => None,
        }
    }
}

/// Genuine, well-attested quotations from Lovelace's 1843 Notes, each paired
/// with the Note it appears in. Verbatim: paraphrase belongs in [`note`], not
/// here.
///
/// [`note`]: Correspondence::note
pub const QUOTATIONS: &[(&str, &str)] = &[
    (
        "The Analytical Engine weaves algebraical patterns just as the \
         Jacquard-loom weaves flowers and leaves.",
        "Note A",
    ),
    (
        "The Analytical Engine has no pretensions whatever to originate \
         anything. It can do whatever we know how to order it to perform.",
        "Note G",
    ),
    (
        "The science of operations, as derived from mathematics more \
         especially, is a science of itself, and has its own abstract truth \
         and value.",
        "Note A",
    ),
    (
        "Many persons imagine that because the business of the engine is to \
         give its results in numerical notation, the nature of its processes \
         must consequently be arithmetical and numerical, rather than \
         algebraical and analytical. This is an error.",
        "Note A",
    ),
];

/// The project's dedication. The superlative is the project's own voice, set
/// apart as a dedication rather than offered as history.
pub fn dedication() -> &'static str {
    "Dedicated to Augusta Ada Lovelace (1815-1852), who first saw that a \
     machine of numbers could weave the patterns of thought - in this \
     project's reckoning the most brilliant mind who ever turned mathematics \
     into music."
}

/// The count of operation cards in a lowering. Lovelace's engine ran on
/// operation cards and variable cards; a lowered operator is their modern
/// form, and this reads how many operation steps the mill was given. It
/// recomputes nothing — it reads the length of the O-code the
/// [`Lowering`](crate::machine::Lowering) already emitted.
pub fn operation_cards(lowered: &crate::machine::Lowered) -> usize {
    lowered.ocode.len()
}

/// Whether a woven mode carries no thread of its text. Lovelace's loom
/// weaves the pattern of a design without carrying a single thread of the
/// original; the [`ladder`](crate::ladder) weaves a sentence's structure as
/// class tokens and carries no word. A written mode is rows of class ids at
/// the mode's level, so the weave holds exactly when every token is a class
/// id below `classes` — never a word. An empty weave trivially holds.
pub fn weaves_no_thread(written: &[Vec<u16>], classes: u16) -> bool {
    written
        .iter()
        .flatten()
        .all(|&token| token < classes)
}
