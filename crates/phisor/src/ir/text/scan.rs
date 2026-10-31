//! UTF-8 cursor and the reversible atom grammar used by the Act reader.

use super::ParseError;

pub(super) struct Cursor<'a> {
    text: &'a str,
    at: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, at: 0 }
    }

    pub fn offset(&self) -> usize {
        self.at
    }

    pub fn skip(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.text[self.at..].chars().next()
    }

    pub fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.at += ch.len_utf8();
        Some(ch)
    }

    pub fn eat(&mut self, wanted: char) -> bool {
        self.skip();
        if self.peek() == Some(wanted) {
            self.bump();
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, wanted: char, what: &str) -> Result<(), ParseError> {
        if self.eat(wanted) {
            Ok(())
        } else {
            Err(self.error(format!("expected {what} {wanted:?}")))
        }
    }

    pub fn starts(&mut self, prefix: &str) -> bool {
        self.skip();
        self.text[self.at..].starts_with(prefix)
    }

    pub fn take(&mut self, prefix: &str) -> bool {
        if self.starts(prefix) {
            self.at += prefix.len();
            true
        } else {
            false
        }
    }

    /// Consume a natural determiner only when whitespace follows it.
    pub fn keyword(&mut self, word: &str) -> bool {
        self.skip();
        let rest = &self.text[self.at..];
        let Some(after) = rest.strip_prefix(word) else {
            return false;
        };
        if !after.chars().next().is_some_and(char::is_whitespace) {
            return false;
        }
        self.at += word.len();
        self.skip();
        true
    }

    pub fn atom(&mut self) -> Result<String, ParseError> {
        self.skip();
        if self.peek() == Some('"') {
            return self.quoted();
        }
        let start = self.at;
        while self.peek().is_some_and(safe) {
            self.bump();
        }
        if self.at == start {
            Err(self.error("expected an atom"))
        } else {
            Ok(self.text[start..self.at].to_string())
        }
    }

    /// Read a feature name up to the next comma or the frame's closing mark.
    pub fn segment(&mut self, close: char) -> Result<String, ParseError> {
        self.skip();
        let start = self.at;
        while self.peek().is_some_and(|ch| ch != ',' && ch != close) {
            self.bump();
        }
        let value = self.text[start..self.at].trim();
        if value.is_empty() {
            Err(ParseError::new(start, "expected a frame feature"))
        } else {
            Ok(value.to_string())
        }
    }

    pub fn finish(&mut self) -> Result<(), ParseError> {
        self.skip();
        match self.peek() {
            None => Ok(()),
            Some(ch) => Err(self.error(format!("unexpected trailing character {ch:?}"))),
        }
    }

    pub fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::new(self.at, message)
    }

    fn quoted(&mut self) -> Result<String, ParseError> {
        self.bump();
        let mut out = String::new();
        loop {
            match self.bump() {
                Some('"') => return Ok(out),
                Some('\\') => out.push(self.escape()?),
                Some(ch) => out.push(ch),
                None => return Err(self.error("unterminated quoted atom")),
            }
        }
    }

    fn escape(&mut self) -> Result<char, ParseError> {
        match self.bump() {
            Some('\\') => Ok('\\'),
            Some('"') => Ok('"'),
            Some('n') => Ok('\n'),
            Some('r') => Ok('\r'),
            Some('t') => Ok('\t'),
            Some('u') => self.unicode(),
            Some(ch) => Err(self.error(format!("unknown escape \\{ch}"))),
            None => Err(self.error("unterminated escape")),
        }
    }

    fn unicode(&mut self) -> Result<char, ParseError> {
        if self.bump() != Some('{') {
            return Err(self.error("expected '{' after \\u"));
        }
        let start = self.at;
        while self.peek().is_some_and(|ch| ch.is_ascii_hexdigit()) {
            self.bump();
        }
        let digits = &self.text[start..self.at];
        if digits.is_empty() || self.bump() != Some('}') {
            return Err(ParseError::new(start, "invalid Unicode escape"));
        }
        let value = u32::from_str_radix(digits, 16)
            .map_err(|_| ParseError::new(start, "invalid Unicode escape"))?;
        char::from_u32(value).ok_or_else(|| ParseError::new(start, "invalid Unicode scalar"))
    }
}

fn safe(ch: char) -> bool {
    ch.is_alphanumeric() || matches!(ch, '_' | '-' | '.' | '/')
}
