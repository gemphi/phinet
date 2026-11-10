//! Express frames said as a fixed phrase: "thank you for your help",
//! "welcome to london". The opening words come from `expressive_phrase.tsv`.

use super::{adjuncts, read_prepositions, say_filler, subject_filler};
use crate::ir::{Act, Aspect, Force, Frame, Polarity, Role, Tense, Voice};
use crate::lexicon::list;

/// The sentence of an Express frame whose predicate has a fixed phrase and
/// that is a plain present positive active: the phrase, the theme when the
/// phrase takes one, then the adjuncts. The agent is the speaker and is not
/// said. `None` leaves the frame to the ordinary statement.
pub(super) fn phrase(force: Force, frame: &Frame) -> Option<Vec<String>> {
    let plain = force == Force::Express
        && frame.tense == Tense::Present
        && frame.aspect == Aspect::Simple
        && frame.voice == Voice::Active
        && frame.polarity == Polarity::Positive
        && frame.modal.is_none();
    if !plain {
        return None;
    }
    let entry = list::expressive_phrases().iter().find(|e| e.pred == frame.pred)?;
    let mut words = entry.opening.clone();
    if entry.theme {
        words.extend(say_filler(frame.get(&Role::Theme)?));
    }
    words.extend(adjuncts(frame, None));
    Some(words)
}

/// The Express frame a sentence says as a fixed phrase, the reverse of
/// [`phrase`] over the same `expressive_phrase.tsv`: the opening words, the
/// theme when the phrase takes one, then prepositional adjuncts that must
/// consume the rest. The role the phrase leaves unsaid is restored from the
/// file. `None` when no phrase opens the sentence or the rest is not clean, so
/// the ordinary reader gets the sentence.
pub(super) fn read(words: &[String]) -> Option<Act> {
    for entry in list::expressive_phrases() {
        let Some(mut rest) = words.strip_prefix(entry.opening.as_slice()) else { continue };
        let mut frame = Frame::new(&entry.pred, Tense::Present);
        if let Some((role, who)) = &entry.unsaid {
            frame = frame.with(Role::named(role), subject_filler(who));
        }
        if entry.theme {
            let who = rest.first().filter(|w| list::pronouns().contains(&w.as_str()))?;
            frame = frame.with(Role::Theme, subject_filler(who));
            rest = &rest[1..];
        }
        let (preps, used) = read_prepositions(rest);
        if used != rest.len() {
            return None;
        }
        for (role, filler) in preps {
            frame = frame.with(role, filler);
        }
        // A phrase that does not say the words back lost some of them (a
        // modifier the noun phrase reader skips): leave it to abstain.
        let act = Act::new(Force::Express, frame);
        let said = format!("{}.", words.join(" "));
        return (super::realise(&act) == said).then_some(act);
    }
    None
}
