//! The blocked-arithmetic path (R9.14).
//!
//! A measure needing arithmetic that neither phical nor phistd holds records
//! itself [`Blocked`], names the arithmetic and the crate the arithmetic
//! belongs in, computes no local substitute, and the measures after it still
//! run. [`run_measures`] carries that rule: it returns one [`Outcome`] per
//! measure, in order, and never stops at the first block.

/// A measure that cannot be computed because the arithmetic it needs lives
/// in neither phical nor phistd (R9.14). It records what it needs and where
/// that arithmetic belongs, computes no local substitute, and the measures
/// after it still run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blocked {
    /// The measure that is blocked.
    pub measure: &'static str,
    /// The arithmetic it needs and cannot reach.
    pub arithmetic: &'static str,
    /// The crate the arithmetic should be added to: `phical` or `phistd`.
    pub proposed_home: Home,
}

/// The crate a blocked measure's arithmetic is proposed for. The two are the
/// only homes R9.13 permits new arithmetic over phisor's types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Home {
    Phical,
    Phistd,
}

impl Home {
    pub fn name(self) -> &'static str {
        match self {
            Home::Phical => "phical",
            Home::Phistd => "phistd",
        }
    }
}

impl Blocked {
    pub fn new(measure: &'static str, arithmetic: &'static str, proposed_home: Home) -> Self {
        Blocked {
            measure,
            arithmetic,
            proposed_home,
        }
    }
}

/// The outcome of one measure in a run: a number, or a [`Blocked`] record.
/// A blocked measure does not stop the run — the sequence of outcomes holds
/// every measure's result, blocked or not, in order (R9.14).
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Ran(f64),
    Blocked(Blocked),
}

/// One measure handed to [`run_measures`]: a computed number, or a measure
/// that blocked before it could compute one.
pub enum Measure {
    Ready(f64),
    Blocked(Blocked),
}

/// Run a sequence of measures. A blocked measure contributes its record and
/// the remaining measures still run (R9.14): the returned vector has one
/// outcome per measure, in order, and never stops short.
pub fn run_measures(measures: Vec<Measure>) -> Vec<Outcome> {
    measures
        .into_iter()
        .map(|measure| match measure {
            Measure::Ready(number) => Outcome::Ran(number),
            Measure::Blocked(blocked) => Outcome::Blocked(blocked),
        })
        .collect()
}
