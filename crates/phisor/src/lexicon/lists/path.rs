use std::path::PathBuf;

/// Where the lexicon data files live: `$PHISOR_LEX_DIR` when set, otherwise
/// the foundation's `data/lex` beside this crate. The one path helper every
/// list, and every caller that reads a lexicon file, goes through.
pub fn lex_dir() -> PathBuf {
    match std::env::var_os("PHISOR_LEX_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../phiano/data/lex")),
    }
}

/// The text of a lexicon data file, or empty when it cannot be read.
pub(super) fn read_file(file: &str) -> String {
    std::fs::read_to_string(lex_dir().join(file)).unwrap_or_default()
}

/// The data lines of a file: trimmed, blank lines and `#` comments dropped
/// (the format and parser are `phistd::lexfile`, shared by every crate).
pub fn data_lines(text: &str) -> Vec<String> {
    phistd::lexfile::parse(text).into_iter().map(|l| l.trim().to_string()).collect()
}

/// The tab-separated rows of a file's data lines, each field trimmed.
pub fn data_rows(text: &str) -> Vec<Vec<String>> {
    data_lines(text).iter().map(|l| l.split('\t').map(|f| f.trim().to_string()).collect()).collect()
}
