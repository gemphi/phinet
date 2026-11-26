//! More text, for training only: a book or a list of sentences read the
//! guide's way (the same marks, the same words, the same shortest
//! sentence), each sentence unlabelled. As docs/207's `--more` had it, the
//! extra text goes to training only, so dev and the held-out sentences stay
//! the guide's own (`Split::with_more`).

use super::unit::pieces;
use super::{words, Function, Sentence, SHORTEST};

/// `text`'s sentences, all in `unit`. A paragraph's lines are joined first,
/// so a sentence a book wraps over two lines reads as one; a list with one
/// sentence to a line reads each by its mark.
pub fn read(text: &str, unit: usize) -> Vec<Sentence> {
    let mut out = Vec::new();
    let mut paragraph = String::new();
    for line in text.lines().map(str::trim).chain(std::iter::once("")) {
        match line.is_empty() {
            true => {
                for (piece, mark) in pieces(&paragraph) {
                    let w = words(piece);
                    match w.len() >= SHORTEST {
                        true => out.push(Sentence { words: w, function: Function::Explain, mark, unit, run: 0 }),
                        false => {}
                    }
                }
                paragraph.clear();
            }
            false => {
                paragraph.push_str(line);
                paragraph.push(' ');
            }
        }
    }
    out
}
