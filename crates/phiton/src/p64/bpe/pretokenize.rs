//! GPT-2's pre-tokeniser, by hand (the pattern needs a look-ahead the `regex` crate lacks).

/// GPT-2's pre-tokeniser, by hand:
/// `'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+`
pub fn pretokenize(text: &str) -> Vec<&str> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let n = chars.len();
    let at = |i: usize| chars.get(i).map(|c| c.1);
    let mut out = Vec::new();
    let mut i = 0;
    let end_of = |j: usize| if j < n { chars[j].0 } else { text.len() };
    while i < n {
        let start = chars[i].0;
        let c = chars[i].1;
        // contractions
        if c == '\'' {
            let rest: String = chars[i + 1..(i + 3).min(n)].iter().map(|c| c.1).collect();
            let take = ["re", "ve", "ll"].iter().find(|s| rest.starts_with(*s)).map(|_| 2).or_else(|| {
                rest.chars().next().filter(|c| matches!(c, 's' | 't' | 'm' | 'd')).map(|_| 1)
            });
            if let Some(k) = take {
                out.push(&text[start..end_of(i + 1 + k)]);
                i += 1 + k;
                continue;
            }
        }
        // ` ?\p{L}+`, ` ?\p{N}+`, ` ?[^\s\p{L}\p{N}]+`
        let (lead, j0) = if c == ' ' && i + 1 < n && !chars[i + 1].1.is_whitespace() { (true, i + 1) } else { (false, i) };
        let c0 = chars[j0].1;
        let class = |ch: char| -> u8 {
            if ch.is_alphabetic() { 1 } else if ch.is_numeric() { 2 } else if ch.is_whitespace() { 0 } else { 3 }
        };
        let k = class(c0);
        if k != 0 && (lead || j0 == i) {
            let mut j = j0;
            while j < n && class(chars[j].1) == k {
                j += 1;
            }
            out.push(&text[start..end_of(j)]);
            i = j;
            continue;
        }
        // whitespace: `\s+(?!\S)` then `\s+`
        let mut j = i;
        while j < n && chars[j].1.is_whitespace() {
            j += 1;
        }
        // if followed by a non-space, leave the last whitespace char for the next chunk
        if j < n && j - i > 1 && at(j).is_some_and(|ch| !ch.is_whitespace()) {
            j -= 1;
        }
        if j == i {
            j = i + 1;
        }
        out.push(&text[start..end_of(j)]);
        i = j;
    }
    out
}
