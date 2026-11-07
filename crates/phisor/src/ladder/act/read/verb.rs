//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Read a surface past participle back to its lemma, the inverse of
/// [`participle`]. An irregular is recognised by trying each known lemma and
/// confirming it re-realises to the surface; a regular `-ed` form is inverted
/// by the silent-e and consonant rules, again confirmed by re-realising.
/// Returns `None` when no lemma reproduces the surface, so an ambiguous or
/// unknown word makes the whole verb group abstain rather than guess.
pub(in crate::ladder::act) fn read_participle(word: &str) -> Option<String> {
    // Irregulars: a known lemma whose participle is this surface word.
    for lemma in crate::lexicon::irregular::lemmas().iter() {
        if participle(lemma) == word {
            return Some(lemma.to_string());
        }
    }
    if let Some(lemma) = crate::lexicon::lemma_of_participle(word) {
        return Some(lemma);
    }
    // Regular `-ed`: the candidate lemmas are the bare stem and the stem with
    // a restored silent e (preferred for the known silent-e verbs); the
    // consonant+y rule restores `y` from `ied`. Confirmed by re-realising.
    let candidates = regular_ed_candidates(word);
    for candidate in &candidates {
        if lists::silent_e_lemmas().contains(&candidate.as_str()) && participle(candidate) == word {
            return Some(candidate.clone());
        }
    }
    for candidate in &candidates {
        if participle(candidate) == word {
            return Some(candidate.clone());
        }
    }
    None
}

/// Read a surface `-ing` present participle back to its lemma, the inverse of
/// [`ing`]. The `-ing` suffix loses whether a silent e was dropped ("living"
/// could be `liv` or `live`), so the reader restores the e only for the known
/// silent-e verbs the Guide uses and otherwise keeps the bare stem; each
/// candidate is confirmed by re-realising, and an unresolved form abstains.
pub(in crate::ladder::act) fn read_ing(word: &str) -> Option<String> {
    let stem = word.strip_suffix("ing")?;
    if stem.is_empty() {
        return None;
    }
    // Prefer the silent-e restoration for the closed set of e-dropping verbs
    // the Guide uses; otherwise the bare stem. Confirm by re-realising so a
    // mismatch abstains rather than guesses.
    if let Some(lemma) = undouble(stem).filter(|l| ing(l) == word) {
        return Some(lemma);
    }
    let with_e = format!("{stem}e");
    if lists::silent_e_lemmas().contains(&with_e.as_str()) && ing(&with_e) == word {
        return Some(with_e);
    }
    if ing(stem) == word {
        return Some(stem.to_string());
    }
    if ing(&with_e) == word {
        return Some(with_e);
    }
    None
}

