use super::{dense, halve, halve_from, halve_with, Grow, Halving, Settings};

/// The same uneven draw `halving_tests` uses: three determiners (0, 1, 2)
/// and three nouns (3, 4, 5), alternating, so the counts look like text.
fn alternating(n: usize) -> Vec<Vec<usize>> {
    let mut state = 7u64;
    let mut draw = |m: u64| {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((state >> 33) % m) as usize
    };
    (0..n)
        .map(|_| (0..8).map(|i| draw(3) + 3 * (i % 2)).collect())
        .collect()
}

/// How many classes the code holds once cut to each depth `1..=depth`: the
/// class count the run reaches at every depth, read off the finished codes.
fn classes_by_depth(units: usize, seqs: &[Vec<usize>], grow: Grow) -> Vec<usize> {
    let h = halve_with(units, seqs, grow, &Settings::default());
    (1..=h.depth)
        .map(|d| {
            dense(&(0..units).map(|u| h.cut(u, d)).collect::<Vec<_>>())
                .0
                .len()
        })
        .collect()
}

/// `Settings::default()` maps each field to the const or no-op it replaced,
/// so `halve_with(..., &default())` is `halve(...)` field for field — the
/// class count at every depth is identical to the pre-change run.
#[test]
fn the_defaults_reproduce_the_class_count_at_every_depth() {
    let seqs = alternating(200);
    for grow in [Grow::Fixed(1), Grow::Fixed(3), Grow::Paid] {
        let base = halve(6, &seqs, grow);
        let with = halve_with(6, &seqs, grow, &Settings::default());
        assert_eq!(base.depth, with.depth, "depth drifted at {grow:?}");
        assert_eq!(
            base.classes, with.classes,
            "final class count drifted at {grow:?}"
        );
        assert_eq!(base.codes, with.codes, "codes drifted at {grow:?}");
        assert_eq!(base.kept, with.kept, "kept-per-depth drifted at {grow:?}");
        // The class count read at each depth matches too, not just the end.
        let a = classes_by_depth(6, &seqs, grow);
        let b: Vec<usize> = (1..=base.depth)
            .map(|d| {
                dense(&(0..6).map(|u| base.cut(u, d)).collect::<Vec<_>>())
                    .0
                    .len()
            })
            .collect();
        assert_eq!(a, b, "per-depth class count drifted at {grow:?}");
    }
}

/// The default fields equal exactly the consts and no-ops they stand in for.
#[test]
fn the_default_fields_equal_the_consts() {
    let s = Settings::default();
    assert_eq!(s.max_passes, super::MAX_PASSES);
    assert_eq!(s.stop_fraction, super::STOP);
    assert_eq!(s.target_classes, 0);
    assert_eq!(s.min_unit_frequency, 0);
    assert_eq!(s.split_cost_penalty, 0.0);
    assert_eq!(s.move_order_seed, 0);
}

/// Two runs part the units alike when two units share a class in one exactly
/// when they share one in the other.
fn same_partition(a: &[u16], b: &[u16]) -> bool {
    (0..a.len()).all(|u| (0..a.len()).all(|v| (a[u] == a[v]) == (b[u] == b[v])))
}

/// A class that stopped early is padded with 0 bits; its path ends where no
/// other class shares its bits.
#[test]
fn a_path_names_its_class_without_the_padding() {
    let h = Halving { codes: vec![0b000, 0b000, 0b100, 0b101, 0b110], depth: 3, classes: 4, ..Halving::default() };
    let paths = h.paths();
    assert_eq!(paths[0], [false]);
    assert_eq!(paths[2], [true, false, false]);
    assert_eq!(paths[3], [true, false, true]);
    assert_eq!(paths[4], [true, true]);
}

/// Resumed from its own paths, a run keeps the classes it learned; a unit
/// with no path (a new one) finds its class among them.
#[test]
fn a_run_resumed_from_its_paths_keeps_its_classes_and_places_a_new_unit() {
    let seqs = alternating(200);
    let first = halve(6, &seqs, Grow::Paid);
    let again = halve_from(6, &seqs, Grow::Paid, &Settings::default(), &first.paths());
    assert_eq!(again.classes, first.classes);
    assert!(same_partition(&first.codes, &again.codes), "{:?} against {:?}", first.codes, again.codes);
    let mut prior = first.paths();
    prior[5] = Vec::new();
    let placed = halve_from(6, &seqs, Grow::Paid, &Settings::default(), &prior);
    assert!(same_partition(&first.codes, &placed.codes), "{:?} against {:?}", first.codes, placed.codes);
}
