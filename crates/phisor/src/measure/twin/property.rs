//! Property 48: probability and amplitude combinations are distributions.

use super::*;
use phistd::gen::{for_all, Gen, Seeded, CASES};

#[derive(Debug)]
struct Pair {
    probabilities: (Vec<f64>, Vec<f64>),
    amplitudes: (Vec<c64>, Vec<c64>),
}

#[derive(Debug)]
struct CombinationCases(Vec<Pair>);

impl Gen for CombinationCases {
    fn draw(g: &mut Seeded) -> Self {
        let n = 2 + g.below(63) as usize;
        let random_a = random_distribution(g, n);
        let random_b = random_distribution(g, n);
        let uniform = vec![1.0 / n as f64; n];
        let mut concentrated_a = vec![0.0; n];
        let mut concentrated_b = vec![0.0; n];
        concentrated_a[g.below(n as u64) as usize] = 1.0;
        concentrated_b[g.below(n as u64) as usize] = 1.0;

        CombinationCases(vec![
            pair(g, random_a, random_b),
            pair(g, uniform.clone(), uniform),
            pair(g, concentrated_a, concentrated_b),
        ])
    }
}

fn random_distribution(g: &mut Seeded, n: usize) -> Vec<f64> {
    let mut values: Vec<f64> = (0..n).map(|_| g.unit()).collect();
    let total: f64 = values.iter().sum();
    values.iter_mut().for_each(|value| *value /= total);
    values
}

fn pair(g: &mut Seeded, a: Vec<f64>, b: Vec<f64>) -> Pair {
    let amplitudes = (
        amplitude_vector(g, &a),
        amplitude_vector(g, &b),
    );
    Pair { probabilities: (a, b), amplitudes }
}

fn amplitude_vector(g: &mut Seeded, probabilities: &[f64]) -> Vec<c64> {
    probabilities
        .iter()
        .map(|probability| c64::from_polar(probability.sqrt(), g.phase()))
        .collect()
}

fn is_distribution(values: &[f64]) -> bool {
    !values.is_empty()
        && values.iter().all(|value| *value >= 0.0)
        && (values.iter().sum::<f64>() - 1.0).abs() <= 1e-12
}

#[test]
fn property_48_each_combination_of_the_two_readings_is_a_distribution() {
    // **Property 48: Each combination of the two readings is a distribution**
    // **Validates: Requirements 9.9, 9.10**
    let run = for_all::<CombinationCases>(0x48, CASES, |cases| {
        cases.0.iter().all(|case| {
            is_distribution(&combine_probabilities(
                &case.probabilities.0,
                &case.probabilities.1,
            )) && is_distribution(&combine_amplitudes(
                &case.amplitudes.0,
                &case.amplitudes.1,
            ))
        })
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
