//! The roles a frame's slots are filled in. Four are named because the
//! clause skeleton tells them apart by position and voice; every other
//! argument keeps its preposition's name (`in: London`, `of: France`) until
//! the roles are learned from the templates that carry them.

/// The role a filler plays in a frame.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Role {
    /// Who does it: the subject of an active verb, the `by` of a passive.
    Agent,
    /// What it is done to: the object, the subject of a passive or of `be`.
    Theme,
    /// Who receives: the first of two objects ("gave her a book").
    Recipient,
    /// What `be` and `become` assign to their theme ("was the first king").
    As,
    /// An argument named by its preposition.
    Via(String),
}

impl Role {
    /// The four named roles. Every other role is a [`Role::Via`] and there
    /// are as many of those as there are prepositions, so there is no `ALL`.
    pub const NAMED: [Role; 4] = [Role::Agent, Role::Theme, Role::Recipient, Role::As];

    pub fn name(&self) -> &str {
        match self {
            Role::Agent => "agent",
            Role::Theme => "theme",
            Role::Recipient => "recipient",
            Role::As => "as",
            Role::Via(p) => p,
        }
    }

    /// The role a written name stands for. Total rather than optional: the
    /// four names are reserved and any other name is the argument's own
    /// preposition, which is what [`Role::Via`] holds.
    pub fn named(name: &str) -> Role {
        match Role::NAMED.into_iter().find(|r| r.name() == name) {
            Some(r) => r,
            None => Role::Via(name.to_string()),
        }
    }
}
