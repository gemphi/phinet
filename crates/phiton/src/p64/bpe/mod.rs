//! (phiton::p64::bpe — it was phiano's `bpe`.)
//!
//! Byte-level BPE — the tokenizer of Phi-4 (and GPT-2/GPT-4), read from the
//! `vocab.json` + `merges.txt` pair in `refs/Phi-4-multimodal-instruct/`.
//!
//! Why it is here: the rare-target-side problem (Paper 1 §9) is a word
//! problem — a word seen twice has almost no relations. A subword lexicon
//! has no rare words, only rare *pieces*, and Phi-4's 200k-piece vocabulary
//! is the one a 5.6B-parameter model was trained on. This module lets a
//! Phiano lexicon be trained on those pieces instead of whitespace words,
//! and `bin/bpe_check` measures what that changes.
//!
//! What is and is not reproduced: the byte→unicode table and the merge
//! algorithm are GPT-2's (which Phi-4 inherits); the pre-tokeniser is the
//! GPT-2 pattern, hand-written because it needs a look-ahead the `regex`
//! crate does not have. Phi-4's own `tokenizer.json` (which carries the
//! o200k pre-tokeniser that splits digits in threes) is a Git-LFS pointer
//! in `refs/`, not a file, so that variant is not implemented. Checked
//! against Hugging Face `tokenizers` on the same files: identical ids on
//! Austen (`torch/bpe_parity.py`).

mod bytes;
mod pretokenize;
#[cfg(test)]
mod tests;

pub use pretokenize::pretokenize;
use bytes::bytes_to_unicode;
use std::collections::HashMap;

/// A byte-level BPE tokenizer.
#[derive(Debug, Clone)]
pub struct Bpe {
    encoder: HashMap<String, u32>,
    decoder: Vec<String>,
    ranks: HashMap<(String, String), usize>,
    byte_to_char: [char; 256],
    char_to_byte: HashMap<char, u8>,
}

impl Bpe {
    /// Loads `vocab.json` (piece → id) and `merges.txt` (one merge per line,
    /// first line a `#version` header).
    pub fn load(vocab_json: &str, merges_txt: &str) -> Result<Self, String> {
        let vocab: HashMap<String, u32> =
            serde_json::from_str(vocab_json).map_err(|e| format!("vocab.json: {e}"))?;
        let mut decoder = vec![String::new(); vocab.values().map(|&i| i as usize + 1).max().unwrap_or(0)];
        for (p, &i) in &vocab {
            decoder[i as usize] = p.clone();
        }
        let mut ranks = HashMap::new();
        for (n, line) in merges_txt.lines().enumerate() {
            if n == 0 && line.starts_with("#version") {
                continue;
            }
            let mut it = line.split(' ');
            if let (Some(a), Some(b)) = (it.next(), it.next()) {
                ranks.insert((a.to_string(), b.to_string()), ranks.len());
            }
        }
        let (byte_to_char, char_to_byte) = bytes_to_unicode();
        Ok(Self { encoder: vocab, decoder, ranks, byte_to_char, char_to_byte })
    }

    /// Loads the pair from a directory holding `vocab.json` and `merges.txt`.
    pub fn from_dir(dir: &str) -> Result<Self, String> {
        let v = std::fs::read_to_string(format!("{dir}/vocab.json")).map_err(|e| e.to_string())?;
        let m = std::fs::read_to_string(format!("{dir}/merges.txt")).map_err(|e| e.to_string())?;
        Self::load(&v, &m)
    }

    pub fn vocab_size(&self) -> usize {
        self.decoder.len()
    }

    /// The pieces of a text, as strings in the byte-level alphabet
    /// (a leading `Ġ` is a space).
    pub fn pieces(&self, text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for chunk in pretokenize(text) {
            let mapped: String = chunk.bytes().map(|b| self.byte_to_char[b as usize]).collect();
            out.extend(self.merge(&mapped));
        }
        out
    }

    /// Token ids.
    pub fn encode(&self, text: &str) -> Vec<u32> {
        self.pieces(text).iter().filter_map(|p| self.encoder.get(p).copied()).collect()
    }

    /// Text back from ids (lossy on invalid UTF-8, which BPE cannot produce
    /// from valid input).
    pub fn decode(&self, ids: &[u32]) -> String {
        let bytes: Vec<u8> = ids
            .iter()
            .filter_map(|&i| self.decoder.get(i as usize))
            .flat_map(|p| p.chars().filter_map(|c| self.char_to_byte.get(&c).copied()).collect::<Vec<_>>())
            .collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// A piece as readable text (`Ġthe` → ` the`).
    pub fn piece_text(&self, piece: &str) -> String {
        let bytes: Vec<u8> = piece.chars().filter_map(|c| self.char_to_byte.get(&c).copied()).collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn merge(&self, word: &str) -> Vec<String> {
        let mut parts: Vec<String> = word.chars().map(|c| c.to_string()).collect();
        if parts.len() < 2 {
            return parts;
        }
        loop {
            let mut best: Option<(usize, usize)> = None; // (rank, index)
            for i in 0..parts.len() - 1 {
                if let Some(&r) = self.ranks.get(&(parts[i].clone(), parts[i + 1].clone())) {
                    if best.is_none_or(|(br, _)| r < br) {
                        best = Some((r, i));
                    }
                }
            }
            match best {
                None => break,
                Some((_, i)) => {
                    let merged = format!("{}{}", parts[i], parts[i + 1]);
                    parts[i] = merged;
                    parts.remove(i + 1);
                    if parts.len() < 2 {
                        break;
                    }
                }
            }
        }
        parts
    }
}
