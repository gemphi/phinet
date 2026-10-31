//! The one path a frozen index table may move along.
//!
//! Requirement 6 criterion 3 and Requirement 11 criterion 10 together say
//! the same thing twice: a re-recording carries the previous table, the new
//! table and the stated reason, and if any one of those three is absent the
//! re-recording is written with the verdict `unjudged` and the run fails.
//! The point is that an index table cannot drift quietly — either the change
//! arrives with its before, its after and its why, or it arrives labelled as
//! not judged at all.
//!
//! The verdict is never "rejected": an incomplete re-recording is still
//! written out, because a refused write leaves no trace of the attempt. It
//! is written as [`Verdict::Unjudged`], which fails.

/// One of the three pieces a re-recording must carry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Previous,
    New,
    Reason,
}

impl Part {
    /// The three in the order a re-recording page writes them.
    pub const ALL: [Part; 3] = [Part::Previous, Part::New, Part::Reason];

    /// The part's name as the page spells it.
    pub const fn name(self) -> &'static str {
        match self {
            Part::Previous => "previous",
            Part::New => "new",
            Part::Reason => "reason",
        }
    }
}

/// What a re-recording came to. `Recorded` is the only outcome that does not
/// fail, and it needs all three parts present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    Recorded,
    /// One or more of the three parts was absent; the absent ones are named.
    Unjudged { absent: Vec<Part> },
}

impl Verdict {
    /// Whether this verdict fails the run. Only [`Verdict::Unjudged`] does.
    pub fn fails(&self) -> bool {
        matches!(self, Verdict::Unjudged { .. })
    }

    /// The verdict as the page spells it.
    pub fn name(&self) -> &'static str {
        match self {
            Verdict::Recorded => "recorded",
            Verdict::Unjudged { .. } => "unjudged",
        }
    }
}

/// A re-recording of the frozen index table: the table as it was, the table
/// as it now is, and why it moved. All three are optional in the type so
/// that an incomplete attempt can be built, judged and written — which is
/// precisely the case the requirement is about.
#[derive(Clone, Debug, Default)]
pub struct Rerecord {
    pub previous: Option<String>,
    pub new: Option<String>,
    pub reason: Option<String>,
}

impl Rerecord {
    /// A complete re-recording: the previous table, the new table and the
    /// reason.
    pub fn new(previous: &str, table: &str, reason: &str) -> Self {
        Rerecord {
            previous: Some(previous.to_string()),
            new: Some(table.to_string()),
            reason: Some(reason.to_string()),
        }
    }

    /// Which of the three parts are absent, in [`Part::ALL`] order. A part
    /// present but blank — a reason of spaces, an empty table — counts as
    /// absent: an unstated reason is unstated however it is spelled.
    pub fn absent(&self) -> Vec<Part> {
        let held = [&self.previous, &self.new, &self.reason];
        Part::ALL
            .into_iter()
            .zip(held)
            .filter(|(_, text)| text.as_deref().is_none_or(|t| t.trim().is_empty()))
            .map(|(part, _)| part)
            .collect()
    }

    /// The verdict: `Recorded` when all three parts are present, otherwise
    /// `Unjudged` naming the absent ones.
    pub fn judge(&self) -> Verdict {
        let absent = self.absent();
        if absent.is_empty() {
            Verdict::Recorded
        } else {
            Verdict::Unjudged { absent }
        }
    }

    /// The re-recording written out, verdict first so the reader sees whether
    /// it counts before reading what it claims. An absent part is written as
    /// the part's name against nothing, so the page shows the gap rather than
    /// hiding it.
    pub fn page(&self) -> String {
        let verdict = self.judge();
        let mut page = format!("# index table re-recording — verdict: {}\n", verdict.name());
        if let Verdict::Unjudged { absent } = &verdict {
            let names: Vec<&str> = absent.iter().map(|p| p.name()).collect();
            page.push_str(&format!("# absent: {}\n", names.join(", ")));
        }
        let held = [&self.previous, &self.new, &self.reason];
        for (part, text) in Part::ALL.into_iter().zip(held) {
            page.push_str(&format!("\n## {}\n", part.name()));
            match text.as_deref().map(str::trim).filter(|t| !t.is_empty()) {
                Some(body) => {
                    page.push_str(body);
                    page.push('\n');
                }
                None => page.push_str("(absent)\n"),
            }
        }
        page
    }
}
