//! The losses over a score table against finite differences.

use super::super::*;

    #[test]
    fn test_cross_entropy_gradients_match_finite_differences() {
        let s = 8;
        let build = |pred: Tensor, table: Tensor| -> (Tape, Var, Var, Var) {
            let mut tape = Tape::new();
            let p = tape.param(pred);
            let t = tape.param(table);
            let sc = tape.cosine_table(p, t);
            let loss = tape.cross_entropy(sc, &[2, 0, 4], 6.0);
            (tape, loss, p, t)
        };
        let pred0 = Tensor::randn([3, 1, s], 1.0, 31);
        let table0 = Tensor::randn([5, 1, s], 1.0, 32);
        let (mut tape, loss, p, t) = build(pred0.clone(), table0.clone());
        let base = tape.value(loss).get(0, 0, 0).re;
        assert!(base > 0.0 && base < 20.0);
        tape.backward(loss);
        let gp = tape.grad(p).unwrap().clone();
        let gt = tape.grad(t).unwrap().clone();
        let eps = 1e-6;
        for (which, g) in [(0, &gp), (1, &gt)] {
            for k in (0..g.len()).step_by(3) {
                for part in 0..2 {
                    let (mut pr, mut tb) = (pred0.clone(), table0.clone());
                    let tgt = if which == 0 { &mut pr } else { &mut tb };
                    if part == 0 { tgt.data[k].re += eps; } else { tgt.data[k].im += eps; }
                    let (tape2, loss2, _, _) = build(pr, tb);
                    let fd = (tape2.value(loss2).get(0, 0, 0).re - base) / eps;
                    let an = if part == 0 { g.data[k].re } else { g.data[k].im };
                    assert!((fd - an).abs() < 1e-4 * (1.0 + an.abs()), "which {which} k {k} part {part}: fd {fd:.6e} vs {an:.6e}");
                }
            }
        }
    }


    #[test]
    fn test_mixture_gradients_match_finite_differences() {
        let s = 8;
        let build = |pred: Tensor, table: Tensor| -> (Tape, Var, Var, Var) {
            let mut tape = Tape::new();
            let p = tape.param(pred);
            let t = tape.param(table);
            let sc = tape.cosine_table(p, t);
            let terms = MixtureTerms {
                targets: vec![2, 0, 4],
                k: vec![0.01, 0.2, 0.0],
                coef: vec![0.9, 0.5, 1.0],
                gamma: 0.6,
                other: vec![0.05, 0.01, 0.2],
                beta: 8.0,
            };
            let loss = tape.mixture_nll(sc, terms);
            (tape, loss, p, t)
        };
        let pred0 = Tensor::randn([3, 1, s], 1.0, 21);
        let table0 = Tensor::randn([5, 1, s], 1.0, 22);
        let (mut tape, loss, p, t) = build(pred0.clone(), table0.clone());
        let base = tape.value(loss).get(0, 0, 0).re;
        tape.backward(loss);
        let gp = tape.grad(p).unwrap().clone();
        let gt = tape.grad(t).unwrap().clone();
        let eps = 1e-6;
        let mut checked = 0;
        for (which, g) in [(0, &gp), (1, &gt)] {
            for k in (0..g.len()).step_by(2) {
                for part in 0..2 {
                    let (mut pr, mut tb) = (pred0.clone(), table0.clone());
                    let tgt = if which == 0 { &mut pr } else { &mut tb };
                    if part == 0 {
                        tgt.data[k].re += eps;
                    } else {
                        tgt.data[k].im += eps;
                    }
                    let (tape2, loss2, _, _) = build(pr, tb);
                    let fd = (tape2.value(loss2).get(0, 0, 0).re - base) / eps;
                    let an = if part == 0 { g.data[k].re } else { g.data[k].im };
                    assert!((fd - an).abs() < 1e-4 * (1.0 + an.abs()), "which {which} k {k} part {part}: fd {fd:.6e} vs {an:.6e}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 30);
    }

