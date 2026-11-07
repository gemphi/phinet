use super::words::{load_phrases, load_plain, Words};
use std::sync::OnceLock;

plain_list!(
    /// Adverbs the IR has no slot for, set aside on reading.
    set_aside_adverbs,
    "set_aside_adverb.txt"
);

/// Multi-word adverbials set aside as a unit, one phrase (a word list) each.
pub fn set_aside_adverb_phrases() -> &'static Vec<Vec<String>> {
    static CELL: OnceLock<Vec<Vec<String>>> = OnceLock::new();
    CELL.get_or_init(|| load_phrases("set_aside_adverb_phrase.txt"))
}
plain_list!(
    /// Postpositions: words that follow their noun phrase ("two hours ago").
    postpositions,
    "postposition.txt"
);
