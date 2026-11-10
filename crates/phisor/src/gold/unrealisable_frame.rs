//! The gold frames the realiser cannot say because the frame has no field for
//! a word the sentence holds, named and counted apart so a bar measures the
//! realiser and not the gold (spec phinet-system-completion, R1.4, R1.8).
//!
//! A frame is listed only when the strict round trip misses it and the cause
//! is a missing field: the sentence has a word (`sentence_word`) that the gold
//! `Act` has no slot to carry, so no realiser reading the `Act` alone can say
//! it. Every other miss is a realiser fault and is fixed in the realiser.
//!
//! The list lives in code and the gold file is untouched: the exclusion
//! exists only in a reported figure. The pinning test beside the realiser
//! (`GOLD_MISSES`) keeps this list equal to the set of frames still failing,
//! so a frame that starts to round-trip must leave the list.
//!
//! An index is the frame's 0-based position in the file, the one
//! [`super::part_of`] and `Gold::entries` use. The file's own ids run from 1.

/// One unrealisable frame and the field it lacks (R1.4).
#[derive(Clone, Copy, Debug)]
pub struct Unrealisable {
    /// 0-based position of the frame in the gold file.
    pub index: usize,
    /// The field the gold `Act` does not have.
    pub missing: &'static str,
    /// The word of the sentence that field would carry.
    pub sentence_word: &'static str,
}

/// Every unrealisable frame, by index.
pub const UNREALISABLE: [Unrealisable; 13] = [
    Unrealisable { index: 0, missing: "count word `one` before `hundred`", sentence_word: "one" },
    Unrealisable { index: 27, missing: "verb particle", sentence_word: "out" },
    Unrealisable { index: 38, missing: "adverb", sentence_word: "before" },
    Unrealisable { index: 42, missing: "adverb", sentence_word: "already" },
    Unrealisable { index: 43, missing: "`never` value and `such` determiner", sentence_word: "never" },
    Unrealisable { index: 61, missing: "adverb", sentence_word: "later" },
    Unrealisable { index: 102, missing: "politeness marker", sentence_word: "please" },
    Unrealisable { index: 104, missing: "adverb", sentence_word: "again" },
    Unrealisable { index: 121, missing: "`going to` apart from modal `will`", sentence_word: "going" },
    Unrealisable { index: 146, missing: "result complement", sentence_word: "declared" },
    Unrealisable { index: 147, missing: "coordinated `As` filler", sentence_word: "wife" },
    Unrealisable { index: 148, missing: "adverb", sentence_word: "hereby" },
    Unrealisable { index: 149, missing: "adverb", sentence_word: "hereby" },
];

/// The field the frame at `index` lacks, or `None` when the realiser can say
/// it.
pub fn unrealisable(index: usize) -> Option<&'static Unrealisable> {
    UNREALISABLE.iter().find(|u| u.index == index)
}
