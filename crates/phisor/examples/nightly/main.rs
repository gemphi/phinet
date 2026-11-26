//! `nightly` — phisor learns a night at a time and keeps what it learned
//! (docs/phisor/learn.md). The night takes the next sources from the queue
//! (`data/learn/source.tsv`) and learns from the guide's training sentences
//! and every source taken so far. The word code (with opening senses) and
//! the kinds (the opening template, read inside, at 16, 32 and 64) resume
//! from the newest saved night, so they grow instead of starting over. The
//! kinds are read against the guide's held-out sentences, never told; the
//! night's trees, its row in `night.tsv` and the page `learned.html` are
//! saved under `data/learn`.
//!
//! ```text
//! cargo run --release -p phisor --example nightly -- [--take 2] [--stamp 2026-11-25-0200]
//! cargo run --release -p phisor --example nightly -- --page   (draw the page again, learn nothing)
//! ```

mod page;

use phical::classes::{Grow, Halving};
use phisor::checkpoint::{self, record, source, state};
use phisor::code::{Code, Coder, LEVELS};
use phisor::guide::{self, Function, Mark, Part, Split};
use phisor::kind::{sensing, Kinds, Reading, Rule, Sense, Tables, Twin};
use phistd::stats::purity;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// The first unit number a source's sentences take: past every unit of the guide.
const MORE_UNIT: usize = 1_000_000;

/// Where tonight's word code started.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Start {
    /// No night was saved.
    First,
    /// From the newest night's paths, which learned better than fresh.
    Resumed,
    /// From nothing, which learned better than resuming.
    Fresh,
}

impl Start {
    fn name(self) -> &'static str {
        match self {
            Start::First => "first",
            Start::Resumed => "resumed",
            Start::Fresh => "fresh",
        }
    }
}

