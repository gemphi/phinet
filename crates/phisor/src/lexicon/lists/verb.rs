use super::words::{load_pairs, load_plain, Words};
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
    /// Pro-verbs: a verb that stands for a clause ("what does she want to
    /// do?" asks for the clause itself), `machine::told::nested`.
    pro_verbs,
    "pro_verb.txt"
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
/// A modal's past form as `(modal, past)`, from `modal_past.tsv`.
pub fn modal_past() -> &'static Vec<(String, String)> {
    static CELL: OnceLock<Vec<(String, String)>> = OnceLock::new();
    CELL.get_or_init(|| load_pairs("modal_past.tsv"))
}
