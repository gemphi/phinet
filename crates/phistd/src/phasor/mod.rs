//! Phasor ranges: the resolutions and quanta every phinet computation can be
//! taken at, and every combination of them, enumerable — the way a tensor
//! framework offers dtypes and devices rather than one fixed choice.
//!
//! Two axes, both powers of two:
//!
//! * **Resolution** `P16 | P32 | P64 | P128` — how many samples a function
//!   has on the circle (Phinet's `space`; Phiano's Phinum16/32/64 engines).
//! * **Quantum** `Q16 | Q32 | Q64` — how many sectors a phase is snapped to
//!   (Phical's Q32 and Q64 junctures; the bound on a phase after snapping is
//!   `π / sectors`).
//!
//! A [`Range`] is one `(resolution, quantum)` pair; [`Range::ALL`] is the
//! twelve of them. Every function here — resampling between resolutions,
//! snapping to a quantum, the sector a value or a function is in, the loss a
//! snap costs — takes the range as an argument, and an operator moves
//! between resolutions with `phical::Fno::at_resolution`. `examples/ranges.rs`
//! measures every combination.

mod functions;
#[cfg(test)]
mod tests;

pub use functions::{quantisation_loss, quantise, resample, sector_of, table};

use crate::c64;
use std::f64::consts::TAU;

/// Samples on the circle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resolution {
    P16 = 16,
    P32 = 32,
    P64 = 64,
    P128 = 128,
}

/// Phase sectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quantum {
    Q16 = 16,
    Q32 = 32,
    Q64 = 64,
}

impl Resolution {
    pub const ALL: [Resolution; 4] = [Resolution::P16, Resolution::P32, Resolution::P64, Resolution::P128];
    pub fn space(self) -> usize {
        self as usize
    }
    pub fn from_space(space: usize) -> Option<Self> {
        Self::ALL.iter().copied().find(|r| r.space() == space)
    }
    /// The octaves a spectrum at this resolution has (six at P64).
    pub fn octaves(self) -> usize {
        (self.space() as f64).log2() as usize
    }
}

impl Quantum {
    pub const ALL: [Quantum; 3] = [Quantum::Q16, Quantum::Q32, Quantum::Q64];
    pub fn sectors(self) -> usize {
        self as usize
    }
    /// Largest phase error a snap can introduce: half a sector.
    pub fn bound(self) -> f64 {
        std::f64::consts::PI / self.sectors() as f64
    }
    /// The sector of a phase in `[0, τ)`.
    pub fn sector_of_phase(self, phase: f64) -> usize {
        let n = self.sectors();
        ((phase.rem_euclid(TAU) / TAU * n as f64).floor() as usize) % n
    }
    /// The sector a complex value's argument falls in.
    pub fn sector(self, z: c64) -> usize {
        self.sector_of_phase(z.arg())
    }
    /// The centre phase of a sector.
    pub fn phase(self, sector: usize) -> f64 {
        (sector as f64 + 0.5) / self.sectors() as f64 * TAU
    }
    /// Snaps a value's phase to its sector centre, keeping its magnitude.
    pub fn snap(self, z: c64) -> c64 {
        c64::from_polar(z.norm(), self.phase(self.sector(z)))
    }
}

/// One resolution and one quantum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub resolution: Resolution,
    pub quantum: Quantum,
}

impl Range {
    /// Every combination: 4 resolutions × 3 quanta.
    pub const ALL: [Range; 12] = {
        let mut out = [Range { resolution: Resolution::P16, quantum: Quantum::Q16 }; 12];
        let rs = Resolution::ALL;
        let qs = Quantum::ALL;
        let mut i = 0;
        while i < 12 {
            out[i] = Range { resolution: rs[i / 3], quantum: qs[i % 3] };
            i += 1;
        }
        out
    };
    /// Phiano's default: 64 samples, 64 sectors.
    pub const DEFAULT: Range = Range { resolution: Resolution::P64, quantum: Quantum::Q64 };

    pub fn new(resolution: Resolution, quantum: Quantum) -> Self {
        Self { resolution, quantum }
    }
    pub fn space(self) -> usize {
        self.resolution.space()
    }
    pub fn sectors(self) -> usize {
        self.quantum.sectors()
    }
    /// `P64·Q32` and the like.
    pub fn name(self) -> String {
        format!("P{}·Q{}", self.space(), self.sectors())
    }
}
