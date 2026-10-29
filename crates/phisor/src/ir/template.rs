//! The template: a frame with its fillers taken out, as in C++. "a girl was
//! wearing a blue suit" and "a boy was wearing a white cap" say different
//! things the same way, and the same way is one template,
//! `ASSERT(wear(agent: $N, theme: $N{$A}), past, progressive)`. Take the
//! predicate out too and it is the construction,
//! `ASSERT($V(agent: $N, theme: $N{$A}), past, progressive)`, which "a man
//! was driving a red car" shares.
//!
//! A template is kept as its written form, so two templates are equal
//! exactly when they are written the same, and a count of templates is a
//! count of strings.

use super::show::{self, Mode};
use super::Act;
use std::fmt;

/// An act's shape: its written form with the fillers blinded.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Template(String);

impl Template {
    /// The lexical template: the fillers blinded, the predicate kept.
    pub fn of(act: &Act) -> Self {
        Template(written(act, Mode::Shape))
    }

    /// The construction: the predicate blinded too.
    pub fn construction(act: &Act) -> Self {
        Template(written(act, Mode::Construction))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn written(act: &Act, mode: Mode) -> String {
    let mut s = String::new();
    // Writing into a String cannot fail.
    let _ = show::act(&mut s, act, mode);
    s
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
