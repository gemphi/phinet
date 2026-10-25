//! Every op through a small network against finite differences, and the Möbius map.

use super::super::*;
use crate::c64;

    thread_local! {
        static TABLE_OVERRIDE: std::cell::RefCell<Option<Tensor>> = const { std::cell::RefCell::new(None) };
    }

    /// Finite-difference check of every op through a small network.
    #[test]
    fn test_gradients_match_finite_differences() {
        let s = 8;
        let build = |tape: &mut Tape, lift: Tensor, r: Tensor, w: Tensor, bias: Tensor, proj: Tensor| -> (Var, Vec<Var>) {
            let table_t = TABLE_OVERRIDE
                .with(|o| o.borrow().clone())
                .unwrap_or_else(|| Tensor::randn([5, 1, s], 1.0, 1));
            let table = tape.param(table_t);
            let raw = tape.gather(table, &[4, 0, 2]);
            let x = tape.unit(raw);
            let target = tape.gather(table, &[1, 3, 0]);
            let decoy = tape.constant(Tensor::randn([3, 1, s], 1.0, 3));
            let lift = tape.param(lift);
            let r = tape.param(r);
            let w = tape.param(w);
            let bias = tape.param(bias);
            let proj = tape.param(proj);
            let h = tape.linear(x, lift);
            let h = tape.gelu(h);
            let hf = tape.fft(h);
            let hm = tape.mode_mix(hf, r);
            let hs = tape.ifft(hm);
            let hl = tape.linear(h, w);
            let hl = tape.add_bias(hl, bias);
            let h2 = tape.add(hs, hl);
            let h2 = tape.gelu(h2);
            let h2 = tape.scale(h2, 0.7);
            let y = tape.linear(h2, proj);
            let sp = tape.cosine(y, target);
            let sn = tape.cosine(y, decoy);
            let loss = tape.hinge(sp, sn, 1.5);
            (loss, vec![lift, r, w, bias, proj, table])
        };
        let width = 3;
        let init = || {
            (
                Tensor::randn([1, width, 1], 0.7, 11),
                Tensor::randn([width, width, s], 0.5, 12),
                Tensor::randn([width, width, 1], 0.5, 13),
                Tensor::randn([1, width, 1], 0.3, 14),
                Tensor::randn([width, 1, 1], 0.7, 15),
            )
        };
        let (l, r, w, b, p) = init();
        let mut tape = Tape::new();
        let (loss, params) = build(&mut tape, l, r, w, b, p);
        let base = tape.value(loss).get(0, 0, 0).re;
        assert!(base > 0.0, "hinge must be active for the check to mean anything");
        tape.backward(loss);
        let grads: Vec<Tensor> = params.iter().map(|p| tape.grad(*p).unwrap().clone()).collect();

        let eps = 1e-6;
        let mut checked = 0;
        for (pi, _) in params.iter().enumerate() {
            let g = &grads[pi];
            for k in (0..g.len()).step_by(3) {
                for part in 0..2 {
                    let mut ts = init();
                    let mut table_bump: Option<(usize, usize)> = None;
                    if pi < 5 {
                        let t: &mut Tensor = match pi {
                            0 => &mut ts.0,
                            1 => &mut ts.1,
                            2 => &mut ts.2,
                            3 => &mut ts.3,
                            _ => &mut ts.4,
                        };
                        if part == 0 {
                            t.data[k].re += eps;
                        } else {
                            t.data[k].im += eps;
                        }
                    } else {
                        table_bump = Some((k, part));
                    }
                    let mut tape2 = Tape::new();
                    if let Some((k, part)) = table_bump {
                        // The table is created inside `build` from seed 1;
                        // rebuild it here with the same seed and bump.
                        let mut t = Tensor::randn([5, 1, s], 1.0, 1);
                        if part == 0 {
                            t.data[k].re += eps;
                        } else {
                            t.data[k].im += eps;
                        }
                        TABLE_OVERRIDE.with(|o| *o.borrow_mut() = Some(t));
                    }
                    let (loss2, _) = build(&mut tape2, ts.0, ts.1, ts.2, ts.3, ts.4);
                    TABLE_OVERRIDE.with(|o| *o.borrow_mut() = None);
                    let lp = tape2.value(loss2).get(0, 0, 0).re;
                    let fd = (lp - base) / eps;
                    let an = if part == 0 { g.data[k].re } else { g.data[k].im };
                    assert!(
                        (fd - an).abs() < 1e-4 * (1.0 + an.abs()),
                        "param {pi} elem {k} part {part}: finite diff {fd:.6e} vs autograd {an:.6e}"
                    );
                    checked += 1;
                }
            }
        }
        assert!(checked > 40, "checked {checked} partials");
    }

    /// Finite differences through `cosine_table` and `mixture_nll`.

    #[test]
    fn test_mobius_gradients_match_finite_differences() {
        let s = 8;
        let build = |tape: &mut Tape, x: Tensor, m: Tensor| -> (Var, Var, Var) {
            let x = tape.param(x);
            let m = tape.param(m);
            let target = tape.constant(Tensor::randn([1, 1, s], 1.0, 21));
            let w = tape.mobius(x, m);
            let u = tape.unit(w);
            let c = tape.cosine(u, target);
            let l = tape.mean_real(c);
            (l, x, m)
        };
        let x0 = Tensor::randn([1, 1, s], 1.0, 22);
        let m0 = Tensor::from_vec([1, 1, 4], vec![c64::new(0.9, 0.3), c64::new(0.2, -0.1), c64::new(0.1, 0.2), c64::new(1.0, 0.0)]);
        let mut tape = Tape::new();
        let (l, x, m) = build(&mut tape, x0.clone(), m0.clone());
        let base = tape.value(l).get(0, 0, 0).re;
        tape.backward(l);
        let gx = tape.grad(x).unwrap().clone();
        let gm = tape.grad(m).unwrap().clone();
        let eps = 1e-6;
        for (which, n) in [(0usize, s), (1usize, 4)] {
            for k in 0..n {
                for part in 0..2 {
                    let (mut x1, mut m1) = (x0.clone(), m0.clone());
                    let t = if which == 0 { &mut x1 } else { &mut m1 };
                    if part == 0 { t.data[k].re += eps } else { t.data[k].im += eps }
                    let mut t2 = Tape::new();
                    let (l2, _, _) = build(&mut t2, x1, m1);
                    let fd = (t2.value(l2).get(0, 0, 0).re - base) / eps;
                    let g = if which == 0 { &gx } else { &gm };
                    let an = if part == 0 { g.data[k].re } else { g.data[k].im };
                    assert!((fd - an).abs() < 1e-4 * (1.0 + an.abs()), "mobius {which}/{k}/{part}: fd {fd:.6e} vs {an:.6e}");
                }
            }
        }
    }

    /// Identity coefficients leave the input unchanged; rotation is bind.
    #[test]
    fn test_mobius_identity_and_rotation() {
        let x0 = Tensor::randn([1, 1, 8], 1.0, 5);
        let mut tape = Tape::new();
        let x = tape.constant(x0.clone());
        let id = tape.constant(Tensor::from_vec([1, 1, 4], vec![c64::new(1.0, 0.0), c64::new(0.0, 0.0), c64::new(0.0, 0.0), c64::new(1.0, 0.0)]));
        let y = tape.mobius(x, id);
        for (a, b) in tape.value(y).data.iter().zip(&x0.data) {
            assert!((a - b).norm() < 1e-12);
        }
        let rot = c64::from_polar(1.0, 0.7);
        let r = tape.constant(Tensor::from_vec([1, 1, 4], vec![rot, c64::new(0.0, 0.0), c64::new(0.0, 0.0), c64::new(1.0, 0.0)]));
        let y = tape.mobius(x, r);
        for (a, b) in tape.value(y).data.iter().zip(&x0.data) {
            assert!((a - b * rot).norm() < 1e-12);
        }
    }

