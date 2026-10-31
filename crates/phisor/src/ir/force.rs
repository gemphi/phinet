//! The force: what a sentence is for. Searle (Speech Acts, 1969; Expression
//! and Meaning, 1979) found five illocutionary points; phisor writes seven
//! forces, because a question and a wish are worth their own names here. A
//! question asks the machine to run a query, and a wish is the goal the
//! machine plans toward (top-down: "I wish I could do this"). Each maps to
//! one of the five points and that point's direction of fit.

/// What a sentence is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Force {
    /// Says how things are: "a girl was wearing a blue suit".
    Assert,
    /// Asks for what fills a hole: "who was the first king of France?"
    Ask,
    /// Asks the hearer to act: "complete the sentences using have".
    Direct,
    /// Binds the speaker to act: "I will explain".
    Commit,
    /// Shows a state of mind about what is presupposed: "thank you".
    Express,
    /// Wants what is not: "I wish I could sing".
    Wish,
    /// Makes a fact by saying it: "I name this ship".
    Declare,
}

/// Searle's five illocutionary points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Point {
    Assertive,
    Directive,
    Commissive,
    Expressive,
    Declarative,
}

/// The direction of fit between the words and the world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Fit {
    /// The words are to match the world (assertives).
    WordToWorld,
    /// The world is to come to match the words (directives, commissives, desire).
    WorldToWord,
    /// Neither: the fit is presupposed (expressives).
    Null,
    /// Both at once: saying it makes it so (declaratives).
    Both,
}

impl Force {
    pub const ALL: [Force; 7] = [
        Force::Assert,
        Force::Ask,
        Force::Direct,
        Force::Commit,
        Force::Express,
        Force::Wish,
        Force::Declare,
    ];

    /// The illocutionary point: a question is a directive (it asks for an
    /// answer), a wish an expressive (of desire).
    pub fn point(self) -> Point {
        match self {
            Force::Assert => Point::Assertive,
            Force::Ask | Force::Direct => Point::Directive,
            Force::Commit => Point::Commissive,
            Force::Express | Force::Wish => Point::Expressive,
            Force::Declare => Point::Declarative,
        }
    }

    /// The direction of fit. A wish is the one expressive with a fit of its
    /// own: a desire is satisfied when the world comes to match it.
    pub fn fit(self) -> Fit {
        match self {
            Force::Assert => Fit::WordToWorld,
            Force::Ask | Force::Direct | Force::Commit | Force::Wish => Fit::WorldToWord,
            Force::Express => Fit::Null,
            Force::Declare => Fit::Both,
        }
    }

    /// The written name: `ASSERT`, `ASK`, …
    pub fn name(self) -> &'static str {
        match self {
            Force::Assert => "ASSERT",
            Force::Ask => "ASK",
            Force::Direct => "DIRECT",
            Force::Commit => "COMMIT",
            Force::Express => "EXPRESS",
            Force::Wish => "WISH",
            Force::Declare => "DECLARE",
        }
    }

    /// The force a written name stands for.
    pub fn named(name: &str) -> Option<Force> {
        Force::ALL.into_iter().find(|f| f.name() == name)
    }
}
