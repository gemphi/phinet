use super::{record, source, state};
use std::path::PathBuf;

/// A fresh folder of its own under the system's temporary folder.
fn folder(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("phisor-checkpoint-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn a_tree_saved_reads_back_its_paths() {
    let d = folder("tree");
    let file = state::night(&d, "2026-11-25").join(state::CODE);
    state::write(&file, &[("<s>", vec![], 0), ("the", vec![false, true, true], 40), ("^do", vec![true], 3)]).unwrap();
    let back = state::read(&file);
    assert_eq!(back["<s>"], Vec::<bool>::new());
    assert_eq!(back["the"], [false, true, true]);
    assert_eq!(back["^do"], [true]);
    assert_eq!(state::newest(&d), Some(state::night(&d, "2026-11-25")));
    std::fs::create_dir_all(state::night(&d, "2026-11-26")).unwrap();
    assert_eq!(state::newest(&d), Some(state::night(&d, "2026-11-26")));
    assert_eq!(state::kind_file(6), "kind-64.tsv");
}

#[test]
fn a_night_takes_the_next_source_not_yet_taken() {
    let d = folder("queue");
    std::fs::write(d.join("source.tsv"), "# name\tpath\nemma\tbooks/emma.txt\nalice\tbooks/alice.txt\nlist\t/x/list.txt\n").unwrap();
    let root = PathBuf::from("root");
    let (old, new) = source::tonight(&d, &root, 1);
    assert!(old.is_empty());
    assert_eq!(new.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["emma"]);
    assert_eq!(new[0].path, root.join("books/emma.txt"));
    source::take(&d, "2026-11-25", &new[0], 9000).unwrap();
    let (old, new) = source::tonight(&d, &root, 2);
    assert_eq!(old.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["emma"]);
    assert_eq!(new.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["alice", "list"]);
}

#[test]
fn the_record_gains_a_row_a_night() {
    let d = folder("record");
    let n = record::Night { stamp: "2026-11-25".into(), sources: 1, modes: vec![record::Mode { classes: 16, ..Default::default() }], ..Default::default() };
    record::night(&d, &n).unwrap();
    record::night(&d, &record::Night { stamp: "2026-11-26".into(), ..n.clone() }).unwrap();
    let rows = phistd::lexfile::read(d.join("night.tsv"));
    assert_eq!(rows.len(), 2);
    assert!(rows[1].starts_with("2026-11-26\t1\t"), "{}", rows[1]);
}
