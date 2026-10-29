//! The frame: a predicate with its roles filled, and what the verb group
//! says about it. It is the structure of a sentence; the force is its
//! function.

use super::features::{Aspect, Modal, Polarity, Tense, Voice};
use super::{Filler, Role};

/// A predicate, its slots, and its tense, aspect, modality, voice and
/// polarity.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    /// The verb's lemma: `wear`, `be`.
    pub pred: String,
    /// The slots in the order the sentence fills them.
    pub slots: Vec<(Role, Filler)>,
    pub tense: Tense,
    pub aspect: Aspect,
    pub modal: Option<Modal>,
    pub voice: Voice,
    pub polarity: Polarity,
}

impl Frame {
    /// A frame with no slots: simple aspect, active, positive, no modal.
    pub fn new(pred: &str, tense: Tense) -> Self {
        Frame { pred: pred.to_string(), slots: Vec::new(), tense, aspect: Aspect::Simple, modal: None, voice: Voice::Active, polarity: Polarity::Positive }
    }

    pub fn with(mut self, role: Role, filler: Filler) -> Self {
        self.slots.push((role, filler));
        self
    }

    pub fn aspect(mut self, aspect: Aspect) -> Self {
        self.aspect = aspect;
        self
    }

    pub fn modal(mut self, modal: Modal) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn passive(mut self) -> Self {
        self.voice = Voice::Passive;
        self
    }

    pub fn negative(mut self) -> Self {
        self.polarity = Polarity::Negative;
        self
    }

    /// The filler of a role, if the frame has it.
    pub fn get(&self, role: &Role) -> Option<&Filler> {
        self.slots.iter().find(|(r, _)| r == role).map(|(_, f)| f)
    }
}
