//! `ops` — operation cards: operators added at run time, from files.
//!
//! Lovelace's engine took two kinds of card. The mill's operations were
//! fixed — so many wheels — and a new operation was never cut into the mill;
//! it was punched on an **operation card**, a sequence of the mill's
//! operations, and fed in from outside. phisor keeps the same division. The
//! mill is the keyboard's 64 Operators and phiclm's 39 primitives, fixed and
//! law-checked. Every other operator is a card: a `define` of a lambda over
//! them, kept in a file under `data/ops/`, read into every machine at its
//! birth, and **punched at run time** by [`add`] — appended to the file and
//! defined in the running machine in one move, so the next machine is born
//! knowing it (the ruling of 2026-11-07: no data in code, every level of
//! what is known lives in the file system and can be appended to).
//!
//! | file | what |
//! |---|---|
//! | `data/ops/core.lisp` | the cards the crate ships: `meet-all`, `union-all`, `events`, `told-answer` … |
//! | `data/ops/learned.lisp` | the cards punched at run time; created by the first [`add`] |
//!
//! The 64 keyboard Operators are cards too, cut by the lowering: every
//! machine [`load`]s them as `phisor-op-<index>-<name>` before the files.

use std::path::PathBuf;

use phiclm::lisp::{read, Env, Host, Machine, Value};

use super::Lowering;

/// Where the cards live: the crate's own `data/ops/`, or `$PHISOR_OPS`.
pub fn dir() -> PathBuf {
    match std::env::var_os("PHISOR_OPS") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/data/ops")),
    }
}

/// The file a punched card is appended to.
pub fn learned() -> PathBuf {
    dir().join("learned.lisp")
}

/// The card files, in name order: `core.lisp` before `learned.lisp`.
pub fn files() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir())
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "lisp"))
        .collect();
    out.sort();
    out
}

/// What a machine was born knowing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Loaded {
    /// The 64 keyboard Operators, as the lowering's lambdas.
    pub keyboard: usize,
    /// The cards read from the files, by name, in the order defined.
    pub cards: Vec<String>,
}

/// Loads the keyboard's lambdas and every card file into `machine`. A file
/// that does not read or evaluate is the error, named; the files before it
/// stay loaded.
pub fn load<H: Host>(machine: &mut Machine<H>, env: &mut Env) -> Result<Loaded, String> {
    let mut loaded = Loaded::default();
    for lowered in Lowering::new().lower_all().lowered {
        for form in read(&lowered.lisp)? {
            machine.eval(&form, env)?;
        }
        loaded.keyboard += 1;
    }
    for file in files() {
        let src = std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
        for form in read(&src).map_err(|e| format!("{}: {e}", file.display()))? {
            if let Some(name) = defined(&form) {
                loaded.cards.push(name);
            }
            machine.eval(&form, env).map_err(|e| format!("{}: {e}", file.display()))?;
        }
    }
    Ok(loaded)
}

/// Punches a card: `source` must be one `(define name (lambda …))`. It is
/// appended to `learned.lisp` (created if missing, not doubled) and defined
/// in `machine`, so this machine and every later one know it. Returns the
/// name defined.
pub fn add<H: Host>(machine: &mut Machine<H>, env: &mut Env, source: &str) -> Result<String, String> {
    let forms = read(source)?;
    let [form] = forms.as_slice() else {
        return Err(format!("a card is one form, not {}", forms.len()));
    };
    let name = defined(form).ok_or_else(|| format!("a card is (define name (lambda (…) …)), not {form}"))?;
    let line = source.trim().replace(['\r', '\n'], " ");
    phistd::lexfile::append(learned(), &line).map_err(|e| format!("{}: {e}", learned().display()))?;
    machine.eval(form, env)?;
    Ok(name)
}

/// The name a `(define name (lambda …))` form defines.
fn defined(form: &Value) -> Option<String> {
    let Value::List(items) = form else { return None };
    match items.as_slice() {
        [Value::Sym(d), Value::Sym(name), Value::List(body), ..] if d == "define" => match body.first() {
            Some(Value::Sym(l)) if l == "lambda" => Some(name.clone()),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests;
