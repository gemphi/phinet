//! The seeded property generator: `for_all` over independently seeded cases,
//! built on [`mix64`](crate::mix64), [`unit_random`](crate::unit_random) and
//! [`unit_phase`](crate::unit_phase) and on no outside crate, because phinet
//! holds no `proptest`, `quickcheck`, `arbitrary` or `rand` and takes on no
//! new dependency to get one.
//!
//! A draw is a function of `(seed, case, position)` and nothing else, and case
//! `i` is seeded `mix64(seed ^ i)` — per case, not as a running chain. So case
//! 412 is reconstructible without running cases 0 to 411, and there is no
//! shrinking: a failure is reported as `(seed, case, value)` and reproduced
//! exactly by [`replay`].
//!
//! ```
//! use phistd::gen::{for_all, replay, Gen, Seeded, CASES};
//!
//! #[derive(Debug)]
//! struct Small(u64);
//! impl Gen for Small {
//!     fn draw(g: &mut Seeded) -> Self {
//!         Small(g.below(10))
//!     }
//! }
//!
//! let failure = for_all::<Small>(0x5EED, CASES, |x| x.0 != 7).unwrap_err();
//! assert_eq!(replay::<Small>(failure.seed, failure.case).0, failure.value.0);
//! ```

use crate::{mix64, random::unit_interval, unit_phase, unit_random, Function};
use std::fmt;

/// The declared case count for a cheap pure property: 256, well above the
/// 100-case minimum.
pub const CASES: usize = 256;

/// The declared case count where one case runs phiclm's machine, so a case
/// costs a compile and a run: 64.
pub const CASES_MACHINE: usize = 64;

/// A pure generator. No interior state beyond a counter, so a draw is a
/// function of `(seed, case, position)`.
pub struct Seeded {
    seed: u64,
    position: u64,
}

impl Seeded {
    /// Case `case` of a property, seeded `mix64(seed ^ case)`.
    pub fn for_case(seed: u64, case: usize) -> Self {
        Seeded { seed: mix64(seed ^ case as u64), position: 0 }
    }

    /// The next 64-bit word: the case seed mixed with the draw's position.
    #[allow(clippy::should_implement_trait)]
    pub fn u64(&mut self) -> u64 {
        self.position = self.position.wrapping_add(1);
        mix64(self.seed ^ mix64(self.position.wrapping_mul(0x9E37_79B9_7F4A_7C15)))
    }

    /// A word in `0..n`, by the high half of a 128-bit product, so no value is
    /// drawn more often than another. A zero bound has no value in it and
    /// answers 0.
    pub fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        ((self.u64() as u128 * n as u128) >> 64) as u64
    }

    /// A uniform in [0, 1): a word's top 53 bits.
    pub fn unit(&mut self) -> f64 {
        unit_interval(self.u64())
    }

    /// An angle in [0, τ).
    pub fn phase(&mut self) -> f64 {
        self.unit() * std::f64::consts::TAU
    }

    /// One of `xs`, uniformly. Panics on an empty slice: a property with
    /// nothing to draw from is a broken property, not a passing one.
    pub fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        assert!(!xs.is_empty(), "pick from an empty slice");
        &xs[self.below(xs.len() as u64) as usize]
    }

    /// The hard float cases, three draws in four from the awkward table —
    /// ±0.0, both subnormal edges, ±inf, a quiet and a signalling NaN bit
    /// pattern, the magnitude extremes — and the fourth an arbitrary bit
    /// pattern read as an `f64`.
    pub fn awkward_f64(&mut self) -> f64 {
        const AWKWARD: [u64; 14] = [
            0x0000_0000_0000_0000, // +0.0
            0x8000_0000_0000_0000, // -0.0
            0x0000_0000_0000_0001, // the smallest subnormal
            0x800F_FFFF_FFFF_FFFF, // the largest negative subnormal
            0x0010_0000_0000_0000, // the smallest normal
            0x7FF0_0000_0000_0000, // +inf
            0xFFF0_0000_0000_0000, // -inf
            0x7FF8_0000_0000_0000, // a quiet NaN
            0xFFF8_0000_0000_1234, // a quiet NaN carrying a payload
            0x7FF0_0000_0000_0001, // a signalling NaN
            0x7FEF_FFFF_FFFF_FFFF, // f64::MAX
            0xFFEF_FFFF_FFFF_FFFF, // f64::MIN
            0x3CB0_0000_0000_0000, // f64::EPSILON
            0x3FF0_0000_0000_0000, // 1.0
        ];
        if self.below(4) == 0 {
            f64::from_bits(self.u64())
        } else {
            f64::from_bits(AWKWARD[self.below(AWKWARD.len() as u64) as usize])
        }
    }

    /// A unit-energy function on `space` samples, from [`unit_random`].
    pub fn function(&mut self, space: usize) -> Function {
        unit_random(space, self.u64())
    }

    /// A unit-magnitude-per-sample function on `space` samples — the shape a
    /// role has — from [`unit_phase`].
    pub fn role(&mut self, space: usize) -> Function {
        unit_phase(space, self.u64())
    }
}

/// A value a property can be run over.
pub trait Gen: Sized {
    fn draw(g: &mut Seeded) -> Self;
}

impl Gen for u64 {
    fn draw(g: &mut Seeded) -> Self {
        g.u64()
    }
}

impl Gen for bool {
    fn draw(g: &mut Seeded) -> Self {
        g.u64() & 1 == 1
    }
}

/// A drawn `f64` is an awkward one: in this workspace a float is carried as
/// bits, and the bit patterns are where a round trip either survives or does
/// not.
impl Gen for f64 {
    fn draw(g: &mut Seeded) -> Self {
        g.awkward_f64()
    }
}

impl<A: Gen, B: Gen> Gen for (A, B) {
    fn draw(g: &mut Seeded) -> Self {
        (A::draw(g), B::draw(g))
    }
}

/// The first failing case: the seed it was run under, its case index and the
/// value drawn. There is no shrinking — the value is whatever size the
/// generator drew, and `replay(seed, case)` returns it again.
#[derive(Debug)]
pub struct Failure<T> {
    pub seed: u64,
    pub case: usize,
    pub value: T,
}

impl<T: fmt::Debug> fmt::Display for Failure<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "case {} of seed {:#018x} failed on {:?}; replay with gen::replay({:#018x}, {})",
            self.case, self.seed, self.value, self.seed, self.case
        )
    }
}

/// Runs `cases` independently seeded cases and returns the first one `f`
/// answers false for.
pub fn for_all<T: Gen + fmt::Debug>(
    seed: u64,
    cases: usize,
    f: impl Fn(&T) -> bool,
) -> Result<(), Failure<T>> {
    for case in 0..cases {
        let value = replay::<T>(seed, case);
        if !f(&value) {
            return Err(Failure { seed, case, value });
        }
    }
    Ok(())
}

/// Reconstructs one case, for a debugger or a failing report.
pub fn replay<T: Gen>(seed: u64, case: usize) -> T {
    T::draw(&mut Seeded::for_case(seed, case))
}

#[cfg(test)]
mod tests;
