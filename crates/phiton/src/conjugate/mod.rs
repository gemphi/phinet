//! The phase-conjugate mirror: a relation as an all-pass filter on the
//! circle (docs/153). It stands beside [`crate::chiton`]: the chiton is the
//! wave and its spin, the mirror returns that wave time-reversed. It was
//! phical's `conjugate`.
//!
//! A word is a function on the circle, kept here as its spectrum: `R`
//! complex Fourier coefficients of unit norm (the unitary FFT keeps every
//! inner product, so this is the same function the FNO's table holds). A
//! kind is a phase per mode, `H_k = e^{iθ_k}`: a filter that shifts phases
//! and keeps amplitudes, so it loses nothing and has an exact inverse. A
//! relation `w ⇒ v` under a kind is read as the matched filter
//!
//! `s(w, kind, v) = Re ⟨H · a_w, a_v⟩ = Re Σ_k e^{iθ_k} a_w(k) conj(a_v(k))`
//!
//! which is the cosine, since `|H a_w| = |a_w| = 1`.
//!
//! Because `H` is unitary, `Re ⟨H a_w, a_v⟩ = Re ⟨a_w, H̄ a_v⟩`. The
//! relation read backwards, `v ⇐ w`, is the same filter conjugated: the
//! time-reversed wave, with nothing trained for it. Powers compose: `H²`
//! carries a word two steps on the circle, `H⁻¹ = H̄` one step back. This is
//! RotatE (Sun et al. 2019) and unit-modulus ComplEx (Trouillon et al. 2016)
//! read as Fourier operators: the FNO's spectral weight `R_k` restricted to
//! one channel and to `|R_k| = 1`.
//!
//! The amplitude a reading carries is, in Phiano, the counts' (the pump),
//! mixed with these phase readings by weights counted on told rows
//! (`phiano` `story/reader.rs`). A **pumped** mirror (docs/154) gives each
//! kind a gain per mode as well, `H_k = e^{g_k + iθ_k}`: unconstrained
//! ComplEx. Its score is then the cosine `Re⟨H a_w, a_v⟩ / |H a_w|`, and its
//! conjugate `H̄` is the adjoint, no longer the inverse — four-wave mixing
//! with gain, which can return more than it receives in some modes and less
//! in others.

mod train;
#[cfg(test)]
mod tests;

pub use train::Moment;

use phistd::{c64, mix64};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Words as unit spectra, kinds as phases per mode — and, in a pumped
/// mirror, a gain per mode as well.
#[derive(Clone)]
pub struct Mirror {
    pub modes: usize,
    pub names: Vec<String>,
    pub spectra: Vec<Vec<c64>>,
    pub kinds: Vec<String>,
    pub phases: Vec<Vec<f64>>,
    /// Log-gains per kind and mode, `|H_k| = e^{g_k}`; all zero (and never
    /// trained) unless the mirror is `pumped` (docs/154).
    pub gains: Vec<Vec<f64>>,
    pub pumped: bool,
    pub index: HashMap<String, usize>,
    kind_index: HashMap<String, usize>,
}

/// A mirror on disk: spectra as `(re, im)` pairs.
#[derive(Serialize, Deserialize)]
struct Saved {
    modes: usize,
    names: Vec<String>,
    spectra: Vec<Vec<(f64, f64)>>,
    kinds: Vec<String>,
    phases: Vec<Vec<f64>>,
    gains: Vec<Vec<f64>>,
    pumped: bool,
}

/// The layout before gains (docs/153): read as an unpumped mirror.
#[derive(Deserialize)]
struct SavedUnpumped {
    modes: usize,
    names: Vec<String>,
    spectra: Vec<Vec<(f64, f64)>>,
    kinds: Vec<String>,
    phases: Vec<Vec<f64>>,
}

/// A told relation by index: `about ⇒ other` under `kind`.
#[derive(Clone, Copy, Debug)]
pub struct Told {
    pub about: usize,
    pub kind: usize,
    pub other: usize,
}

