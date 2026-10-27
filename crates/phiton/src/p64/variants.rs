//! P64 variation engine — 16/32/64 level primitives.
//!
//! Each level doubles the classification granularity:
//! - **P16**: 16 core categories — fast, coarse
//! - **P32**: 32 core categories — balanced
//! - **P64**: 64 core categories — fine, complete
//!
//! The "64 ways to look at anything" principle: every linguistic unit
//! (word, phrase, sentence, paragraph) can be classified at any of
//! these three levels. The variations and their links form the spider
//! net that captures language without storing examples.

use serde::{Deserialize, Serialize};

/// The three P64 granularity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Level {
    /// 16 core classifications — coarse, fast.
    P16,
    /// 32 core classifications — balanced.
    P32,
    /// 64 core classifications — fine, complete.
    P64,
}

impl Level {
    /// Returns the number of categories at this level.
    pub fn count(self) -> usize {
        match self {
            Self::P16 => 16,
            Self::P32 => 32,
            Self::P64 => 64,
        }
    }

    /// Returns the bit width for indexing at this level.
    pub fn bits(self) -> u32 {
        match self {
            Self::P16 => 4,
            Self::P32 => 5,
            Self::P64 => 6,
        }
    }

    /// Returns the next finer level, or None if already at P64.
    pub fn finer(self) -> Option<Self> {
        match self {
            Self::P16 => Some(Self::P32),
            Self::P32 => Some(Self::P64),
            Self::P64 => None,
        }
    }

    /// Returns the next coarser level, or None if already at P16.
    pub fn coarser(self) -> Option<Self> {
        match self {
            Self::P16 => None,
            Self::P32 => Some(Self::P16),
            Self::P64 => Some(Self::P32),
        }
    }
}

/// A single variation slot within a P64 level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variation {
    /// The P64 level this variation belongs to.
    pub level: Level,
    /// Index [0, count) within the level.
    pub index: u8,
}

impl Variation {
    pub fn new(level: Level, index: u8) -> Self {
        let max = level.count() as u8;
        Self { level, index: index % max }
    }

    /// Returns a human-readable label for this variation.
    pub fn label(self) -> String {
        let prefix = match self.level {
            Level::P16 => "n16",
            Level::P32 => "n32",
            Level::P64 => "n64",
        };
        format!("{}#{}", prefix, self.index)
    }
}

/// Trait for P64-level engines.
pub trait Engine: Sized {
    const LEVEL: Level;

    /// Total number of classification slots.
    const SLOTS: usize;

    /// Classifies a hash value into a variation slot.
    fn classify_hash(hash: u64) -> Variation {
        let mask = (Self::SLOTS as u64) - 1;
        Variation::new(Self::LEVEL, ((hash & mask) % Self::SLOTS as u64) as u8)
    }

    /// Classifies a string into a variation slot using the global config.
    fn classify_str(s: &str) -> Variation {
        let hash = super::config::Config::global().hash_str(s);
        Self::classify_hash(hash)
    }
}

/// 16-core classification engine — coarse, fast.
#[derive(Debug, Clone, Copy)]
pub struct P16;

impl Engine for P16 {
    const LEVEL: Level = Level::P16;
    const SLOTS: usize = 16;
}

/// 32-core classification engine — balanced.
#[derive(Debug, Clone, Copy)]
pub struct P32;

impl Engine for P32 {
    const LEVEL: Level = Level::P32;
    const SLOTS: usize = 32;
}

/// 64-core classification engine — fine, complete.
#[derive(Debug, Clone, Copy)]
pub struct P64;

impl Engine for P64 {
    const LEVEL: Level = Level::P64;
    const SLOTS: usize = 64;
}
