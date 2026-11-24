//! The files under a directory: the walk, said once.
//!
//! Four readers in phient each had their own copy of it — one for `.rs`,
//! one for `.b`, two for `.txt` — and each copy also did the reading it
//! walked for. The walk finds files and does nothing else; what to do
//! with one is the caller's own job.

use std::path::{Path, PathBuf};

/// Every file under `dir` whose extension is `ext`, the whole tree walked,
/// in path order.
///
/// A `target` or `.git` folder is never entered and a folder that cannot be
/// read is skipped, so a corpus beside a build tree reads the same as one
/// on its own.
pub fn under(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut out = Vec::new();
    for p in entries.filter_map(|e| e.ok().map(|e| e.path())) {
        let folder = p.is_dir() && p.file_name().is_some_and(|n| n != "target" && n != ".git");
        match (folder, p.extension().is_some_and(|x| x == ext)) {
            (true, _) => out.extend(under(&p, ext)),
            (_, true) => out.push(p),
            _ => {}
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::under;

    /// A tree with a nested folder, a build folder, and a file of another
    /// kind beside the ones asked for.
    #[test]
    fn every_file_of_one_kind_in_path_order() {
        let root = std::env::temp_dir().join("phistd_tree_under");
        let _ = std::fs::remove_dir_all(&root);
        for d in ["deep", "target"] {
            std::fs::create_dir_all(root.join(d)).unwrap();
        }
        for f in ["b.txt", "a.txt", "notes.md", "deep/c.txt", "target/built.txt"] {
            std::fs::write(root.join(f), "x").unwrap();
        }

        let found = under(&root, "txt");
        let said: Vec<String> =
            found.iter().map(|p| p.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/")).collect();
        assert_eq!(said, ["a.txt", "b.txt", "deep/c.txt"], "sorted, nested, and no build folder");
        assert!(under(&root, "md").len() == 1, "another kind is found by its own extension");
        assert!(under(&root.join("nowhere"), "txt").is_empty(), "a folder that is not there is no walk at all");
        let _ = std::fs::remove_dir_all(&root);
    }
}
