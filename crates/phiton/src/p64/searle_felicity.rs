//! What each Searle act commits the speaker to: direction of fit, felicity
//! conditions and the effect on the hearer.

use super::searle::SpeechAct;
use serde::{Deserialize, Serialize};

/// Direction of fit - Searle's distinction.
/// Mind→World: the mind should match the world (beliefs, assertions).
/// World→Mind: the world should change to match the mind (desires, commands).
/// None: no direction of fit (expressives - just express a state).
/// Both: both directions (declaratives - create a fact by representing it).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DirectionOfFit {
    MindToWorld,
    WorldToMind,
    None,
    Both,
}

impl DirectionOfFit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MindToWorld => "mind→world",
            Self::WorldToMind => "world→mind",
            Self::None => "none (expressive)",
            Self::Both => "both (declarative)",
        }
    }
}

/// Searle's felicity conditions for speech acts.
/// For an illocutionary act to be successful, these must be met.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FelicityCondition {
    /// The propositional content must be appropriate for the act type.
    pub propositional_content_rule: String,
    /// What must be true for the act to make sense.
    pub preparatory_condition: String,
    /// The psychological state the speaker must have.
    pub sincerity_condition: String,
    /// What the act counts as.
    pub essential_condition: String,
    /// Whether all conditions are met.
    pub satisfied: bool,
}

impl SpeechAct {
    /// The act's direction of fit.
    pub fn direction_of_fit(self) -> DirectionOfFit {
        match self {
            Self::Assertive => DirectionOfFit::MindToWorld,
            Self::Directive | Self::Commissive => DirectionOfFit::WorldToMind,
            Self::Expressive => DirectionOfFit::None,
            Self::Declaration => DirectionOfFit::Both,
        }
    }

    /// Searle's felicity conditions for this act.
    pub fn felicity(self) -> FelicityCondition {
        let (rule, prep, sincerity, essential) = match self {
            Self::Assertive => (
                "Must express a proposition that can be true or false",
                "Speaker has evidence for the truth of P",
                "Speaker believes P is true",
                "Tally as an undertaking that P represents an actual state of affairs",
            ),
            Self::Directive => (
                "Must express a future act A of the hearer",
                "Hearer is able to do A; speaker has authority to request",
                "Speaker wants hearer to do A",
                "Tally as an attempt to get hearer to do A",
            ),
            Self::Commissive => (
                "Must express a future act A of the speaker",
                "Speaker is able to do A",
                "Speaker intends to do A",
                "Tally as an undertaking of an obligation to do A",
            ),
            Self::Expressive => (
                "Expresses a psychological state about a state of affairs",
                "The state of affairs must be relevant to speaker and hearer",
                "Speaker genuinely has the expressed psychological state",
                "Tally as an expression of the psychological state",
            ),
            Self::Declaration => (
                "Must express a state of affairs S that the declaration brings about",
                "Speaker must have the institutional authority to bring about S",
                "Speaker intends to bring about S",
                "Tally as bringing about S by declaration",
            ),
        };
        FelicityCondition {
            propositional_content_rule: rule.into(),
            preparatory_condition: prep.into(),
            sincerity_condition: sincerity.into(),
            essential_condition: essential.into(),
            satisfied: true,
        }
    }

    /// Perlocutionary effect - the effect on the hearer.
    pub fn perlocutionary_effect(self) -> &'static str {
        match self {
            Self::Assertive => "convince/persuade - hearer comes to believe P",
            Self::Directive => "compliance - hearer performs the requested act",
            Self::Commissive => "trust - hearer relies on speaker's commitment",
            Self::Expressive => "rapport - hearer feels acknowledged",
            Self::Declaration => "institutional change - the world is altered by the declaration",
        }
    }
}
