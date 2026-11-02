//! The re-recording path (R11.10) — the same three-part rule the index
//! table's `Rerecord` holds, written out here because this target links no
//! crate. Previous, new and reason all present is `Recorded`; any absent is
//! `Unjudged`, which fails.

/// One of the three pieces a re-recording must carry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Previous,
    New,
    Reason,
}

impl Part {
    pub const ALL: [Part; 3] = [Part::Previous, Part::New, Part::Reason];

    pub const fn name(self) -> &'static str {
        match self {
            Part::Previous => "previous",
            Part::New => "new",
            Part::Reason => "reason",
        }
    }
}

/// What a re-recording came to. `Recorded` is the only outcome that does not
/// fail, and it needs all three parts present (R11.10).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    Recorded,
    Unjudged { absent: Vec<Part> },
}

impl Verdict {
    /// Whether this verdict fails the Harness. Only `Unjudged` does.
    pub fn fails(&self) -> bool {
        matches!(self, Verdict::Unjudged { .. })
    }

    pub fn name(&self) -> &'static str {
        match self {
            Verdict::Recorded => "recorded",
            Verdict::Unjudged { .. } => "unjudged",
        }
    }
}

/// A re-recording of the pinned numbers: the values as they were, the values
/// as they now are, and why they moved. All three are optional so an
/// incomplete attempt can be built, judged and written out — the case R11.10
/// is about.
#[derive(Clone, Debug, Default)]
pub struct Rerecord {
    pub previous: Option<String>,
    pub new: Option<String>,
    pub reason: Option<String>,
}

impl Rerecord {
    /// A complete re-recording: the previous values, the new values and the
    /// reason.
    pub fn new(previous: &str, new: &str, reason: &str) -> Self {
        Rerecord {
            previous: Some(previous.to_string()),
            new: Some(new.to_string()),
            reason: Some(reason.to_string()),
        }
    }

    /// Which of the three parts are absent, in `Part::ALL` order. A part
    /// present but blank counts as absent: an unstated reason is unstated
    /// however it is spelled.
    pub fn absent(&self) -> Vec<Part> {
        let held = [&self.previous, &self.new, &self.reason];
        Part::ALL
            .into_iter()
            .zip(held)
            .filter(|(_, text)| text.as_deref().is_none_or(|t| t.trim().is_empty()))
            .map(|(part, _)| part)
            .collect()
    }

    /// `Recorded` when all three parts are present, else `Unjudged` naming
    /// the absent ones.
    pub fn judge(&self) -> Verdict {
        let absent = self.absent();
        if absent.is_empty() {
            Verdict::Recorded
        } else {
            Verdict::Unjudged { absent }
        }
    }

    /// The re-recording written out, verdict first. An absent part is written
    /// as its name against nothing, so the page shows the gap.
    pub fn page(&self) -> String {
        let verdict = self.judge();
        let mut page = format!("# the-numbers re-recording — verdict: {}\n", verdict.name());
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

/// Task 4.2's re-recording: the move-keeping rule moved from `gain > 0.0` to
/// `phical::liskov::staged` reporting the move better and not worse, which
/// changes what the learner learns and so the numbers on The_Guide. It
/// arrives with its previous values, its new values and its reason, so it is
/// `Recorded` rather than a silent drift. The values it names are the pins
/// `numbers` holds — the change having been re-recorded, the pins are the
/// after.
pub fn task_4_2_rerecording() -> Rerecord {
    Rerecord::new(
        "loss 101.2; scores 21.3 / 101.2 / 24.8 / 84.7 under the gain > 0.0 move rule",
        "loss 101.2; scores 21.3 / 101.2 / 24.8 / 84.7 under phical::liskov::staged",
        "task 4.2 changes what is learned: a move is kept exactly when staged reports it \
         better and not worse, not on gain > 0.0 (R9.3). The numbers are re-recorded through \
         this path rather than failing as a silent drift (R11.10).",
    )
}
