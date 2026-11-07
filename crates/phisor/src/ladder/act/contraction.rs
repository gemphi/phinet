//! Moved from `ladder/act.rs` unchanged.
use super::*;

/// Expand contractions into the words they stand for, so the reader meets "do
/// not" where the text wrote "don't" (phiecs tokenises a contraction as one
/// word). The expansion is rule-driven, from data files (see
/// `crate::lexicon::contraction`): an irregular whole form (`won't`, `can't`)
/// first, then a clitic suffix (`n't`, `'re`, `'ve`, `'ll`, `'m`) on any token.
/// The genuinely ambiguous `'s` (is/has/possessive) is resolved by the word
/// that follows it, and only after he/she/it; `'d` (would/had) is left as read,
/// so the reader abstains on them rather than guess. This directly lifts
/// coverage on real text, which writes its auxiliaries contracted.
pub(in crate::ladder::act) fn expand_contractions(words: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for (idx, w) in words.iter().enumerate() {
        // The rule-driven contractions (data/lex/contraction_*.tsv): an
        // irregular whole form first, then any clitic suffix on any token.
        if let Some(parts) = crate::lexicon::contraction::expand(w) {
            out.extend(parts);
            continue;
        }
        // Ambiguous: 's (is/has/possessive) is left as the surface word unless
        // its host is a pronoun that takes 's as a verb (he, she, it), so the
        // reader abstains rather than guess. "he's", "she's", "it's" is "is" or
        // "has", and what follows says which: a cue word ("been", "got") means
        // has; an -ing form, or a word that cannot be a participle, means is
        // ("it's raining", "he's a nice person"). An -ed or irregular
        // participle ("he's tired") stays ambiguous, so it is left as read.
        match w.strip_suffix("'s").filter(|b| lists::s_hosts().contains(b)) {
            Some(base) => match following_word(words, idx) {
                Some(next) if lists::s_has_cues().contains(next) => {
                    out.extend([base.to_string(), "has".to_string()])
                }
                Some(next) if read_ing(next).is_some() || read_participle(next).is_none() => {
                    out.extend([base.to_string(), "is".to_string()])
                }
                _ => out.push(w.clone()),
            },
            None => out.push(w.clone()),
        }
    }
    out
}
/// The word after position `idx`, skipping the adverbs the reader sets aside,
/// so a contraction can be resolved by the word that really follows it.
pub(in crate::ladder::act) fn following_word(words: &[String], idx: usize) -> Option<&str> {
    words[idx + 1..].iter().map(String::as_str).find(|w| !lists::set_aside_adverbs().contains(w))
}

/// Whether the verb of a clause read as a complement is plausibly a verb. The
/// reader takes the word after a subject as its verb without checking, so a
/// preposition could be read as one; when the foundation lexicon is primed it
/// is asked, and the copula and auxiliaries are always accepted.
pub(in crate::ladder::act) fn clause_verb_is_verbal(pred: &str) -> bool {
    lists::aux_lemmas().contains(pred)
        || !crate::lexicon::is_primed()
        || crate::lexicon::verb_capable(pred)
}


