//! The act height: an honest first English reader and realiser (R5.11,
//! R10.10, R10.11, task 13.7).
//!
//! This reads a training sentence into an [`Act`] and realises an Act back to
//! a sentence, for the gold-frame shapes it genuinely handles: simple
//! declaratives and copulas, present and past, the four aspects, active and
//! passive, positive and negative, with subject, object, a `be`-complement,
//! and prepositional phrases. It does not claim to parse all English. What it
//! cannot read it reports as uncovered, and the four act-height numbers are
//! measured honestly over what it can - a recorded miss is the truth when the
//! reader falls short of a Bar, which is better than leaving the number
//! unjudged.
//!
//! The reader and the realiser are inverse over the shapes they share, which
//! is what [`round_trip`] measures: a sentence read to an Act and realised
//! back equals the original under [`normalise`](crate::guide::holdout::normalise),
//! the terminating mark kept (R10.11).

use crate::guide::holdout::normalise;
use crate::lexicon::auxiliary::{Entry, Family};
use crate::lexicon::lists;
use crate::ir::{Act, Aspect, Filler, Force, Frame, Polarity, Role, Tense, Thing, Voice, Wh};

mod express;
mod names;
mod wish;
mod canonical;
mod contraction;
mod inflect;
mod measure;
mod read;
mod realise;
mod word_class;
#[cfg(test)]
mod rank;
#[cfg(test)]
mod tests;

/// The terminating marks a sentence may keep (R10.11).
const MARKS: [char; 3] = ['.', '?', '!'];

pub(in crate::ladder::act) use canonical::*;
pub(in crate::ladder::act) use contraction::*;
pub(in crate::ladder::act) use inflect::*;
pub use measure::{word_lists, round_trip, round_trip_canonical, force_purity, read_realise_rate};
pub(in crate::ladder::act) use read::*;
pub use read::{read};
pub(in crate::ladder::act) use realise::*;
pub use realise::{realise, realise_cased};
pub(in crate::ladder::act) use word_class::*;