impl Mirror {
    /// Random unit spectra for `names`, and every kind's phases and gains at
    /// zero (the identity filter). A `pumped` mirror trains its gains.
    pub fn new(names: Vec<String>, kinds: Vec<String>, modes: usize, pumped: bool, seed: u64) -> Mirror {
        let spectra = (0..names.len()).map(|i| phistd::unit_random(modes, mix64(seed ^ i as u64))).collect();
        let (phases, gains) = (vec![vec![0.0; modes]; kinds.len()], vec![vec![0.0; modes]; kinds.len()]);
        let mut m = Mirror { modes, names, spectra, kinds, phases, gains, pumped, index: HashMap::new(), kind_index: HashMap::new() };
        m.reindex();
        m
    }

    fn reindex(&mut self) {
        self.index = self.names.iter().enumerate().map(|(i, n)| (n.clone(), i)).collect();
        self.kind_index = self.kinds.iter().enumerate().map(|(i, n)| (n.clone(), i)).collect();
    }

    pub fn kind(&self, name: &str) -> Option<usize> {
        self.kind_index.get(name).copied()
    }

    /// The filter of `kind` raised to `power`, `H^p` with
    /// `H_k = e^{g_k + iθ_k}`. A negative power is the conjugate, `H̄^|p|`:
    /// the relation read backwards — the inverse while the gains are zero,
    /// the adjoint once they are pumped.
    pub fn filter(&self, kind: usize, power: i32) -> Vec<c64> {
        let p = power as f64;
        self.phases[kind].iter().zip(&self.gains[kind]).map(|(t, g)| c64::from_polar((g * p.abs()).exp(), t * p)).collect()
    }

    /// The wave `H^p · a_w`: what `word` reaches `power` steps along `kind`.
    pub fn wave(&self, word: usize, kind: usize, power: i32) -> Vec<c64> {
        self.filter(kind, power).iter().zip(&self.spectra[word]).map(|(h, a)| h * a).collect()
    }

    /// The matched filter of a wave against every word: `Re ⟨q, a_v⟩`.
    pub fn correlate(&self, q: &[c64]) -> Vec<f64> {
        self.spectra.iter().map(|a| q.iter().zip(a).map(|(x, y)| (x * y.conj()).re).sum()).collect()
    }

    /// Scores over every word for `word ⇒ ?` under `kind`, `power` steps on
    /// (negative: back). `None` for a word or kind the mirror never saw.
    pub fn read(&self, word: &str, kind: &str, power: i32) -> Option<Vec<f64>> {
        let (w, k) = (*self.index.get(word)?, self.kind(kind)?);
        Some(self.correlate(&self.wave(w, k, power)))
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let spectra = self.spectra.iter().map(|f| f.iter().map(|z| (z.re, z.im)).collect()).collect();
        let s = Saved { modes: self.modes, names: self.names.clone(), spectra, kinds: self.kinds.clone(), phases: self.phases.clone(), gains: self.gains.clone(), pumped: self.pumped };
        let bytes = bincode::serialize(&s).map_err(std::io::Error::other)?;
        std::fs::write(path, bytes)
    }

    pub fn load(path: &str) -> std::io::Result<Mirror> {
        let bytes = std::fs::read(path)?;
        let s: Saved = match bincode::deserialize(&bytes) {
            Ok(s) => s,
            Err(e) => {
                let u: SavedUnpumped = bincode::deserialize(&bytes).map_err(|_| std::io::Error::other(e))?;
                let gains = vec![vec![0.0; u.modes]; u.kinds.len()];
                Saved { modes: u.modes, names: u.names, spectra: u.spectra, kinds: u.kinds, phases: u.phases, gains, pumped: false }
            }
        };
        let spectra = s.spectra.into_iter().map(|f| f.into_iter().map(|(re, im)| c64::new(re, im)).collect()).collect();
        let mut m = Mirror { modes: s.modes, names: s.names, spectra, kinds: s.kinds, phases: s.phases, gains: s.gains, pumped: s.pumped, index: HashMap::new(), kind_index: HashMap::new() };
        m.reindex();
        Ok(m)
    }
}
