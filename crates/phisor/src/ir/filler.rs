//! What fills a slot: a thing, a name, a pronoun, a bare word (an
//! attribute's value), a hole (what a question asks for), or a clause: a
//! frame as an argument, so operators take operators.

use super::Frame;

/// The determiner, by what it does rather than how it is spelled: the
/// realiser chooses `a` or `an`, `this` or `these`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Det {
    /// `the`
    Definite,
    /// `a`, `an`
    Indefinite,
    /// `this`, `these`
    Near,
    /// `that`, `those`
    Far,
    /// `some`, `any`, `no`, `every`, `each`, `all`, `many` …
    Quant(String),
    /// `my`, `your`, `his`, `her`, `its`, `our`, `their`
    Owner(String),
}

/// What an attribute of a thing is.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Attr {
    /// An adjective whose kind the world does not know yet: `mod`.
    Mod,
    /// An adjective whose kind the world knows: `colour`, read from the
    /// row `blue is-a colour`.
    Kind(String),
    /// An ordinal: `first`, `second`, `last`.
    Order,
    /// A numeral: `two`, `three`.
    Count,
    /// A prepositional phrase on the noun: `of: France`.
    Via(String),
}

/// What a question asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Wh {
    Who,
    What,
    Which,
    Where,
    When,
    Why,
    How,
    HowMany,
}

/// A noun phrase: its head, its determiner, its number, its attributes.
#[derive(Clone, Debug, PartialEq)]
pub struct Thing {
    pub head: String,
    pub det: Option<Det>,
    pub plural: bool,
    pub attrs: Vec<(Attr, Filler)>,
}

/// What fills a slot of a frame or an attribute of a thing.
#[derive(Clone, Debug, PartialEq)]
pub enum Filler {
    Thing(Thing),
    /// A proper name: `France`, `Helen`.
    Name(String),
    /// A pronoun, kept as spelled in the nominative: `I`, `she`, `they`.
    Pronoun(String),
    /// A bare word as an attribute's value: `blue`, `first`.
    Word(String),
    /// The hole a question asks the machine to fill.
    Hole(Wh),
    /// A frame as an argument: "I wish [I could sing]".
    Clause(Box<Frame>),
}

impl Thing {
    /// A bare singular noun with no determiner.
    pub fn new(head: &str) -> Self {
        Thing {
            head: head.to_string(),
            det: None,
            plural: false,
            attrs: Vec::new(),
        }
    }

    pub fn det(mut self, det: Det) -> Self {
        self.det = Some(det);
        self
    }

    pub fn plural(mut self) -> Self {
        self.plural = true;
        self
    }

    pub fn with(mut self, attr: Attr, value: Filler) -> Self {
        self.attrs.push((attr, value));
        self
    }
}

impl Filler {
    /// A bare thing: `Filler::Thing(Thing::new(head))`.
    pub fn thing(head: &str) -> Self {
        Filler::Thing(Thing::new(head))
    }

    pub fn name(name: &str) -> Self {
        Filler::Name(name.to_string())
    }

    pub fn pronoun(word: &str) -> Self {
        Filler::Pronoun(word.to_string())
    }

    pub fn word(word: &str) -> Self {
        Filler::Word(word.to_string())
    }

    pub fn clause(frame: Frame) -> Self {
        Filler::Clause(Box::new(frame))
    }
}

impl Wh {
    pub const ALL: [Wh; 8] = [
        Wh::Who,
        Wh::What,
        Wh::Which,
        Wh::Where,
        Wh::When,
        Wh::Why,
        Wh::How,
        Wh::HowMany,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Wh::Who => "who",
            Wh::What => "what",
            Wh::Which => "which",
            Wh::Where => "where",
            Wh::When => "when",
            Wh::Why => "why",
            Wh::How => "how",
            Wh::HowMany => "how-many",
        }
    }

    /// The hole a written name stands for.
    pub fn named(name: &str) -> Option<Wh> {
        Wh::ALL.into_iter().find(|w| w.name() == name)
    }
}

impl Attr {
    pub fn name(&self) -> &str {
        match self {
            Attr::Mod => "mod",
            Attr::Kind(k) => k,
            Attr::Order => "order",
            Attr::Count => "count",
            Attr::Via(p) => p,
        }
    }
}
