//! Huberman's order on a stage and a measure (her 1968 thesis, eqs. 3.6 and
//! 3.8; `phiano/docs/how/A1_better_worse_lineage.md`). A position has a
//! coarse **stage**, a discrete subgoal index, and a fine **measure** of
//! progress within it, where less is nearer. The two are compared
//! lexicographically, never summed: advancing a stage is better whatever
//! the measure does.

/// A position's stage and measure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Staged {
    pub stage: u64,
    pub measure: f64,
}

/// Whether `q` is better than `p` (3.6): a higher stage, or the same stage
/// with a smaller measure.
pub fn better(p: Staged, q: Staged) -> bool {
    match q.stage.cmp(&p.stage) {
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Equal => q.measure < p.measure,
        std::cmp::Ordering::Less => false,
    }
}

/// Whether `q` is worse than `p`, read for a learner: a lower stage, or the
/// same stage with a larger measure. The thesis's 3.8 counts only a fall
/// back to stage 0 as worse (`q.stage == 0`), so a drop to a lower stage
/// above 0 is neither better nor worse there; here it is worse.
pub fn worse(p: Staged, q: Staged) -> bool {
    match q.stage.cmp(&p.stage) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Equal => q.measure > p.measure,
        std::cmp::Ordering::Greater => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stage_outranks_any_measure() {
        let s = |stage, measure| Staged { stage, measure };
        assert!(better(s(1, 0.0), s(2, 99.0)));
        assert!(better(s(1, 5.0), s(1, 4.0)));
        assert!(!better(s(1, 4.0), s(1, 4.0)) && !worse(s(1, 4.0), s(1, 4.0)));
        assert!(worse(s(2, 0.0), s(1, 0.0)));
        assert!(worse(s(1, 4.0), s(1, 5.0)));
        // Each is the other read backwards.
        for (p, q) in [(s(1, 1.0), s(2, 3.0)), (s(3, 1.0), s(3, 2.0)), (s(0, 0.5), s(0, 0.5))] {
            assert_eq!(better(p, q), worse(q, p));
        }
    }
}
