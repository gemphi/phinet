//! I Ching for P64. The hexagram, its trigrams, its King Wen number and
//! its name are phistd's (`phistd::hexagram`); what is P64's own is
//! reading a hexagram off a syntax key, and the speech act a hexagram
//! stands for.

#[cfg(test)]
mod tests;

pub use phistd::hexagram::{Hexagram, Trigram, KING_WEN_MAP};

use super::searle::SpeechAct;
use super::syntax::SyntaxKey;

/// The hexagram a structural syntax key hashes to (the one hash, FNV-1a).
pub fn of_syntax_key(key: &SyntaxKey) -> Hexagram {
    Hexagram::from_id((phistd::fnv1a(&key.key) % 64) as u8)
}

/// The Searle speech act a hexagram defaults to, by its upper trigram.
pub fn speech_act(h: Hexagram) -> SpeechAct {
    match h.upper {
        Trigram::Unity | Trigram::Thunder => SpeechAct::Declaration,
        Trigram::Lake | Trigram::Fire => SpeechAct::Expressive,
        Trigram::Wind => SpeechAct::Directive,
        Trigram::Water => SpeechAct::Commissive,
        Trigram::Mountain | Trigram::Earth => SpeechAct::Assertive,
    }
}
