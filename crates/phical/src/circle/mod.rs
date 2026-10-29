//! `circle` — angles on the closed circle and the statistics phient read off
//! them: the turn from one angle to another, the resultant of many and its
//! direction, agreement between offset vectors, k-means on the torus, offsets
//! with their means taken out, and the leave-one-out mean field of the
//! top-down descent. phient computed all of it inline; it lives here once
//! (the ruling of 2026-10-28: the calculations are phical's).
//!
//! Each function is written as its sites wrote it, in their operation order,
//! and two sites share one only where their arithmetic is the same to the
//! bit. Where copies differed — `atan2` alone, `atan2` then `rem_euclid(τ)`,
//! `atan2` then `+ τ` when negative; a length test by `hypot` or a box test
//! on each sum — each form has its own name, so no recorded number moves.
//!
//! Nothing here reads a facet: phical sits below phiecs, so every function
//! takes plain angles, weights, counts and slices, and phient keeps the loop
//! that reads the words and hands the numbers over.
//!
//! | file | what |
//! |---|---|
//! | [`angle`] | two or three angles: `delta`, `distance`, `delta_euclid`, `step_toward`, `analogy` |
//! | [`resultant`] | `Resultant` (Σ w·(cos θ, sin θ) and its readings), `mean`, `VANISH` |
//! | [`agreement`] | mean cos Δ between offset vectors: `agreement`, `nearest`, `centroid`, `mean_agreement`, `mean_nearest`, `shape_similarity` |
//! | [`kmeans`] | `kmeans` on the torus with farthest-point seeds, `Cluster` |
//! | [`centre`] | offsets with a mean taken out: `reduce`, `centred`, `double_centred` |
//! | [`field`] | the descent's votes: `add_scaled`, `leave_one_out`, `is_silent`, `add_directions`, `rayleigh_floor`, `scattered`, `vote_target` |

pub mod agreement;
pub mod angle;
pub mod centre;
pub mod field;
pub mod kmeans;
pub mod resultant;
#[cfg(test)]
mod tests;

pub use agreement::{agreement, centroid, mean_agreement, mean_nearest, nearest, shape_similarity};
pub use angle::{analogy, delta, delta_euclid, distance, step_toward};
pub use centre::{centred, double_centred, reduce};
pub use field::{add_directions, add_scaled, is_silent, leave_one_out, rayleigh_floor, scattered, vote_target};
pub use kmeans::{kmeans, Cluster};
pub use resultant::{mean, Resultant, VANISH};
