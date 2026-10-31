//! The backward mirror and Feynman combination measures (R9.6–R9.10).
//!
//! Inputs are held-out readings. A failed dynamic Bar remains in the report,
//! so [`measure_modes`] always finishes all three Modes.

use crate::guide::Function;
use crate::ladder::{Judged as BarJudged, Measured, Unit};
use phical::keyboard::{apply, table, Arena, Family, Operand};
use phical::c64;
use phistd::stats::purity;
use phiton::p64::variants::Level;
use std::collections::BTreeMap;

/// Both ends must beat the better single end by two percentage points.
pub const BOTH_MARGIN: f64 = 2.0;
/// The disagreement report is deliberately bounded.
pub const MAX_LISTED: usize = 100;
/// The Modes in report order.
pub const TWIN_MODES: [Level; 3] = [Level::P16, Level::P32, Level::P64];

/// The three unsupervised Kind readings on one held-out set.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwinPurity {
    pub opening: f64,
    pub closing: f64,
    pub both: f64,
}

/// One held-out sentence and both hard and distributed readings of it.
#[derive(Clone, Debug, PartialEq)]
pub struct TestReading {
    pub split_index: usize,
    pub label: Function,
    pub opening_kind: usize,
    pub closing_kind: usize,
    pub both_kind: usize,
    /// Both vectors use the same Kind coordinate space.
    pub opening_probabilities: Vec<f64>,
    pub closing_probabilities: Vec<f64>,
    /// Magnitudes carry probabilities and arguments carry learned phases.
    pub opening_amplitudes: Vec<c64>,
    pub closing_amplitudes: Vec<c64>,
}

