//! The Key at the back of the book (docs/194), read as phiano's workshop
//! reads it (`examples/workshop/key.rs`): `UNIT n`, then `e.m`, then
//! `k answer / alternative` with wrapped lines joined. The first
//! alternative of every answer is kept, as docs/201 keeps it. The Key is
//! never told: every one of its sentences is held out.

use super::{words, Function, Mark, Sentence, SHORTEST};

/// Every answer's first alternative, as a sentence that answers.
pub fn read(raw: &str) -> Vec<Sentence> {
    // The LAST `Key to Exercises` heading: the contents line comes first.
    let start = raw.match_indices("Key to Exercises").map(|(i, _)| i).nth(1).unwrap_or(0);
    let mut out = Vec::new();
    let mut unit = 0usize;
    let mut open: Option<String> = None;
    let close = |open: &mut Option<String>, out: &mut Vec<Sentence>, unit: usize| {
        let Some(text) = open.take() else { return };
        let first = text.split(" / ").map(words).find(|a| !a.is_empty());
        match (first, unit) {
            (Some(w), u) if u > 0 && w.len() >= SHORTEST => out.push(Sentence { words: w, function: Function::Answer, mark: Mark::Line, unit: u, run: 1 }),
            _ => {}
        }
    };
    for line in raw[start..].lines() {
        let t = line.trim();
        match t.is_empty() {
            true => continue,
            false => {}
        }
        match t.strip_prefix("UNIT ").and_then(|n| n.trim().parse::<usize>().ok()) {
            Some(n) => {
                close(&mut open, &mut out, unit);
                unit = n;
                continue;
            }
            None => {}
        }
        match t.starts_with("Key to Additional") {
            true => {
                close(&mut open, &mut out, unit);
                break;
            }
            false => {}
        }
        // `17.2`: an exercise heading.
        match t.split_once('.').is_some_and(|(a, b)| a.parse::<usize>().is_ok() && b.parse::<usize>().is_ok()) {
            true => {
                close(&mut open, &mut out, unit);
                continue;
            }
            false => {}
        }
        let mut it = t.splitn(2, ' ');
        match (it.next().and_then(|k| k.parse::<usize>().ok()), it.next()) {
            (Some(_), Some(rest)) => {
                close(&mut open, &mut out, unit);
                open = Some(rest.to_string());
            }
            (Some(_), None) => {} // a page number
            _ => match open.as_mut() {
                Some(o) => {
                    o.push(' ');
                    o.push_str(t);
                }
                None => {}
            },
        }
    }
    close(&mut open, &mut out, unit);
    out
}
