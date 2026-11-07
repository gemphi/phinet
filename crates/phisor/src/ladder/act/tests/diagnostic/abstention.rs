//! Ignored diagnostics: why the reader abstains.

use super::*;

/// Diagnostic: which words are over-represented in the sentences the reader
/// abstains on, compared with those it reads? A word that appears in many
/// abstained sentences and in few read ones is a likely blocker. Run alone:
/// `cargo test -p phisor abstention_words -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: ranks words by how much they mark an abstained sentence"]
fn abstention_words() {
    use std::collections::{HashMap, HashSet};
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
    let (mut abst, mut read_n) = (0usize, 0usize);
    let mut in_abst: HashMap<String, usize> = HashMap::new();
    let mut in_read: HashMap<String, usize> = HashMap::new();
    for s in &clean {
        let words: HashSet<String> = s.trim_end_matches(['.', '?']).split_whitespace().map(str::to_string).collect();
        if read(s).is_some() { read_n += 1; for w in words { *in_read.entry(w).or_default() += 1; } }
        else { abst += 1; for w in words { *in_abst.entry(w).or_default() += 1; } }
    }
    println!("clean {} ; read {read_n} ; abstained {abst}", clean.len());
    // Rank by abstained count, showing the share of sentences with the word that were abstained.
    let mut rows: Vec<(String, usize, usize)> = in_abst.iter()
        .map(|(w, a)| (w.clone(), *a, *in_read.get(w).unwrap_or(&0)))
        .filter(|(_, a, r)| *a >= 12 && (*a as f64) / ((*a + *r) as f64) >= 0.9)
        .collect();
    rows.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
    println!("words in >= 12 abstained sentences and >= 90% abstained:");
    for (w, a, r) in rows.iter().take(60) { println!("{a:4} abst  {r:3} read  {w}"); }
}

/// Diagnostic: how many words must be removed to make each abstained clean
/// sentence read? Greedily remove the single word whose removal most helps
/// (makes it read, or failing that keeps trying), up to four, and report the
/// distribution of distances. Also rank the words that, removed, carry a
/// sentence the last step into reading. Run alone:
/// `cargo test -p phisor distance_to_readable -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: counts how many fixes each abstained sentence is from reading"]
fn distance_to_readable() {
    use std::collections::HashMap;
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

    let reads = |ws: &[String], mark: char| -> bool {
        if ws.len() < 3 { return false; }
        read(&format!("{}{}", ws.join(" "), mark)).is_some()
    };
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut last_word: HashMap<String, usize> = HashMap::new();
    let mut abst = 0usize;
    for s in &clean {
        if read(s).is_some() { continue; }
        abst += 1;
        let mark = s.chars().last().unwrap();
        let mut ws: Vec<String> = s.trim_end_matches(['.', '?']).split_whitespace().map(str::to_string).collect();
        let mut removed = 0usize;
        let mut done = false;
        let mut final_word = String::new();
        while removed < 4 {
            // Is any single removal enough to read now?
            let mut hit = None;
            for i in 0..ws.len() {
                let mut t = ws.clone();
                let w = t.remove(i);
                if reads(&t, mark) { hit = Some((i, w)); break; }
            }
            if let Some((i, w)) = hit {
                final_word = w;
                ws.remove(i);
                removed += 1;
                done = true;
                break;
            }
            // Otherwise remove the word whose removal shrinks the sentence
            // toward a known blocker: just drop the last content word and retry.
            if ws.len() <= 3 { break; }
            ws.pop();
            removed += 1;
        }
        let d = if done { removed } else { 9 };
        *dist.entry(d).or_default() += 1;
        if done && d <= 2 && !final_word.is_empty() {
            *last_word.entry(final_word).or_default() += 1;
        }
    }
    println!("clean {} ; abstained {abst}", clean.len());
    let mut keys: Vec<_> = dist.keys().copied().collect();
    keys.sort();
    for k in keys {
        let label = if k == 9 { "4+ (or never)".to_string() } else { format!("{k} removal(s)") };
        println!("  {:>13}: {}", label, dist[&k]);
    }
    println!("final unlocking word (sentences 1-2 removals from reading):");
    let mut rows: Vec<_> = last_word.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (w, n) in rows.iter().take(40) { println!("  {n:4}  {w}"); }
}

/// Diagnostic: classify the clean sentences the reader still abstains on, by a
/// text-only guess at why each is hard, to judge whether more reader rules can
/// pay. Buckets are mutually exclusive, tried in order. Run alone:
/// `cargo test -p phisor abstention_kinds -- --ignored --nocapture`.
#[test]
#[ignore = "diagnostic: classifies remaining abstentions by structural kind"]
fn abstention_kinds() {
    use std::collections::HashMap;
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

    let mut kinds: HashMap<&str, usize> = HashMap::new();
    let mut examples: HashMap<&str, Vec<String>> = HashMap::new();
    let mut abst = 0usize;
    for s in &clean {
        if read(s).is_some() { continue; }
        abst += 1;
        let kind = crate::ladder::abstention_kind(s);
        *kinds.entry(kind).or_default() += 1;
        let e = examples.entry(kind).or_default();
        if e.len() < 4 { e.push((*s).clone()); }
    }
    println!("clean {} ; abstained {abst}", clean.len());
    let (matched, covered, total) = read_realise_rate(clean.iter().map(|s| s.as_str()));
    println!("clean coverage {covered}/{total} ; soundness {matched}/{covered}");
    if let Ok(kind) = std::env::var("PHISOR_SHOW_KIND") {
        for s in clean.iter().filter(|s| read(s).is_none() && crate::ladder::abstention_kind(s) == kind) {
            println!("  | {s}");
        }
    }
    let mut rows: Vec<_> = kinds.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));
    for (k, n) in &rows {
        println!("{n:4}  {k}");
        for ex in &examples[k] { println!("        {ex}"); }
    }
}
