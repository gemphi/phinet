//! Mathematical constants the family shares (moved from `phiano::config`, 14.4).
//!
//! The golden ratio itself is `phiton::units::PHI`; its square is here because
//! `phistd` sits below `phiton` and the square is a plain number.

/// Golden ratio squared (φ²), the product `phiton::units::PHI * PHI`.
pub const PHI_SQUARED: f64 = 2.6180339887498949;

/// Pi (π).
pub const PI_CONST: f64 = std::f64::consts::PI;

/// Euler's number (e). Used in exponential decay and growth for novelty, sync
/// and saturation.
pub const E: f64 = std::f64::consts::E;

/// The first prime ≥ 64: mixing constant for sector hashing.
pub const PRIME_64: u64 = 67;

/// The first prime ≥ 128: mixing constant at 128-sector resolution.
pub const PRIME_128: u64 = 131;

/// The first prime ≥ 256: mixing constant at 256-sector resolution.
pub const PRIME_256: u64 = 257;

/// The first prime ≥ 512: mixing constant at 512-sector resolution.
pub const PRIME_512: u64 = 521;

/// The first prime ≥ 1024: mixing constant at 1024-sector resolution.
pub const PRIME_1024: u64 = 1031;

/// Number of color bands in the sector color wheel (always 16).
pub const COLOR_BANDS: u16 = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phi_squared_is_phi_times_phi() {
        let phi: f64 = 1.6180339887498948;
        assert_eq!(PHI_SQUARED, phi * phi);
    }

    #[test]
    fn each_prime_is_coprime_to_its_sector_count() {
        for (p, s) in [(PRIME_64, 64), (PRIME_128, 128), (PRIME_256, 256), (PRIME_512, 512), (PRIME_1024, 1024)] {
            assert!(p > s && p % 2 == 1);
        }
    }
}