//! The lexical reading of a speech act: the same five acts as
//! `SpeechAct::classify`, read from marker phrases in the text where that one
//! reads the syntax key. Two readings of one taxonomy, one type.

use super::searle::SpeechAct;
use super::searle_marker::SearleMarker;
use super::words::Tokenizer;

impl SpeechAct {
    /// Classifies `text` by the marker phrases `m` lists, in Searle's order.
    /// Indirect requests ("can you X") direct; commissives, expressives and
    /// declarations are tried before directives, so "i will explain" commits.
    pub fn classify_marked(text: &str, m: &SearleMarker) -> Self {
        if SearleMarker::starts_with_any(text, &m.indirect_patterns) {
            return Self::Directive;
        }
        match () {
            _ if SearleMarker::contains_any(text, &m.commissive_markers) => Self::Commissive,
            _ if SearleMarker::contains_any(text, &m.expressive_markers) => Self::Expressive,
            _ if SearleMarker::contains_any(text, &m.declarative_markers) => Self::Declaration,
            _ if SearleMarker::contains_any(text, &m.directive_question_markers) => Self::Directive,
            _ if SearleMarker::contains_any(text, &m.directive_command_markers) => Self::Directive,
            _ => Self::Assertive,
        }
    }
}

impl SearleMarker {
    /// The propositional content: the text with its leading force marker cut
    /// off, or its first five tokens when nothing is left.
    pub fn propositional_content(&self, text: &str) -> String {
        let p = text.to_lowercase();
        let lists: [&[String]; 6] = [
            &self.directive_question_markers,
            &self.directive_command_markers,
            &self.indirect_patterns,
            &self.commissive_markers,
            &self.declarative_markers,
            &self.expressive_markers,
        ];
        let mut content = p.clone();
        for list in lists {
            for marker in list {
                if content.starts_with(marker.as_str()) {
                    content = content[marker.len()..].trim().to_string();
                    break;
                }
            }
            if content != p {
                break;
            }
        }
        match content.is_empty() {
            true => Tokenizer::tokenize(text).iter().take(5).cloned().collect::<Vec<_>>().join(" "),
            false => content,
        }
    }

    /// The literal meaning and the speaker's, which differ for indirect
    /// requests and rhetorical questions.
    pub fn literal_and_speaker(&self, text: &str) -> (String, String) {
        let literal = text.to_string();
        if Self::starts_with_any(text, &self.indirect_patterns) {
            return (literal, format!("Do the requested action (indirect request): {}", text));
        }
        if Self::contains_any(text, &self.rhetorical_markers) {
            return (literal, format!("Nobody knows/cares - rhetorical (not a real question): {}", text));
        }
        (literal.clone(), literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn markers() -> SearleMarker {
        serde_json::from_str(
            r#"{"indirect_patterns":["can you"],"commissive_markers":["i will"],
            "expressive_markers":["thank you"],"declarative_markers":["hereby"],
            "directive_question_markers":["what"],"directive_command_markers":["show"],
            "rhetorical_markers":["who cares"],"institutional_markers":[],
            "brute_markers":[],"counts_as_rules":{"money":"a medium of exchange"},
            "observer_relative_markers":[]}"#,
        )
        .unwrap()
    }

    #[test]
    fn marked_classification_follows_searle_order() {
        let m = markers();
        let act = |t| SpeechAct::classify_marked(t, &m);
        assert_eq!(act("Can you help"), SpeechAct::Directive);
        assert_eq!(act("i will show you"), SpeechAct::Commissive);
        assert_eq!(act("thank you"), SpeechAct::Expressive);
        assert_eq!(act("we hereby close"), SpeechAct::Declaration);
        assert_eq!(act("show me"), SpeechAct::Directive);
        assert_eq!(act("the sky is blue"), SpeechAct::Assertive);
    }

    #[test]
    fn content_and_meaning_read_the_markers() {
        let m = markers();
        assert_eq!(m.propositional_content("I will go home"), "go home");
        let (lit, sp) = m.literal_and_speaker("Can you open it");
        assert_eq!(lit, "Can you open it");
        assert!(sp.starts_with("Do the requested action"));
        assert_eq!(m.counts_as("some money"), vec!["a medium of exchange".to_string()]);
    }
}
