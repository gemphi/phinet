//! Schedules, AdamW, and Adam reducing a loss.

use super::super::*;
use crate::c64;

    #[test]
    fn test_schedules() {
        let c = Schedule::Cosine { warmup: 10, total: 110, floor: 0.1 };
        assert!((c.factor(5) - 0.5).abs() < 1e-12);
        assert!((c.factor(10) - 1.0).abs() < 1e-12);
        assert!((c.factor(60) - 0.55).abs() < 1e-9, "{}", c.factor(60));
        assert!((c.factor(110) - 0.1).abs() < 1e-12 && c.factor(500) == 0.1);
        let st = Schedule::Step { every: 3, gamma: 0.5 };
        assert_eq!(st.factor(2), 1.0); assert_eq!(st.factor(3), 0.5); assert_eq!(st.factor(7), 0.25);
        assert_eq!(Schedule::Constant.factor(999), 1.0);
        let mut a = Adam::new(0.1).with_schedule(st);
        assert_eq!(a.current_lr(), 0.1);
        let mut tape = Tape::new();
        let p = tape.param(Tensor::from_vec([1, 1, 1], vec![c64::new(1.0, 0.0)]));
        let l = tape.mean_real(p); tape.backward(l);
        for _ in 0..3 { a.step(&mut tape, &[p]); }
        assert!((a.current_lr() - 0.05).abs() < 1e-12);
        assert_eq!(Adam::from_state(a.state()).schedule, st);
    }


    #[test]
    fn test_adamw_decays_and_state_round_trips() {
        let mut tape = Tape::new();
        let p = tape.param(Tensor::from_vec([1, 1, 2], vec![c64::new(1.0, 0.0), c64::new(0.0, 1.0)]));
        let l = tape.mean_real(p);
        tape.backward(l);
        let mut plain = Adam::new(0.1);
        let mut decayed = Adam::new(0.1).with_decay(0.5);
        let mut t2 = tape.clone_for_test();
        plain.step(&mut tape, &[p]);
        decayed.step(&mut t2, &[p]);
        let a = tape.value(p).data[0];
        let b = t2.value(p).data[0];
        // decoupled decay: b = a − lr·wd·1.0 exactly (the Adam term is identical)
        assert!(((a - b).re - 0.05).abs() < 1e-12, "{a} {b}");
        let state = decayed.state();
        let again = Adam::from_state(state.clone());
        assert_eq!(again.steps(), 1);
        assert_eq!(again.state().m, state.m);
    }


    #[test]
    fn test_adam_reduces_a_loss() {
        let s = 16;
        let mut tape = Tape::new();
        let target = Tensor::randn([4, 1, s], 1.0, 5);
        let x = tape.constant(Tensor::randn([4, 1, s], 1.0, 6));
        let tgt = tape.constant(target);
        let w = tape.param(Tensor::randn([1, 1, s], 0.1, 8));
        let keep = tape.len();
        let mut adam = Adam::new(0.05);
        let mut first = None;
        let mut last = 0.0;
        for _ in 0..200 {
            tape.truncate(keep);
            let xf = tape.fft(x);
            let ym = tape.mode_mix(xf, w);
            let y = tape.ifft(ym);
            let c = tape.cosine(y, tgt);
            let m = tape.mean_real(c);
            let loss = tape.scale(m, -1.0);
            last = tape.value(loss).get(0, 0, 0).re;
            if first.is_none() {
                first = Some(last);
            }
            tape.backward(loss);
            adam.step(&mut tape, &[w]);
        }
        let first = first.unwrap();
        assert!(last < first - 0.3, "loss must fall: {first:.3} -> {last:.3}");
    }
