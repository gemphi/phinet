//! Value-moving operations recorded on the tape: the arithmetic, the spectral
//! maps, the losses, and the scores a relation is read with.
mod basic;
mod loss;
mod score;

pub use loss::MixtureTerms;
