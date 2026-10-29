//! A sentence's variation: its template in a mode. Three are fixed shapes,
//! how it opens and how it closes; the twins read it from one end each
//! (Bentov's mirror: the wave read backwards), and `Both` reads both ends,
//! as docs/201's signature did. The fourth is grown from the data: the
//! longest opening the training sentences show at least `TOLD` times, of
//! any length, so the templates are as many and as long as the guide makes
//! them (the user: "built dynamically, we can't limit the templates").

use std::collections::HashMap;

/// Which template a sentence's variation is read as.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Twin {
    /// The first two classes.
    Open,
    /// The last two classes.
    Close,
    /// The first two and the last.
    Both,
    /// The longest opening told at least `TOLD` times in training.
    Grown,
}

impl Twin {
    pub const ALL: [Twin; 4] = [Twin::Open, Twin::Close, Twin::Both, Twin::Grown];

    pub fn name(self) -> &'static str {
        match self {
            Twin::Open => "open",
            Twin::Close => "close",
            Twin::Both => "both",
            Twin::Grown => "grown",
        }
    }
}

fn written(classes: &[u16]) -> String {
    classes.iter().map(|x| format!("c{x}")).collect::<Vec<_>>().join(" ")
}

/// A sentence's variation as text: `c9 c2 …` for the opening, `… c5 c11`
/// for the closing, `c9 c2 … c11` for both. A sentence is at least four
/// words, so both ends are always there. `Grown` needs the training
/// openings: see [`grown`].
pub fn shape(classes: &[u16], twin: Twin) -> String {
    let n = classes.len();
    let c = |i: usize| classes.get(i).map_or("·".to_string(), |x| format!("c{x}"));
    match twin {
        Twin::Open | Twin::Grown => format!("{} {} …", c(0), c(1)),
        Twin::Close => format!("… {} {}", c(n.saturating_sub(2)), c(n.saturating_sub(1))),
        Twin::Both => format!("{} {} … {}", c(0), c(1), c(n.saturating_sub(1))),
    }
}

/// How often each opening (each prefix, of every length) was told.
pub fn openings<'a>(sentences: impl Iterator<Item = &'a [u16]>) -> HashMap<Vec<u16>, usize> {
    let mut told: HashMap<Vec<u16>, usize> = HashMap::new();
    for s in sentences {
        for n in 1..=s.len() {
            *told.entry(s[..n].to_vec()).or_default() += 1;
        }
    }
    told
}

/// The first `n` classes of a sentence as a template; `…` marks an opening
/// shorter than the sentence.
pub fn opening(classes: &[u16], n: usize) -> String {
    match n < classes.len() {
        true => format!("{} …", written(&classes[..n])),
        false => written(classes),
    }
}

/// How long the longest opening of `classes` told at least `least` times
/// is; 1 when none is.
pub fn grown_length(classes: &[u16], told: &HashMap<Vec<u16>, usize>, least: usize) -> usize {
    (1..=classes.len()).take_while(|&n| told.get(&classes[..n]).is_some_and(|&t| t >= least)).last().unwrap_or(1).min(classes.len())
}

/// The longest opening of `classes` told at least `least` times, as a
/// template; the first class alone when none is.
pub fn grown(classes: &[u16], told: &HashMap<Vec<u16>, usize>, least: usize) -> String {
    opening(classes, grown_length(classes, told, least))
}
