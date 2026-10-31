//! `host` — phisor as a host on phiclm's Lisp, and the one check that keeps
//! its primitives out of the core's way.
//!
//! phiclm's machine binds the 39 core primitives and then the host's, and
//! `lisp::Machine::with_host` **asserts** that no host name repeats a core
//! one. An assert is a panic; Requirement 7 criterion 4 asks for a message —
//! the colliding primitive named, the reserved name it collides with, and no
//! machine constructed. [`PhisorHost::check_names`] is that message, and
//! [`checked`] is the one door a machine is built through, so the check runs
//! before the assert can fire.
//!
//! Every name phisor adds carries the `phisor-` prefix, which both documents
//! the host and puts the names out of reach of the 39 — none of which holds a
//! dash before a word. The comparison is still made, as whole names with case
//! significant: `Union` is not `union`, and neither `union-` nor `-union` is
//! either (Requirement 7 criterion 3).
//!
//! `phisor-rows` is answered here (Requirement 7 criterion 2): the rows of a
//! named Kind, an empty row sequence for a Kind holding none, and an error
//! naming the Kind when the set holds no Kind of that name. The four other
//! names are bound here and answered by this module's siblings as they land.

use std::collections::BTreeMap;
use std::fmt;

use phical::keyboard::Arena;
use phiclm::lisp::{as_text, Host, Machine, Value};

/// phiclm's 39 reserved core names, held as a static so the count is checked
/// where it is written and [`PhisorHost::core_names`] can hand out a
/// `'static` reference to it.
static CORE: [&str; 39] = phiclm::lisp::NAMES;

/// The primitives phisor adds. Every one is prefixed, and the five are
/// declared together here so the disjointness check sees the whole set at
/// once rather than one name per sibling file.
pub static NAMES: [&str; 5] = [
    "phisor-rows",
    "phisor-ask",
    "phisor-apply",
    "phisor-encode",
    "phisor-decode",
];

/// The one way a host can be wrong: a primitive repeating one of the 39.
/// Both fields carry the same text, because equality of whole names *is* the
/// collision — the two are named separately so the message says which side
/// each came from (Requirement 7 criterion 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostError {
    CoreNameCollision {
        primitive: &'static str,
        core: &'static str,
    },
}

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostError::CoreNameCollision { primitive, core } => {
                write!(
                    f,
                    "host primitive {primitive} collides with the reserved core name {core}"
                )
            }
        }
    }
}

/// A machine over `host`, its names checked against the 39 first. On a
/// collision the error is returned and **no machine is constructed**: the
/// host is dropped with nothing bound, and `Machine::with_host`'s assert is
/// never reached. On success the machine is built in the one place, so no
/// caller can skip the check.
pub fn checked<H: Host>(host: H) -> Result<Machine<H>, HostError> {
    PhisorHost::check_names(host.names())?;
    Ok(Machine::with_host(host))
}

/// The host phisor registers: the Arena its handles are minted and resolved
/// against, and the rows its `phisor-rows` primitive reads.
pub struct PhisorHost {
    arena: Arena,
    kinds: KindRows,
}

impl PhisorHost {
    /// A host over `arena` and `kinds`, refused if any of [`NAMES`] repeats
    /// a core name. The check is here as well as in [`checked`] so a host
    /// built for its own sake is held to it too.
    pub fn new(arena: Arena, kinds: KindRows) -> Result<Self, HostError> {
        Self::check_names(&NAMES)?;
        Ok(PhisorHost { arena, kinds })
    }

