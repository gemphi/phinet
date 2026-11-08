//! `lexicon::lists` - the reader's closed word lists, read from data files.
//!
//! No language data is written in Rust. Every list the reader and the
//! realiser consult lives in a plain-text file under `data/lex/` (beside
//! `pos.tsv` and `function.txt`): one entry per line, `#` starts a comment,
//! and a mapping or a tagged list is a `.tsv`. Each list is read lazily, once,
//! the first time it is asked for, from [`lex_dir`]; a file that is missing
//! reads as an empty list, so the reader abstains rather than guesses. A list
//! learned later is appended to its file and picked up by the next run.
//!
//! Tagged lists share one file per concept: `pronoun.tsv` carries a word and a
//! comma-separated set of the roles it plays (`subject`, `object`, ...), and
//! each caller asks for the tag it needs, so no word is written twice.
#[macro_use]
mod declare;
mod determiner;
mod adverb;
mod conjunction;
mod path;
mod performative;
mod phrase;
mod pronoun;
mod spelling;
mod verb;
mod word_class;
mod word_role;
mod words;

pub use adverb::*;
pub use conjunction::*;
pub use determiner::*;
pub use path::*;
pub use performative::*;
pub use phrase::*;
pub use pronoun::*;
pub use spelling::*;
pub use verb::*;
pub use word_class::*;
pub use word_role::*;
pub use words::*;
