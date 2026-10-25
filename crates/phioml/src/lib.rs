//! # phioml — active vision as rows
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and
//! composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical
//! stores no point: a word is 32–64 relations, each a complex phasor on a
//! closed circle with a count; meaning is a side effect of pairs of words
//! moving under a condition.
//!
//! Ported from `gemphi/phi/crates/phi-oml` (docs/167), the portable
//! replacement for Apple's CoreML/MLCompute/Metal, and re-based here on
//! phistd's `c64` so the eye and the language share one number:
//!
//! - [`Aoc`] — PhiFly's AOC signal: phase + amplitude, interference by
//!   complex addition. It is one channel of a phasor, said in vision's words.
//! - [`Aoc::resonance`] — phi-oml's `cpu_resonance`: a complex multiply by a
//!   micro-jitter, O(N), twelve flops a signal, no backprop anywhere.
//! - [`Flow`] — the flow field: a grid of signals twitched twice; the phase
//!   difference under the jitter is parallax, parallax is depth, and the
//!   field is **told as rows** (`flow::rows`), so an agent `watch`es a scene
//!   the way it watches the kit — movement generates data, data is rows.

pub mod flow;
pub mod signal;

pub use flow::Flow;
pub use signal::Aoc;
