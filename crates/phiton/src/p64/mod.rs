//! P64 — the 16/32/64 variation engine for language classification (it was
//! phiano's `phinum`; it lives with the light whose 64 sectors it counts).
//!
//! It is the numerical aspect of language understanding. It provides
//! three granularity levels — `P16`, `P32`, `P64` — that
//! classify words, sentences, and paragraphs into structural keys without
//! storing examples. The variations and their links form a "spider net"
//! that captures language topology.
//!
//! # Architecture
//!
//! ```text
//!   P16  ──▶  16 core classifications  (fast, coarse)
//!       │
//!   P32  ──▶  32 core classifications  (balanced)
//!       │
//!   P64  ──▶  64 core classifications  (fine, complete)
//! ```
//!

pub mod bpe;
pub mod config;
pub mod iching;
pub mod lexicon;
pub mod searle;
pub mod syntax;
pub mod topology;
pub mod variants;
pub mod words;

#[allow(unused_imports)]
pub use config::Config;
#[allow(unused_imports)]
pub use iching::{Hexagram, Trigram};
#[allow(unused_imports)]
pub use lexicon::{Lexicon, WordClass};
#[allow(unused_imports)]
pub use searle::{SearleClassifier, SpeechAct};
#[allow(unused_imports)]
pub use syntax::{PartOfSpeech, PosDictionary, SyntaxKey, SyntaxParser};
#[allow(unused_imports)]
pub use words::Tokenizer;
#[allow(unused_imports)]
pub use topology::{SentenceType, ParagraphType, SpiderNet};
#[allow(unused_imports)]
pub use variants::{P16, P32, P64, Level, Variation};
