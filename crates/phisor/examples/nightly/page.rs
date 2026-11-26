//! The page: `view/learned.html` with the record and the night's files read
//! into it, each as it is in a `<script type="text/plain">` the page parses,
//! so a browser opens it from disk with no server.

use phisor::checkpoint::state;
use phisor::kind::Tables;
use std::io;
use std::path::{Path, PathBuf};

/// The modes the night learns kinds at, as their classes.
const MODES: [u32; 3] = [16, 32, 64];

/// Each kind's labels on the held-out sentences: `kind <tab> explain,
/// instruct, answer <tab> stop, question, exclamation, line`; a kind no
/// held-out sentence fell in is left out.
pub fn labels(file: &Path, t: &Tables) -> io::Result<()> {
    let join = |r: &[f64]| r.iter().map(|c| format!("{c:.0}")).collect::<Vec<_>>().join(",");
    let mut text = String::from("# kind\texplain,instruct,answer\tstop,question,exclamation,line\n");
    for (k, (f, m)) in t.function.iter().zip(&t.mark).enumerate() {
        match f.iter().chain(m).any(|&c| c > 0.0) {
            true => text.push_str(&format!("{k}\t{}\t{}\n", join(f), join(m))),
            false => {}
        }
    }
    std::fs::write(file, text)
}

/// Write `dir/learned.html` from the view and tonight's files.
pub fn write(dir: &Path, night: &Path) -> io::Result<PathBuf> {
    let view = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("view").join("learned.html"))?;
    let mut files: Vec<(String, PathBuf)> = vec![
        ("night".into(), dir.join("night.tsv")),
        ("taken".into(), dir.join("taken.tsv")),
        ("code".into(), night.join(state::CODE)),
        ("growth".into(), night.join("growth.tsv")),
    ];
    for c in MODES {
        files.push((format!("kind-{c}"), night.join(state::kind_file(c.trailing_zeros()))));
        files.push((format!("label-{c}"), night.join(format!("label-{c}.tsv"))));
    }
    let mut data = String::new();
    for (id, file) in files {
        let text = std::fs::read_to_string(file).unwrap_or_default().replace("</script", "<\\/script");
        data.push_str(&format!("<script type=\"text/plain\" id=\"{id}\">\n{text}</script>\n"));
    }
    let out = dir.join("learned.html");
    std::fs::write(&out, view.replace("<!--learned-->", &data))?;
    Ok(out)
}
