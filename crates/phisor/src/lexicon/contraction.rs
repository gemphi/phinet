//! `lexicon::contraction` - expand a contracted token by rule, from data files.
//!
//! Two files drive it, nothing is written here:
//! - `contraction_exception.tsv`: irregular whole forms (`won't` -> `will not`),
//!   checked first;
//! - `contraction_clitic.tsv`: clitic suffixes (`'re` -> `are`, `n't` -> `not`),
//!   found on any token, so no pronoun or auxiliary is listed.
//!
//! `'s` is not a rule: it is is/has/possessive, which only the following word
//! can settle, so the reader resolves it (see `contraction_s.tsv`).
use super::lists::{data_rows, lex_dir};
use std::collections::HashMap;
use std::sync::OnceLock;

/// The parsed contraction tables.
#[derive(Debug, Default)]
pub struct Contractions {
    /// Irregular whole forms and the words they expand to.
    exceptions: HashMap<String, Vec<String>>,
    /// Clitic suffixes with the words they stand for, longest suffix first.
    clitics: Vec<(String, Vec<String>)>,
}

impl Contractions {
    /// Parse the two files' text (`form<TAB>words`, words space-separated).
    pub fn parse(clitic_tsv: &str, exception_tsv: &str) -> Contractions {
        let rows = |text: &str| -> Vec<(String, Vec<String>)> {
            data_rows(text)
                .into_iter()
                .filter(|r| r.len() >= 2 && !r[0].is_empty())
                .map(|r| (r[0].to_lowercase(), r[1].split_whitespace().map(str::to_string).collect()))
                .collect()
        };
        let mut clitics = rows(clitic_tsv);
        clitics.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        Contractions { exceptions: rows(exception_tsv).into_iter().collect(), clitics }
    }

    /// The words a contracted token stands for, or `None` when it is not one.
    /// An exception wins; otherwise a clitic suffix on a non-empty alphabetic
    /// stem splits off, the stem first ("we're" -> we are).
    pub fn expand(&self, word: &str) -> Option<Vec<String>> {
        if let Some(words) = self.exceptions.get(word) {
            return Some(words.clone());
        }
        self.clitics.iter().find_map(|(suffix, words)| {
            let stem = word.strip_suffix(suffix.as_str())?;
            if stem.is_empty() || !stem.chars().all(char::is_alphabetic) {
                return None;
            }
            let mut out = vec![stem.to_string()];
            out.extend(words.iter().cloned());
            Some(out)
        })
    }
}

/// The contraction tables, read once from `data/lex`.
pub fn contractions() -> &'static Contractions {
    static CELL: OnceLock<Contractions> = OnceLock::new();
    CELL.get_or_init(|| {
        let read = |f: &str| std::fs::read_to_string(lex_dir().join(f)).unwrap_or_default();
        Contractions::parse(&read("contraction_clitic.tsv"), &read("contraction_exception.tsv"))
    })
}

/// Expand a contracted token by the data-driven rules; see [`Contractions`].
pub fn expand(word: &str) -> Option<Vec<String>> {
    contractions().expand(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tables() -> Contractions {
        Contractions::parse(
            "# rules\n're\tare\n'll\twill\nn't\tnot\n'm\tam\n",
            "won't\twill not\ncan't\tcannot\n",
        )
    }
    fn words(w: Option<Vec<String>>) -> Vec<String> {
        w.unwrap_or_default()
    }

    #[test]
    fn a_clitic_splits_off_any_stem_and_an_exception_wins() {
        let t = tables();
        assert_eq!(words(t.expand("we're")), ["we", "are"]);
        assert_eq!(words(t.expand("zog'll")), ["zog", "will"]);
        assert_eq!(words(t.expand("don't")), ["do", "not"]);
        assert_eq!(words(t.expand("i'm")), ["i", "am"]);
        assert_eq!(words(t.expand("won't")), ["will", "not"]);
        assert_eq!(words(t.expand("can't")), ["cannot"]);
    }

    #[test]
    fn a_plain_word_or_a_bare_clitic_is_not_a_contraction() {
        let t = tables();
        assert_eq!(t.expand("run"), None);
        assert_eq!(t.expand("'re"), None);
        assert_eq!(t.expand("he's"), None);
    }

    #[test]
    fn the_shipped_files_cover_what_the_old_table_did() {
        let old = [
            ("don't", "do not"), ("doesn't", "does not"), ("didn't", "did not"),
            ("isn't", "is not"), ("aren't", "are not"), ("wasn't", "was not"),
            ("weren't", "were not"), ("haven't", "have not"), ("hasn't", "has not"),
            ("hadn't", "had not"), ("won't", "will not"), ("wouldn't", "would not"),
            ("can't", "cannot"), ("cannot", "cannot"), ("couldn't", "could not"),
            ("shouldn't", "should not"), ("mustn't", "must not"), ("mightn't", "might not"),
            ("shan't", "shall not"), ("i'm", "i am"), ("you're", "you are"),
            ("we're", "we are"), ("they're", "they are"), ("i've", "i have"),
            ("you've", "you have"), ("we've", "we have"), ("they've", "they have"),
            ("i'll", "i will"), ("you'll", "you will"), ("we'll", "we will"),
            ("they'll", "they will"), ("he'll", "he will"), ("she'll", "she will"),
            ("it'll", "it will"),
        ];
        for (form, expansion) in old {
            let got = words(expand(form)).join(" ");
            assert_eq!(got, expansion, "{form}");
        }
    }
}
