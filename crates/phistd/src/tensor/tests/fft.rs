//! FFT tests: unitarity, a pure tone in one bin, the torus.

use super::super::*;
use crate::c64;
use super::super::fft::*;

    #[test]
    fn test_fft_is_unitary_and_invertible() {
        let x = Tensor::randn([1, 1, 64], 1.0, 7);
        let mut y = x.data.clone();
        fft_inplace(&mut y, false);
        let ny: f64 = y.iter().map(|z| z.norm_sqr()).sum();
        assert!((ny - x.norm_sq()).abs() < 1e-9, "Parseval: {ny} vs {}", x.norm_sq());
        fft_inplace(&mut y, true);
        for (a, b) in x.data.iter().zip(&y) {
            assert!((a - b).norm() < 1e-9);
        }
    }

    #[test]
    fn test_fft_of_a_pure_tone_is_one_bin() {
        let n = 64;
        let k = 5;
        let data: Vec<c64> = (0..n)
            .map(|t| c64::from_polar(1.0, 2.0 * PI * (k * t) as f64 / n as f64))
            .collect();
        let mut y = data.clone();
        fft_inplace(&mut y, false);
        for (i, z) in y.iter().enumerate() {
            if i == k {
                assert!((z.norm() - (n as f64).sqrt()).abs() < 1e-9, "bin {i}: {}", z.norm());
            } else {
                assert!(z.norm() < 1e-9, "bin {i} leaked {}", z.norm());
            }
        }
    }


    #[test]
    fn test_fft2_is_unitary_and_its_backward_is_the_inverse() {
        let (s1, s2) = (4, 8);
        let x = Tensor::randn([2, 1, s1 * s2], 1.0, 41);
        let mut tape = Tape::new();
        let xv = tape.param(x.clone());
        let f = tape.fft2(xv, s1, s2);
        let back = tape.ifft2(f, s1, s2);
        for (a, b) in tape.value(back).data.iter().zip(&x.data) {
            assert!((a - b).norm() < 1e-12);
        }
        assert!((tape.value(f).norm_sq() - x.norm_sq()).abs() < 1e-9, "Parseval on the torus");
        let mut tone = vec![c64::new(0.0, 0.0); s1 * s2];
        for i in 0..s1 { for j in 0..s2 {
            let ph = std::f64::consts::TAU * (1.0 * i as f64 / s1 as f64 + 3.0 * j as f64 / s2 as f64);
            tone[i * s2 + j] = c64::from_polar(1.0, ph);
        } }
        fft2_inplace(&mut tone, s1, s2, false);
        let (mut best, mut bi) = (0.0, 0);
        for (k, z) in tone.iter().enumerate() { if z.norm() > best { best = z.norm(); bi = k; } }
        assert_eq!(bi, s2 + 3, "a pure 2-D tone lands in one bin");
        assert!((best - ((s1 * s2) as f64).sqrt()).abs() < 1e-9);
        let l = tape.mean_real(f);
        tape.backward(l);
        let g = tape.grad(xv).unwrap().clone();
        let eps = 1e-6;
        for k in (0..x.len()).step_by(7) {
            let mut xp = x.clone(); xp.data[k].re += eps;
            let mut t2 = Tape::new(); let v = t2.constant(xp); let f2 = t2.fft2(v, s1, s2); let l2 = t2.mean_real(f2);
            let fd = (t2.value(l2).data[0].re - tape.value(l).data[0].re) / eps;
            assert!((fd - g.data[k].re).abs() < 1e-5, "k {k}: {fd} vs {}", g.data[k].re);
        }
    }

