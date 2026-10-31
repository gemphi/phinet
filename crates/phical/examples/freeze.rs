//! Writes the keyboard's index table where `phical/tests/keyboard.rs` reads
//! it. This is the re-recording tool: run it, then record the change through
//! `phical::keyboard::index::Rerecord` with the previous table, the new table
//! and the stated reason.
//!
//! ```text
//! cargo run -p phical --example freeze
//! ```

use std::path::PathBuf;

fn main() {
    let at: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "tests", "keyboard", "index.golden"].iter().collect();
    let previous = std::fs::read_to_string(&at).unwrap_or_default().replace("\r\n", "\n");
    let table = phical::keyboard::index::render();

    std::fs::create_dir_all(at.parent().expect("the path has a parent")).expect("the directory");
    std::fs::write(&at, table.as_bytes()).expect("the record is writable");

    let rows = table.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).count();
    println!("{} — {rows} rows", at.display());
    if previous.is_empty() {
        println!("no previous record: this is the first freezing");
    } else if previous == table {
        println!("unchanged: no re-recording needed");
    } else {
        println!("the record has moved; re-record it with Rerecord, naming the reason");
    }
}
