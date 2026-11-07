//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// Read "A <conjunction> B" where A and B each read as a clause of their own.
/// A conjunction between two nouns ("karen and joe played tennis") is not a
/// split, because the words before it are not a clause; so this only fires
/// when both sides read, and both are plain assertions.
pub(in crate::ladder::act) fn read_compound(words: &[String]) -> Option<Act> {
    if words.len() < 5 {
        return None;
    }
    // Find the first conjunction whose left side is a clause of its own: that
    // clause is the head the trailing clauses hang under. A conjunction between
    // two nouns ("karen and joe played tennis") has no clause on its left, so
    // it is skipped and the sentence stays uncovered.
    for k in 2..=(words.len() - 3) {
        if !lists::clause_conjunctions().contains(&words[k].as_str()) {
            continue;
        }
        let Some(head) = read_plain_clause(&words[..k]) else { continue };
        // Attach every trailing "<conjunction> <clause>" in turn, looping past
        // the first conjunction so a sentence with three or more clauses reads
        // whole. Each trailing clause is a plain verbal assertion delimited by
        // the next clause conjunction (or the end); any gap that is not such a
        // clause means the whole sentence is uncovered - never a partial frame.
        if let Some(act) = read_trailing_clauses(head, words, k) {
            return Some(act);
        }
    }
    None
}

/// A span of words read as a plain verbal assertion: the unit a compound hangs
/// its clauses on. `None` unless it reads whole as an `Assert` over a verbal
/// predicate, so a bare noun phrase or a question is never taken for a clause.
pub(in crate::ladder::act) fn read_plain_clause(words: &[String]) -> Option<Frame> {
    let act = read_inner("x.", words.to_vec())?;
    (act.force == Force::Assert && clause_verb_is_verbal(&act.frame.pred)).then_some(act.frame)
}

/// Attach each trailing "<conjunction> <clause>" onto `head`, starting at the
/// conjunction `first`. Loops over every following clause conjunction, reading
/// the span up to the next one (or the sentence end) as a plain clause. Returns
/// the whole frame only when every span reads; otherwise `None`, never a frame
/// missing any trailing words.
pub(in crate::ladder::act) fn read_trailing_clauses(head: Frame, words: &[String], first: usize) -> Option<Act> {
    let mut frame = head;
    let mut conj = first;
    loop {
        let start = conj + 1;
        // The next clause conjunction after this one bounds the current clause;
        // with none left, the clause runs to the end of the sentence.
        let next = (start + 1..words.len())
            .find(|&i| lists::clause_conjunctions().contains(&words[i].as_str()));
        let end = next.unwrap_or(words.len());
        let clause = read_plain_clause(&words[start..end])?;
        frame = frame.with(Role::named(&words[conj]), Filler::clause(clause));
        match next {
            Some(n) => conj = n,
            None => return Some(Act::new(Force::Assert, frame)),
        }
    }
}

/// Read the words after the verb group into the frame: an optional object
/// (theme), trailing prepositional phrases, and - in the passive - a trailing
/// `by <np>` demoted agent (Murphy Unit 42-44). Returns the Act, or `None`
/// when any word is left unaccounted for, so a partial parse abstains rather
/// than dropping material it cannot place.
///
/// A sentence that reads whole today is never disturbed: the plain reading is
/// tried first, and only when it abstains is a trailing purpose infinitive
/// ("went to the shop to buy bread") peeled off and read (R3.3, task 6.3).
pub(in crate::ladder::act) fn finish_predicate(frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    finish_plain(frame.clone(), words, from, force)
        .or_else(|| finish_with_purpose(frame, words, from, force))
}

/// The purpose-infinitive reading of a predicate tail: the first `to` followed
/// by a bare verb splits the tail in two. The words before it must read whole
/// on their own (object and prepositional phrases), and the words after it read
/// as an infinitive clause sharing the matrix agent. The clause is stored under
/// `Via("to")`, the way a prepositional `to` phrase is. Returns `None` unless
/// the whole tail is accounted for - never a partial frame (R3.4).
///
/// Not read: a passive or agentless frame (no controlling subject), and a
/// control verb, whose `to` is its own complement and is read by the control
/// branch, so a control infinitive is never read twice.
pub(in crate::ladder::act) fn finish_with_purpose(frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    if frame.voice != Voice::Active || lists::control_verbs().contains(&frame.pred.as_str()) {
        return None;
    }
    let agent = frame.get(&Role::Agent)?.clone();
    for k in from..words.len().saturating_sub(1) {
        if words[k] != "to" || !is_purpose_verb(&words[k + 1]) {
            continue;
        }
        if k > 0 && lists::to_modified_hosts().contains(&words[k - 1].as_str()) {
            continue;
        }
        let Some(head) = finish_plain(frame.clone(), &words[..k], from, force) else { continue };
        let Some(inner) = read_infinitive(&words[k + 1..], &agent) else { continue };
        let frame = head.frame.with(Role::Via("to".into()), Filler::clause(inner.frame));
        return Some(Act::new(force, frame));
    }
    None
}

