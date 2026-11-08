//! Moved from `ladder/act.rs` unchanged.
use crate::ladder::act::*;

/// The objects in English order: recipient then theme in the active, the
/// be-complement in the `as` role. Passive puts neither after the verb.
pub(in crate::ladder::act) fn complements(frame: &Frame, subject_suppressed: bool) -> Vec<String> {
    complements_skipping(frame, None, subject_suppressed)
}

/// Say a matrix verb's theme. A clause that shares the matrix subject and is
/// otherwise a plain present active positive ("i want [i go to london]") is a
/// control infinitive and is said "to go to london", the clause's own
/// complements following and a nested infinitive composing ("promise to try
/// to pay"). Any other clause - a finite one such as "i wish i could sing" -
/// is said as before.
pub(in crate::ladder::act) fn say_theme(matrix: &Frame, theme: &Filler) -> Vec<String> {
    if let Filler::Clause(inner) = theme {
        if lists::control_verbs().contains(&matrix.pred.as_str()) && is_control_infinitive(matrix, inner) {
            let mut out = vec!["to".to_string(), inner.pred.clone()];
            out.extend(complements_skipping(inner, None, true));
            out.extend(adjuncts(inner, None));
            return out;
        }
        // A gerund complement: the `-ing` form, then the clause's complements.
        if (lists::gerund_verbs().contains(&matrix.pred.as_str()) || lists::control_verbs().contains(&matrix.pred.as_str()))
            && is_gerund_complement(matrix, inner)
        {
            let mut out = vec![inflect(&inner.pred, Suffix::ING)];
            out.extend(complements_skipping(inner, None, true));
            out.extend(adjuncts(inner, None));
            return out;
        }
        // Any other clause is finite: its own subject, verb group and complements.
        return statement(inner);
    }
    say_filler(theme)
}

/// Whether `inner` is a control infinitive of `matrix`: the same agent, and no
/// tense, modal, aspect, voice or polarity of its own to say.
pub(in crate::ladder::act) fn is_control_infinitive(matrix: &Frame, inner: &Frame) -> bool {
    let same_subject = matches!(
        (matrix.get(&Role::Agent), inner.get(&Role::Agent)),
        (Some(a), Some(b)) if a == b
    );
    same_subject
        && inner.modal.is_none()
        && inner.tense == Tense::Present
        && inner.aspect == Aspect::Simple
        && inner.voice == Voice::Active
        && inner.polarity == Polarity::Positive
}

/// As [`complements`], but omit the slot a fronted wh already consumed.
pub(in crate::ladder::act) fn complements_skipping(frame: &Frame, skip: Option<&Role>, subject_suppressed: bool) -> Vec<String> {
    let mut out = Vec::new();
    let passive = frame.voice == Voice::Passive;
    let drop = |r: &Role| skip == Some(r);
    // The theme is a post-verbal object only when it is not the surface
    // subject: an active frame with an agent, or an imperative whose subject
    // is suppressed. An agentless active statement (a copula, an unaccusative)
    // has the theme as its subject, so it is not repeated here.
    if !passive && (frame.get(&Role::Agent).is_some() || subject_suppressed) {
        if let Some(recipient) = frame.get(&Role::Recipient) {
            if !drop(&Role::Recipient) {
                out.extend(say_filler(recipient));
            }
        }
        if let Some(theme) = frame.get(&Role::Theme) {
            if !drop(&Role::Theme) {
                out.extend(say_theme(frame, theme));
            }
        }
    }
    if let Some(c) = frame.get(&Role::As) {
        if !drop(&Role::As) {
            out.extend(say_filler(c));
        }
    }
    out
}

/// The prepositional and `by`-agent adjuncts in slot order, skipping a
/// fronted wh's slot. A bare time/place adverb (tomorrow, here, later) drops
/// its preposition: the gold stores "on=tomorrow" but English says
/// "tomorrow", not "on tomorrow".
pub(in crate::ladder::act) fn adjuncts(frame: &Frame, skip: Option<&Role>) -> Vec<String> {
    let mut out = Vec::new();
    let passive = frame.voice == Voice::Passive;
    for (role, filler) in &frame.slots {
        if skip == Some(role) {
            // "who was the window broken by?" - the passive agent is the
            // fronted wh, but English strands the "by" at the end.
            if passive && role == &Role::Agent && matches!(filler, Filler::Hole(_)) {
                out.push("by".into());
            }
            continue;
        }
        match role {
            Role::Via(prep) => {
                if let Filler::Hole(_) = filler {
                    continue; // a wh already fronted, or an unfronted hole dropped
                }
                // A conjunction and its clause: "... and he stays", "... if i have time".
                if let Filler::Clause(inner) = filler {
                    out.push(prep.clone());
                    // A purpose infinitive ("... to buy bread"): `to` over a
                    // clause that shares the matrix agent is said as `to` + the
                    // bare verb and its complements, not as a finite statement.
                    if lists::gerund_prepositions().contains(&prep.as_str()) && is_gerund_complement(frame, inner) {
                        // A preposition + gerund ("... for helping him").
                        out.push(inflect(&inner.pred, Suffix::ING));
                        out.extend(complements_skipping(inner, None, true));
                        out.extend(adjuncts(inner, None));
                    } else if crate::lexicon::lists::is_role(prep, "infinitive_marker") && is_control_infinitive(frame, inner) {
                        out.push(inner.pred.clone());
                        out.extend(complements_skipping(inner, None, true));
                        out.extend(adjuncts(inner, None));
                    } else {
                        out.extend(statement(inner));
                    }
                    continue;
                }
                if lists::postpositions().contains(&prep.as_str()) {
                    out.extend(say_filler(filler));
                    out.push(prep.clone());
                } else if is_bare_adverb(filler) {
                    out.extend(say_filler(filler));
                } else {
                    out.push(prep.clone());
                    out.extend(say_filler(filler));
                }
            }
            Role::Agent if passive && frame.get(&Role::Theme).is_some() => {
                out.push("by".into());
                out.extend(say_filler(filler));
            }
            _ => {}
        }
    }
    out
}

