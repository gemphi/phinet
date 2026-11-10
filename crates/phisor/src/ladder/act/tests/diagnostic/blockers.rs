//! Ignored diagnostics: blocking spans and copula probes.

use super::*;

/// Diagnostic: which spans stop the reader reading a clean held-out sentence?
/// For every abstained sentence, find the shortest leading, trailing or single
/// word span whose removal makes it readable, and rank those spans. Run alone:
/// `cargo test -p phisor blocker_spans -- --ignored --nocapture`. It primes the
/// real lexicon, so it must not share a process with other priming tests.
#[test]
#[ignore = "diagnostic: ranks the spans that block the reader on clean held-out text"]
fn blocker_spans() {
    use std::collections::HashMap;
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    let (text, key) = (
        std::fs::read_to_string(format!("{base}/person/murphy.txt")),
        std::fs::read_to_string(format!("{base}/person/murphy_key.txt")),
    );
    let (Ok(text), Ok(key)) = (text, key) else { println!("no guide"); return; };
    let split = crate::guide::Split::new(crate::guide::read(&text, &key));
    let clean = split.clean_strings(crate::guide::Part::Test);

    let (mut abstained, mut unlocked) = (0usize, 0usize);
    let mut spans: HashMap<String, usize> = HashMap::new();
    for s in &clean {
        if read(s).is_some() { continue; }
        abstained += 1;
        let mark = s.chars().last().unwrap();
        let words: Vec<&str> = s.trim_end_matches(['.', '?']).split_whitespace().collect();
        let mut found: Option<String> = None;
        // Shortest removal first: 1 word anywhere, then 2-word leading/trailing, then 3.
        'search: for width in 1..=3usize {
            for start in 0..words.len() {
                let end = start + width;
                if end > words.len() { break; }
                let edge = start == 0 || end == words.len();
                if width > 1 && !edge { continue; }
                let kept: Vec<&str> = words[..start].iter().chain(words[end..].iter()).copied().collect();
                if kept.len() < 3 { continue; }
                let cand = format!("{}{}", kept.join(" "), mark);
                if read(&cand).is_some() {
                    let place = if start == 0 { "LEAD" } else if end == words.len() { "TAIL" } else { "MID " };
                    found = Some(format!("{place} {}", words[start..end].join(" ")));
                    break 'search;
                }
            }
        }
        if let Some(f) = found { unlocked += 1; *spans.entry(f).or_default() += 1; }
    }
    println!("clean {} ; abstained {abstained} ; unlocked by removing one span {unlocked}", clean.len());
    let mut ranked: Vec<_> = spans.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (span, n) in ranked.iter().take(45) { println!("{n:4}  {span}"); }
}

#[test]
#[ignore = "diagnostic: copula sentences one removal from reading"]
fn copula_blockers() {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    let (text, key) = (
        std::fs::read_to_string(format!("{base}/person/murphy.txt")),
        std::fs::read_to_string(format!("{base}/person/murphy_key.txt")),
    );
    let (Ok(text), Ok(key)) = (text, key) else { println!("no guide"); return; };
    crate::lexicon::prime_irregular(&text);
    let split = crate::guide::Split::new(crate::guide::read(&text, &key));
    let clean = split.clean_strings(crate::guide::Part::Test);
    let mut shown = 0;
    for s in &clean {
        if read(s).is_some() { continue; }
        // A copula somewhere, and removing exactly one word makes it read.
        let has_cop = s.split_whitespace().any(|w| matches!(w, "is" | "are" | "was" | "were" | "be"));
        if !has_cop { continue; }
        let mark = s.chars().last().unwrap();
        let ws: Vec<&str> = s.trim_end_matches(['.', '?']).split_whitespace().collect();
        for i in 0..ws.len() {
            let kept: Vec<&str> = ws.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, w)| *w).collect();
            if kept.len() >= 3 && read(&format!("{}{}", kept.join(" "), mark)).is_some() {
                if shown < 40 { println!("drop [{}] <- {s}", ws[i]); shown += 1; }
                break;
            }
        }
    }
}

#[test]
#[ignore = "diagnostic: probe specific copula shapes"]
fn copula_probe() {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
    if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) { crate::lexicon::prime(&p); }
    if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) { crate::lexicon::prime_function(&f); }
    for s in [
        "the office is empty.",
        "both restaurants are good.",
        "this notebook is important.",
        "rachel is from australia.",
        "this brush is for washing.",
        "it was a hot day.",
        "the house is dirty.",
    ] {
        println!("{:7} {s}", if read(s).is_some() { "READ" } else { "abstain" });
    }
}
