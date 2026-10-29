//! Plain statistics the workspace reads its runs by: a mean, a population
//! standard deviation, precision, recall and F1, and the log-scale share of
//! a gap closed. Each was written inline in phient (the ruling of
//! 2026-10-28); here, exactly as it was. Then the readings of a contingency
//! table (rows: what a model found; columns: what was told): its purity and
//! its mutual information, and a z-score against shuffled tables (phisor's
//! measures, 2026-10-29).

/// The mean; 0 for none.
pub fn mean(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    xs.iter().sum::<f64>() / xs.len() as f64
}

/// The population standard deviation; 0 for fewer than two.
pub fn population_sd(xs: &[f64]) -> f64 {
    if xs.len() < 2 {
        return 0.0;
    }
    let avg = mean(xs);
    (xs.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / xs.len() as f64).sqrt()
}

/// The share of the predicted that hit; 0 when nothing was predicted.
pub fn precision(hit: f64, predicted: usize) -> f64 {
    match predicted {
        0 => 0.0,
        n => hit / n as f64,
    }
}

/// The share of the expected that was hit; 0 when nothing was expected.
pub fn recall(hit: f64, expected: usize) -> f64 {
    match expected {
        0 => 0.0,
        n => hit / n as f64,
    }
}

/// F1 from a hit count and the sizes of the predicted and the expected sets;
/// 0 when either is empty.
pub fn f1(hit: f64, predicted: usize, expected: usize) -> f64 {
    if predicted == 0 || expected == 0 {
        return 0.0;
    }
    let precision = precision(hit, predicted);
    let recall = recall(hit, expected);
    match precision + recall > 0.0 {
        true => 2.0 * precision * recall / (precision + recall),
        false => 0.0,
    }
}

/// The share of the gap between `floor` and `ceiling` that `x` closes, on a
/// log scale: `(ln floor − ln x) / (ln floor − ln ceiling)`. `None` when the
/// gap is under 1e-9 nats.
pub fn log_share(floor: f64, ceiling: f64, x: f64) -> Option<f64> {
    let span = floor.ln() - ceiling.ln();
    match span.abs() < 1e-9 {
        true => None,
        false => Some((floor.ln() - x.ln()) / span),
    }
}

/// The share of a table's mass in each row's largest cell: Σ over rows of
/// the row's max, over the total. 1 when every row is one column; 0 for an
/// empty table.
pub fn purity(table: &[Vec<f64>]) -> f64 {
    let total: f64 = table.iter().flatten().sum();
    match total > 0.0 {
        true => table.iter().map(|r| r.iter().cloned().fold(0.0, f64::max)).sum::<f64>() / total,
        false => 0.0,
    }
}

/// The mutual information of a table's rows and columns, in bits:
/// `Σ p(x,y) log2(p(x,y) / (p(x) p(y)))`. Rows may be ragged (a missing cell
/// is 0); 0 for an empty table.
pub fn mutual_information(table: &[Vec<f64>]) -> f64 {
    let total: f64 = table.iter().flatten().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let width = table.iter().map(Vec::len).max().unwrap_or(0);
    let mut cols = vec![0.0; width];
    for r in table {
        for (j, v) in r.iter().enumerate() {
            cols[j] += v;
        }
    }
    let mut bits = 0.0;
    for r in table {
        let row: f64 = r.iter().sum();
        for (j, &v) in r.iter().enumerate().filter(|(_, v)| **v > 0.0) {
            bits += v / total * (v * total / (row * cols[j])).log2();
        }
    }
    bits
}

/// How far `x` stands from a null's draws, in the null's population
/// standard deviations; 0 when the null does not spread.
pub fn z_score(x: f64, null: &[f64]) -> f64 {
    let sd = population_sd(null);
    match sd > 0.0 {
        true => (x - mean(null)) / sd,
        false => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_plain_statistics() {
        assert_eq!(mean(&[]), 0.0);
        assert_eq!(mean(&[1.0, 3.0]), 2.0);
        assert_eq!(population_sd(&[2.0]), 0.0);
        assert_eq!(population_sd(&[1.0, 3.0]), 1.0);
        assert_eq!(f1(0.0, 0, 3), 0.0);
        assert!((f1(1.0, 2, 2) - 0.5).abs() < 1e-15);
        assert_eq!(log_share(10.0, 10.0, 5.0), None);
        assert!((log_share(100.0, 10.0, 10.0).unwrap() - 1.0).abs() < 1e-12);
        assert_eq!((precision(1.0, 0), recall(1.0, 0)), (0.0, 0.0));
        assert_eq!((precision(1.0, 4), recall(1.0, 2)), (0.25, 0.5));
        assert_eq!(z_score(3.0, &[1.0, 1.0]), 0.0);
        assert_eq!(z_score(3.0, &[0.0, 2.0]), 2.0);
    }

    #[test]
    fn a_table_read_for_its_purity_and_information() {
        let diagonal = vec![vec![5.0, 0.0], vec![0.0, 5.0]];
        let even = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        assert_eq!(purity(&diagonal), 1.0);
        assert_eq!(purity(&even), 0.5);
        assert_eq!(purity(&[]), 0.0);
        assert!((mutual_information(&diagonal) - 1.0).abs() < 1e-12);
        assert!(mutual_information(&even).abs() < 1e-12);
        assert_eq!(mutual_information(&[vec![], vec![0.0]]), 0.0);
        // Ragged: the missing cell is a zero.
        let ragged = vec![vec![2.0], vec![0.0, 2.0]];
        assert!((mutual_information(&ragged) - 1.0).abs() < 1e-12);
    }
}