    /// The 39 reserved core names, for the disjointness check.
    pub fn core_names() -> &'static [&'static str; 39] {
        &CORE
    }

    /// Whether `names` is disjoint from the 39, compared as whole names with
    /// case significant. The first collision is returned, naming the
    /// primitive and the reserved name.
    pub fn check_names(names: &[&str]) -> Result<(), HostError> {
        for name in names {
            if let Some(core) = CORE.iter().copied().find(|core| *core == *name) {
                return Err(HostError::CoreNameCollision {
                    primitive: core,
                    core,
                });
            }
        }
        Ok(())
    }

    /// The Arena handles are minted and resolved against. The carriage
    /// (task 11.3) stamps its [`ArenaId`](phical::keyboard::ArenaId) into the
    /// machine's globals and checks it on every resolve.
    pub fn arena(&self) -> &Arena {
        &self.arena
    }

    pub fn arena_mut(&mut self) -> &mut Arena {
        &mut self.arena
    }

    /// The rows `phisor-rows` reads.
    pub fn kinds(&self) -> &KindRows {
        &self.kinds
    }
}

impl Host for PhisorHost {
    fn names(&self) -> &'static [&'static str] {
        &NAMES
    }

    fn call(&mut self, name: &'static str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "phisor-rows" => self.kinds.value(&as_text(&args, 0, name)?),
            "phisor-apply" => Err(format!(
                "{name}: the lowering is not built yet (machine::lower)"
            )),
            "phisor-encode" | "phisor-decode" => Err(format!(
                "{name}: the machine-word carriage is not built yet (machine::word)"
            )),
            "phisor-ask" => Err(format!(
                "{name}: pass an Act to machine::answer; ASK is lowered to local set algebra before host dispatch"
            )),
            other => Err(format!("unknown primitive {other}")),
        }
    }
}

/// Rows by Kind name. A `BTreeMap`, so the Kind names come out in one order
/// whatever order they were told in, and a run reproduces.
///
/// A Kind is in the set as soon as it is told, rows or no rows. That is the
/// distinction Requirement 7 criterion 2 turns on: a Kind holding no rows
/// answers an empty row sequence, and only a name the set does not hold at
/// all is an error.
#[derive(Clone, Debug, Default)]
pub struct KindRows(BTreeMap<String, Vec<phiecs::Row>>);

impl KindRows {
    pub fn new() -> Self {
        Self::default()
    }

    /// Every row grouped under its own `kind`, which is how a World's rows
    /// arrive. A Kind with no rows cannot come from rows, so it is told with
    /// [`told`](Self::told).
    pub fn of(rows: &[phiecs::Row]) -> Self {
        let mut set = BTreeMap::<String, Vec<phiecs::Row>>::new();
        for row in rows {
            set.entry(row.kind.clone()).or_default().push(row.clone());
        }
        KindRows(set)
    }

    /// Hold `rows` under the Kind `name`, replacing any rows told before.
    /// `told(name, &[])` is how a Kind holding no rows gets into the set.
    pub fn told(mut self, name: &str, rows: &[phiecs::Row]) -> Self {
        self.0.insert(name.to_string(), rows.to_vec());
        self
    }

    /// The rows of the Kind `name` — empty when it holds none, an error
    /// naming the Kind when the set holds no Kind of that name.
    pub fn rows(&self, name: &str) -> Result<&[phiecs::Row], String> {
        match self.0.get(name) {
            Some(rows) => Ok(rows),
            None => Err(format!("phisor-rows: no Kind named {name}")),
        }
    }

    /// The Kind names held, in order.
    pub fn names(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect()
    }

    /// How many Kinds the set holds, told rows or not.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The rows of the Kind `name` as the machine reads them: a sequence of
    /// `(about other)` pairs. The sequence is both the row sequence the
    /// criterion asks for and exactly what phiclm's `as_relation` reads, so
    /// `(referents (phisor-rows "is-a") king)` needs no conversion between
    /// the primitive and the set algebra.
    ///
    /// The Kind is the argument, so a row does not repeat it; the weight is
    /// not carried, the algebra having no place to put one.
    pub fn value(&self, name: &str) -> Result<Value, String> {
        let pair = |r: &phiecs::Row| {
            Value::List(vec![
                Value::Str(r.about.clone()),
                Value::Str(r.other.clone()),
            ])
        };
        Ok(Value::List(self.rows(name)?.iter().map(pair).collect()))
    }
}

#[cfg(test)]
mod tests;
