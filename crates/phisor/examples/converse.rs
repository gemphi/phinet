//! `converse` — tell it English, ask it English (docs/211). Every sentence
//! that ends in `.` is told: the reader makes it an act, `ground::unfold` makes
//! the act rows about an event, a clause in it rows about a sub-event. Every
//! sentence that ends in `?` is asked: the reader makes it an act with one
//! hole, and `answer_told` reads the hole off the events the question's fixed
//! roles name, in phiclm's Lisp. A sentence the reader cannot read is said so,
//! not guessed at.
//!
//! ```text
//! cargo run -p phisor --example converse -- "sarah is driving to work." "tom was reading a book." "where is sarah driving?" "what was tom reading?"
//! cargo run -p phisor --example converse -- --file story.txt [--prime]
//! ```
//!
//! `--define "(define twice (lambda (x) (+ x x)))"` punches an operation card
//! (docs/218): appended to `data/ops/learned.lisp`, known to every machine from
//! then on. `--ops` lists what a machine is born knowing.
//!
//! `--guide murphy.txt --key murphy_key.txt` runs the guide's own sentences,
//! in order, as the harness would: its statements told, its questions asked.
//! `--show` prints the act the reader made of each line. `--usage` prints, at the end, what answering invoked - which of phiclm's
//! primitives, which of the 64 Operators, which cards, how often
//! (`machine::usage`); the count phisor's TODO asks for before any Operator
//! is added to the keyboard.
//!
//! `--prime` reads the mined lexicon (`crates/phiano/data/lex/pos.tsv`) into
//! the reader first, as the harness does: more words, and a stricter reader
//! that refuses a noun-and-verb in a statement.

use phisor::ground::unfold;
use phisor::ladder::{read, realise};
use phisor::machine::{answer_usage, Usage};

/// A machine over no rows, its cards loaded and listed.
fn bare_machine() -> (phiclm::lisp::Machine<phisor::machine::PhisorHost>, phiclm::lisp::Env) {
    let host = phisor::machine::PhisorHost::new(phical::keyboard::Arena::new(1), phisor::machine::KindRows::new()).expect("the phisor- names are free");
    let mut m = phisor::machine::checked(host).expect("a machine");
    let mut env = phiclm::lisp::Env::new();
    let loaded = phisor::machine::load_ops(&mut m, &mut env).expect("the cards load");
    println!("ops   {} keyboard Operators · {} cards: {}", loaded.keyboard, loaded.cards.len(), loaded.cards.join(" "));
    (m, env)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut lines: Vec<String> = Vec::new();
    let mut usage: Option<Usage> = None;
    let mut quiet = false;
    let mut show = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                let text = std::fs::read_to_string(&args[i + 1]).expect("read the file");
                lines.extend(text.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from));
                i += 2;
            }
            "--define" => {
                let (mut m, mut env) = bare_machine();
                match phisor::machine::add_op(&mut m, &mut env, &args[i + 1]) {
                    Ok(name) => println!("card  {name} punched → {}", phisor::machine::ops::learned().display()),
                    Err(e) => println!("card  refused: {e}"),
                }
                i += 2;
            }
            "--show" => {
                show = true;
                i += 1;
            }
            "--usage" => {
                usage = Some(Usage::default());
                i += 1;
            }
            // The guide's sentences in their order, the Key never told (phisor::guide).
            "--guide" => {
                let key = args.iter().position(|a| a == "--key").and_then(|k| args.get(k + 1)).map(|p| std::fs::read_to_string(p).expect("read the key")).unwrap_or_default();
                let text = std::fs::read_to_string(&args[i + 1]).expect("read the guide");
                lines.extend(phisor::guide::read(&text, &key).iter().filter(|s| s.function != phisor::guide::Function::Answer).map(|s| {
                    let mark = match s.mark {
                        phisor::guide::Mark::Question => "?",
                        phisor::guide::Mark::Exclamation => "!",
                        _ => ".",
                    };
                    format!("{}{mark}", s.words.join(" "))
                }));
                quiet = true;
                i += 2;
            }
            "--key" => i += 2,
            "--ops" => {
                let (_, _) = bare_machine();
                i += 1;
            }
            "--prime" => {
                let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data");
                if let Ok(p) = std::fs::read_to_string(format!("{base}/lex/pos.tsv")) {
                    phisor::lexicon::prime(&p);
                }
                if let Ok(f) = std::fs::read_to_string(format!("{base}/lex/function.txt")) {
                    phisor::lexicon::prime_function(&f);
                }
                i += 1;
            }
            other => {
                lines.push(other.to_string());
                i += 1;
            }
        }
    }
    if lines.is_empty() {
        lines = ["sarah is driving to work.", "tom was reading a book.", "where is sarah driving?", "what was tom reading?"].map(String::from).to_vec();
    }

    let mut rows = Vec::new();
    let (mut told, mut asked, mut unread, mut answered) = (0, 0, 0, 0);
    // Questions read but not answered, by what stopped them.
    let mut refused: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for line in &lines {
        let Some(act) = read(line) else {
            if !quiet {
                println!("  ?  {line}   (the reader cannot read this)");
            }
            unread += 1;
            continue;
        };
        if show {
            println!("  act {act:?}");
        }
        match line.ends_with('?') {
            false => {
                told += 1;
                let event = format!("e{told}");
                let these = unfold(&act, &event);
                if !quiet {
                    println!("told  {line}   → {} · {} rows about {event}", realise(&act), these.len());
                }
                rows.extend(these);
            }
            true => {
                asked += 1;
                match answer_usage(&act, &rows) {
                    Ok(run) => {
                        answered += 1;
                        if let Some(u) = usage.as_mut() {
                            u.add(&run.usage);
                        }
                        if !quiet || run.answered.count > 0 {
                            println!("asked {line}   → {}", match run.answered.count {
                                0 => "nothing I was told".to_string(),
                                _ => run.answered.members.join(", "),
                            });
                        }
                    }
                    Err(e) => {
                        let why = match &e {
                            phisor::machine::AskError::NotAsk { .. } => "read as a statement".to_string(),
                            phisor::machine::AskError::HoleCount { given } => format!("{given} holes"),
                            phisor::machine::AskError::NoConstraints => "no fixed role to name the events".to_string(),
                            phisor::machine::AskError::Host(_) | phisor::machine::AskError::Machine(_) => "the machine faulted".to_string(),
                        };
                        *refused.entry(why).or_insert(0) += 1;
                        if !quiet {
                            println!("asked {line}   → cannot answer: {e}");
                        }
                    }
                }
            }
        }
    }
    println!("\n{told} told ({} rows) · {asked} asked, {answered} answered · {unread} not read of {}", rows.len(), lines.len());
    for (why, n) in &refused {
        println!("  {n} read but not answered: {why}");
    }
    if let Some(u) = usage {
        println!("\n{u}");
    }
}
