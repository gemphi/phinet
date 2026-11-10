//! The Searle marker lists: phrases that signal an illocutionary force, read
//! from a JSON data file the caller names. No word list lives in Rust; the
//! app that owns the data passes its path (phiano: `data/searle_markers.json`).

use serde::Deserialize;
use std::collections::HashMap;

/// Marker phrases per speech-act cue, as the data file lists them.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SearleMarker {
    pub indirect_patterns: Vec<String>,
    pub commissive_markers: Vec<String>,
    pub expressive_markers: Vec<String>,
    pub declarative_markers: Vec<String>,
    pub directive_question_markers: Vec<String>,
    pub directive_command_markers: Vec<String>,
    pub rhetorical_markers: Vec<String>,
    pub institutional_markers: Vec<String>,
    pub brute_markers: Vec<String>,
    pub counts_as_rules: HashMap<String, String>,
    pub observer_relative_markers: Vec<String>,
}

impl SearleMarker {
    /// Reads the markers from the JSON file at `path`. An unreadable or
    /// malformed file gives empty lists, and says so on stderr.
    pub fn from_file(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(text) => serde_json::from_str(&text).unwrap_or_else(|e| {
                eprintln!("[searle] Failed to parse markers JSON: {}", e);
                Self::default()
            }),
            Err(e) => {
                eprintln!("[searle] Could not load {}: {}", path, e);
                Self::default()
            }
        }
    }

    /// Checks if text contains any marker from a list.
    pub fn contains_any(text: &str, markers: &[String]) -> bool {
        let p = text.to_lowercase();
        markers.iter().any(|m| p.contains(m.as_str()))
    }

    /// Checks if text starts with any marker from a list.
    pub fn starts_with_any(text: &str, markers: &[String]) -> bool {
        let p = text.to_lowercase();
        markers.iter().any(|m| p.starts_with(m.as_str()))
    }

    /// Tally how many markers from a list appear in text.
    pub fn count_matches(text: &str, markers: &[String]) -> usize {
        let p = text.to_lowercase();
        markers.iter().filter(|m| p.contains(m.as_str())).count()
    }

    /// Constitutive rules, "X counts as Y": the meanings of every rule
    /// whose `X` the text contains.
    pub fn counts_as(&self, text: &str) -> Vec<String> {
        let p = text.to_lowercase();
        self.counts_as_rules
            .iter()
            .filter(|(k, _)| p.contains(k.as_str()))
            .map(|(_, v)| v.clone())
            .collect()
    }
}
