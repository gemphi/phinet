//! Moved from `ladder/act.rs` unchanged.
use super::*;

/// The present third-person `-s` form: `boil` -> `boils`, `go` -> `goes`,
/// `fly` -> `flies`. The closed rule Murphy Unit 2 teaches.
pub(in crate::ladder::act) fn third_s(verb: &str) -> String {
    if crate::lexicon::lists::is_role(verb, "have_lemma") {
        return crate::lexicon::lists::word_of("have_3sg").into();
    }
    if verb.ends_with('s') || verb.ends_with('x') || verb.ends_with("ch") || verb.ends_with("sh") || verb.ends_with('o') {
        format!("{verb}es")
    } else if let Some(stem) = verb.strip_suffix('y') {
        if stem.chars().last().is_some_and(|c| !"aeiou".contains(c)) {
            return format!("{stem}ies");
        }
        format!("{verb}s")
    } else {
        format!("{verb}s")
    }
}

/// The `-ing` present participle: `wear` -> `wearing`, `wait` -> `waiting`,
/// `rain` -> `raining`, a silent-e dropped (`make` -> `making`).
pub(in crate::ladder::act) fn ing(verb: &str) -> String {
    if let Some(form) = crate::lexicon::irregular::ing(verb) {
        return form;
    }
    if doubles_final(verb) {
        return format!("{verb}{}ing", verb.chars().last().unwrap_or_default());
    }
    if let Some(stem) = verb.strip_suffix('e') {
        if !stem.is_empty() && !verb.ends_with("ee") {
            return format!("{stem}ing");
        }
    }
    format!("{verb}ing")
}

/// Whether `-ed`/`-ing` double the verb's final consonant: a one-syllable
/// lemma that ends consonant + single vowel + consonant (stop, sit, run,
/// plan), or a named stressed multi-syllable verb. A final `w`, `x` or `y`
/// never doubles, and `qu` counts as a consonant (quit -> quitting). The
/// irregular table overrides this for the past (`past` consults it first).
pub(in crate::ladder::act) fn doubles_final(verb: &str) -> bool {
    if lists::doubling_stressed().contains(&verb) {
        return true;
    }
    let masked = verb.replace("qu", "kw");
    let chars: Vec<char> = masked.chars().collect();
    let n = chars.len();
    let vowel = |c: char| "aeiou".contains(c);
    if n < 3 || !chars.iter().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    let groups = chars.windows(2).filter(|w| vowel(w[1]) && !vowel(w[0])).count() + usize::from(vowel(chars[0]));
    !vowel(chars[n - 1])
        && !"wxy".contains(chars[n - 1])
        && vowel(chars[n - 2])
        && !vowel(chars[n - 3])
        && groups == 1
}

/// Undo the doubling in a stem read off `-ed`/`-ing` (`stopp` -> `stop`). The
/// bare stem wins when the lexicon knows it as a verb ("add", "kiss", "purr");
/// `s`, `f` and `z` never undouble, and a lemma under three letters is not
/// proposed. Otherwise the result is a lemma that would double again.
pub(in crate::ladder::act) fn undouble(stem: &str) -> Option<String> {
    let mut it = stem.chars().rev();
    let (last, prev) = (it.next()?, it.next()?);
    if last != prev || "aeiouyswfz".contains(last) || crate::lexicon::verb_capable(stem) {
        return None;
    }
    let lemma = &stem[..stem.len() - last.len_utf8()];
    let named = lists::doubling_stressed().contains(&lemma);
    if (last == 'l' && !named) || lemma.len() < 3 || !doubles_final(lemma) {
        return None;
    }
    Some(lemma.to_string())
}

/// The past participle used by the perfect and the passive. Irregulars the
/// gold set uses are named; the rest take `-ed`. These are the book's own
/// irregular list, learned once.
pub(in crate::ladder::act) fn participle(verb: &str) -> String {
    crate::lexicon::irregular_participle(verb)
        .or_else(|| crate::lexicon::irregular::participle(verb))
        .unwrap_or_else(|| past(verb))
}

/// The lemma candidates for a regular `-ed` participle: the bare stem, the
/// stem with a restored silent e, and the consonant+y restoration from `ied`.
pub(in crate::ladder::act) fn regular_ed_candidates(word: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(stem) = word.strip_suffix("ied") {
        out.push(format!("{stem}y"));
    }
    if let Some(stem) = word.strip_suffix("ed") {
        out.extend(undouble(stem));
        out.push(stem.to_string());
        out.push(format!("{stem}e"));
    }
    out
}

/// The simple past form. Irregulars the gold set uses are named; the rest
/// take `-ed` (with the silent-e and consonant rules the book gives).
pub(in crate::ladder::act) fn past(verb: &str) -> String {
    if let Some(p) = crate::lexicon::irregular_past(verb).or_else(|| crate::lexicon::irregular::past(verb)) {
        return p;
    }
    if doubles_final(verb) {
        return format!("{verb}{}ed", verb.chars().last().unwrap_or_default());
    }
    if let Some(stem) = verb.strip_suffix('e') {
        if !stem.is_empty() {
            return format!("{verb}d");
        }
    }
    if let Some(stem) = verb.strip_suffix('y') {
        if stem.chars().last().is_some_and(|c| !"aeiou".contains(c)) {
            return format!("{stem}ied");
        }
    }
    format!("{verb}ed")
}

/// The plural surface form. Irregulars the gold set uses are named; a noun
/// ending vowel+y takes `-s` (monday -> mondays), consonant+y takes `-ies`,
/// a sibilant `-es`, the rest `-s`.
pub(in crate::ladder::act) fn pluralise(head: &str) -> String {
    if let Some((_, plural)) = lists::irregular_plurals().iter().find(|(singular, _)| singular == head) {
        return plural.clone();
    }
    if head.ends_with('s') || head.ends_with('x') || head.ends_with("ch") || head.ends_with("sh") {
        format!("{head}es")
    } else if let Some(stem) = head.strip_suffix('y') {
        if stem.chars().last().is_some_and(|c| "aeiou".contains(c)) {
            format!("{head}s")
        } else {
            format!("{stem}ies")
        }
    } else {
        format!("{head}s")
    }
}
// ----- The reader: English -> Act, for the shapes it handles -----

/// Reduce a surface head to `(singular, is_plural)` by the crude inverse of
/// [`pluralise`]. Honest and shallow: irregular plurals (child/children) are
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
        if (stem.ends_with('s') || stem.ends_with('x') || stem.ends_with("ch")) && pluralise(stem) == head {
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
        if stem.len() >= 2 && pluralise(stem) == head {
            return (stem.to_string(), true);
        }
    }
    (head.to_string(), false)
}

