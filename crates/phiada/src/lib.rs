//! # phiada
//!
//! The Lovelace Analytical Engine as a crate of its own: the store, the mill,
//! the cards and Note G.
//!
//! The material already exists as `phisor::lovelace` (the correspondence
//! between Lovelace's Notes and phisor's parts), `phisor::machine` (the mill
//! and the operation cards) and the papers 27 and 28 under `phiano/docs/papers`.
//! This crate re-exports the correspondence and moves nothing. Whether the code
//! itself should move here, out of `phisor`, is the maintainer's call and is not
//! made by this crate.

pub use phisor::lovelace::{Correspondence, QUOTATIONS};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_five_correspondences_are_the_ones_phisor_carries() {
        assert_eq!(Correspondence::ALL.len(), 5);
        assert!(!QUOTATIONS.is_empty());
    }
}
