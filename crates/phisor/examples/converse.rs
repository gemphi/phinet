//! `converse` — tell it English, ask it English (docs/211). Every sentence
//! that ends in `.` is told: the reader makes it an act, `ground` makes the
//! act rows about an event. Every sentence that ends in `?` is asked: the
//! reader makes it an act with one hole, and `answer_told` reads the hole
//! off the events the question's fixed roles name, in phiclm's Lisp. A
//! sentence the reader cannot read is said so, not guessed at.
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
//! `--prime` reads the mined lexicon (`crates/phiano/data/lex/pos.tsv`) into
//! the reader first, as the harness does: more words, and a stricter reader
//! that refuses a noun-and-verb in a statement.

use phisor::ground::ground;
use phisor::ladder::{read, realise};
use phisor::machine::answer_told;

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
    let (mut told, mut asked, mut unread) = (0, 0, 0);
    for line in &lines {
        let Some(act) = read(line) else {
            println!("  ?  {line}   (the reader cannot read this)");
            unread += 1;
            continue;
        };
        match line.ends_with('?') {
            false => {
                told += 1;
                let event = format!("e{told}");
                let these = ground(&act, &event);
                println!("told  {line}   → {} · {} rows about {event}", realise(&act), these.len());
                rows.extend(these);
            }
            true => {
                asked += 1;
                match answer_told(&act, &rows) {
                    Ok(a) => println!("asked {line}   → {}", match a.count {
                        0 => "nothing I was told".to_string(),
                        _ => a.members.join(", "),
                    }),
                    Err(e) => println!("asked {line}   → cannot answer: {e}"),
                }
            }
        }
    }
    println!("\n{told} told ({} rows) · {asked} asked · {unread} not read", rows.len());
}
