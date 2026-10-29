use super::halve::{halve, Grow};

/// Three determiners (0, 1, 2) and three nouns (3, 4, 5), alternating, each
/// drawn by a small generator so the counts are uneven, as in text.
fn alternating(n: usize) -> Vec<Vec<usize>> {
    let mut state = 7u64;
    let mut draw = |m: u64| {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((state >> 33) % m) as usize
    };
    (0..n).map(|_| (0..8).map(|i| draw(3) + 3 * (i % 2)).collect()).collect()
}

#[test]
fn one_bit_parts_what_comes_before_from_what_comes_after() {
    let h = halve(6, &alternating(200), Grow::Fixed(1));
    let (d, n) = (h.codes[0], h.codes[3]);
    assert_ne!(d, n);
    assert!(h.codes[..3].iter().all(|&c| c == d), "{:?}", h.codes);
    assert!(h.codes[3..].iter().all(|&c| c == n), "{:?}", h.codes);
}

#[test]
fn every_kept_move_gains_and_the_code_is_nested() {
    let seqs = alternating(200);
    let deep = halve(6, &seqs, Grow::Fixed(3));
    assert!(deep.passes.iter().flatten().all(|&g| g >= 0.0));
    assert_eq!((deep.passes.len(), deep.depth), (3, 3));
    // The likelihood never falls from one depth to the next: a split can
    // only tell more apart.
    assert!(deep.likelihood.windows(2).all(|w| w[1] >= w[0] - 1e-9), "{:?}", deep.likelihood);
    assert!(deep.codes.iter().all(|&c| c < 8));
    // Cut to one bit, the deep code is the one-bit code's split.
    let cut: Vec<u16> = (0..6).map(|u| deep.cut(u, 1)).collect();
    assert!(cut[..3].iter().all(|&c| c == cut[0]) && cut[3..].iter().all(|&c| c == cut[3]) && cut[0] != cut[3], "{cut:?}");
}

#[test]
fn the_data_says_how_many_classes_it_pays_for() {
    // Determiners and nouns part; three determiners drawn alike do not pay
    // for parting further, nor do three nouns.
    let h = halve(6, &alternating(200), Grow::Paid);
    assert_eq!(h.classes, 2, "{:?} kept {:?}", h.codes, h.kept);
    assert_eq!(h.depth, 1);
    assert!(h.codes[..3].iter().all(|&c| c == h.codes[0]) && h.codes[0] != h.codes[3]);
}
