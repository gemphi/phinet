use super::words::{load_plain, Words};
use std::sync::OnceLock;

plain_list!(
    /// Verbs that take a to-infinitive with the same subject (control verbs).
    control_verbs,
    "control_verb.txt"
);

plain_list!(
    /// Verbs that take a -ing complement.
    gerund_verbs,
    "gerund_verb.txt"
);

plain_list!(
    /// Prepositions that take an -ing clause ("thanked me for helping").
    gerund_prepositions,
    "gerund_preposition.txt"
);

plain_list!(
    /// Verbs that take a finite clause as their complement.
    complement_verbs,
    "complement_verb.txt"
);

plain_list!(
    /// Explicit performative verbs.
    performatives,
    "performative.txt"
);

plain_list!(
    /// Politeness markers that may open an imperative ("please close the door").
    politeness_markers,
    "politeness_marker.txt"
);

plain_list!(
    /// Base-form verbs that give an instruction.
    imperative_verbs,
    "imperative_verb.txt"
);

plain_list!(
    /// Verbs that take a recipient as their first object.
    ditransitive_verbs,
    "ditransitive_verb.txt"
);

plain_list!(
    /// Weather verbs, which take no argument and an empty expletive `it`.
    weather_verbs,
    "weather_verb.txt"
);