/// Every held-out reading for one Mode.
#[derive(Clone, Debug, PartialEq)]
pub struct ModeInput {
    pub level: Level,
    pub sentences: Vec<TestReading>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KindDisagreement {
    pub split_index: usize,
    pub opening_kind: usize,
    pub closing_kind: usize,
}

/// Total disagreements and no more than their first one hundred examples.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Disagreement {
    pub count: usize,
    pub listed: Vec<KindDisagreement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CombinationReport {
    pub probability: f64,
    pub amplitude: Measured,
    pub difference_points: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModeReport {
    pub level: Level,
    pub scored: usize,
    pub purity: TwinPurity,
    pub both_ends: Measured,
    pub disagreements: Disagreement,
    pub combinations: CombinationReport,
}

/// Measure all three supplied Modes. A Miss in one report cannot stop the
/// later reports because it is retained as [`BarJudged::Miss`], not returned
/// as an error.
pub fn measure_modes(inputs: &[ModeInput; 3]) -> [ModeReport; 3] {
    std::array::from_fn(|i| measure_mode(&inputs[i]))
}

pub fn measure_mode(input: &ModeInput) -> ModeReport {
    let labels: Vec<Function> = input.sentences.iter().map(|row| row.label).collect();
    let opening: Vec<usize> = input.sentences.iter().map(|row| row.opening_kind).collect();
    let closing: Vec<usize> = input.sentences.iter().map(|row| row.closing_kind).collect();
    let both: Vec<usize> = input.sentences.iter().map(|row| row.both_kind).collect();
    let indices: Vec<usize> = input.sentences.iter().map(|row| row.split_index).collect();
    let purity = TwinPurity {
        opening: assignment_purity(&opening, &labels),
        closing: assignment_purity(&closing, &labels),
        both: assignment_purity(&both, &labels),
    };
    let both_ends = measured_twins(purity);

    let probability_kinds: Vec<usize> = input
        .sentences
        .iter()
        .map(|row| greatest(&combine_probabilities(&row.opening_probabilities, &row.closing_probabilities)))
        .collect();
    let amplitude_kinds: Vec<usize> = input
        .sentences
        .iter()
        .map(|row| greatest(&combine_amplitudes(&row.opening_amplitudes, &row.closing_amplitudes)))
        .collect();
    let probability = assignment_purity(&probability_kinds, &labels);
    let amplitude = assignment_purity(&amplitude_kinds, &labels);

    ModeReport {
        level: input.level,
        scored: input.sentences.len(),
        purity,
        both_ends,
        disagreements: disagreements_at(&indices, &opening, &closing),
        combinations: CombinationReport {
            probability,
            amplitude: Measured::at_least(
                "learn.amplitudes",
                amplitude,
                probability,
                Unit::Percentage,
            ),
            difference_points: one_decimal(amplitude - probability),
        },
    }
}

/// Judge the both-ends reading against the dynamic two-point Bar.
pub fn judge_twins(reading: TwinPurity) -> BarJudged {
    measured_twins(reading).verdict
}

fn measured_twins(reading: TwinPurity) -> Measured {
    Measured::at_least(
        "learn.twins.both",
        reading.both,
        reading.opening.max(reading.closing) + BOTH_MARGIN,
        Unit::Percentage,
    )
}

/// Disagreements indexed from zero when no external Split_Rule indices are
/// supplied. [`disagreements_at`] records the corpus indices used by a run.
pub fn disagreements(opening: &[usize], closing: &[usize]) -> Disagreement {
    let indices: Vec<usize> = (0..opening.len().min(closing.len())).collect();
    disagreements_at(&indices, opening, closing)
}

pub fn disagreements_at(indices: &[usize], opening: &[usize], closing: &[usize]) -> Disagreement {
    let mut report = Disagreement::default();
    for ((&split_index, &opening_kind), &closing_kind) in
        indices.iter().zip(opening).zip(closing)
    {
        if opening_kind != closing_kind {
            report.count += 1;
            if report.listed.len() < MAX_LISTED {
                report.listed.push(KindDisagreement { split_index, opening_kind, closing_kind });
            }
        }
    }
    report
}

/// Add corresponding probabilities, then restore unit mass.
pub fn combine_probabilities(a: &[f64], b: &[f64]) -> Vec<f64> {
    let weights = (0..a.len().max(b.len()))
        .map(|i| positive(a.get(i).copied().unwrap_or(0.0)) + positive(b.get(i).copied().unwrap_or(0.0)))
        .collect();
    distribution(weights)
}

/// Add corresponding amplitudes and square through task 3.4's Circle-family
/// `twins` Operator. That Operator owns the better-and-not-worse phase move
/// and the amplitude arithmetic; this layer only normalises its scores.
pub fn combine_amplitudes(a: &[c64], b: &[c64]) -> Vec<f64> {
    let op = table()
        .into_iter()
        .find(|op| op.index == 13 && op.family == Family::Circle && op.name == "twins")
        .expect("the frozen keyboard must contain Circle operator 13 `twins`");
    let zero = c64::new(0.0, 0.0);
    let mut arena = Arena::new(0);
    let weights = (0..a.len().max(b.len()))
        .map(|i| {
            let args = [
                Operand::Phasor(a.get(i).copied().unwrap_or(zero)),
                Operand::Phasor(b.get(i).copied().unwrap_or(zero)),
            ];
            match apply(&op, &args, &mut arena).expect("Circle `twins` accepts two phasors") {
                Operand::Phasor(score) => positive(score.re),
                _ => unreachable!("Circle `twins` declares a phasor result"),
            }
        })
        .collect();
    distribution(weights)
}

fn assignment_purity(kinds: &[usize], labels: &[Function]) -> f64 {
    let mut rows = BTreeMap::<usize, Vec<f64>>::new();
    for (&kind, &label) in kinds.iter().zip(labels) {
        let row = rows.entry(kind).or_insert_with(|| vec![0.0; Function::ALL.len()]);
        let column = Function::ALL.iter().position(|&candidate| candidate == label).unwrap();
        row[column] += 1.0;
    }
    one_decimal(100.0 * purity(&rows.into_values().collect::<Vec<_>>()))
}

fn greatest(values: &[f64]) -> usize {
    values
        .iter()
        .enumerate()
        .max_by(|(ai, a), (bi, b)| a.total_cmp(b).then_with(|| bi.cmp(ai)))
        .map_or(0, |(i, _)| i)
}

fn distribution(mut weights: Vec<f64>) -> Vec<f64> {
    let total: f64 = weights.iter().sum();
    if total.is_finite() && total > 0.0 {
        weights.iter_mut().for_each(|weight| *weight /= total);
    } else if !weights.is_empty() {
        let uniform = 1.0 / weights.len() as f64;
        weights.fill(uniform);
    }
    weights
}

fn positive(value: f64) -> f64 {
    match value.is_finite() && value > 0.0 {
        true => value,
        false => 0.0,
    }
}

fn one_decimal(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

#[cfg(test)]
#[path = "twins/tests.rs"]
mod tests;
#[cfg(test)]
#[path = "twins/property.rs"]
mod property;
