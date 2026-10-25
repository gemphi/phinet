//! Quantising, reading a sector, resampling between resolutions, and the twelve-row table.

use super::{Quantum, Range, Resolution};
use crate::tensor::fft_inplace;
use crate::{c64, cosine, Function};

/// Snaps every sample of a function to the quantum's sector centres.
pub fn quantise(f: &[c64], q: Quantum) -> Function {
    f.iter().map(|z| q.snap(*z)).collect()
}

/// The sector of a function's centroid — `hexagram_of` at any quantum.
pub fn sector_of(f: &[c64], q: Quantum) -> usize {
    let c: c64 = f.iter().sum();
    q.sector(c)
}

/// What a snap costs: `1 − cos(f, quantise(f))`.
pub fn quantisation_loss(f: &[c64], q: Quantum) -> f64 {
    1.0 - cosine(f, &quantise(f, q))
}

/// A function at another resolution, by spectral zero-padding or truncation:
/// sample *values* are interpolated (a pure tone keeps its amplitude), so a
/// band-limited function is represented exactly and its energy scales with
/// the sample count. Cosines between functions are unchanged by it.
pub fn resample(f: &[c64], to: Resolution) -> Function {
    let n = f.len();
    let m = to.space();
    if n == m {
        return f.to_vec();
    }
    let mut spec = f.to_vec();
    fft_inplace(&mut spec, false);
    let mut out = vec![c64::new(0.0, 0.0); m];
    let keep = n.min(m) / 2;
    for k in 0..=keep {
        if k < m && k < n {
            out[k] = spec[k];
        }
        if k > 0 && k < keep {
            out[m - k] = spec[n - k];
        }
    }
    // The Nyquist bin of the smaller circle is shared by ±keep; keep it once.
    if n < m {
        out[keep] = spec[keep];
    } else {
        out[keep] = spec[keep] + spec[n - keep];
    }
    // Unitary scaling: a spectrum of N samples carries √N of the energy.
    let scale = (m as f64 / n as f64).sqrt();
    for z in out.iter_mut() {
        *z *= scale;
    }
    fft_inplace(&mut out, true);
    out
}

/// Every combination with the sector the function is in and what snapping
/// it there costs — the "all variations" table for one function.
pub fn table(f: &[c64]) -> Vec<(Range, usize, f64)> {
    Range::ALL
        .iter()
        .map(|r| {
            let g = resample(f, r.resolution);
            (*r, sector_of(&g, r.quantum), quantisation_loss(&g, r.quantum))
        })
        .collect()
}
