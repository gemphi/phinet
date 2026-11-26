//! `checkpoint` — what phisor has learned, kept between runs, so learning
//! accumulates (docs/phisor/learn.md). Every night takes the next text from
//! a queue the run reads (`source`), learns the word code and the kinds
//! again from the newest saved night (`state`), and adds a row to the
//! nights' record (`record`). Everything is a plain file under the learn
//! folder: a person can read it, a page can draw it, and the next night
//! starts from it.
//!
//! | file | what |
//! |---|---|
//! | `source` | the queue (`source.tsv`, kept by hand) and what was taken (`taken.tsv`, appended) |
//! | `state` | a night's paths: each unit's place in the tree, saved and read back; the newest night |
//! | `record` | the nights' measures (`night.tsv`, appended) and tonight's growth (`growth.tsv`) |

pub mod record;
pub mod source;
pub mod state;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

/// The learn folder: `$PHISOR_LEARN`, else the crate's `data/learn`.
pub fn dir() -> PathBuf {
    match std::env::var_os("PHISOR_LEARN") {
        Some(d) => PathBuf::from(d),
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join("learn"),
    }
}

/// The workspace root a relative path in the queue starts from (phinet).
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}
