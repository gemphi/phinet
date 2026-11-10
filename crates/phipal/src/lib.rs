//! # phipal
//!
//! The social model. Two sources are on record for it and they are kept apart
//! here because they have not been reconciled:
//!
//! - the maintainer's basis for the phinet crate: Searle's speech acts,
//!   Russell and Whitehead's *Principia Mathematica*, and process over turns;
//! - the puikit registry entry for Phipal: "your personal agent for world
//!   economies", an agent on the Phiegg marketplace that spawns a team and
//!   plays the economy for the user's goals.
//!
//! What exists today is Searle's side, in `phisor::ir`: the seven forces and
//! the five illocutionary points with their direction of fit. This crate
//! re-exports them and moves nothing. The Principia side (the Lisp on Russell's
//! set algebra in `phiclm`) and the process side are not wired here.

pub use phisor::ir::{Fit, Force, Point};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_commitment_is_a_force_of_its_own() {
        assert_ne!(Force::Commit, Force::Assert);
        assert_eq!(Point::Commissive, Point::Commissive);
        let _ = Fit::WorldToWord;
    }
}
