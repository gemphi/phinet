use super::words::{load_pairs, load_plain, Words};
use std::sync::OnceLock;

plain_list!(
    /// Stressed multi-syllable verbs that double their final consonant.
    doubling_stressed,
    "doubling_stressed.txt"
);

plain_list!(
    /// Silent-e verbs whose `-ing`/`-ed` drops the e.
    silent_e_lemmas,
    "silent_e_lemma.txt"
);

plain_list!(
    /// Nouns ending in `s` that are singular.
    singular_s_nouns,
    "singular_s_noun.txt"
);

/// Irregular noun plurals as `(singular, plural)`.
pub fn irregular_plurals() -> &'static Vec<(String, String)> {
    static CELL: OnceLock<Vec<(String, String)>> = OnceLock::new();
    CELL.get_or_init(|| load_pairs("irregular_plural.tsv"))
}

/// Words whose first letter misleads the indefinite article, as
/// `(prefix, article)`; the first matching prefix wins.
pub fn article_exceptions() -> &'static Vec<(String, String)> {
    static CELL: OnceLock<Vec<(String, String)>> = OnceLock::new();
    CELL.get_or_init(|| load_pairs("article_exception.tsv"))
}
