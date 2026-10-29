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
    pub fn name(self) -> &'static str {
        match self {
            Tense::Past => "past",
            Tense::Present => "present",
        }
    }
}

impl Aspect {
    pub fn name(self) -> &'static str {
        match self {
            Aspect::Simple => "simple",
            Aspect::Progressive => "progressive",
            Aspect::Perfect => "perfect",
            Aspect::PerfectProgressive => "perfect progressive",
        }
    }
}

impl Modal {
    pub fn name(self) -> &'static str {
        match self {
            Modal::Will => "will",
            Modal::Can => "can",
            Modal::May => "may",
            Modal::Must => "must",
            Modal::Shall => "shall",
        }
    }
}
