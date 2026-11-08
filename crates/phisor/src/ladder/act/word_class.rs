//! Moved from `ladder/act.rs` unchanged.
use super::*;

/// Whether a Via filler is a bare adverb that English says without its
/// stored preposition: tomorrow, today, yesterday, here, there, now, later.
pub(in crate::ladder::act) fn is_bare_adverb(filler: &Filler) -> bool {
    matches!(
        filler,
        Filler::Thing(t) if t.det.is_none() && t.attrs.is_empty()
            && (is_time_adverb(&t.head) || is_place_adverb(&t.head))
    )
}

/// Whether a word starts with a vowel *sound*, so the indefinite article is
/// `an`: the vowel letters, plus silent-h words (hour, honest, honour), minus
/// the `u`/`eu`-as-consonant words (university, european) the book notes.
pub(in crate::ladder::act) fn starts_vowel_sound(word: &str) -> bool {
    let w = word.to_lowercase();
    if let Some((_, article)) = list::article_exceptions().iter().find(|(prefix, _)| w.starts_with(prefix.as_str())) {
        return list::is_role(article, "vowel_article");
    }
    w.chars().next().is_some_and(|c| "aeiou".contains(c))
}
/// Whether `word` can open a purpose infinitive: a bare lexical verb. The
/// primed foundation lexicon decides (verb-capable, not a function word);
/// unprimed, only the reader's closed instructional verbs count, so the rule
/// stays sound without the lexicon. An inflected form ("buying", "buys") is
/// not a bare verb and does not qualify.
pub(in crate::ladder::act) fn is_purpose_verb(word: &str) -> bool {
    if is_aux(word) || is_determiner(word) || is_preposition(word) || list::pronouns().contains(&word) {
        return false;
    }
    let known = if crate::lexicon::is_primed() {
        crate::lexicon::verb_capable(word) && !crate::lexicon::function_word(word)
    } else {
        list::imperative_verbs().contains(&word)
    };
    known && read_verb(word).0 == word
}

/// A bare time adverb the Guide stores under a preposition: "tomorrow" ->
/// on=tomorrow, mirroring the realiser's bare-adverb handling.
pub(in crate::ladder::act) fn is_time_adverb(word: &str) -> bool {
    list::time_adverbs().contains(word)
}

/// A bare place adverb the Guide stores under `in`: "here" -> in=here,
/// mirroring the realiser's bare-adverb handling.
pub(in crate::ladder::act) fn is_place_adverb(word: &str) -> bool {
    list::place_adverbs().contains(word)
}

/// The determiner a word denotes, for the reader (inverse of
/// [`determiner_word`]). Only the closed set the gold frames use.
pub(in crate::ladder::act) fn determiner_of(word: &str) -> Option<crate::ir::Det> {
    list::determiner(word)
}

/// Whether a word can be an adjective modifying a head after it. The
/// foundation lexicon decides when primed - adjective-capable and not a
/// function word - so the reader needs no adjective list; a word that is
/// also a noun or a verb ("white", "blue", "light") still qualifies, since
/// the noun-phrase reader takes a modifier only when a head follows it, and
/// the position, not the tag, says which it is. Unprimed, only a small set
/// the gold uses is treated as one, to stay sound.
pub(in crate::ladder::act) fn is_adjective_word(word: &str) -> bool {
    if crate::lexicon::is_primed() {
        return matches!(crate::lexicon::tag(word), Some(t) if t.contains('a')) && !crate::lexicon::function_word(word);
    }
    list::fallback_adjectives().contains(word)
}

/// Whether a word is a cardinal number that counts the head it precedes
/// ("four languages", "ten people", "20 years"): a number word the Guide uses
/// or a run of digits. Stored as a Count attribute, which the realiser writes
/// back before the head.
pub(in crate::ladder::act) fn is_cardinal(word: &str) -> bool {
    list::cardinals().contains(word) || (word.len() <= 4 && word.chars().all(|c| c.is_ascii_digit()))
}

/// A shallow ordinal/number modifier the noun phrase may carry before its
/// head.
pub(in crate::ladder::act) fn is_modifier(word: &str) -> bool {
    list::modifiers().contains(word)
}

/// The tense of a copula form the reader builds a `be` frame from: present for
/// is/are/am, past for was/were. `None` for any other word.
pub(in crate::ladder::act) fn copula_tense(word: &str) -> Option<Tense> {
    aux_entry(word).filter(|e| e.family == Family::Be).map(|e| e.tense)
}

/// A present-tense copula the reader builds a `be` frame from.
pub(in crate::ladder::act) fn is_copula(word: &str) -> bool {
    list::copulas().contains(word)
}



/// The closed auxiliaries the first reader declines to parse.
/// The determiners the first reader sees at the head of an unsegmented
/// subject noun phrase; their presence makes it abstain.
pub(in crate::ladder::act) fn is_determiner(word: &str) -> bool {
    list::determiners().contains(word)
}

pub(in crate::ladder::act) fn is_aux(word: &str) -> bool {
    list::auxiliaries().contains(word)
}

/// The object-form pronouns that read as a recipient at the head of a
/// ditransitive command.
pub(in crate::ladder::act) fn is_object_pronoun(word: &str) -> bool {
    list::object_pronouns().contains(word)
}
/// The closed set of prepositions the reader recognises at the head of an
/// adjunct, from the gold frames.
pub(in crate::ladder::act) fn is_preposition(word: &str) -> bool {
    list::prepositions().contains(word)
}
// ----- The round-trip measure -----

