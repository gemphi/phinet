//! The explicit performative verbs and the force each names, read from
//! `performative.txt` (verb, tab, role). The role is a force's written name
//! (`COMMIT`, `WISH`), so a caller compares a [`Force`], never a word.
use super::words::{load_pairs, Words};
use crate::ir::Force;
use std::sync::OnceLock;

fn rows() -> &'static Vec<(String, Option<Force>)> {
    static CELL: OnceLock<Vec<(String, Option<Force>)>> = OnceLock::new();
    CELL.get_or_init(|| load_pairs("performative.txt").into_iter().map(|(v, r)| (v, Force::named(&r))).collect())
}

/// Explicit performative verbs.
pub fn performatives() -> &'static Words {
    static CELL: OnceLock<Words> = OnceLock::new();
    CELL.get_or_init(|| Words::new(rows().iter().map(|(v, _)| v.clone()).collect()))
}

/// The force a performative verb names, if it is one.
pub fn role_of(verb: &str) -> Option<Force> {
    rows().iter().find(|(v, _)| v == verb).and_then(|(_, r)| *r)
}

/// The first performative verb that names `force`, for the Realiser to say.
pub fn verb_of(force: Force) -> Option<&'static str> {
    rows().iter().find(|(_, r)| *r == Some(force)).map(|(v, _)| v.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_performative_names_a_force_and_says_back_its_own_verb() {
        assert!(!performatives().is_empty());
        for verb in performatives().iter() {
            let force = role_of(verb).unwrap_or_else(|| panic!("{verb} has no role"));
            assert_eq!(verb_of(force), Some(verb.as_str()));
        }
        assert_eq!(role_of(&performatives().as_slice()[0].repeat(2)), None);
    }
}