/// Read a surface verb into its lemma and the tense it carries, the inverse
/// of the realiser's morphology. Irregular pasts the gold set uses are named;
/// a regular `-ed` or third-person `-s` is undone by rule. A bare lemma reads
/// as the present. This lets the reader recover "boils" -> (boil, Present) and
/// "wore" -> (wear, Past) instead of storing the surface string.
pub(in crate::ladder::act) fn read_verb(word: &str) -> (String, Tense) {
    if word == "has" {
        return ("have".into(), Tense::Present);
    }
    if let Some(lemma) = crate::lexicon::lemma_of_past(word) {
        // A form identical to its base ("put", "cut") is the present.
        if lemma != word {
            return (lemma, Tense::Past);
        }
    }
    // Irregular past -> lemma from the file (the inverse of the `past` table).
    if let Some(lemma) = crate::lexicon::irregular::lemma_of_past(word) {
        return (lemma, Tense::Past);
    }
    // Regular past `-ed`: "discovered" -> discover, "arrived" -> arrive,
    // "studied" -> study. Undo the doubling the realiser never adds for the
    // gold set, then the silent-e and the consonant+y rules.
    if let Some(stem) = word.strip_suffix("ied") {
        return (format!("{stem}y"), Tense::Past);
    }
    if let Some(stem) = word.strip_suffix("ed") {
        // "arrived" lost a silent e: if adding it back yields a known shape we
        // cannot tell, so prefer the e-form when the stem ends in a consonant
        // that the realiser would have written with `d` only. The gold's
        // regular pasts are e-verbs (arrive, like) or plain (-ed) verbs; the
        // reader keeps the lemma that re-realises, which `past` decides.
        if let Some(lemma) = undouble(stem).filter(|l| past(l) == word) {
            return (lemma, Tense::Past);
        }
        // A stem ending in a doubled consonant ("add", "miss", "call") is the
        // lemma itself: "added" is add, never the non-word "adde".
        let mut tail = stem.chars().rev();
        let doubled = matches!((tail.next(), tail.next()), (Some(a), Some(b)) if a == b && !"aeiou".contains(a));
        if doubled && past(stem) == word {
            return (stem.to_string(), Tense::Past);
        }
        let with_e = format!("{stem}e");
        // Both stems re-realise to the same surface ("wanted" from want or
        // wante); a stem the lexicon knows as a verb wins unless the e-form is
        // a named silent-e verb, so "wanted" is want, never the non-word wante.
        let bare_is_verb = crate::lexicon::is_primed()
            && crate::lexicon::verb_capable(stem)
            && !lists::silent_e_lemmas().contains(&with_e.as_str());
        if bare_is_verb && past(stem) == word {
            return (stem.to_string(), Tense::Past);
        }
        if past(&with_e) == word {
            return (with_e, Tense::Past);
        }
        if past(stem) == word {
            return (stem.to_string(), Tense::Past);
        }
        // Fall through: treat as past with the bare stem.
        return (stem.to_string(), Tense::Past);
    }
    // Third-person present `-s`: "boils" -> boil, "goes" -> go, "flies" ->
    // fly. Only undo it when re-realising the lemma reproduces the surface,
    // so a genuine noun-like verb is not mangled.
    if let Some(stem) = word.strip_suffix("ies") {
        let lemma = format!("{stem}y");
        if third_s(&lemma) == word {
            return (lemma, Tense::Present);
        }
    }
    if let Some(stem) = word.strip_suffix("es") {
        if third_s(stem) == word {
            return (stem.to_string(), Tense::Present);
        }
    }
    if let Some(stem) = word.strip_suffix('s') {
        if !stem.is_empty() && third_s(stem) == word {
            return (stem.to_string(), Tense::Present);
        }
    }
    (word.to_string(), Tense::Present)
}

/// Read an auxiliary verb group starting at `words[from]` into its lemma,
/// tense, aspect, voice, polarity and the number of words it spans - the
/// inverse of the realiser's [`verb_group`]. It recognises the perfect
/// (`have`+participle), the progressive (`be`+`-ing`), the perfect
/// progressive (`have been`+`-ing`), the passive (`be`+participle), and their
/// combinations, with an optional `not` after the first auxiliary. It returns
/// `None` the moment a word does not read as the auxiliary or participle the
/// chain needs, so an ambiguous group abstains rather than guesses. A modal
/// group is left to a later reader (returns `None`), as are plain simple
/// verbs (which the caller reads directly).
pub(in crate::ladder::act) fn read_verb_group(words: &[String], from: usize) -> Option<(String, Tense, Aspect, Voice, bool, usize)> {
    let mut i = from;
    let head = words.get(i)?;
    // The first auxiliary fixes tense and the family. `have`/`has`/`had` open
    // a perfect; `be`-forms open a progressive or a passive.
    let (tense, family) = match aux_entry(head) {
        Some(Entry { tense, family: Family::Have, .. }) => (tense, Aux::Have),
        Some(Entry { tense, family: Family::Be, .. }) => (tense, Aux::Be),
        _ => return None, // a modal or a plain verb: not this reader's job
    };
    i += 1;

    // Optional `not` right after the first auxiliary: "have not finished",
    // "was not broken".
    let mut neg = false;
    if words.get(i).map(String::as_str) == Some("not") {
        neg = true;
        i += 1;
    }

    match family {
        Aux::Have => {
            // Perfect family. A bare `been` continues to a perfect progressive
            // (`been` + -ing) or a perfect passive (`been` + participle);
            // otherwise the next word is the participle of a perfect active.
            if words.get(i).map(String::as_str) == Some("been") {
                i += 1;
                let next = words.get(i)?;
                if let Some(lemma) = read_ing(next) {
                    return Some((lemma, tense, Aspect::PerfectProgressive, Voice::Active, neg, i + 1 - from));
                }
                let lemma = read_participle(next)?;
                return Some((lemma, tense, Aspect::Perfect, Voice::Passive, neg, i + 1 - from));
            }
            let lemma = read_participle(words.get(i)?)?;
            Some((lemma, tense, Aspect::Perfect, Voice::Active, neg, i + 1 - from))
        }
        Aux::Be => {
            // Progressive or passive family. `being` + participle is a
            // progressive passive; a bare `-ing` is a progressive active; a
            // bare participle is a simple passive. A word that is neither
            // abstains (it is a copula complement, handled by the caller).
            if words.get(i).map(String::as_str) == Some("being") {
                i += 1;
                let lemma = read_participle(words.get(i)?)?;
                return Some((lemma, tense, Aspect::Progressive, Voice::Passive, neg, i + 1 - from));
            }
            let next = words.get(i)?;
            if let Some(lemma) = read_ing(next) {
                return Some((lemma, tense, Aspect::Progressive, Voice::Active, neg, i + 1 - from));
            }
            let lemma = read_participle(next)?;
            Some((lemma, tense, Aspect::Simple, Voice::Passive, neg, i + 1 - from))
        }
    }
}

