//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Whether the subject is third-person singular (a noun, or he/she/it), so
/// the present verb takes `-s` and negation uses `does`. The first reader's
/// gold shapes are third-person; a fuller reader would track number.
pub(in crate::ladder::act) fn third_singular(frame: &Frame) -> bool {
    match frame.get(&Role::Agent).or_else(|| frame.get(&Role::Theme)) {
        Some(Filler::Pronoun(p)) => list::third_singular_pronouns().contains(p.to_lowercase()),
        Some(Filler::Thing(t)) => !t.plural,
        _ => true,
    }
}

/// Whether the subject is first person (`I`/`we`), so the present copula is
/// `am`/`are` and a fuller agreement can be read off it.
pub(in crate::ladder::act) fn is_first_person(frame: &Frame) -> bool {
    match frame.get(&Role::Agent).or_else(|| frame.get(&Role::Theme)) {
        Some(Filler::Pronoun(p)) => {
            list::first_person_pronouns().contains(p.to_lowercase())
        }
        _ => false,
    }
}

/// Whether the frame's agent is a first-person pronoun (`I`/`we`).
pub(in crate::ladder::act) fn is_first_person_agent(frame: &Frame) -> bool {
    matches!(frame.get(&Role::Agent),
        Some(Filler::Pronoun(p)) if list::first_person_pronouns().contains(p.to_lowercase()))
}

/// Say a filler as words: a name/pronoun/word as itself, a thing as its
/// determiner, attributes and head.
pub(in crate::ladder::act) fn say_filler(filler: &Filler) -> Vec<String> {
    match filler {
        Filler::Name(n) | Filler::Pronoun(n) | Filler::Word(n) => vec![n.clone()],
        Filler::Hole(wh) => vec![wh_surface(*wh)],
        Filler::Thing(thing) => say_thing(thing),
        Filler::Clause(frame) => verb_group(frame),
    }
}

/// Say a thing: determiner, modifiers, head (pluralised crudely with `s`).
pub(in crate::ladder::act) fn say_thing(thing: &Thing) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(det) = &thing.det {
        if let Some(mut word) = determiner_word(det, thing.plural) {
            // `a` -> `an` before a vowel sound (the head, or an adjective if
            // one precedes it). A shallow vowel-letter test, which the gold
            // set's words obey.
            if word == "a" {
                let next = thing.attrs.iter().find_map(|(_, f)| match f {
                    Filler::Word(w) => Some(w.clone()),
                    _ => None,
                }).unwrap_or_else(|| thing.head.clone());
                if starts_vowel_sound(&next) {
                    word = "an".into();
                }
            }
            out.push(word);
        }
    }
    // The attributes the book teaches: an adjective (mod or a known kind), an
    // ordinal, a count. They sit before the head: "a blue suit", "cold
    // climates", "one hundred degrees". A prepositional attribute (`~of`)
    // follows the head.
    let mut post: Vec<String> = Vec::new();
    for (attr, value) in &thing.attrs {
        use crate::ir::Attr;
        match attr {
            Attr::Via(prep) => {
                post.push(prep.clone());
                // A host infinitive ("something to read"): `to`, the bare
                // verb and its complements; it has no subject to say.
                match value {
                    Filler::Clause(inner) if crate::lexicon::list::is_role(prep, "infinitive_marker") => {
                        post.push(inner.pred.clone());
                        post.extend(complements_skipping(inner, None, true));
                        post.extend(adjuncts(inner, None));
                    }
                    _ => post.extend(say_filler(value)),
                }
            }
            _ => out.extend(say_filler(value)),
        }
    }
    // Never pluralise a function word that slipped into a Thing head (a
    // mis-segmented "we"/"at"): the foundation lexicon keeps it whole.
    let head = if thing.plural && crate::lexicon::inflectable(&thing.head) {
        inflect(&thing.head, Suffix::PLURAL)
    } else {
        thing.head.clone()
    };
    out.push(head);
    out.extend(post);
    out
}

/// The determiner surface form, agreeing with number: `this`/`these`,
/// `that`/`those`.
pub(in crate::ladder::act) fn determiner_word(det: &crate::ir::Det, plural: bool) -> Option<String> {
    use crate::ir::Det;
    use crate::lexicon::list::determiner_spelling as spell;
    Some(match det {
        Det::Definite => spell("definite", plural)?,
        Det::Indefinite => spell("indefinite", plural)?,
        Det::Near => spell("near", plural)?,
        Det::Far => spell("far", plural)?,
        Det::Quant(q) => q.clone(),
        Det::Owner(o) => o.clone(),
    })
}

