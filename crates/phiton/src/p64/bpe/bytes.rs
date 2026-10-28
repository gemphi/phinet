//! GPT-2's byte ↔ unicode table.

use std::collections::HashMap;

/// GPT-2's byte ↔ unicode table: printable bytes map to themselves, the rest
/// to code points from 256 upward, so every byte string is a valid string.
pub(super) fn bytes_to_unicode() -> ([char; 256], HashMap<char, u8>) {
    let mut table = ['\0'; 256];
    let mut rev = HashMap::new();
    let mut n = 0u32;
    for b in 0..=255u8 {
        let printable = (b'!'..=b'~').contains(&b) || (0xA1..=0xAC).contains(&b) || (0xAE..=0xFF).contains(&b);
        let c = if printable {
            char::from_u32(b as u32).unwrap()
        } else {
            let c = char::from_u32(256 + n).unwrap();
            n += 1;
            c
        };
        table[b as usize] = c;
        rev.insert(c, b);
    }
    (table, rev)
}
