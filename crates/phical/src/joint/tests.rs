//! Joint training moves the items and lowers the loss.

use super::*;
    use crate::operator::{FnoConfig, Tie};
    use crate::unit_random;

    #[test]
    fn test_joint_training_moves_items_and_lowers_loss() {
        let space = 32;
        let table: Vec<Function> = (0..16).map(|i| unit_random(space, i as u64 + 1)).collect();
        // Sequences cycle through four groups; the target is always the next
        // item in the group.
        let mut examples = Vec::new();
        for s in 0..80 {
            let g = s % 4;
            let items = [g, g + 4, g + 8, g + 12];
            for e in 1..4 {
                let ctx: Vec<(usize, f64)> = (0..e).map(|j| (items[j], 1.0 / (e - j) as f64)).collect();
                examples.push(IndexExample { ctx, target: items[e] });
            }
        }
        let pool: Vec<usize> = (0..16).collect();
        let cfg = FnoConfig { space, width: 4, layers: 1, modes: 8, margin: 0.3, lr: 5e-3, tie: Tie::Circle };
        let mut joint = Joint::new(Fno::new(cfg, 3), &table, 3);
        let before = joint.functions();
        let mut a1 = Adam::new(cfg.lr);
        let mut a2 = Adam::new(cfg.lr * 0.5);
        let first = joint.train_epoch(&examples, &pool, 16, &mut a1, &mut a2, 0);
        let mut last = first;
        for e in 1..15 {
            last = joint.train_epoch(&examples, &pool, 16, &mut a1, &mut a2, e);
        }
        assert!(last < first * 0.7, "loss must fall: {first:.3} -> {last:.3}");
        let after = joint.functions();
        let moved = before
            .iter()
            .zip(&after)
            .filter(|(b, a)| b.iter().zip(a.iter()).any(|(p, q)| (p - q).norm() > 1e-6))
            .count();
        assert!(moved > 8, "items moved: {moved} of 16");
    }

    #[test]
    fn test_each_example_keeps_its_own_context_weights() {
        // Two examples in one batch with different weights: each context
        // function is unit(Σ w · item) with its OWN weights.
        let space = 16;
        let table: Vec<Function> = (0..4).map(|i| unit_random(space, i as u64 + 10)).collect();
        let cfg = FnoConfig { space, width: 2, layers: 1, modes: 4, margin: 0.3, lr: 1e-2, tie: Tie::Circle };
        let mut joint = Joint::new(Fno::new(cfg, 1), &table, 2);
        let a = IndexExample { ctx: vec![(0, 0.25), (1, 1.0)], target: 2 };
        let b = IndexExample { ctx: vec![(2, 3.0), (3, 0.5)], target: 0 };
        let v = joint.context_on_tape(&[&a, &b]);
        let got = crate::unstack(joint.fno.tape.value(v));
        for (ex, row) in [(&a, &got[0]), (&b, &got[1])] {
            let mut want = vec![c64::new(0.0, 0.0); space];
            for &(item, w) in &ex.ctx {
                for (k, z) in want.iter_mut().enumerate() {
                    *z += table[item][k] * w;
                }
            }
            crate::normalise(&mut want);
            assert!(row.iter().zip(&want).all(|(p, q)| (p - q).norm() < 1e-9), "target {}", ex.target);
        }
    }

