//! What the learned layer may hand the Reader (design section 5): a
//! [`Proposal`] of a token, its class and a role, and nothing else. It never
//! carries a Force and never builds a Frame.
//!
//! The Reader checks a proposal against a sentence that holds the token. It
//! accepts only when it can read the whole sentence into a frame by its own
//! rules, the token fills the proposed role in that frame, and the Realiser
//! writes the frame back to the sentence under Canonical_Form (`normalise`).
//! Otherwise it rejects. The Reader's own reading is the only output, so a
//! proposal can vouch for a token but cannot change what the Reader returns.
//! With the flag off the proposals are not looked at.
use super::Config;
use crate::guide::holdout::normalise;
use crate::ir::{Act, Filler, Frame, Role};
use crate::ladder::{read, realise};

/// A token, the class the learned layer put it in, and the role it offers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub token: String,
    pub class: u16,
    pub role: Role,
}

/// Whether a filler is, or is headed by, the token.
fn holds(filler: &Filler, token: &str) -> bool {
    match filler {
        Filler::Thing(t) => t.head == token,
        Filler::Name(w) | Filler::Pronoun(w) | Filler::Word(w) => w.eq_ignore_ascii_case(token),
        Filler::Hole(_) => false,
        Filler::Clause(f) => fills(f, token, None),
    }
}

/// Whether the token fills `role` in the frame (any role when `None`),
/// looking into clause arguments too.
fn fills(frame: &Frame, token: &str, role: Option<&Role>) -> bool {
    frame.slots.iter().any(|(r, f)| match f {
        Filler::Clause(inner) if role.is_none() => fills(inner, token, None),
        _ => role.map_or(true, |want| want == r) && holds(f, token),
    })
}

impl Proposal {
    /// The Reader's verdict on this proposal for one sentence: the act the
    /// Reader built when it accepts, `None` when it rejects.
    pub fn check(&self, sentence: &str) -> Option<Act> {
        let act = read(sentence)?;
        let whole = realise(&act) == normalise(sentence);
        (whole && fills(&act.frame, &self.token, Some(&self.role))).then_some(act)
    }
}

/// The Reader's output for a sentence beside the proposals it checked.
#[derive(Debug, PartialEq)]
pub struct Review {
    /// What the Reader returns. Always its own reading.
    pub act: Option<Act>,
    pub accepted: Vec<Proposal>,
    pub rejected: Vec<Proposal>,
}

/// Read a sentence and check the proposals against it. With the flag off the
/// proposals are set aside untouched; either way `act` is `read(sentence)`.
pub fn review(config: Config, proposals: Vec<Proposal>, sentence: &str) -> Review {
    let act = read(sentence);
    if !config.enabled {
        return Review { act, accepted: Vec::new(), rejected: Vec::new() };
    }
    let (accepted, rejected) = proposals.into_iter().partition(|p| p.check(sentence).is_some());
    Review { act, accepted, rejected }
}

#[cfg(test)]
mod tests;
