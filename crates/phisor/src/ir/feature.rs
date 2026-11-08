//! What the verb group says about a frame: tense, aspect, modality, voice
//! and polarity. The closed stratum carries every one of them (`was` is
//! past, `be + -ing` progressive, `have + -en` perfect, `could` is `can` in
//! the past), so they are given, translated from the book; the open words
//! are what is learned.

/// English has two tenses in its forms; `will` is a modal (Murphy units 19–25).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Tense {
    Past,
    #[default]
    Present,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Aspect {
    #[default]
    Simple,
    /// `be` + `-ing`: "was wearing".
    Progressive,
    /// `have` + the past participle: "has worn".
    Perfect,
    /// `have been` + `-ing`: "has been wearing".
    PerfectProgressive,
}

/// The modals, each as its present form; `could` is `Can` in the past.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Modal {
    Will,
    Can,
    May,
    Must,
    Shall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Voice {
    #[default]
    Active,
    /// `be` + the past participle: "was broken".
    Passive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Polarity {
    #[default]
    Positive,
    Negative,
}

impl Tense {
    pub const ALL: [Tense; 2] = [Tense::Past, Tense::Present];

    pub fn name(self) -> &'static str {
        match self {
            Tense::Past => "past",
            Tense::Present => "present",
        }
    }

    /// The tense a written name stands for, as [`Force::named`] is to
    /// [`Force::name`].
    ///
    /// [`Force::name`]: super::Force::name
    /// [`Force::named`]: super::Force::named
    pub fn named(name: &str) -> Option<Tense> {
        Tense::ALL.into_iter().find(|t| t.name() == name)
    }
}

impl Aspect {
    pub const ALL: [Aspect; 4] = [
        Aspect::Simple,
        Aspect::Progressive,
        Aspect::Perfect,
        Aspect::PerfectProgressive,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Aspect::Simple => "simple",
            Aspect::Progressive => "progressive",
            Aspect::Perfect => "perfect",
            Aspect::PerfectProgressive => "perfect progressive",
        }
    }

    pub fn named(name: &str) -> Option<Aspect> {
        Aspect::ALL.into_iter().find(|a| a.name() == name)
    }
}

impl Modal {
    pub const ALL: [Modal; 5] = [
        Modal::Will,
        Modal::Can,
        Modal::May,
        Modal::Must,
        Modal::Shall,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Modal::Will => "will",
            Modal::Can => "can",
            Modal::May => "may",
            Modal::Must => "must",
            Modal::Shall => "shall",
        }
    }

    pub fn named(name: &str) -> Option<Modal> {
        Modal::ALL.into_iter().find(|m| m.name() == name)
    }
}

impl Voice {
    pub const ALL: [Voice; 2] = [Voice::Active, Voice::Passive];

    pub fn name(self) -> &'static str {
        match self {
            Voice::Active => "active",
            Voice::Passive => "passive",
        }
    }

    pub fn named(name: &str) -> Option<Voice> {
        Voice::ALL.into_iter().find(|v| v.name() == name)
    }
}

impl Polarity {
    pub const ALL: [Polarity; 2] = [Polarity::Positive, Polarity::Negative];

    pub fn name(self) -> &'static str {
        match self {
            Polarity::Positive => "positive",
            Polarity::Negative => "negative",
        }
    }

    pub fn named(name: &str) -> Option<Polarity> {
        Polarity::ALL.into_iter().find(|p| p.name() == name)
    }
}
