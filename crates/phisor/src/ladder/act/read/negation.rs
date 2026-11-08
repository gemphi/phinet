//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Negation detection starting the verb search at `from` (just past the
/// subject), so a determiner-led subject does not shift the verb index.
pub(in crate::ladder::act) fn detect_negation_from(words: &[String], from: usize) -> (bool, usize) {
    for i in from..words.len().saturating_sub(1) {
        if aux_is(&words[i], Family::Do) && crate::lexicon::lists::is_role(&words[i + 1], "negator") {
            return (true, i + 2);
        }
    }
    (false, from)
}

