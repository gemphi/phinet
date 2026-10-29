//! The units' lines, read as docs/201 reads them (`examples/intent.rs`):
//! `Unit` on one line and `N  Title` on the next opens unit N; an exercise
//! line (`17.2  Complete the sentences …`) instructs; every other line
//! explains. Each line is cut at `.` `?` `!`, and each piece keeps the mark
//! that ended it.
//!
//! One departure from docs/201: an exercise item's number (`3 I have a car
//! …`) is dropped, as the Key's reader drops the Key's. Kept, the numbering
//! alone told the units' lines from the answers (measured 2026-10-29: a
//! whole word class was the item numbers).

use super::{words, Function, Mark, Sentence, SHORTEST};

/// `17.2 Complete the sentences using have.`: an exercise number, then text.
fn is_instruction(l: &str) -> bool {
    let mut it = l.splitn(2, char::is_whitespace);
    let head = it.next().unwrap_or("");
    let numbered = head.split_once('.').is_some_and(|(x, y)| x.parse::<usize>().is_ok() && y.parse::<usize>().is_ok());
    numbered && it.next().is_some_and(|r| r.len() > 12)
}

/// A line without its item number: `3 I have a car` → `I have a car`.
fn without_item(l: &str) -> &str {
    let mut it = l.splitn(2, char::is_whitespace);
    match (it.next().map(|h| !h.is_empty() && h.chars().all(|c| c.is_ascii_digit())), it.next()) {
        (Some(true), Some(rest)) => rest.trim_start(),
        _ => l,
    }
}

/// A line's pieces, each with the mark that ended it (`Line` for the last
/// piece when the line ends without one).
fn pieces(body: &str) -> Vec<(&str, Mark)> {
    let mut out = Vec::new();
    let mut start = 0;
    for (i, c) in body.char_indices() {
        match Mark::of(c) {
            Some(m) => {
                out.push((&body[start..i], m));
                start = i + c.len_utf8();
            }
            None => {}
        }
    }
    out.push((&body[start..], Mark::Line));
    out
}

pub fn read(text: &str) -> Vec<Sentence> {
    let mut out = Vec::new();
    let lines: Vec<&str> = text.lines().map(str::trim).collect();
    let mut unit = 0usize;
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        i += 1;
        match (l.starts_with("Key to Exercises") && unit > 0, l == "Unit") {
            (true, _) => break,
            (false, true) => {
                // The layout puts `Unit` on one line and `N  Title` on the next.
                match lines.get(i).and_then(|h| h.split_whitespace().next()).and_then(|n| n.parse().ok()) {
                    Some(n) => {
                        unit = n;
                        i += 1;
                        continue;
                    }
                    None => {}
                }
            }
            (false, false) => {}
        }
        match (unit, l.is_empty()) {
            (0, _) | (_, true) => continue,
            _ => {}
        }
        let (function, body) = match is_instruction(l) {
            true => (Function::Instruct, l.splitn(2, char::is_whitespace).nth(1).unwrap_or("")),
            false => (Function::Explain, without_item(l)),
        };
        for (piece, mark) in pieces(body) {
            let w = words(piece);
            match w.len() >= SHORTEST {
                true => out.push(Sentence { words: w, function, mark, unit, run: 0 }),
                false => {}
            }
        }
    }
    out
}
