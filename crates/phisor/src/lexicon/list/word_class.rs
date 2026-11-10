use super::word::{load_tagged, Words};
use std::sync::OnceLock;

tagged_list!(
    /// Determiners, which open a noun phrase.
    determiners,
    "word_class.tsv",
    "determiner"
);

tagged_list!(
    /// Closed auxiliaries the first reader declines to parse.
    auxiliaries,
    "word_class.tsv",
    "aux"
);

tagged_list!(
    /// Present-tense copulas the reader builds a `be` frame from.
    copulas,
    "word_class.tsv",
    "copula"
);

tagged_list!(
    /// Prepositions that open an adjunct.
    prepositions,
    "word_class.tsv",
    "preposition"
);

tagged_list!(
    /// Ordinal/number modifiers a noun phrase may carry before its head.
    modifiers,
    "word_class.tsv",
    "modifier"
);

tagged_list!(
    /// Cardinal number words that count the head they precede.
    cardinals,
    "word_class.tsv",
    "cardinal"
);

tagged_list!(
    /// Bare time adverbs the Guide stores under a preposition.
    time_adverbs,
    "word_class.tsv",
    "time_adverb"
);

tagged_list!(
    /// Bare place adverbs the Guide stores under `in`.
    place_adverbs,
    "word_class.tsv",
    "place_adverb"
);

tagged_list!(
    /// Adjectives used only when the POS lexicon is not primed.
    fallback_adjectives,
    "word_class.tsv",
    "adjective"
);

tagged_list!(
    /// The bare auxiliary verbs, always accepted as a clause verb.
    aux_lemmas,
    "word_class.tsv",
    "aux_lemma"
);

/// The determiner a spelling stands for, from the `det_*` classes of
/// `word_class.tsv`. A quantifier or owner is carried by its own spelling.
pub fn determiner(word: &str) -> Option<crate::ir::Det> {
    use crate::ir::Det;
    tagged_list!(
        /// Definite determiners.
        definite,
        "word_class.tsv",
        "det_definite"
    );
    tagged_list!(
        /// Indefinite determiners.
        indefinite,
        "word_class.tsv",
        "det_indefinite"
    );
    tagged_list!(
        /// Near determiners.
        near,
        "word_class.tsv",
        "det_near"
    );
    tagged_list!(
        /// Far determiners.
        far,
        "word_class.tsv",
        "det_far"
    );
    tagged_list!(
        /// Quantifiers.
        quant,
        "word_class.tsv",
        "det_quant"
    );
    tagged_list!(
        /// Possessives.
        owner,
        "word_class.tsv",
        "det_owner"
    );
    Some(if definite().contains(word) {
        Det::Definite
    } else if indefinite().contains(word) {
        Det::Indefinite
    } else if near().contains(word) {
        Det::Near
    } else if far().contains(word) {
        Det::Far
    } else if quant().contains(word) {
        Det::Quant(word.to_string())
    } else if owner().contains(word) {
        Det::Owner(word.to_string())
    } else {
        return None;
    })
}
