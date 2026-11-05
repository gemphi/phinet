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

/// Whether a reconstructed Guide sentence is real prose and not exercise
/// scaffolding. Excluded: an `etc` fragment; a stray single letter other than
/// `a`/`i` (an item label `b`, or a split stem `s`, `es`); a word repeated
/// straight away (an answer-key doubling, `shut shut`); and a leading label
/// (`or ...`, or `a` followed directly by a pronoun, as in `a i'm not ...`).
fn well_formed(s: &str) -> bool {
    let body = s.trim_end_matches(['.', '?']);
    let toks: Vec<&str> = body.split_whitespace().collect();
    if toks.iter().any(|t| *t == "etc") {
        return false;
    }
    if toks.iter().any(|t| t.chars().count() == 1 && t.chars().all(char::is_alphabetic) && *t != "a" && *t != "i") {
        return false;
    }
    if toks.windows(2).any(|w| w[0] == w[1]) {
        return false;
    }
    if toks.first() == Some(&"or") {
        return false;
    }
    const PRONOUNS: [&str; 7] = ["i", "you", "he", "she", "it", "we", "they"];
    if toks.first() == Some(&"a") {
        if let Some(second) = toks.get(1) {
            let head = second.split('\'').next().unwrap_or("");
            if PRONOUNS.contains(&head) {
                return false;
            }
        }
    }
    true
}