/// The plain reading of the words after the verb group (see
/// [`finish_predicate`]): objects and prepositional phrases only.
pub(in crate::ladder::act) fn finish_plain(mut frame: Frame, words: &[String], from: usize, force: Force) -> Option<Act> {
    let passive = frame.voice == Voice::Passive;
    let pred = frame.pred.clone();
    let rest = &words[from..];
    let mut i = 0;

    // The active voice takes post-verbal objects: a ditransitive recipient
    // first ("could have told you"), then the theme. The passive has promoted
    // its theme to subject, so it reads no object - but a bare time/place
    // adverb that reads under a preposition is still an adjunct in either
    // voice ("will be finished tomorrow").
    if !passive {
        if leading_recipient(&pred, &rest[i..]) {
            frame = frame.with(Role::Recipient, Filler::pronoun(&canon_pronoun(&rest[i])));
            i += 1;
        }
        let here = &rest[i..];
        if !here.is_empty() && !is_preposition(&here[0]) && !is_by_agent(here) {
            if let Some((role, filler, end)) = read_theme(here) {
                let (filler, extra) = match role {
                    Role::Theme => with_host_infinitive(filler, &here[end..], frame.get(&Role::Agent)),
                    _ => (filler, 0),
                };
                frame = frame.with(role, filler);
                i += end + extra;
            }
        }
    } else {
        // In the passive, read only a leading bare adverb as its adjunct
        // (not a theme); other objects are ungrammatical here.
        let here = &rest[i..];
        if here.first().is_some_and(|w| is_time_adverb(w) || is_place_adverb(w)) {
            if let Some((role, filler, end)) = read_theme(here) {
                frame = frame.with(role, filler);
                i += end;
            }
        }
    }

    let tail = &rest[i..];
    // The passive demoted agent: a trailing `by <np>` is the agent, not a
    // `Via("by")` adjunct ("has been repaired by a mechanic").
    let (by_words, after_by) = if passive && is_by_agent(tail) {
        match read_noun_phrase(&tail[1..]) {
            Some((agent, end)) => {
                frame = frame.with(Role::Agent, agent);
                (1 + end, &tail[1 + end..])
            }
            None => (0, tail),
        }
    } else {
        (0, tail)
    };
    let (preps, prep_words) = read_prepositions(after_by);
    for (role, filler) in preps {
        frame = frame.with(role, filler);
    }
    if i + by_words + prep_words != rest.len() {
        // Trailing words the reader cannot account for: abstain rather than
        // drop them silently and claim a partial read.
        return None;
    }
    Some(Act::new(force, frame))
}

/// Whether the words begin a passive `by <noun phrase>` demoted agent, as
/// opposed to a `by` adjunct of means. A shallow test: `by` followed by a
/// noun phrase, used only in the passive where the gold stores the agent.
pub(in crate::ladder::act) fn is_by_agent(words: &[String]) -> bool {
    words.first().map(String::as_str) == Some("by") && words.len() > 1
}

/// Read a post-verbal object into its role, filler and the words it spans. A
/// pronoun object stays a `Pronoun` ("do it"); a bare time adverb is stored
/// under `on` as the Guide does ("come tomorrow" -> on=tomorrow); otherwise a
/// noun phrase fills the theme. Returns `None` when nothing reads cleanly.
pub(in crate::ladder::act) fn read_theme(words: &[String]) -> Option<(Role, Filler, usize)> {
    let first = words.first()?;
    if is_object_pronoun(first) || lists::pronouns().contains(&first.as_str()) {
        return Some((Role::Theme, Filler::pronoun(&canon_pronoun(first)), 1));
    }
    if is_time_adverb(first) {
        return Some((Role::named("on"), Filler::thing(first), 1));
    }
    if is_place_adverb(first) {
        return Some((Role::named("in"), Filler::thing(first), 1));
    }
    let (thing, end) = read_noun_phrase(words)?;
    Some((Role::Theme, thing, end))
}

