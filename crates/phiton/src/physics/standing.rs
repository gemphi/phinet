//! Standing waves (Bentov, *Stalking the Wild Pendulum*): a bounded medium
//! sustains only integer modes, so the modes of a medium of `n` channels are
//! exactly `n` — moved here from phiano's `kinds`, which reads relations as
//! these modes.

/// The rotation of standing-wave mode `m` on a medium of `channels` channels.
///
/// Bentov, *Stalking the Wild Pendulum*, ch. 1: "Standing waves cannot exist
/// unless they divide their medium into an integral number of half waves. A
/// standing wave having a fractional wavelength cannot be sustained... The
/// dimensions of the plate are the factors that govern what is the size or
/// wavelength of the standing wave that can be sustained in the plate."
///
/// That is a derivation of the bounded-type claim rather than an assertion of
/// it. If a relation is a standing wave across the channel index, the medium
/// admits only integer modes, and a 64-channel torus sustains exactly 64 of them
/// — mode 64 aliases back onto mode 0. **The bound is the channel count.** It
/// was never a number to be chosen; `MAX_RELATION_TYPES = PHASE_CHANNELS` is
/// forced by the geometry.
///
/// The experimental consequence is sharper than the philosophical one. A
/// harmonic basis is *fixed a priori*: it fits nothing, has no free parameters,
/// and is identical on every corpus and every seed. K-means fits 64 arbitrary
/// 64-dimensional rotations to the data — 4,096 fitted numbers. If the fixed
/// basis holds its own against that, relations in this manifold are modes of the
/// medium rather than arbitrary directions in it, and the entire discovery
/// apparatus was solving a problem the geometry had already answered.
pub fn harmonic_rotation(m: usize, channels: usize) -> Vec<f64> {
    let n = channels.max(1);
    (0..n)
        .map(|k| (crate::units::TWO_PI * (m % n) as f64 * k as f64 / n as f64)
            .rem_euclid(crate::units::TWO_PI))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_medium_of_n_channels_sustains_n_modes() {
        let n = 64;
        assert!(harmonic_rotation(0, n).iter().all(|t| *t == 0.0), "mode 0 does not turn");
        assert_eq!(harmonic_rotation(n, n), harmonic_rotation(0, n), "mode n aliases onto mode 0");
        let one = harmonic_rotation(1, n);
        assert!((one[1] - crate::units::TWO_PI / n as f64).abs() < 1e-12, "mode 1 turns once round the medium");
    }
}