fn main() -> std::io::Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let flag = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned();
    let take: usize = flag("--take").and_then(|s| s.parse().ok()).unwrap_or(1);
    let stamp = flag("--stamp").unwrap_or_else(|| SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs()).to_string());
    let (dir, root) = (checkpoint::dir(), checkpoint::root());
    // `--page` draws the page again from the newest night and learns nothing.
    match (a.iter().any(|x| x == "--page"), state::newest(&dir)) {
        (true, Some(night)) => return page::write(&dir, &night).map(|p| println!("the page {}", p.display())),
        (true, None) => return Ok(eprintln!("no night is saved yet")),
        _ => {}
    }
    let started = Instant::now();
    let person = root.join("crates/phiano/data/person");
    let split = Split::new(guide::read(&std::fs::read_to_string(person.join("murphy.txt"))?, &std::fs::read_to_string(person.join("murphy_key.txt"))?));
    let (old, new) = source::tonight(&dir, &root, take);
    let mut more = Vec::new();
    let mut fresh = Vec::new();
    for (i, s) in old.iter().chain(&new).enumerate() {
        match std::fs::read_to_string(&s.path) {
            Ok(text) => {
                let got = guide::more::read(&text, MORE_UNIT + i);
                match new.contains(s) {
                    true => fresh.push((s, got.len())),
                    false => {}
                }
                more.extend(got);
            }
            Err(e) => eprintln!("could not read {} ({e}); it is not taken tonight", s.path.display()),
        }
    }
    let split = split.with_more(more);
    let resumed = state::newest(&dir);
    let prior = |file: &str| resumed.as_ref().map(|n| state::read(&n.join(file))).unwrap_or_default();
    println!("# night {stamp}\n");
    println!("taken before: {} · tonight: {} · resumed from: {}", old.len(), fresh.iter().map(|(s, n)| format!("{} ({n} sentences)", s.name)).collect::<Vec<_>>().join(", "), resumed.as_ref().map_or("nothing (the first night)".to_string(), |p| p.display().to_string()));

    let train = split.of(Part::Train);
    let told: Vec<&[String]> = train.iter().map(|&i| split.sentences[i].words.as_slice()).collect();
    // Two starts, and the better is kept (Liskov's better): the code resumed
    // from the saved paths, and the code learned fresh. They are judged on the
    // training likelihood alone; the held-out sentences never choose. Resuming
    // keeps the tree where it was; a fresh start can find a better one on the
    // larger text (docs/phisor/learn.md, "Resumed or fresh").
    let likelihood = |c: &Code| c.halving.likelihood.last().copied().unwrap_or(f64::MIN);
    let kept = Code::resume(&told, Grow::Paid, true, &prior(state::CODE));
    let (code, start) = match resumed.is_some() {
        false => (kept, Start::First),
        true => {
            let fresh = Code::learn_opening(&told, Grow::Paid);
            println!("likelihood: resumed {:.1} · fresh {:.1}", likelihood(&kept), likelihood(&fresh));
            match likelihood(&fresh) > likelihood(&kept) {
                true => (fresh, Start::Fresh),
                false => (kept, Start::Resumed),
            }
        }
    };
    // The kinds' saved paths name templates in the saved code's classes: they
    // resume only when the code did.
    let kind_prior = |file: &str| match start {
        Start::Resumed => prior(file),
        _ => Default::default(),
    };
    let night = state::night(&dir, &stamp);
    state::write(&night.join(state::CODE), &code.paths())?;
    let pairs: usize = told.iter().map(|s| s.len() + 1).sum();
    let mut row = record::Night {
        stamp: stamp.clone(),
        sources: old.len() + fresh.len(),
        sentences: told.len(),
        words: code.len(),
        classes: code.halving.classes,
        depth: code.halving.depth,
        likelihood: likelihood(&code) / pairs.max(1) as f64,
        start: start.name().to_string(),
        ..Default::default()
    };
    println!("training sentences: {} · word code ({}): {} units, {} classes at depth {}", told.len(), start.name(), code.len(), code.halving.classes, code.halving.depth);
    let mut trees: Vec<(String, Halving)> = vec![("words".to_string(), code.halving.clone())];
    let coder = Coder::Learned(code);
    let question = Mark::ALL.iter().position(|&m| m == Mark::Question).unwrap_or(0);
    let instruct = Function::ALL.iter().position(|&f| f == Function::Instruct).unwrap_or(0);
    let f1 = |set: &[Sense], j: usize| set.get(j).map_or(0.0, |s| 100.0 * s.f1());
    for &level in &LEVELS {
        let file = state::kind_file(level.count().trailing_zeros());
        let kinds = Kinds::resume(&split, &coder, level, Twin::Open, Reading::Inside, Grow::Paid, &kind_prior(&file));
        state::write(&night.join(&file), &kinds.paths())?;
        let held = Tables::read(&kinds, &split, Part::Test);
        page::labels(&night.join(format!("label-{}.tsv", level.count())), &held)?;
        let [function, mark] = sensing(&kinds, &split, Rule::Majority);
        let [function_lift, mark_lift] = sensing(&kinds, &split, Rule::Lift);
        let m = record::Mode {
            classes: level.count(),
            templates: kinds.templates(),
            kinds: kinds.occupied(),
            function: 100.0 * purity(&held.function),
            mark: 100.0 * purity(&held.mark),
            question: f1(&mark, question),
            instruct: f1(&function, instruct),
            question_lift: f1(&mark_lift, question),
            instruct_lift: f1(&function_lift, instruct),
        };
        println!("mode {}: {} templates in {} kinds · function {:.1}% · mark {:.1}% · questions F1 {:.1}% · instructions F1 {:.1}% (by lift {:.1}% · {:.1}%)", m.classes, m.templates, m.kinds, m.function, m.mark, m.question, m.instruct, m.question_lift, m.instruct_lift);
        row.modes.push(m);
        trees.push((format!("kinds {}", level.count()), kinds.halving.clone()));
    }
    let named: Vec<(String, &Halving)> = trees.iter().map(|(n, h)| (n.clone(), h)).collect();
    record::growth(&night.join("growth.tsv"), &named)?;
    row.seconds = started.elapsed().as_secs_f64();
    record::night(&dir, &row)?;
    for (s, n) in fresh {
        source::take(&dir, &stamp, s, n)?;
    }
    let page = page::write(&dir, &night)?;
    println!("\nsaved {} · the page {} · {:.0}s", night.display(), page.display(), row.seconds);
    Ok(())
}
