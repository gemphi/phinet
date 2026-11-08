//! Participles and the reader's lemma/number; the `-s/-ing/-ed` forms
//! themselves are `lexicon::suffix::inflect`, read from `suffix.tsv`.
use super::*;

/// The past participle used by the perfect and the passive. Irregulars the
/// gold set uses are named; the rest take `-ed`. These are the book's own
/// irregular list, learned once.
pub(in crate::ladder::act) fn participle(verb: &str) -> String {
    crate::lexicon::irregular_participle(verb)
        .or_else(|| crate::lexicon::irregular::participle(verb))
        .unwrap_or_else(|| inflect(verb, Suffix::PAST))
}
// ----- The reader: English -> Act, for the shapes it handles -----

/// Reduce a surface head to `(singular, is_plural)` by the crude inverse of
/// [`inflect`] with `Suffix::PLURAL`. Honest and shallow: irregular plurals (child/children) are
/// left as read, so the reader does not claim a lemma it cannot justify.
pub(in crate::ladder::act) fn lemma_number(head: &str) -> (String, bool) {
    // Irregular plurals the realiser's `pluralise` names, inverted so the
    // reader recovers the singular lemma: "children" -> (child, plural).
    if let Some((singular, _)) = lists::irregular_plurals().iter().find(|(_, plural)| plural == head) {
        return (singular.clone(), true);
    }
    if let Some(stem) = head.strip_suffix("ies") {
        return (format!("{stem}y"), true);
    }
    if let Some(stem) = head.strip_suffix("es") {
        if (stem.ends_with('s') || stem.ends_with('x') || stem.ends_with("ch")) && inflect(stem, Suffix::PLURAL) == head {
            return (stem.to_string(), true);
        }
    }
    // A singular noun that happens to end in `s` is not a plural: the Guide's
    // closed set of these is kept whole ("bus", not "bu").
    if lists::singular_s_nouns().contains(&head) {
        return (head.to_string(), false);
    }
    // A plain `-s`: a plural when re-pluralising the stem reproduces the
    // surface.
    if let Some(stem) = head.strip_suffix('s') {
        if stem.len() >= 2 && inflect(stem, Suffix::PLURAL) == head {
            return (stem.to_string(), true);
        }
    }
    (head.to_string(), false)
}
