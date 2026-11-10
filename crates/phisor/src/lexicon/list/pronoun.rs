use super::word::{load_plain, load_tagged, Words};
use std::sync::OnceLock;

tagged_list!(
    /// Pronouns the reader keeps as filler words, never as verbs.
    pronouns,
    "pronoun.tsv",
    "reader"
);

tagged_list!(
    /// Subject pronouns, which can open a clause.
    subject_pronouns,
    "pronoun.tsv",
    "subject"
);

tagged_list!(
    /// Object pronouns, which read as a recipient after a ditransitive verb.
    object_pronouns,
    "pronoun.tsv",
    "object"
);

plain_list!(
    /// Pronoun hosts whose following 	o + verb modifies them.
    to_modified_hosts,
    "to_modified_host.txt"
);

tagged_list!(
    /// Pronouns that take 's as is/has (he's), never as a possessive.
    s_hosts,
    "pronoun.tsv",
    "s_host"
);

tagged_list!(
    /// Words after 's that make it has (he's been).
    s_has_cues,
    "contraction_s.tsv",
    "has"
);

tagged_list!(
    /// Pronouns whose present verb takes `-s`.
    third_singular_pronouns,
    "pronoun.tsv",
    "third_singular"
);

tagged_list!(
    /// First-person subject pronouns.
    first_person_pronouns,
    "pronoun.tsv",
    "first_person"
);
