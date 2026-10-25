//! The ranges: names, snapping bounds, Q64 ≡ hexagram_of, resampling, quanta ordering.

use super::*;
use crate::{unit_random, Function};
use std::f64::consts::TAU;


    #[test]
    fn twelve_ranges_and_their_names() {
        assert_eq!(Range::ALL.len(), 12);
        assert_eq!(Range::ALL[0].name(), "P16·Q16");
        assert_eq!(Range::ALL[11].name(), "P128·Q64");
        assert_eq!(Range::DEFAULT.space(), 64);
        assert_eq!(Resolution::P64.octaves(), 6);
    }

    #[test]
    fn snapping_is_within_the_bound_and_idempotent() {
        for q in Quantum::ALL {
            for k in 0..200 {
                let z = c64::from_polar(1.0 + k as f64 * 0.01, k as f64 * 0.137);
                let s = q.snap(z);
                let err = (s.arg() - z.arg()).rem_euclid(TAU).min(TAU - (s.arg() - z.arg()).rem_euclid(TAU));
                assert!(err <= q.bound() + 1e-12, "{q:?} {err}");
                assert!((s.norm() - z.norm()).abs() < 1e-12);
                assert!((q.snap(s) - s).norm() < 1e-12);
            }
        }
    }

    #[test]
    fn q64_sector_matches_hexagram_of() {
        let f = unit_random(64, 3);
        assert_eq!(sector_of(&f, Quantum::Q64) as u8, crate::hexagram::hexagram_of(&f));
    }

    #[test]
    fn resampling_up_then_down_is_the_identity_and_keeps_energy() {
        let f = unit_random(32, 5);
        let up = resample(&f, Resolution::P128);
        let e_up: f64 = up.iter().map(|z| z.norm_sqr()).sum();
        // values are interpolated, so energy scales with the sample count
        assert!((e_up - 4.0).abs() < 1e-9, "energy {e_up}");
        let back = resample(&up, Resolution::P32);
        for (a, b) in f.iter().zip(&back) {
            assert!((a - b).norm() < 1e-9);
        }
        // a pure low tone is exact at every resolution
        let tone: Function = (0..16).map(|k| c64::from_polar(0.25, TAU * 2.0 * k as f64 / 16.0)).collect();
        let t64 = resample(&tone, Resolution::P64);
        for (k, z) in t64.iter().enumerate() {
            let want = c64::from_polar(0.25, TAU * 2.0 * k as f64 / 64.0);
            assert!((z - want).norm() < 1e-9, "sample {k}");
        }
    }

    #[test]
    fn coarser_quanta_cost_more() {
        let f = unit_random(64, 9);
        let l16 = quantisation_loss(&f, Quantum::Q16);
        let l32 = quantisation_loss(&f, Quantum::Q32);
        let l64 = quantisation_loss(&f, Quantum::Q64);
        assert!(l16 > l32 && l32 > l64 && l64 < 0.01, "{l16} {l32} {l64}");
        assert_eq!(table(&f).len(), 12);
    }
