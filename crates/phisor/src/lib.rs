//! # phisor — the intermediate language between English and the machine
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and
//! composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon.
//!
//! A sentence is a speech act over a frame: its function is the [`Force`]
//! (Searle's: assert, ask, direct, commit, express, wish, declare), its
//! structure a [`Frame`] (a predicate, its roles, tense, aspect, modality).
//! With its fillers taken out a frame is a [`Template`], as in C++: two
//! sentences that say different things in the same way are one template
//! with two fillings.
//!
//! ```
//! use phisor::ir::{Act, Aspect, Attr, Filler, Force, Frame, Role, Tense, Thing};
//!
//! let wearing = |who: &str, what: &str, colour: &str| {
//!     let theme = Thing::new(what).with(Attr::Kind("colour".into()), Filler::word(colour));
//!     Act::new(Force::Assert, Frame::new("wear", Tense::Past)
//!         .aspect(Aspect::Progressive)
//!         .with(Role::Agent, Filler::thing(who))
//!         .with(Role::Theme, Filler::Thing(theme)))
//! };
//! let girl = wearing("girl", "suit", "blue");
//! let boy = wearing("boy", "cap", "white");
//! assert_eq!(girl.to_string(), "ASSERT(wear(agent: girl, theme: suit{colour: blue}), past, progressive)");
//! assert_eq!(girl.template(), boy.template());
//! ```
//!
//! Below the act, phisor is a code at 16, 32 and 64 levels. A sentence
//! written in a mode is structure only, and its shape, how it opens and
//! closes, is its variation. The variations are clustered into kinds of
//! intention by the river, which variation follows which. One exchange
//! routine, phical's (`phical::classes::halve`), learns the word code and
//! the kinds alike, as far as the data pays for, and no label is told.
//!
//! | module | what |
//! |---|---|
//! | [`ir`] | the language: `Act`, `Force` and Searle's `Point` and `Fit`, `Frame` and its features, `Role`, `Filler`, `Template`, the written form |
//! | [`guide`] | the English guide: its units' sentences, each with its function and mark, the Key's answers, docs/203's split |
//! | [`code`] | a word as its class at 16, 32, 64: the hash (the null), the given dictionary, the learned code |
//! | [`kind`] | a sentence's variation (fixed shapes or grown templates), the kinds found by the river, the labels read against them, family resemblance |
//! | [`units`] | names as unit ids: what phisor hands the one routine, `phical::classes::halve` |
//!
//! The plan, the order and the measures: `phinet/docs/phisor/plan.md`.

pub mod code;
pub mod guide;
pub mod ir;
pub mod kind;
pub mod units;

pub use ir::{Act, Filler, Force, Frame, Role, Template};
