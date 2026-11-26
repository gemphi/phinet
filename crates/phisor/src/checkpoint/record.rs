//! The record. `night.tsv` gains one row a night: what it learned from and
//! how the kinds read the guide's held-out sentences at each mode, so the
//! nights can be read against each other. `growth.tsv`, in the night's
//! folder, is how each tree grew that night: depth by depth, the splits
//! kept, the likelihood, and the gain pass by pass (Babbage's differences).

use phical::classes::Halving;
use std::io;
use std::path::Path;

/// How the kinds at one mode read the held-out sentences.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mode {
    /// The mode's classes: 16, 32 or 64.
    pub classes: usize,
    pub templates: usize,
    pub kinds: usize,
    /// Function and mark purity, and the F1 of the kinds that sense questions
    /// and instructions, in percent: each kind named by its dev majority, and
    /// by its lift (`kind::Rule`).
    pub function: f64,
    pub mark: f64,
    pub question: f64,
    pub instruct: f64,
    pub question_lift: f64,
    pub instruct_lift: f64,
}

/// One night's row.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Night {
    pub stamp: String,
    /// Sources learned from so far, tonight's included.
    pub sources: usize,
    /// Training sentences: the guide's and every source's.
    pub sentences: usize,
    /// Units the word code holds, its classes and its depth.
    pub words: usize,
    pub classes: usize,
    pub depth: u32,
    /// The word code's likelihood per training word: higher is better.
    pub likelihood: f64,
    /// Where the word code started: `first`, `resumed` or `fresh`.
    pub start: String,
    pub modes: Vec<Mode>,
    pub seconds: f64,
}

const HEADER: &str = "# stamp\tsources\tsentences\twords\tclasses\tdepth\tlikelihood per word\tseconds\tthen per mode: classes templates kinds function% mark% question-F1% instruct-F1%\tthen the start: first, resumed or fresh\tthen per mode: question-F1% instruct-F1% by lift";

/// Add tonight's row to `dir/night.tsv`.
pub fn night(dir: &Path, n: &Night) -> io::Result<bool> {
    let file = dir.join("night.tsv");
    match file.exists() {
        true => {}
        false => std::fs::write(&file, format!("{HEADER}\n"))?,
    }
    let mut row = format!("{}\t{}\t{}\t{}\t{}\t{}\t{:.4}\t{:.0}", n.stamp, n.sources, n.sentences, n.words, n.classes, n.depth, n.likelihood, n.seconds);
    for m in &n.modes {
        row.push_str(&format!("\t{}\t{}\t{}\t{:.1}\t{:.1}\t{:.1}\t{:.1}", m.classes, m.templates, m.kinds, m.function, m.mark, m.question, m.instruct));
    }
    row.push_str(&format!("\t{}", n.start));
    for m in &n.modes {
        row.push_str(&format!("\t{:.1}\t{:.1}", m.question_lift, m.instruct_lift));
    }
    phistd::lexfile::append(file, &row)
}

/// Save how each named tree grew tonight.
pub fn growth(file: &Path, trees: &[(String, &Halving)]) -> io::Result<()> {
    let mut text = String::from("# tree\tdepth\tsplits kept\tlikelihood\tgain pass by pass\n");
    for (name, h) in trees {
        for (d, gains) in h.passes.iter().enumerate() {
            let passes: Vec<String> = gains.iter().map(|g| format!("{g:.1}")).collect();
            text.push_str(&format!("{name}\t{}\t{}\t{:.1}\t{}\n", d + 1, h.kept.get(d).copied().unwrap_or(0), h.likelihood.get(d).copied().unwrap_or(0.0), passes.join(",")));
        }
    }
    std::fs::write(file, text)
}
