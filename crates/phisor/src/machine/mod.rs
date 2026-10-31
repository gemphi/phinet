//! `machine` — phisor on phiclm's machine: one description that both reads
//! and writes (Requirement 7). The Rust reader of `phical::keyboard` and
//! phiclm's machine run the same 64 Operators, and the agreement between
//! them is measured rather than assumed.
//!
//! Each piece lands in its own single-word file beside this one, as
//! `phical::keyboard::family`'s six do. The host, typed lowering, exact
//! machine-word carriage, differential agreement and local ASK answering
//! are here.
//!
//! | file | what | lands in |
//! |---|---|---|
//! | [`host`] | `PhisorHost`, the 39-core-name disjointness check, `KindRows` and `phisor-rows` | task 11.1 |
//! | [`lower`] | `Lowering::lower`, the `define` of a `lambda`, the 1-to-64 O-code emission | task 11.2 |
//! | [`word`] | the operand in phiclm's `Vec<i64>` store, the `ArenaId` stamp, the unrepresentable refusal | task 11.3 |
//! | `agree` | the 64 Operators compared byte for byte against the Rust reader | task 11.4 |
//! | [`ask`] | an `ASK` answered by `domain`, `meet`, `identity` and `referents` alone | task 11.5 |
//!
//! **State of this file.** [`host`], [`lower`], [`word`], [`agree`] and
//! [`ask`] have landed. A typed operation becomes a Lisp lambda and a bounded
//! O-code call site, each operand crosses the store under its declared
//! layout, the machine result is checked byte for byte against the Rust
//! reader, and ASK intersects held referents without an Oracle.

pub mod agree;
pub mod ask;
pub mod host;
pub mod lower;
pub mod word;

pub use agree::{agree, Agreement, Mismatch, STEP_CAP};
pub use ask::{answer, answer_kinds, answer_with_oracle, Answered, AskError};

pub use host::{checked, HostError, KindRows, PhisorHost};
pub use lower::{
    is_declared_ocode, Bar, Judged, Lowered, Lowering, LoweringReport, Operation, Sense, Unlowered,
    DISPATCH_SLOT, MAX_OCODE, MIN_OCODE, OCODE_NAMES,
};
pub use word::{
    arena_id, load, read, represent, resolve, word_count, write, StoreError, Unrepresentable,
    ARENA_ID_SLOT, MACHINE_WORD_BYTES,
};
