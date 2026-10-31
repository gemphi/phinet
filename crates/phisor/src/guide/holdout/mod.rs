//! The Holdout_Guard: it proves no test sentence, no dev sentence and no
//! line of The_Key reaches a training input (requirements §10).
//!
//! Every comparison runs on a sentence twice: by its Split_Rule index and by
//! its [`normalise`]d string. Normalisation is applied identically on both
//! sides of every comparison and to every cut in play (R10.3), so a cut that
//! trims and folds one side but not the other cannot slip a held-out sentence
//! through. A [`Breach`] stops the run before any weight is written; a
//! legitimate duplicate — a sentence that is train under every cut in play —
//! is counted and excluded, but is not a breach (R10.5).
//!
//! | file | what |
//! |---|---|
//! | `breach` | the four counts (`Counts`) and the breach shapes (`Breach`) a run stops on |
//! | `guard` | the guard itself (`HoldoutGuard`): the index and string comparisons, and the per-phase label reads |

mod breach;
mod guard;
#[cfg(test)]
mod tests;

pub use breach::{Breach, Counts};
pub use guard::HoldoutGuard;

/// Which sentence cut a role was assigned under. docs/201's layout cut and
/// docs/203's token cut assign different indices to the same text, which is
/// why every comparison names its cut.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CutId {
    Layout,
    Tokens,
}

/// The Split_Rule's three roles (R10.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Test,
    Dev,
    Train,
}

/// The Split_Rule: `i % 10 == 0` is Test, `i % 20 == 5` is Dev, every other
/// index is Train (R10.1). Test wins the index 0, which is both.
pub fn role_of(index: usize) -> Role {
    match (index % 10, index % 20) {
        (0, _) => Role::Test,
        (_, 5) => Role::Dev,
        _ => Role::Train,
    }
}

/// Trim, collapse each run of internal whitespace to one space, fold case —
/// applied identically on both sides of every comparison and to every cut in
/// play (R10.3).
pub fn normalise(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// Which phase a label was read in (R10.8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    BuildTrainingInput,
    ScoreNumber,
}
