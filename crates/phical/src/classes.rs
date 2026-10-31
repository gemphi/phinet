//! Word classes by exchange (Brown et al. 1992; Martin, Liermann and Ney
//! 1998): the likelihood a class bigram table gives its text, and what one
//! word's move from class `a` to class `b` would change it by.
//!
//! The tables are counts. `pairs` is k×k row-major: `pairs[x·k + y]` counts
//! a word of class `x` followed by a word of class `y`. `left[x] = Σ_y
//! pairs[x][y]` and `right[y] = Σ_x pairs[x][y]`. Up to constants of the
//! text, the log-likelihood of the class bigram model is
//! `Σ nln(pairs) − Σ nln(left) − Σ nln(right)`, with `nln(x) = x ln x`.
//! A move touches rows a and b, columns a and b, and four marginals, so its
//! gain is read in O(k), not O(k²).
//!
//! The exchange itself (from phisor's `exchange`, 2026-10-29): units in
//! sequences, their pairs counted, and a binary code grown by halving. At
//! each depth every class still growing splits in two and every unit may
//! move to its sibling; a move is kept if it gains, and a split only while
//! it pays for itself (`split_cost`), so the data says how many classes
//! there are.
//!
//! | file | what |
//! |---|---|
//! | `classes/table` | the pairs, sparse per unit; the class table over dense class ids |
//! | `classes/halve` | `halve`, `Halving`, `Grow`: the depths, the passes, Babbage's stop |
//! | `classes/pay` | a split kept only if its gain exceeds `split_cost` |

mod compare;
mod halve;
#[cfg(test)]
mod halving_tests;
mod pay;
mod table;

pub use halve::{halve, halve_with, Grow, Halving, Settings, MAX_DEPTH, MAX_PASSES, STOP};

/// `x ln x`, 0 at 0 (and for the negative zero a subtraction can leave).
fn nln(x: f64) -> f64 {
    match x > 0.0 {
        true => x * x.ln(),
        false => 0.0,
    }
}

fn ind(c: bool) -> f64 {
    match c {
        true => 1.0,
        false => 0.0,
    }
}

/// `Σ nln(pairs) − Σ nln(left) − Σ nln(right)`.
pub fn likelihood(pairs: &[f64], left: &[f64], right: &[f64]) -> f64 {
    let sum = |xs: &[f64]| xs.iter().map(|&x| nln(x)).sum::<f64>();
    sum(pairs) - sum(left) - sum(right)
}

/// One word's bigram counts, by the class of the other word. `out[y]`
/// counts (w, v) for v ≠ w in class y; `inn[x]` counts (v, w) for v ≠ w in
/// class x; `own` counts (w, w).
pub struct WordCounts<'a> {
    pub out: &'a [f64],
    pub inn: &'a [f64],
    pub own: f64,
}

/// The cell (x, y) once the word has moved from class `a` to class `b`.
fn after(pairs: &[f64], k: usize, w: &WordCounts, a: usize, b: usize, x: usize, y: usize) -> f64 {
    pairs[x * k + y] + (ind(x == b) - ind(x == a)) * w.out[y] + (ind(y == b) - ind(y == a)) * w.inn[x]
        + (ind(x == b && y == b) - ind(x == a && y == a)) * w.own
}

/// The likelihood after the word moves from class `a` to class `b`, less the
/// likelihood before. 0 when `a == b`.
pub fn move_gain(pairs: &[f64], left: &[f64], right: &[f64], k: usize, w: &WordCounts, a: usize, b: usize) -> f64 {
    if a == b {
        return 0.0;
    }
    let cell = |x: usize, y: usize| nln(after(pairs, k, w, a, b, x, y)) - nln(pairs[x * k + y]);
    let mut gain = 0.0;
    for x in [a, b] {
        gain += (0..k).map(|y| cell(x, y)).sum::<f64>();
    }
    for y in [a, b] {
        gain += (0..k).filter(|&x| x != a && x != b).map(|x| cell(x, y)).sum::<f64>();
    }
    let (so, si) = (w.out.iter().sum::<f64>() + w.own, w.inn.iter().sum::<f64>() + w.own);
    gain -= nln(left[a] - so) + nln(left[b] + so) - nln(left[a]) - nln(left[b]);
    gain -= nln(right[a] - si) + nln(right[b] + si) - nln(right[a]) - nln(right[b]);
    gain
}

