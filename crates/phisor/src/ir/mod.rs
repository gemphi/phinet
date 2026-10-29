//! `ir` — the intermediate language: a sentence as an act, a force over a
//! frame (docs/phisor/ir.md).
//!
//! | file | what |
//! |---|---|
//! | `force` | `Force` (seven), Searle's `Point` (five) and `Fit` |
//! | `features` | `Tense`, `Aspect`, `Modal`, `Voice`, `Polarity`: what the verb group says |
//! | `role` | `Role`: agent, theme, recipient, as, and the prepositions' own |
//! | `filler` | `Filler`, `Thing`, `Det`, `Attr`, `Wh`: what fills a slot |
//! | `frame` | `Frame`: a predicate, its slots, its features |
//! | `template` | `Template`: the act with its fillers blinded, and the construction |
//! | `show` | the written form, `ASSERT(wear(agent: girl, …), past, progressive)` |

mod features;
pub mod filler;
mod force;
mod frame;
mod role;
pub mod show;
mod template;
#[cfg(test)]
mod tests;

pub use features::{Aspect, Modal, Polarity, Tense, Voice};
pub use filler::{Attr, Det, Filler, Thing, Wh};
pub use force::{Fit, Force, Point};
pub use frame::Frame;
pub use role::Role;
pub use template::Template;

/// A sentence in the intermediate language: what it is for, over what it
/// says.
#[derive(Clone, Debug, PartialEq)]
pub struct Act {
    pub force: Force,
    pub frame: Frame,
}

impl Act {
    pub fn new(force: Force, frame: Frame) -> Self {
        Act { force, frame }
    }

    /// The lexical template: the fillers blinded, the predicate kept.
    pub fn template(&self) -> Template {
        Template::of(self)
    }

    /// The construction: the predicate blinded too.
    pub fn construction(&self) -> Template {
        Template::construction(self)
    }
}
