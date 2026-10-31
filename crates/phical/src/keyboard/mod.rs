//! `keyboard` — the closed, designed instruction set: 64 named Operators
//! with fixed indices, declared arities, a nine-type table of fixed byte
//! widths, and one equality rule every result is judged under.
//!
//! This is a *keyboard*, not an oracle. The 64 Operators are designed and
//! closed; phisor's templates, word classes and Kinds are learned and
//! uncapped, and growth there moves no index, name, type or width here. The
//! sixty-four-gates tutorial is the precedent and the warning: read as a
//! next-word predictor the hexagram gates scored 0.9% against 12.8% for
//! "always the most frequent", which is what sixty-four operations are worth
//! as an oracle and says nothing about what they are worth as a keyboard.
//!
//! `phical::operator` is phifno's `Fno` re-export and is untouched by this
//! module; the two share a word and nothing else.
//!
//! | file | what | lands in |
//! |---|---|---|
//! | [`types`] | the nine types, their widths and little-endian layouts, `Operand`, the equality rule | here |
//! | `arena` | `Arena`, `Handle` minting, `ArenaId`, `Canon`, `Fault` | task 3.2 |
//! | `mod` | `Operator`, `Family`, `Arity`, `apply`, the 64-row index table | task 3.3 |
//! | `family/*` | the six families' Operator declarations | tasks 3.4 to 3.9 |
//! | `laws` | the declared idempotence, inverse and composition tables | task 3.10 |
//! | [`index`] | the index table as a frozen record: render, read, drift, re-record | task 3.11 |
//! | [`frozen`] | the frozen operand set the agreement check runs over | task 3.12 |

pub mod arena;
pub mod family;
pub mod frozen;
pub mod index;
pub mod laws;
pub mod operator;
pub mod types;

pub use arena::{Arena, ArenaId, Canon, Fault, MAX_CLAUSE_DEPTH};
pub use frozen::FrozenOperands;
pub use operator::{apply, identity, table, Family, Gate, Inverse, Operator, COUNT};
pub use types::{equal, BadWidth, Handle, Operand, Type, PHASOR_EPSILON};

#[cfg(test)]
mod tests;
