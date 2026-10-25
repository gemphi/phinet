//! The flow field (phi-oml `phifly::flow_field`, re-based): a grid of
//! signals twitched twice. The phase shift a cell shows under the eye's
//! own micro-jitter is parallax; the near moves fast and the far hardly
//! moves (phi-oml's README: `d ∝ 1/Δx`), so one twitch sorts the scene
//! into near, mid and far with no second camera. What is new here is the
//! last step: the field is **told as rows**, so a scene enters a `World`
//! by the same door as a sentence or the kit.

use crate::signal::{jitter, Aoc};
use phiecs::Row;

/// How fast a cell's phase moved under the twitch, binned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pace {
    Near,
    Mid,
    Far,
}

impl Pace {
    pub fn name(self) -> &'static str {
        match self {
            Pace::Near => "near",
            Pace::Mid => "mid",
            Pace::Far => "far",
        }
    }
}

/// A named grid of signals: the eye's current scene.
#[derive(Debug, Clone)]
pub struct Flow {
    pub width: usize,
    pub cells: Vec<(String, Aoc)>,
}

impl Flow {
    /// An empty field, `width` cells to a row.
    pub fn new(width: usize) -> Self {
        Self { width, cells: Vec::new() }
    }

    /// A thing seen at a cell: its name and its signal. Depth is carried
    /// as amplitude by the sensor (a close thing presses harder), and the
    /// twitch reads it back out as phase velocity.
    pub fn see(&mut self, name: &str, signal: Aoc) {
        self.cells.push((name.to_string(), signal));
    }

    /// One micro-jitter: every cell's phase shift under a twitch of
    /// `angle`, scaled by its amplitude — the parallax each thing shows.
    pub fn twitch(&self, angle: f64) -> Vec<(String, f64)> {
        self.cells
            .iter()
            .map(|(n, s)| {
                let before = s.resonance(jitter(0.0));
                let after = s.resonance(jitter(angle));
                (n.clone(), phical::wrap(after - before).abs() * s.amplitude)
            })
            .collect()
    }

    /// The shifts binned against the field's own median: the top third of
    /// movers are near, the bottom third far.
    pub fn paces(&self, angle: f64) -> Vec<(String, Pace, f64)> {
        let mut shifts = self.twitch(angle);
        let mut sorted: Vec<f64> = shifts.iter().map(|x| x.1).collect();
        sorted.sort_by(f64::total_cmp);
        let (lo, hi) = (sorted[sorted.len() / 3], sorted[(sorted.len() * 2) / 3]);
        shifts
            .drain(..)
            .map(|(n, v)| (n, if v >= hi { Pace::Near } else if v > lo { Pace::Mid } else { Pace::Far }, v))
            .collect()
    }

    /// The field told as rows under `scope` (`sight`, say): every thing
    /// `within` the scope, `does` its pace with the shift as weight, and
    /// the scope `says` the thing — the same kinds a kit or a story uses.
    pub fn rows(&self, scope: &str, angle: f64) -> Vec<Row> {
        let scope = format!("scope:{scope}");
        let mut out = Vec::new();
        for (name, pace, v) in self.paces(angle) {
            out.push(Row::new(&name, "within", &scope));
            out.push(Row::weighted(&name, "does", pace.name(), v));
            out.push(Row::new(&scope, "says", &name));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_twitch_sorts_the_scene_and_tells_it() {
        let mut f = Flow::new(3);
        f.see("hand", Aoc::new(0.4, 3.0)); // close: presses hard
        f.see("table", Aoc::new(1.1, 1.0));
        f.see("wall", Aoc::new(2.0, 0.2)); // far: faint
        let p = f.paces(0.25);
        assert_eq!(p[0].1, Pace::Near, "the hand moves most under the twitch");
        assert_eq!(p[2].1, Pace::Far, "the wall hardly moves");
        let rows = f.rows("sight", 0.25);
        assert_eq!(rows.len(), 9);
        assert!(rows.iter().any(|r| r.about == "hand" && r.kind == "does" && r.other == "near"));
        assert!(rows.iter().any(|r| r.about == "scope:sight" && r.kind == "says" && r.other == "wall"));
        // Rows mean a World can be told a scene directly.
        let w = phiecs::world::World::from_rows(&rows);
        assert!(w.lexicon.get("hand").is_some() && w.lexicon.get("near").is_some());
    }
}
