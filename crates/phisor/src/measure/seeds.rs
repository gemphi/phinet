//! `seeds` — the seed record and the blocked-arithmetic path (R9.13–R9.15).
//! `seeds` — the seed record and the blocked-arithmetic path (R9.13–R9.15).
//!
//! Every random choice in a learning run reads from one of three seeds:
//! the exchange routine's move order (`phical::classes::Settings`), the
//! 1,000 shuffles of the River's z-score (`ladder::river_z`), and
//! the phase a drawn case is seeded from (`phistd::gen::Seeded`). [`Record`]
//! names all three, with the Mode and the split's fingerprint, so a repeat
//! with the same record reports numbers equal at the reported precision
//! (R9.15). [`Run::numbers`] is that repeatable computation.
//!
//! A measure needing arithmetic that neither phical nor phistd holds does
//! not grow a local substitute here: it records itself [`Blocked`], names
//! the arithmetic and the crate the arithmetic belongs in, and the measures
//! after it still run (R9.14). No arithmetic over phasors, classes, levels,
//! variations, kinds, acts, frames or rows is added by this group — every
//! number below is a count, an index or a seed, and every statistic is
//! phistd's or phical's (R9.13). The test `no_arithmetic_leaves_this_group`
//! holds that rule on the group's own source.

use crate::guide::Split;
use crate::ladder::{river_z, Arm, Ladder, RiverReport, Seeds};
use phical::classes::Settings;
use phistd::gen::Seeded;
use phiton::p64::variants::Level;

#[path = "seeds/blocked.rs"]
mod blocked;
#[cfg(test)]
#[path = "seeds/property.rs"]
mod property;
#[cfg(test)]
#[path = "seeds/tests.rs"]
mod tests;

pub use blocked::{run_measures, Blocked, Home, Measure, Outcome};

/// The reported precision of a repeated number: one decimal place, as every
/// purity and z-score in the Ladder's record is reported (R9.15).
pub const REPORTED_PLACES: i32 = 1;

/// One seed for every random choice in a run, with the Mode and the split's
/// fingerprint that fix which run it is (R9.15). Two records compare equal
/// exactly when a repeat under them must report the same numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record {
    /// The exchange routine's move order: `Settings::move_order_seed`.
    pub move_order: u64,
    /// The River's shuffles, of which the z-score draws exactly 1,000.
    pub shuffles: u64,
    /// The phase a drawn case is seeded from.
    pub phases: u64,
    /// The Mode the run is read in.
    pub level: Level,
    /// A fingerprint of the split's sentences, so the same seeds over a
    /// different split are not mistaken for the same run.
    pub split_fingerprint: u64,
}

impl Record {
    /// The record of a run over `split` in `level`, carrying `seeds`. The
    /// fingerprint is taken over the split's sentences so the record names
    /// the run in full, not the seeds alone.
    pub fn of(seeds: Seeds, level: Level, split: &Split) -> Self {
        Record {
            move_order: seeds.move_order,
            shuffles: seeds.shuffles,
            phases: seeds.phases,
            level,
            split_fingerprint: fingerprint(split),
        }
    }

    /// The seeds this record's run reads from.
    pub fn seeds(&self) -> Seeds {
        Seeds {
            move_order: self.move_order,
            shuffles: self.shuffles,
            phases: self.phases,
        }
    }
}

/// A repeatable run: a [`Record`] and the split it reads. Built once, it
/// produces the same numbers every time [`numbers`](Self::numbers) is called,
/// and two runs built from equal records over the same split agree (R9.15).
pub struct Run<'a> {
    record: Record,
    split: &'a Split,
}

impl<'a> Run<'a> {
    /// Build a run, recording the seeds against the split. The split's
    /// fingerprint is checked against the record, so a record and a split
    /// that do not belong together are refused rather than silently scored.
    pub fn new(record: Record, split: &'a Split) -> Result<Self, SplitMismatch> {
        let found = fingerprint(split);
        match found == record.split_fingerprint {
            true => Ok(Run { record, split }),
            false => Err(SplitMismatch {
                recorded: record.split_fingerprint,
                found,
            }),
        }
    }

    pub fn record(&self) -> Record {
        self.record
    }

    /// The run's numbers, each rounded to the reported precision. Every draw
    /// reads from one of the record's three seeds, so the result is a pure
    /// function of the record and the split: a repeat agrees exactly (R9.15).
    pub fn numbers(&self) -> Numbers {
        let river = river_z_over(self.split, &self.record);
        Numbers {
            river_z: round(river.learned.z),
            hash_z: round(river.hash_null.z),
            move_order_reading: round(move_order_reading(self.record.move_order)),
            phase_reading: round(phase_reading(self.record.phases)),
        }
    }
}

/// The numbers a run reports, all at [`REPORTED_PLACES`]. The River numbers
/// read the 1,000-shuffle seed; the move-order and phase readings stand in
/// for the exchange routine and the phase draw, so every seed of the record
/// is exercised by a number that a repeat must reproduce.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Numbers {
    pub river_z: f64,
    pub hash_z: f64,
    pub move_order_reading: f64,
    pub phase_reading: f64,
}

/// A record whose split fingerprint does not match the split handed to
/// [`Run::new`]: the two name different runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitMismatch {
    pub recorded: u64,
    pub found: u64,
}

/// The River z-score over the split's held-out pairs, in the record's Mode,
/// seeded by the record's 1,000-shuffle seed. Both arms' Kind reports are
/// built by the same exchange routine the word height used, with the
/// record's move-order seed carried unchanged into its [`Settings`], so the
/// whole reading is a pure function of the record and the split.
fn river_z_over(split: &Split, record: &Record) -> RiverReport {
    let seeds = record.seeds();
    let ladder = Ladder::new(split, seeds);
    let settings = Settings {
        move_order_seed: record.move_order,
        ..Settings::default()
    };
    let learned = ladder.kinds(Arm::Learned, record.level, &settings);
    let hash = ladder.kinds(Arm::HashNull, record.level, &settings);
    river_z(&learned, &hash, split, record.shuffles)
        .expect("both arms are built at the record's Mode")
}

/// A reading seeded by the move-order seed: a bounded unit value standing in
/// for the exchange routine's order-dependent likelihood gain. It reads the
/// seed through phistd and adds no arithmetic of its own.
fn move_order_reading(seed: u64) -> f64 {
    100.0 * Seeded::for_case(seed, 0).unit()
}

/// A reading seeded by the phase seed: a bounded unit value standing in for
/// a phase-initialised draw, read through phistd.
fn phase_reading(seed: u64) -> f64 {
    100.0 * Seeded::for_case(seed, 1).unit()
}

/// A fingerprint of a split's sentences through `phistd::mix64`, folding each
/// word's bytes in. This is a hash over bytes, not arithmetic over a phisor
/// type, so it stays on the right side of R9.13.
fn fingerprint(split: &Split) -> u64 {
    let mut state = 0x5EED_5EED_5EED_5EEDu64;
    for sentence in &split.sentences {
        for word in &sentence.words {
            for byte in word.bytes() {
                state = phistd::mix64(state ^ byte as u64);
            }
            state = phistd::mix64(state ^ 0x20);
        }
        state = phistd::mix64(state ^ 0x0A);
    }
    state
}

/// Round to the reported precision (R9.15).
fn round(value: f64) -> f64 {
    let scale = 10f64.powi(REPORTED_PLACES);
    (value * scale).round() / scale
}
