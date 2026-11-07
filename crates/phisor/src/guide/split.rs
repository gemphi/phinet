//! The split: docs/203's rule on the units' sentences. One in ten is held
//! out (`i % 10 == 0`), and nothing is told it or tuned on it. One in twenty
//! is dev (`i % 20 == 5`); the rest is training. Every answer from the Key
//! is held out, because the Key is never told.

use super::{Function, Sentence};

/// Which part of the split a sentence is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Part {
    Train,
    Dev,
    Test,
}

/// The guide's sentences and the part each is in, in the guide's order.
pub struct Split {
    pub sentences: Vec<Sentence>,
    pub parts: Vec<Part>,
}

impl Split {
    /// docs/203's rule over the units' sentences, counted in order; the Key's
    /// answers held out.
    pub fn new(sentences: Vec<Sentence>) -> Self {
        let mut i = 0usize;
        let parts = sentences
            .iter()
            .map(|s| match s.function {
                Function::Answer => Part::Test,
                _ => {
                    let part = match (i % 10, i % 20) {
                        (0, _) => Part::Test,
                        (_, 5) => Part::Dev,
                        _ => Part::Train,
                    };
                    i += 1;
                    part
                }
            })
            .collect();
        Split { sentences, parts }
    }

    /// The indices of one part.
    pub fn of(&self, part: Part) -> Vec<usize> {
        (0..self.parts.len()).filter(|&i| self.parts[i] == part).collect()
    }

    /// The sentences of one part, reconstructed as reader-ready strings: the
    /// words joined with a single space and the terminating mark put back, so
    /// the reader sees the sentence as English. Only the stop and question
    /// marks are reconstructed (the reader knows those); a sentence the layout
    /// broke (Mark::Line) or ended in `!` is skipped, as the reader reads
    /// neither as a terminator it measures. This is how the held-out Guide
    /// text becomes an honest instrument for the reader - real English it was
    /// never hand-fit to, never the frozen gold.
    pub fn strings(&self, part: Part) -> Vec<String> {
        use super::Mark;
        self.of(part)
            .into_iter()
            .filter_map(|i| {
                let s = &self.sentences[i];
                let mark = match s.mark {
                    Mark::Stop => '.',
                    Mark::Question => '?',
                    _ => return None,
                };
                (s.words.len() >= super::SHORTEST).then(|| format!("{}{mark}", s.words.join(" ")))
            })
            .collect()
    }

    /// As [`strings`](Self::strings), keeping only sentences that are well
    /// formed English rather than the Guide's exercise scaffolding. The test
    /// reads the text alone - never whether the reader succeeds on it - so
    /// the cleaned set cannot be chosen to flatter the reader.
    pub fn clean_strings(&self, part: Part) -> Vec<String> {
        self.strings(part).into_iter().filter(|s| well_formed(s)).collect()
    }

    /// How many sentences of one part each Well_Formed operator rejects, in
    /// [`OPERATORS`] order, over the same strings
    /// [`clean_strings`](Self::clean_strings) filters. A sentence that two
    /// operators reject counts under both.
    pub fn removed_by(&self, part: Part) -> Vec<(&'static str, usize)> {
        let all = self.strings(part);
        OPERATORS.iter().map(|o| (o.name, all.iter().filter(|s| (o.rejects)(s)).count())).collect()
    }

    /// The sentence before `i` in the same run and unit, if there is one.
    pub fn before(&self, i: usize) -> Option<usize> {
        let p = i.checked_sub(1)?;
        let (a, b) = (&self.sentences[p], &self.sentences[i]);
        match a.run == b.run && a.unit == b.unit {
            true => Some(p),
            false => None,
        }
    }

    /// The runs of consecutive training sentences: a held-out sentence, a
    /// new unit or the Key breaks a run, so no pair ever spans one.
    pub fn training_runs(&self) -> Vec<Vec<usize>> {
        let mut runs: Vec<Vec<usize>> = Vec::new();
        let mut current: Vec<usize> = Vec::new();
        for i in 0..self.parts.len() {
            let joined = self.before(i).is_some_and(|p| self.parts[p] == Part::Train);
            match (self.parts[i], joined) {
                (Part::Train, true) => current.push(i),
                (Part::Train, false) => {
                    runs.extend((!current.is_empty()).then(|| std::mem::take(&mut current)));
                    current.push(i);
                }
                _ => runs.extend((!current.is_empty()).then(|| std::mem::take(&mut current))),
            }
        }
        runs.extend((!current.is_empty()).then_some(current));
        runs
    }
}

/// One named Well_Formed operator: a pure, text-only test that says whether a
/// sentence is exercise scaffolding. It never reads the Reader's output, the
/// gold or the answer key, so filtering cannot depend on the Reader succeeding.
pub struct Operator {
    /// Stable name, used in records.
    pub name: &'static str,
    /// One-line definition.
    pub rule: &'static str,
    /// True when the sentence is rejected (is scaffolding).
    pub rejects: fn(&str) -> bool,
}

/// The tokens of a reconstructed sentence, without its terminating mark.
fn tokens(s: &str) -> Vec<&str> {
    s.trim_end_matches(['.', '?']).split_whitespace().collect()
}

fn no_etc(s: &str) -> bool {
    tokens(s).iter().any(|t| *t == "etc")
}

fn no_stray_letter(s: &str) -> bool {
    tokens(s)
        .iter()
        .any(|t| t.chars().count() == 1 && t.chars().all(char::is_alphabetic) && *t != "a" && *t != "i")
}

fn no_doubling(s: &str) -> bool {
    tokens(s).windows(2).any(|w| w[0] == w[1])
}

fn no_leading_or(s: &str) -> bool {
    tokens(s).first() == Some(&"or")
}

fn no_label_a(s: &str) -> bool {
    let toks = tokens(s);
    match (toks.first(), toks.get(1)) {
        (Some(&"a"), Some(second)) => crate::lexicon::lists::subject_pronouns().contains(second.split('\'').next().unwrap_or("")),
        _ => false,
    }
}

/// The Well_Formed operators, defined by name in one place. A sentence is well
/// formed when no operator rejects it.
pub const OPERATORS: [Operator; 5] = [
    Operator { name: "no_etc", rule: "the sentence has no `etc` token (a fragment)", rejects: no_etc },
    Operator {
        name: "no_stray_letter",
        rule: "no token is a single letter other than `a` or `i` (an item label or split stem)",
        rejects: no_stray_letter,
    },
    Operator {
        name: "no_doubling",
        rule: "no token equals the token after it (an answer-key doubling, `shut shut`)",
        rejects: no_doubling,
    },
    Operator { name: "no_leading_or", rule: "the first token is not `or` (a list label)", rejects: no_leading_or },
    Operator {
        name: "no_label_a",
        rule: "the first token is not `a` followed directly by a pronoun or pronoun contraction",
        rejects: no_label_a,
    },
];

/// Whether a reconstructed Guide sentence is real prose and not exercise
/// scaffolding: no operator in [`OPERATORS`] rejects it.
pub(super) fn well_formed(s: &str) -> bool {
    OPERATORS.iter().all(|o| !(o.rejects)(s))
}