/// Read a surface modal word into its sense and tense, the inverse of
/// [`modal_word`]: `could` is `can` in the past, `would` is `will`, and so on.
/// `cannot` reads as `can` (its negation is handled by the caller). Returns
/// `None` for a non-modal word.
pub(in crate::ladder::act) fn read_modal(word: &str) -> Option<(crate::ir::Modal, Tense)> {
    match aux_entry(word)? {
        Entry { tense, family: Family::Modal(modal), .. } => Some((modal, tense)),
        _ => None,
    }
}

/// Read the tenseless tail after a modal into its lemma, aspect, voice and the
/// number of words it spans - the inverse of [`modal_tail`]. After a modal the
/// auxiliaries are bare (`be`, `have`, not `is`/`has`), so this recognises
/// `have been`+-ing (perfect progressive), `have been`+participle (perfect
/// passive), `have`+participle (perfect), `be being`+participle (progressive
/// passive), `be`+-ing (progressive), `be`+participle (simple passive), and a
/// bare lemma (simple active). Abstains on anything else.
pub(in crate::ladder::act) fn read_modal_tail(words: &[String], from: usize) -> Option<(String, Aspect, Voice, usize)> {
    let first = words.get(from)?;
    match first.as_str() {
        "have" => {
            let mut i = from + 1;
            if words.get(i).map(String::as_str) == Some("been") {
                i += 1;
                let next = words.get(i)?;
                if let Some(lemma) = read_ing(next) {
                    return Some((lemma, Aspect::PerfectProgressive, Voice::Active, i + 1 - from));
                }
                let lemma = read_participle(next)?;
                return Some((lemma, Aspect::Perfect, Voice::Passive, i + 1 - from));
            }
            let lemma = read_participle(words.get(i)?)?;
            Some((lemma, Aspect::Perfect, Voice::Active, i + 1 - from))
        }
        "be" => {
            let mut i = from + 1;
            if words.get(i).map(String::as_str) == Some("being") {
                i += 1;
                let lemma = read_participle(words.get(i)?)?;
                return Some((lemma, Aspect::Progressive, Voice::Passive, i + 1 - from));
            }
            let next = words.get(i)?;
            if let Some(lemma) = read_ing(next) {
                return Some((lemma, Aspect::Progressive, Voice::Active, i + 1 - from));
            }
            let lemma = read_participle(next)?;
            Some((lemma, Aspect::Simple, Voice::Passive, i + 1 - from))
        }
        // A bare lemma: simple active. The modal is the only auxiliary, so the
        // tail is a single word - the predicate as read, not an aspect chain.
        _ => {
            if is_aux(first) {
                return None; // an unexpected auxiliary after the modal: abstain
            }
            Some((first.clone(), Aspect::Simple, Voice::Active, 1))
        }
    }
}
