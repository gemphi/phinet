//! Why the reader abstains: a text-only bucket for each sentence it cannot read.
//!
//! The bucket is a guess from the words alone, never from what the reader did,
//! so it can be counted for the harness record (`ladder.act.abstain.<kind>`)
//! and tells which reader rule could pay next. Buckets are mutually exclusive
//! and tried in the order of [`KINDS`].


/// Sentences of this many words or more are `very_long`.
const LONG: usize = 16;

/// Every kind, in the order they are tried. The names are metric suffixes.
pub const KINDS: [&str; 6] =
    ["very_long", "multi_clause", "one_join", "infinitive", "ing_form", "simple"];

/// The kind of an abstained sentence. Always one of [`KINDS`].
pub fn abstention_kind(sentence: &str) -> &'static str {
    let words: Vec<&str> = sentence.trim_end_matches(['.', '?']).split_whitespace().collect();
    let joins = words.iter().filter(|w| crate::lexicon::lists::clause_openers().contains(w)).count();
    if words.len() >= LONG {
        KINDS[0]
    } else if joins >= 2 {
        KINDS[1]
    } else if joins == 1 {
        KINDS[2]
    } else if words.contains(&"to") {
        KINDS[3]
    } else if words.iter().any(|w| w.ends_with("ing")) {
        KINDS[4]
    } else {
        KINDS[5]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_is_reached_in_order() {
        assert_eq!(abstention_kind("a b c d e f g h i j k l m n o p."), "very_long");
        assert_eq!(abstention_kind("i came and i saw but left."), "multi_clause");
        assert_eq!(abstention_kind("i came and left."), "one_join");
        assert_eq!(abstention_kind("i want to go."), "infinitive");
        assert_eq!(abstention_kind("i like swimming."), "ing_form");
        assert_eq!(abstention_kind("she sleeps."), "simple");
    }
}