/// Makes the move in the tables: the same cells and marginals `move_gain`
/// reads. The caller keeps each word's own counts (a neighbour's `out` and
/// `inn` shift from a to b by what it shares with the word that moved).
pub fn apply_move(pairs: &mut [f64], left: &mut [f64], right: &mut [f64], k: usize, w: &WordCounts, a: usize, b: usize) {
    if a == b {
        return;
    }
    let mut cells = Vec::with_capacity(4 * k);
    for x in [a, b] {
        cells.extend((0..k).map(|y| (x, y)));
    }
    for y in [a, b] {
        cells.extend((0..k).filter(|&x| x != a && x != b).map(|x| (x, y)));
    }
    let moved: Vec<f64> = cells.iter().map(|&(x, y)| after(pairs, k, w, a, b, x, y)).collect();
    for (&(x, y), v) in cells.iter().zip(moved) {
        pairs[x * k + y] = v;
    }
    let (so, si) = (w.out.iter().sum::<f64>() + w.own, w.inn.iter().sum::<f64>() + w.own);
    left[a] -= so;
    left[b] += so;
    right[a] -= si;
    right[b] += si;
}

/// What one more class costs, in nats: the BIC penalty `(2k − 1)/2 · ln N`
/// for going from `classes` = k to k + 1 over `pairs` = N bigrams. A split
/// is worth keeping when its likelihood gain is larger, so the data says
/// how many classes there are. The count is the model `likelihood` scores,
/// p(c₂ | c₁) · p(w | c₂): its transitions have k(k − 1) free parameters and
/// its emissions V − k, so a class adds 2k and takes one away.
pub fn split_cost(classes: usize, pairs: f64) -> f64 {
    (2 * classes).saturating_sub(1) as f64 / 2.0 * pairs.max(1.0).ln()
}

#[cfg(test)]
mod tests {
    use super::*;
    use phistd::random::xorshift64;

    #[test]
    fn a_class_costs_more_the_more_there_are_and_the_more_is_told() {
        let e2 = std::f64::consts::E.powi(2);
        assert!((split_cost(1, e2) - 1.0).abs() < 1e-12);
        assert!((split_cost(4, e2) - 7.0).abs() < 1e-12);
        assert!(split_cost(5, 1e4) > split_cost(4, 1e4) && split_cost(4, 1e5) > split_cost(4, 1e4));
        assert_eq!((split_cost(0, 1e4), split_cost(3, 0.5)), (0.0, 0.0));
    }

    /// Class tables from word bigram counts `c` (n×n) and a class per word.
    fn tables(c: &[f64], n: usize, class: &[usize], k: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut pairs = vec![0.0; k * k];
        for v in 0..n {
            for u in 0..n {
                pairs[class[v] * k + class[u]] += c[v * n + u];
            }
        }
        let left = (0..k).map(|x| (0..k).map(|y| pairs[x * k + y]).sum()).collect();
        let right = (0..k).map(|y| (0..k).map(|x| pairs[x * k + y]).sum()).collect();
        (pairs, left, right)
    }

    #[test]
    fn the_gain_is_the_likelihood_recomputed() {
        let (n, k) = (12usize, 4usize);
        let mut s = 0x9E3779B97F4A7C15u64;
        let mut draw = |m: u64| {
            s = xorshift64(s);
            s % m
        };
        let c: Vec<f64> = (0..n * n).map(|_| draw(5) as f64).collect();
        let class: Vec<usize> = (0..n).map(|_| draw(k as u64) as usize).collect();
        let (pairs, left, right) = tables(&c, n, &class, k);
        let before = likelihood(&pairs, &left, &right);
        for w in 0..n {
            let a = class[w];
            let (mut out, mut inn) = (vec![0.0; k], vec![0.0; k]);
            for v in (0..n).filter(|&v| v != w) {
                out[class[v]] += c[w * n + v];
                inn[class[v]] += c[v * n + w];
            }
            let counts = WordCounts { out: &out, inn: &inn, own: c[w * n + w] };
            for b in 0..k {
                let mut moved = class.clone();
                moved[w] = b;
                let (p2, l2, r2) = tables(&c, n, &moved, k);
                let want = likelihood(&p2, &l2, &r2) - before;
                let got = move_gain(&pairs, &left, &right, k, &counts, a, b);
                assert!((got - want).abs() < 1e-9 * (1.0 + want.abs()), "w {w} {a}→{b}: {got} vs {want}");
                let (mut p3, mut l3, mut r3) = (pairs.clone(), left.clone(), right.clone());
                apply_move(&mut p3, &mut l3, &mut r3, k, &counts, a, b);
                assert_eq!((p3, l3, r3), (p2, l2, r2), "w {w} {a}→{b}: the tables after the move");
            }
        }
    }
}
