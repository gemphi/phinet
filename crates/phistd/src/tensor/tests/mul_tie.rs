//! `mul` and `tie` against finite differences: the two ops no other check
//! reached.

use super::super::*;
use crate::c64;

/// The pointwise product of two parameters, times a tied filter, scored by
/// cosine against a fixed target: every partial of `x`, `y` and the tied
/// weights `r` checked against a finite difference.
#[test]
fn test_mul_and_tie_gradients_match_finite_differences() {
    let s = 8;
    let group = [0, 1, 2, 1, 0, 2, 2, 1];
    let build = |tape: &mut Tape, x: Tensor, y: Tensor, r: Tensor| -> (Var, [Var; 3]) {
        let (x, y, r) = (tape.param(x), tape.param(y), tape.param(r));
        let target = tape.constant(Tensor::randn([1, 1, s], 1.0, 31));
        let xy = tape.mul(x, y);
        let filter = tape.tie(r, &group);
        let z = tape.mul(xy, filter);
        let u = tape.unit(z);
        let c = tape.cosine(u, target);
        (tape.mean_real(c), [x, y, r])
    };
    let init = [Tensor::randn([1, 1, s], 1.0, 32), Tensor::randn([1, 1, s], 1.0, 33), Tensor::randn([1, 1, 3], 1.0, 34)];
    let mut tape = Tape::new();
    let (l, params) = build(&mut tape, init[0].clone(), init[1].clone(), init[2].clone());
    let base = tape.value(l).get(0, 0, 0).re;
    tape.backward(l);
    let grads: Vec<Tensor> = params.iter().map(|p| tape.grad(*p).expect("a param has a gradient").clone()).collect();
    let eps = 1e-6;
    let mut checked = 0;
    for (which, g) in grads.iter().enumerate() {
        for k in 0..g.len() {
            for part in 0..2 {
                let mut bumped = init.clone();
                match part {
                    0 => bumped[which].data[k].re += eps,
                    _ => bumped[which].data[k].im += eps,
                }
                let [x1, y1, r1] = bumped;
                let mut t2 = Tape::new();
                let (l2, _) = build(&mut t2, x1, y1, r1);
                let fd = (t2.value(l2).get(0, 0, 0).re - base) / eps;
                let an = match part {
                    0 => g.data[k].re,
                    _ => g.data[k].im,
                };
                assert!((fd - an).abs() < 1e-4 * (1.0 + an.abs()), "param {which} elem {k} part {part}: fd {fd:.6e} vs {an:.6e}");
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 2 * (s + s + 3), "every partial of x, y and the three tied weights");
}

/// `tie` copies each group's weight to its modes, and its gradient is the
/// sum over those modes.
#[test]
fn test_tie_copies_forward_and_sums_backward() {
    let mut tape = Tape::new();
    let r = tape.param(Tensor::from_vec([1, 1, 2], vec![c64::new(2.0, 0.0), c64::new(0.0, 1.0)]));
    let t = tape.tie(r, &[1, 0, 1, 1]);
    let got: Vec<c64> = tape.value(t).data.clone();
    assert_eq!(got, vec![c64::new(0.0, 1.0), c64::new(2.0, 0.0), c64::new(0.0, 1.0), c64::new(0.0, 1.0)]);
    let m = tape.mean_real(t);
    tape.backward(m);
    let g = tape.grad(r).expect("r has a gradient");
    assert!((g.data[0].re - 0.25).abs() < 1e-12, "group 0 feeds one of four modes");
    assert!((g.data[1].re - 0.75).abs() < 1e-12, "group 1 feeds three of four modes");
}
