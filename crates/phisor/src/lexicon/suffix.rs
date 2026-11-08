//! `lexicon::suffix` - the inflection rules, read from `suffix.tsv`.
//!
//! The rules (which ending a stem takes, in which order) are English data and
//! live in `data/lex/suffix.tsv`; the letter classes they test live in
//! `data/lex/letter.tsv`. The code here holds only the named predicates over
//! those classes and the loop that applies the first rule that holds. The
//! irregular forms stay in their own files and are consulted first.
use super::lists::{self, data_rows, read_file};
use std::sync::OnceLock;

/// Which inflection to apply; the name is the `kind` column of `suffix.tsv`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Suffix(&'static str);

impl Suffix {
    /// A verb's present third-person `-s`.
    pub const THIRD: Suffix = Suffix("third");
    /// A noun's plural.
    pub const PLURAL: Suffix = Suffix("plural");
    /// The `-ing` participle.
    pub const ING: Suffix = Suffix("ing");
    /// The simple past `-ed`.
    pub const PAST: Suffix = Suffix("past");
}

type Predicate = fn(&str) -> bool;

/// One row of `suffix.tsv`.
struct Rule {
    kind: String,
    order: u32,
    condition: Option<Predicate>,
    strip: usize,
    add: String,
}

/// The rules, sorted by order, read once. A row naming an unknown condition
/// is dropped, so a typo abstains rather than applying everywhere.
fn rules() -> &'static [Rule] {
    static CELL: OnceLock<Vec<Rule>> = OnceLock::new();
    CELL.get_or_init(|| {
        let mut out: Vec<Rule> = data_rows(&read_file("suffix.tsv"))
            .into_iter()
            .filter(|r| r.len() >= 5)
            .filter_map(|r| {
                let condition = if r[2] == "*" { None } else { Some(predicate(&r[2])?) };
                Some(Rule {
                    kind: r[0].clone(),
                    order: r[1].parse().ok()?,
                    condition,
                    strip: r[3].parse().ok()?,
                    add: r[4].clone(),
                })
            })
            .collect();
        out.sort_by_key(|r| r.order);
        out
    })
}

/// The `(letter, class)` rows of `letter.tsv`, read once.
fn letters() -> &'static [(String, String)] {
    static CELL: OnceLock<Vec<(String, String)>> = OnceLock::new();
    CELL.get_or_init(|| lists::load_pairs("letter.tsv"))
}

/// Whether the letter `c` is listed under `class`.
pub fn letter_is(c: char, class: &str) -> bool {
    letters().iter().any(|(l, k)| k == class && l.chars().eq(std::iter::once(c)))
}

/// Whether `stem` ends in an entry of `class` (a letter or a longer ending).
fn ends_in(stem: &str, class: &str) -> bool {
    letters().iter().any(|(l, k)| k == class && stem.ends_with(l.as_str()))
}

/// The stem's last letter and the one before it, if both exist.
fn last_two(stem: &str) -> Option<(char, char)> {
    let mut it = stem.chars().rev();
    Some((it.next()?, it.next()?))
}

fn ends_sibilant(stem: &str) -> bool {
    ends_in(stem, "sibilant")
}

fn ends_sibilant_o(stem: &str) -> bool {
    ends_sibilant(stem) || ends_in(stem, "verb_es")
}

fn ends_y(stem: &str) -> bool {
    ends_in(stem, "y_letter")
}

/// Ends in `y` with a vowel before it (play, monday).
fn ends_vowel_y(stem: &str) -> bool {
    ends_y(stem) && last_two(stem).is_some_and(|(_, before)| letter_is(before, "vowel"))
}

/// Ends in `y` with a non-vowel before it (fly, carry).
fn ends_consonant_y(stem: &str) -> bool {
    ends_y(stem) && last_two(stem).is_some_and(|(_, before)| !letter_is(before, "vowel"))
}

/// Ends in a silent e with a letter before it (make, agree).
fn ends_silent_e(stem: &str) -> bool {
    ends_in(stem, "silent_e") && last_two(stem).is_some()
}

/// Ends in a single silent e (make), not a doubled one (agree).
fn ends_single_e(stem: &str) -> bool {
    ends_silent_e(stem) && last_two(stem).is_some_and(|(last, before)| last != before)
}

/// Whether `-ed`/`-ing` double the final consonant: a one-syllable stem that
/// ends consonant + single vowel + consonant (stop, sit, quit), or a verb
/// named in `doubling_stressed.txt`. A final `w`, `x` or `y` never doubles,
/// and a `u` after `q` counts as a consonant.
pub fn doubles_final(stem: &str) -> bool {
    if lists::doubling_stressed().contains(stem) {
        return true;
    }
    let chars: Vec<char> = stem.chars().collect();
    let n = chars.len();
    if n < 3 || !chars.iter().all(char::is_ascii_lowercase) {
        return false;
    }
    let qu = |i: usize| i > 0 && letter_is(chars[i - 1], "qu_lead") && letter_is(chars[i], "qu_follow");
    let vowel = |i: usize| letter_is(chars[i], "vowel") && !qu(i);
    let groups = (1..n).filter(|&i| vowel(i) && !vowel(i - 1)).count() + usize::from(vowel(0));
    !vowel(n - 1)
        && !letter_is(chars[n - 1], "no_double")
        && !qu(n - 1)
        && vowel(n - 2)
        && !vowel(n - 3)
        && groups == 1
}

/// The named predicate a `suffix.tsv` condition column refers to.
fn predicate(name: &str) -> Option<Predicate> {
    let table: [(&str, Predicate); 8] = [
        ("ends_sibilant", ends_sibilant),
        ("ends_sibilant_o", ends_sibilant_o),
        ("ends_consonant_y", ends_consonant_y),
        ("ends_vowel_y", ends_vowel_y),
        ("ends_y", ends_y),
        ("ends_silent_e", ends_silent_e),
        ("ends_single_e", ends_single_e),
        ("doubles_final", doubles_final),
    ];
    table.iter().find(|(n, _)| *n == name).map(|(_, p)| *p)
}

/// The irregular form of `stem` for `suffix`, if a lexicon file names one.
fn irregular(stem: &str, suffix: Suffix) -> Option<String> {
    if suffix == Suffix::THIRD {
        return lists::is_role(stem, "have_lemma").then(|| lists::word_of("have_3sg").to_string());
    }
    if suffix == Suffix::ING {
        return super::irregular::ing(stem);
    }
    if suffix == Suffix::PAST {
        return super::irregular_past(stem).or_else(|| super::irregular::past(stem));
    }
    lists::irregular_plurals().iter().find(|(singular, _)| singular == stem).map(|(_, plural)| plural.clone())
}

/// `stem` with `suffix` applied: the irregular form when a file names one,
/// otherwise the first rule of `suffix.tsv` whose condition holds.
pub fn inflect(stem: &str, suffix: Suffix) -> String {
    if let Some(form) = irregular(stem, suffix) {
        return form;
    }
    let last = stem.chars().last().unwrap_or_default();
    for rule in rules().iter().filter(|r| r.kind == suffix.0) {
        if rule.condition.is_none_or(|holds| holds(stem)) {
            let keep = stem.chars().count().saturating_sub(rule.strip);
            let head: String = stem.chars().take(keep).collect();
            return format!("{head}{}", rule.add.replace('~', &last.to_string()));
        }
    }
    stem.to_string()
}
